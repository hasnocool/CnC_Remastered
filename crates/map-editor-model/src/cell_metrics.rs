//! Cell geometry matching the upstream map-editor CellMetrics operations.

use rts_core::{CellCoord, Facing, MapDimensions, MapError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CellMetrics {
    dimensions: MapDimensions,
}

impl CellMetrics {
    pub const fn new(width: u16, height: u16) -> Result<Self, MapError> {
        match MapDimensions::new(width, height) {
            Ok(dimensions) => Ok(Self { dimensions }),
            Err(error) => Err(error),
        }
    }

    pub const fn dimensions(self) -> MapDimensions {
        self.dimensions
    }

    pub const fn length(self) -> usize {
        self.dimensions.cell_count()
    }

    pub const fn contains(self, location: CellCoord) -> bool {
        self.dimensions.contains(location)
    }

    pub const fn get_cell(self, location: CellCoord) -> Option<usize> {
        self.dimensions.index(location)
    }

    pub const fn get_location(self, cell: usize) -> Option<CellCoord> {
        self.dimensions.coord(cell)
    }

    /// Returns the adjacent cell or None when the neighbor is outside the grid.
    ///
    /// Passing None for facing reproduces the editor's None-facing fall-through
    /// behavior, which leaves the location unchanged.
    pub fn adjacent(self, location: CellCoord, facing: Option<Facing>) -> Option<CellCoord> {
        if !self.contains(location) {
            return None;
        }

        let (dx, dy): (i32, i32) = match facing {
            None => (0, 0),
            Some(Facing::North) => (0, -1),
            Some(Facing::NorthEast) => (1, -1),
            Some(Facing::East) => (1, 0),
            Some(Facing::SouthEast) => (1, 1),
            Some(Facing::South) => (0, 1),
            Some(Facing::SouthWest) => (-1, 1),
            Some(Facing::West) => (-1, 0),
            Some(Facing::NorthWest) => (-1, -1),
        };

        let x = i32::from(location.x) + dx;
        let y = i32::from(location.y) + dy;
        if x < 0 || y < 0 {
            return None;
        }

        let adjacent = CellCoord::new(x as u16, y as u16);
        self.contains(adjacent).then_some(adjacent)
    }

    /// Clips a location to the closest cell inside the non-empty grid.
    pub fn clip(self, location: &mut CellCoord) {
        location.x = location.x.min(self.dimensions.width() - 1);
        location.y = location.y.min(self.dimensions.height() - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_cell_conversion_matches_editor_formula() {
        let metrics = CellMetrics::new(12, 8).expect("valid metrics");
        let location = CellCoord::new(5, 3);
        assert_eq!(metrics.get_cell(location), Some(41));
        assert_eq!(metrics.get_location(41), Some(location));
        assert_eq!(metrics.get_location(metrics.length()), None);
    }

    #[test]
    fn adjacent_handles_all_eight_directions_and_none() {
        let metrics = CellMetrics::new(5, 5).expect("valid metrics");
        let origin = CellCoord::new(2, 2);

        assert_eq!(metrics.adjacent(origin, None), Some(origin));
        assert_eq!(metrics.adjacent(origin, Some(Facing::North)), Some(CellCoord::new(2, 1)));
        assert_eq!(metrics.adjacent(origin, Some(Facing::NorthEast)), Some(CellCoord::new(3, 1)));
        assert_eq!(metrics.adjacent(origin, Some(Facing::East)), Some(CellCoord::new(3, 2)));
        assert_eq!(metrics.adjacent(origin, Some(Facing::SouthEast)), Some(CellCoord::new(3, 3)));
        assert_eq!(metrics.adjacent(origin, Some(Facing::South)), Some(CellCoord::new(2, 3)));
        assert_eq!(metrics.adjacent(origin, Some(Facing::SouthWest)), Some(CellCoord::new(1, 3)));
        assert_eq!(metrics.adjacent(origin, Some(Facing::West)), Some(CellCoord::new(1, 2)));
        assert_eq!(metrics.adjacent(origin, Some(Facing::NorthWest)), Some(CellCoord::new(1, 1)));
    }

    #[test]
    fn adjacency_rejects_edges_without_wrapping() {
        let metrics = CellMetrics::new(4, 4).expect("valid metrics");
        assert_eq!(metrics.adjacent(CellCoord::new(0, 0), Some(Facing::North)), None);
        assert_eq!(metrics.adjacent(CellCoord::new(3, 3), Some(Facing::SouthEast)), None);
    }

    #[test]
    fn clip_clamps_to_last_valid_cell() {
        let metrics = CellMetrics::new(4, 7).expect("valid metrics");
        let mut location = CellCoord::new(u16::MAX, 0);
        metrics.clip(&mut location);
        assert_eq!(location, CellCoord::new(3, 0));
    }
}
