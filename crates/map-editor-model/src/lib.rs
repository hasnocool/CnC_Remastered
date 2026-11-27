//! Initial portable map-editor document model.

pub mod cell_metrics;
pub mod cell_grid;
pub mod map_update;
pub mod map_spatial_index;
pub mod map_editor_state;
pub mod placed_objects;
pub mod source_catalogs;
pub mod catalog_validation;
pub mod derived_icons;
pub use derived_icons::{resource_icon, wall_icon, GEM_RESOURCE_ICONS, TIBERIUM_GOLD_RESOURCE_ICONS};
pub use cell_grid::{CellGrid, CellGridError, CellGridEvent};
pub use map_update::{MapUpdateError, MapUpdateState, MapUpdateWork, OverlayInvalidation};
pub use map_spatial_index::{MapSpatialIndex, SpatialIndexError};
pub use map_editor_state::{MapEditorState, MapEditorStateError};
pub use source_catalogs::{red_alert_building_types, red_alert_smudge_types, tiberian_dawn_building_types, tiberian_dawn_smudge_types};
pub use catalog_validation::{validate_building_catalog, validate_smudge_catalog, validate_variant_catalogs, CatalogValidationError};
pub use placed_objects::{BuildingTypeDefinition, EditorColor, OverlayTypeDefinition, OverlayTypeFlag, PlacedBuilding, PlacedOverlay, PlacedSmudge, SmudgeTypeDefinition, SmudgeTypeFlag};
pub mod occupier_set;
pub mod overlapper_set;
pub use cell_metrics::CellMetrics;
pub use occupier_set::{OccupierError, OccupierId, OccupierSet};
pub use overlapper_set::{OverlapRect, OverlapperError, OverlapperId, OverlapperSet};

use std::collections::BTreeMap;

use rts_core::{CellCoord, GameVariant, MapDimensions};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CellLayers {
    pub template: Option<u16>,
    /// Logical icon slot within the selected template type's theater ICON set.
    /// Ignored when template is None.
    pub template_icon: u8,
    pub terrain: Option<u16>,
    /// Legacy serialized overlay catalog slot, retained for format adapters.
    pub overlay: Option<u16>,
    /// Legacy serialized smudge catalog slot, retained for format adapters.
    pub smudge: Option<u16>,
    /// Typed editor instance. The numeric slot remains independent for legacy formats.
    pub overlay_instance: Option<PlacedOverlay>,
    /// Typed editor instance, preserving Icon, Data and Tint.
    pub smudge_instance: Option<PlacedSmudge>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapDocumentError {
    DimensionsExceedVariant { requested: MapDimensions, maximum: MapDimensions },
    CellOutOfBounds(CellCoord),
    WaypointOutOfBounds(CellCoord),
    DuplicateWaypoint(u16),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapDocument {
    variant: GameVariant,
    dimensions: MapDimensions,
    cells: Vec<CellLayers>,
    waypoints: BTreeMap<u16, CellCoord>,
    update_state: MapUpdateState,
}

impl MapDocument {
    pub fn new(
        variant: GameVariant,
        dimensions: MapDimensions,
    ) -> Result<Self, MapDocumentError> {
        let maximum = variant.global_dimensions();
        if dimensions.width() > maximum.width() || dimensions.height() > maximum.height() {
            return Err(MapDocumentError::DimensionsExceedVariant {
                requested: dimensions,
                maximum,
            });
        }

        Ok(Self {
            variant,
            dimensions,
            cells: vec![CellLayers::default(); dimensions.cell_count()],
            waypoints: BTreeMap::new(),
            update_state: MapUpdateState::new(dimensions),
        })
    }

    pub const fn variant(&self) -> GameVariant {
        self.variant
    }

    pub const fn dimensions(&self) -> MapDimensions {
        self.dimensions
    }

    pub fn cell(&self, coordinate: CellCoord) -> Option<&CellLayers> {
        let index = self.dimensions.index(coordinate)?;
        self.cells.get(index)
    }

    /// Iterates template references in the document's row-major cell order.
    ///
    /// The values are map-model template IDs, not necessarily TMP archive tile
    /// indices. A game/theater-specific resolver must translate them before
    /// passing them to the renderer.
    pub fn template_indices(&self) -> impl ExactSizeIterator<Item = Option<u16>> + '_ {
        self.cells.iter().map(|cell| cell.template)
    }

    /// Iterates complete template references in row-major order.
    ///
    /// TD/RA map cells carry both a template type and a logical icon index.
    /// The icon index is resolved through that type's theater-specific ICON
    /// set; it is not a TMP archive tile index.
    pub fn template_cells(&self) -> impl ExactSizeIterator<Item = Option<(u16, u8)>> + '_ {
        self.cells
            .iter()
            .map(|cell| cell.template.map(|template| (template, cell.template_icon)))
    }

    pub fn overlay_instance(&self, coordinate: CellCoord) -> Option<&PlacedOverlay> {
        self.cell(coordinate)?.overlay_instance.as_ref()
    }

    pub fn overlay_instance_mut(&mut self, coordinate: CellCoord) -> Option<&mut PlacedOverlay> {
        let index = self.dimensions.index(coordinate)?;
        self.cells.get_mut(index)?.overlay_instance.as_mut()
    }

    pub fn smudge_instance(&self, coordinate: CellCoord) -> Option<&PlacedSmudge> {
        self.cell(coordinate)?.smudge_instance.as_ref()
    }

    pub fn smudge_instance_mut(&mut self, coordinate: CellCoord) -> Option<&mut PlacedSmudge> {
        let index = self.dimensions.index(coordinate)?;
        self.cells.get_mut(index)?.smudge_instance.as_mut()
    }

    pub fn set_overlay_instance(
        &mut self,
        coordinate: CellCoord,
        overlay: Option<PlacedOverlay>,
    ) -> Result<Option<PlacedOverlay>, MapDocumentError> {
        let index = self.dimensions.index(coordinate)
            .ok_or(MapDocumentError::CellOutOfBounds(coordinate))?;
        Ok(std::mem::replace(&mut self.cells[index].overlay_instance, overlay))
    }

    pub fn set_smudge_instance(
        &mut self,
        coordinate: CellCoord,
        smudge: Option<PlacedSmudge>,
    ) -> Result<Option<PlacedSmudge>, MapDocumentError> {
        let index = self.dimensions.index(coordinate)
            .ok_or(MapDocumentError::CellOutOfBounds(coordinate))?;
        Ok(std::mem::replace(&mut self.cells[index].smudge_instance, smudge))
    }

    pub fn set_cell(
        &mut self,
        coordinate: CellCoord,
        layers: CellLayers,
    ) -> Result<(), MapDocumentError> {
        let index = self
            .dimensions
            .index(coordinate)
            .ok_or(MapDocumentError::CellOutOfBounds(coordinate))?;
        self.cells[index] = layers;
        Ok(())
    }

    pub fn add_waypoint(
        &mut self,
        number: u16,
        coordinate: CellCoord,
    ) -> Result<(), MapDocumentError> {
        if !self.dimensions.contains(coordinate) {
            return Err(MapDocumentError::WaypointOutOfBounds(coordinate));
        }
        if self.waypoints.contains_key(&number) {
            return Err(MapDocumentError::DuplicateWaypoint(number));
        }
        self.waypoints.insert(number, coordinate);
        Ok(())
    }

    pub fn waypoint(&self, number: u16) -> Option<CellCoord> {
        self.waypoints.get(&number).copied()
    }

    pub fn waypoints(&self) -> &BTreeMap<u16, CellCoord> {
        &self.waypoints
    }

    /// Begins a nestable map update batch. Derived work is deferred until the
    /// matching outermost end_update call.
    pub fn begin_update(&mut self) {
        self.update_state.begin_update();
    }

    pub fn end_update(&mut self) -> Result<Option<MapUpdateWork>, MapUpdateError> {
        self.update_state.end_update()
    }

    pub fn invalidate_overlay(
        &mut self,
        coordinate: CellCoord,
        kind: OverlayInvalidation,
    ) -> Result<Option<MapUpdateWork>, MapUpdateError> {
        self.update_state.invalidate_overlay(coordinate, kind)
    }

    pub fn request_overlapper_rebuild(&mut self) -> Option<MapUpdateWork> {
        self.update_state.request_overlapper_rebuild()
    }

    pub fn update_state(&self) -> &MapUpdateState {
        &self.update_state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_indices_preserve_row_major_cell_order() {
        let dimensions = MapDimensions::new(3, 1).expect("valid dimensions");
        let mut map = MapDocument::new(GameVariant::TiberianDawn, dimensions).expect("valid map");
        map.set_cell(CellCoord::new(0, 0), CellLayers { template: Some(7), ..CellLayers::default() }).unwrap();
        map.set_cell(CellCoord::new(2, 0), CellLayers { template: Some(3), ..CellLayers::default() }).unwrap();

        assert_eq!(map.template_indices().collect::<Vec<_>>(), vec![Some(7), None, Some(3)]);
    }

    #[test]
    fn template_cells_preserve_type_and_logical_icon_row_major() {
        let dimensions = MapDimensions::new(3, 1).expect("valid dimensions");
        let mut map = MapDocument::new(GameVariant::TiberianDawn, dimensions).expect("valid map");
        map.set_cell(
            CellCoord::new(0, 0),
            CellLayers { template: Some(7), template_icon: 3, ..CellLayers::default() },
        ).unwrap();
        map.set_cell(
            CellCoord::new(2, 0),
            CellLayers { template: Some(3), template_icon: 11, ..CellLayers::default() },
        ).unwrap();

        assert_eq!(
            map.template_cells().collect::<Vec<_>>(),
            vec![Some((7, 3)), None, Some((3, 11))],
        );
    }

    #[test]
    fn typed_overlay_and_smudge_records_round_trip_with_cell_layers() {
        let dimensions = MapDimensions::new(2, 1).unwrap();
        let mut map = MapDocument::new(GameVariant::RedAlert, dimensions).unwrap();
        let cell = CellCoord::new(1, 0);
        let overlay = PlacedOverlay { type_id: 7, icon: 3, tint: EditorColor { red: 1, green: 2, blue: 3, alpha: 4 } };
        let smudge = PlacedSmudge { type_id: 9, icon: 5, data: 77, tint: EditorColor::WHITE };
        map.set_overlay_instance(cell, Some(overlay.clone())).unwrap();
        map.set_smudge_instance(cell, Some(smudge.clone())).unwrap();
        assert_eq!(map.overlay_instance(cell), Some(&overlay));
        assert_eq!(map.smudge_instance(cell), Some(&smudge));
        assert_eq!(map.cell(cell).unwrap().overlay, None);
        assert_eq!(map.cell(cell).unwrap().smudge, None);
    }

    #[test]
    fn stores_and_reads_cell_layers() {
        let dimensions = MapDimensions::new(10, 10).expect("valid dimensions");
        let mut map = MapDocument::new(GameVariant::TiberianDawn, dimensions).expect("valid map");
        let coordinate = CellCoord::new(2, 3);
        let layers = CellLayers {
            template: Some(12),
            terrain: Some(7),
            overlay: Some(9),
            smudge: None,
        };
        map.set_cell(coordinate, layers).expect("cell is in bounds");
        assert_eq!(map.cell(coordinate), Some(&layers));
    }

    #[test]
    fn rejects_cells_outside_map() {
        let dimensions = MapDimensions::new(10, 10).expect("valid dimensions");
        let mut map = MapDocument::new(GameVariant::TiberianDawn, dimensions).expect("valid map");
        let coordinate = CellCoord::new(10, 0);
        assert_eq!(
            map.set_cell(coordinate, CellLayers::default()),
            Err(MapDocumentError::CellOutOfBounds(coordinate))
        );
    }

    #[test]
    fn enforces_variant_global_extent() {
        let dimensions = MapDimensions::new(65, 64).expect("valid dimensions");
        assert!(matches!(
            MapDocument::new(GameVariant::TiberianDawn, dimensions),
            Err(MapDocumentError::DimensionsExceedVariant { .. })
        ));
        assert!(MapDocument::new(GameVariant::RedAlert, dimensions).is_ok());
    }

    #[test]
    fn map_document_batches_overlay_invalidation_and_overlap_rebuilds() {
        let dimensions = MapDimensions::new(8, 8).expect("valid dimensions");
        let mut map = MapDocument::new(GameVariant::RedAlert, dimensions).expect("valid map");
        map.begin_update();
        map.begin_update();
        assert_eq!(
            map.invalidate_overlay(CellCoord::new(3, 3), OverlayInvalidation::Walls).unwrap(),
            None,
        );
        assert_eq!(map.request_overlapper_rebuild(), None);
        assert_eq!(map.end_update().unwrap(), None);
        let work = map.end_update().unwrap().unwrap();
        assert_eq!(work.walls.len(), 9);
        assert!(work.rebuild_overlappers);
        assert_eq!(map.update_state().batch_depth(), 0);
    }

    #[test]
    fn waypoint_numbers_are_unique() {
        let dimensions = MapDimensions::new(10, 10).expect("valid dimensions");
        let mut map = MapDocument::new(GameVariant::RedAlert, dimensions).expect("valid map");
        let coordinate = CellCoord::new(1, 1);
        map.add_waypoint(3, coordinate).expect("first waypoint");
        assert_eq!(
            map.add_waypoint(3, CellCoord::new(2, 2)),
            Err(MapDocumentError::DuplicateWaypoint(3))
        );
        assert_eq!(map.waypoint(3), Some(coordinate));
    }
}
