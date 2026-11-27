//! Validated mask-based terrain placement for map editing.
//!
//! This module models occupancy only. Render size and overlap bounds are
//! separate source concepts and belong in renderer-facing metadata adapters.
//! The mask uses explicit row-major storage: index = y * width + x.

use crate::{MapDocument, MapDocumentError, MapEditHistory};
use rts_core::CellCoord;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OccupancyMaskError {
    ZeroWidth,
    ZeroHeight,
    IncorrectCellCount { expected: usize, actual: usize },
    EmptyMask,
}

/// Immutable logical occupancy footprint, stored in row-major order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OccupancyMask {
    width: u16,
    height: u16,
    occupied: Vec<bool>,
}

impl OccupancyMask {
    pub fn new(width: u16, height: u16, occupied: Vec<bool>) -> Result<Self, OccupancyMaskError> {
        if width == 0 {
            return Err(OccupancyMaskError::ZeroWidth);
        }
        if height == 0 {
            return Err(OccupancyMaskError::ZeroHeight);
        }
        let expected = usize::from(width) * usize::from(height);
        if occupied.len() != expected {
            return Err(OccupancyMaskError::IncorrectCellCount {
                expected,
                actual: occupied.len(),
            });
        }
        if !occupied.iter().any(|cell| *cell) {
            return Err(OccupancyMaskError::EmptyMask);
        }
        Ok(Self { width, height, occupied })
    }

    pub const fn width(&self) -> u16 { self.width }
    pub const fn height(&self) -> u16 { self.height }

    pub fn is_occupied(&self, x: u16, y: u16) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        self.occupied[usize::from(y) * usize::from(self.width) + usize::from(x)]
    }

    pub fn occupied_cells(&self, origin: CellCoord) -> impl Iterator<Item = CellCoord> + '_ {
        self.occupied.iter().enumerate().filter_map(move |(index, occupied)| {
            if !occupied {
                return None;
            }
            let x = (index % usize::from(self.width)) as u16;
            let y = (index / usize::from(self.width)) as u16;
            let x = origin.x.checked_add(x)?;
            let y = origin.y.checked_add(y)?;
            Some(CellCoord::new(x, y))
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerrainPlacementDefinition {
    pub id: u16,
    pub occupancy: OccupancyMask,
    /// Source-derived transformable behavior is represented as metadata only.
    /// It does not alter occupancy or claim that the retail terrain catalog
    /// has been completely imported.
    pub transformable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerrainPlacementError {
    OutOfBounds(CellCoord),
    Occupied(CellCoord),
    MissingPlacement(CellCoord),
    History(MapDocumentError),
}

/// Places every occupied mask cell atomically. Empty mask cells are untouched.
/// Existing terrain is never overwritten implicitly.
pub fn place_terrain(
    document: &mut MapDocument,
    origin: CellCoord,
    definition: &TerrainPlacementDefinition,
) -> Result<Vec<CellCoord>, TerrainPlacementError> {
    let cells: Vec<_> = definition.occupancy.occupied_cells(origin).collect();
    // checked_add overflow drops a coordinate; detect it by comparing counts.
    let expected = definition.occupancy.occupied.iter().filter(|cell| **cell).count();
    if cells.len() != expected {
        return Err(TerrainPlacementError::OutOfBounds(origin));
    }

    for coordinate in &cells {
        let Some(layers) = document.cell(*coordinate) else {
            return Err(TerrainPlacementError::OutOfBounds(*coordinate));
        };
        if layers.terrain.is_some() {
            return Err(TerrainPlacementError::Occupied(*coordinate));
        }
    }

    // All validation completes before the first mutation.
    for coordinate in &cells {
        let mut layers = *document.cell(*coordinate).expect("validated terrain cell");
        layers.terrain = Some(definition.id);
        layers.terrain_icon = if definition.transformable { 22 } else { 0 };
        document.set_cell(*coordinate, layers).expect("validated terrain cell");
    }
    Ok(cells)
}

/// Removes only cells belonging to the requested definition footprint.
/// The operation validates the complete footprint before mutating anything.
pub fn remove_terrain(
    document: &mut MapDocument,
    origin: CellCoord,
    definition: &TerrainPlacementDefinition,
) -> Result<Vec<CellCoord>, TerrainPlacementError> {
    let cells: Vec<_> = definition.occupancy.occupied_cells(origin).collect();
    let expected = definition.occupancy.occupied.iter().filter(|cell| **cell).count();
    if cells.len() != expected {
        return Err(TerrainPlacementError::OutOfBounds(origin));
    }

    for coordinate in &cells {
        let Some(layers) = document.cell(*coordinate) else {
            return Err(TerrainPlacementError::OutOfBounds(*coordinate));
        };
        if layers.terrain != Some(definition.id) {
            return Err(TerrainPlacementError::MissingPlacement(*coordinate));
        }
    }

    for coordinate in &cells {
        let mut layers = *document.cell(*coordinate).expect("validated terrain cell");
        layers.terrain = None;
        layers.terrain_icon = 0;
        document.set_cell(*coordinate, layers).expect("validated terrain cell");
    }
    Ok(cells)
}



/// Creates the validated cell replacements for a complete terrain footprint.
fn terrain_history_changes(
    document: &MapDocument,
    origin: CellCoord,
    definition: &TerrainPlacementDefinition,
    placing: bool,
) -> Result<Vec<(CellCoord, crate::CellLayers)>, TerrainPlacementError> {
    let cells: Vec<_> = definition.occupancy.occupied_cells(origin).collect();
    let expected = definition.occupancy.occupied.iter().filter(|cell| **cell).count();
    if cells.len() != expected {
        return Err(TerrainPlacementError::OutOfBounds(origin));
    }
    let mut changes = Vec::with_capacity(cells.len());
    for coordinate in cells {
        let mut layers = *document.cell(coordinate)
            .ok_or(TerrainPlacementError::OutOfBounds(coordinate))?;
        if placing {
            if layers.terrain.is_some() { return Err(TerrainPlacementError::Occupied(coordinate)); }
            layers.terrain = Some(definition.id);
            layers.terrain_icon = if definition.transformable { 22 } else { 0 };
        } else {
            if layers.terrain != Some(definition.id) {
                return Err(TerrainPlacementError::MissingPlacement(coordinate));
            }
            layers.terrain = None;
            layers.terrain_icon = 0;
        }
        changes.push((coordinate, layers));
    }
    Ok(changes)
}

/// Places an entire terrain footprint as one undoable history command.
pub fn place_terrain_with_history(
    document: &mut MapDocument,
    history: &mut MapEditHistory,
    origin: CellCoord,
    definition: &TerrainPlacementDefinition,
) -> Result<Vec<CellCoord>, TerrainPlacementError> {
    let changes = terrain_history_changes(document, origin, definition, true)?;
    let coordinates = changes.iter().map(|(coordinate, _)| *coordinate).collect();
    history.apply_cells(document, &changes).map_err(TerrainPlacementError::History)?;
    Ok(coordinates)
}

/// Removes an entire terrain footprint as one undoable history command.
pub fn remove_terrain_with_history(
    document: &mut MapDocument,
    history: &mut MapEditHistory,
    origin: CellCoord,
    definition: &TerrainPlacementDefinition,
) -> Result<Vec<CellCoord>, TerrainPlacementError> {
    let changes = terrain_history_changes(document, origin, definition, false)?;
    let coordinates = changes.iter().map(|(coordinate, _)| *coordinate).collect();
    history.apply_cells(document, &changes).map_err(TerrainPlacementError::History)?;
    Ok(coordinates)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CellLayers;
    use rts_core::{GameVariant, MapDimensions};

    fn document() -> MapDocument {
        MapDocument::new(
            GameVariant::TiberianDawn,
            MapDimensions::new(5, 4).expect("valid dimensions"),
        ).expect("valid document")
    }

    fn definition() -> TerrainPlacementDefinition {
        TerrainPlacementDefinition {
            id: 7,
            occupancy: OccupancyMask::new(2, 2, vec![true, false, true, true]).unwrap(),
            transformable: false,
        }
    }


    #[test]
    fn history_backed_placement_and_removal_are_single_undo_steps() {
        let mut map = document();
        let mut history = crate::MapEditHistory::default();
        let origin = CellCoord::new(1, 1);
        let definition = definition();
        let overlay = CellLayers { overlay: Some(22), ..CellLayers::default() };
        map.set_cell(CellCoord::new(1, 1), overlay).unwrap();

        place_terrain_with_history(&mut map, &mut history, origin, &definition).unwrap();
        assert_eq!(history.undo_depth(), 1);
        assert_eq!(map.cell(CellCoord::new(1, 1)).unwrap().terrain, Some(7));
        assert_eq!(map.cell(CellCoord::new(1, 1)).unwrap().overlay, Some(22));
        history.undo(&mut map).unwrap();
        assert!(map.cells().iter().all(|cell| cell.terrain.is_none()));
        assert_eq!(map.cell(CellCoord::new(1, 1)).unwrap().overlay, Some(22));
        history.redo(&mut map).unwrap();
        assert_eq!(map.cell(CellCoord::new(2, 2)).unwrap().terrain, Some(7));

        remove_terrain_with_history(&mut map, &mut history, origin, &definition).unwrap();
        assert_eq!(history.undo_depth(), 2);
        history.undo(&mut map).unwrap();
        assert_eq!(map.cell(CellCoord::new(2, 2)).unwrap().terrain, Some(7));
        history.redo(&mut map).unwrap();
        assert!(map.cells().iter().all(|cell| cell.terrain.is_none()));
    }

    #[test]
    fn history_backed_placement_rejects_footprint_without_history_or_partial_write() {
        let mut map = document();
        let mut history = crate::MapEditHistory::default();
        map.set_cell(CellCoord::new(1, 2), CellLayers { terrain: Some(99), ..CellLayers::default() }).unwrap();
        assert!(matches!(
            place_terrain_with_history(&mut map, &mut history, CellCoord::new(1, 1), &definition()),
            Err(TerrainPlacementError::Occupied(_))
        ));
        assert_eq!(history.undo_depth(), 0);
        assert!(map.cells().iter().filter(|cell| cell.terrain == Some(7)).count() == 0);
    }

    #[test]
    fn mask_uses_explicit_row_major_coordinates() {
        let mask = definition().occupancy;
        assert_eq!(
            mask.occupied_cells(CellCoord::new(1, 1)).collect::<Vec<_>>(),
            vec![CellCoord::new(1, 1), CellCoord::new(1, 2), CellCoord::new(2, 2)]
        );
    }

    #[test]
    fn placement_and_removal_preserve_other_layers() {
        let mut map = document();
        let at = CellCoord::new(1, 1);
        let mut existing = CellLayers::default();
        existing.overlay = Some(12);
        map.set_cell(CellCoord::new(1, 1), existing).unwrap();

        let placed = place_terrain(&mut map, at, &definition()).unwrap();
        assert_eq!(placed.len(), 3);
        assert_eq!(map.cell(CellCoord::new(1, 1)).unwrap().terrain, Some(7));
        assert_eq!(map.cell(CellCoord::new(1, 1)).unwrap().overlay, Some(12));

        let removed = remove_terrain(&mut map, at, &definition()).unwrap();
        assert_eq!(removed, placed);
        assert_eq!(map.cell(CellCoord::new(1, 1)).unwrap().terrain, None);
        assert_eq!(map.cell(CellCoord::new(1, 1)).unwrap().overlay, Some(12));
    }

    #[test]
    fn placement_rejects_overlap_without_partial_mutation() {
        let mut map = document();
        let blocker = CellLayers { terrain: Some(99), ..CellLayers::default() };
        map.set_cell(CellCoord::new(1, 2), blocker).unwrap();

        assert_eq!(
            place_terrain(&mut map, CellCoord::new(1, 1), &definition()),
            Err(TerrainPlacementError::Occupied(CellCoord::new(1, 2)))
        );
        assert_eq!(map.cell(CellCoord::new(1, 1)).unwrap().terrain, None);
        assert_eq!(map.cell(CellCoord::new(2, 2)).unwrap().terrain, None);
    }

    #[test]
    fn placement_rejects_edge_overflow_without_partial_mutation() {
        let mut map = document();
        assert!(matches!(
            place_terrain(&mut map, CellCoord::new(4, 3), &definition()),
            Err(TerrainPlacementError::OutOfBounds(_))
        ));
        assert!(map.cells().iter().all(|cell| cell.terrain.is_none()));
    }

    #[test]
    fn removal_requires_complete_matching_footprint() {
        let mut map = document();
        map.set_cell(
            CellCoord::new(1, 1),
            CellLayers { terrain: Some(7), ..CellLayers::default() },
        ).unwrap();

        assert_eq!(
            remove_terrain(&mut map, CellCoord::new(1, 1), &definition()),
            Err(TerrainPlacementError::MissingPlacement(CellCoord::new(1, 2)))
        );
        assert_eq!(map.cell(CellCoord::new(1, 1)).unwrap().terrain, Some(7));
    }

    #[test]
    fn invalid_masks_are_rejected() {
        assert_eq!(
            OccupancyMask::new(2, 2, vec![true]),
            Err(OccupancyMaskError::IncorrectCellCount { expected: 4, actual: 1 })
        );
        assert_eq!(
            OccupancyMask::new(2, 2, vec![false; 4]),
            Err(OccupancyMaskError::EmptyMask)
        );
    }
}
