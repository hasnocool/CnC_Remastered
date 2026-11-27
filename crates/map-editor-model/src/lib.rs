//! Initial portable map-editor document model.

pub mod cell_metrics;
pub mod edit_history;
pub mod overlay_catalog;
pub mod template_catalog;
pub mod terrain_placement;
pub mod terrain_catalog;
pub mod terrain_objects;
pub use terrain_placement::{
    place_terrain, place_terrain_with_history, remove_terrain, remove_terrain_with_history,
    OccupancyMask, OccupancyMaskError,
    TerrainPlacementDefinition, TerrainPlacementError,
};
pub use template_catalog::TemplateCatalog;
pub use terrain_catalog::{TerrainCatalog, TerrainDefinition, TerrainTemplateFlag};
pub use edit_history::MapEditHistory;
pub use overlay_catalog::{
    place_overlay_with_history, remove_overlay_with_history, OverlayCatalog,
    OverlayDefinition, OverlayPlacementError, OverlayTypeFlags,
};
pub use cell_metrics::CellMetrics;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use rts_core::{CellCoord, GameVariant, MapDimensions};
pub use terrain_objects::{TerrainObject, TerrainObjectError, TerrainObjectId};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CellLayers {
    pub template: Option<u16>,
    /// Logical icon slot within the selected template type's theater ICON set.
    /// Ignored when template is None.
    pub template_icon: u8,
    pub terrain: Option<u16>,
    /// Terrain shape index: 22 for transformable TerrainTypes, otherwise 0.
    #[serde(default)]
    pub terrain_icon: u8,
    pub overlay: Option<u16>,
    pub smudge: Option<u16>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapDocumentError {
    DimensionsExceedVariant { requested: MapDimensions, maximum: MapDimensions },
    CellOutOfBounds(CellCoord),
    WaypointOutOfBounds(CellCoord),
    DuplicateWaypoint(u16),
    DuplicateCellEdit(CellCoord),
    CellStateConflict(CellCoord),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapDocument {
    variant: GameVariant,
    dimensions: MapDimensions,
    cells: Vec<CellLayers>,
    waypoints: BTreeMap<u16, CellCoord>,
    terrain_objects: BTreeMap<TerrainObjectId, TerrainObject>,
    terrain_occupancy: Vec<Option<TerrainObjectId>>,
    next_terrain_object_id: u64,
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
            terrain_objects: BTreeMap::new(),
            terrain_occupancy: vec![None; dimensions.cell_count()],
            next_terrain_object_id: 1,
        })
    }

    pub const fn variant(&self) -> GameVariant {
        self.variant
    }

    pub const fn dimensions(&self) -> MapDimensions {
        self.dimensions
    }

    /// Read-only row-major cell storage for renderers and editor serialization.
    pub fn cells(&self) -> &[CellLayers] {
        &self.cells
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
