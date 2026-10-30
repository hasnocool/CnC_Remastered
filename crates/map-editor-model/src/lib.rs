//! Initial portable map-editor document model.

use std::collections::BTreeMap;

use rts_core::{CellCoord, GameVariant, MapDimensions};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CellLayers {
    pub template: Option<u16>,
    pub terrain: Option<u16>,
    pub overlay: Option<u16>,
    pub smudge: Option<u16>,
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
