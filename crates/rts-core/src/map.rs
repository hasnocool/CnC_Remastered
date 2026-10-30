/// Region dimensions from the upstream map implementation.
pub const REGION_WIDTH: u16 = 4;
pub const REGION_HEIGHT: u16 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameVariant {
    TiberianDawn,
    RedAlert,
}

impl GameVariant {
    /// Global map array dimensions encoded by the respective upstream DEFINES.H.
    pub const fn global_dimensions(self) -> MapDimensions {
        match self {
            Self::TiberianDawn => MapDimensions { width: 64, height: 64 },
            Self::RedAlert => MapDimensions { width: 128, height: 128 },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellCoord {
    pub x: u16,
    pub y: u16,
}

impl CellCoord {
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MapDimensions {
    width: u16,
    height: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapError {
    ZeroWidth,
    ZeroHeight,
}

impl MapDimensions {
    pub const fn new(width: u16, height: u16) -> Result<Self, MapError> {
        if width == 0 {
            return Err(MapError::ZeroWidth);
        }
        if height == 0 {
            return Err(MapError::ZeroHeight);
        }
        Ok(Self { width, height })
    }

    pub const fn width(self) -> u16 {
        self.width
    }

    pub const fn height(self) -> u16 {
        self.height
    }

    pub const fn cell_count(self) -> usize {
        self.width as usize * self.height as usize
    }

    pub const fn contains(self, cell: CellCoord) -> bool {
        cell.x < self.width && cell.y < self.height
    }

    /// Row-major cell index, matching the engine's packed global cell grid.
    pub const fn index(self, cell: CellCoord) -> Option<usize> {
        if self.contains(cell) {
            Some(cell.y as usize * self.width as usize + cell.x as usize)
        } else {
            None
        }
    }

    pub const fn coord(self, index: usize) -> Option<CellCoord> {
        if index >= self.cell_count() {
            return None;
        }
        Some(CellCoord {
            x: (index % self.width as usize) as u16,
            y: (index / self.width as usize) as u16,
        })
    }

    /// Region-grid dimensions include the two-region safety border used upstream.
    pub const fn region_grid_width(self) -> u16 {
        (((self.width as u32 + REGION_WIDTH as u32 - 1) / REGION_WIDTH as u32) + 2) as u16
    }

    pub const fn region_grid_height(self) -> u16 {
        (((self.height as u32 + REGION_HEIGHT as u32 - 1) / REGION_HEIGHT as u32) + 2) as u16
    }

    /// Region index includes a one-region border on the left and top.
    pub const fn region_index(self, cell: CellCoord) -> Option<usize> {
        if !self.contains(cell) {
            return None;
        }
        let region_x = cell.x / REGION_WIDTH + 1;
        let region_y = cell.y / REGION_HEIGHT + 1;
        Some(region_y as usize * self.region_grid_width() as usize + region_x as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_global_dimensions_match_upstream_defines() {
        let td = GameVariant::TiberianDawn.global_dimensions();
        let ra = GameVariant::RedAlert.global_dimensions();
        assert_eq!((td.width(), td.height()), (64, 64));
        assert_eq!((ra.width(), ra.height()), (128, 128));
        assert_eq!(td.cell_count(), 4_096);
        assert_eq!(ra.cell_count(), 16_384);
    }

    #[test]
    fn cell_indices_round_trip_and_reject_out_of_bounds() {
        let dimensions = MapDimensions::new(64, 64).expect("valid dimensions");
        let cell = CellCoord::new(7, 11);
        let index = dimensions.index(cell).expect("cell is in bounds");
        assert_eq!(dimensions.coord(index), Some(cell));
        assert_eq!(dimensions.index(CellCoord::new(64, 0)), None);
        assert_eq!(dimensions.coord(dimensions.cell_count()), None);
    }

    #[test]
    fn region_grid_matches_upstream_border_formula() {
        let td = GameVariant::TiberianDawn.global_dimensions();
        let ra = GameVariant::RedAlert.global_dimensions();
        assert_eq!((td.region_grid_width(), td.region_grid_height()), (18, 18));
        assert_eq!((ra.region_grid_width(), ra.region_grid_height()), (34, 34));
        assert!(td.region_index(CellCoord::new(0, 0)).is_some());
        assert!(td.region_index(CellCoord::new(64, 0)).is_none());
    }

    #[test]
    fn zero_dimensions_are_rejected() {
        assert_eq!(MapDimensions::new(0, 64), Err(MapError::ZeroWidth));
        assert_eq!(MapDimensions::new(64, 0), Err(MapError::ZeroHeight));
    }
}
