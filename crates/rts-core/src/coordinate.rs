//! Packed RTS coordinate arithmetic ported from both COORD.CPP variants.
//!
//! COORDINATE stores X in the low 16 bits and Y in the high 16 bits. Each
//! component is a signed 16-bit lepton coordinate; a map cell is 256 leptons.
//! Movement uses the exact 256-byte lookup tables from REDALERT/COORD.CPP and
//! the original signed 16-bit multiply/shift convention (product >> 7).

use crate::facing::DirType;
use crate::map::{CellCoord, GameVariant};

pub const COS_TABLE: [i8; 256] = [
    0, 3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36, 39, 42, 45,
    48, 51, 54, 57, 59, 62, 65, 67, 70, 73, 75, 78, 80, 82, 85, 87,
    89, 91, 94, 96, 98, 100, 101, 103, 105, 107, 108, 110, 111, 113, 114, 116,
    117, 118, 119, 120, 121, 122, 123, 123, 124, 125, 125, 126, 126, 126, 126, 126,
    127, 126, 126, 126, 126, 126, 125, 125, 124, 123, 123, 122, 121, 120, 119, 118,
    117, 116, 114, 113, 112, 110, 108, 107, 105, 103, 102, 100, 98, 96, 94, 91,
    89, 87, 85, 82, 80, 78, 75, 73, 70, 67, 65, 62, 59, 57, 54, 51,
    48, 45, 42, 39, 36, 33, 30, 27, 24, 21, 18, 15, 12, 9, 6, 3,
    0, -3, -6, -9, -12, -15, -18, -21, -24, -27, -30, -33, -36, -39, -42, -45,
    -48, -51, -54, -57, -59, -62, -65, -67, -70, -73, -75, -78, -80, -82, -85, -87,
    -89, -91, -94, -96, -98, -100, -102, -103, -105, -107, -108, -110, -111, -113, -114, -116,
    -117, -118, -119, -120, -121, -122, -123, -123, -124, -125, -125, -126, -126, -126, -126, -126,
    -126, -126, -126, -126, -126, -126, -125, -125, -124, -123, -123, -122, -121, -120, -119, -118,
    -117, -116, -114, -113, -112, -110, -108, -107, -105, -103, -102, -100, -98, -96, -94, -91,
    -89, -87, -85, -82, -80, -78, -75, -73, -70, -67, -65, -62, -59, -57, -54, -51,
    -48, -45, -42, -39, -36, -33, -30, -27, -24, -21, -18, -15, -12, -9, -6, -3,
];

pub const SIN_TABLE: [i8; 256] = [
    127, 126, 126, 126, 126, 126, 125, 125, 124, 123, 123, 122, 121, 120, 119, 118,
    117, 116, 114, 113, 112, 110, 108, 107, 105, 103, 102, 100, 98, 96, 94, 91,
    89, 87, 85, 82, 80, 78, 75, 73, 70, 67, 65, 62, 59, 57, 54, 51,
    48, 45, 42, 39, 36, 33, 30, 27, 24, 21, 18, 15, 12, 9, 6, 3,
    0, -3, -6, -9, -12, -15, -18, -21, -24, -27, -30, -33, -36, -39, -42, -45,
    -48, -51, -54, -57, -59, -62, -65, -67, -70, -73, -75, -78, -80, -82, -85, -87,
    -89, -91, -94, -96, -98, -100, -102, -103, -105, -107, -108, -110, -111, -113, -114, -116,
    -117, -118, -119, -120, -121, -122, -123, -123, -124, -125, -125, -126, -126, -126, -126, -126,
    -126, -126, -126, -126, -126, -126, -125, -125, -124, -123, -123, -122, -121, -120, -119, -118,
    -117, -116, -114, -113, -112, -110, -108, -107, -105, -103, -102, -100, -98, -96, -94, -91,
    -89, -87, -85, -82, -80, -78, -75, -73, -70, -67, -65, -62, -59, -57, -54, -51,
    -48, -45, -42, -39, -36, -33, -30, -27, -24, -21, -18, -15, -12, -9, -6, -3,
    0, 3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36, 39, 42, 45,
    48, 51, 54, 57, 59, 62, 65, 67, 70, 73, 75, 78, 80, 82, 85, 87,
    89, 91, 94, 96, 98, 100, 101, 103, 105, 107, 108, 110, 111, 113, 114, 116,
    117, 118, 119, 120, 121, 122, 123, 123, 124, 125, 125, 126, 126, 126, 126, 126,
];

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Coordinate(pub u32);

impl Coordinate {
    pub const ZERO: Self = Self(0);
    pub const CELL_CENTER_LEPTON: u16 = 0x0080;
    pub const LEPTONS_PER_CELL: i32 = 0x0100;

    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    pub const fn from_leptons(x: i16, y: i16) -> Self {
        Self((x as u16 as u32) | ((y as u16 as u32) << 16))
    }

    pub const fn from_cell_center(cell: CellCoord) -> Self {
        Self::from_leptons(
            ((cell.x as u32 * 0x100) + 0x80) as u16 as i16,
            ((cell.y as u32 * 0x100) + 0x80) as u16 as i16,
        )
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    pub const fn x(self) -> i16 {
        self.0 as u16 as i16
    }

    pub const fn y(self) -> i16 {
        (self.0 >> 16) as u16 as i16
    }

    pub const fn with_x(self, x: i16) -> Self {
        Self::from_leptons(x, self.y())
    }

    pub const fn with_y(self, y: i16) -> Self {
        Self::from_leptons(self.x(), y)
    }

    /// Source Coord_Snap: round each lepton coordinate down to the cell
    /// address and place it at the center (fraction 0x80).
    pub const fn snap(self) -> Self {
        let x = (self.x() as u16 & 0xFF00) | Self::CELL_CENTER_LEPTON;
        let y = (self.y() as u16 & 0xFF00) | Self::CELL_CENTER_LEPTON;
        Self::from_leptons(x as i16, y as i16)
    }

    /// Source Cell_Coord: map cell index to the center coordinate.
    pub const fn cell_center(cell: u16, map_width: u16) -> Self {
        let x = cell % map_width;
        let y = cell / map_width;
        Self::from_cell_center(CellCoord::new(x, y))
    }

    /// Extract the global-map cell using the variant's map width.
    pub const fn cell(self, variant: GameVariant) -> Option<CellCoord> {
        let dimensions = variant.global_dimensions();
        let x = ((self.x() as u16) >> 8) & (dimensions.width() - 1);
        let y = ((self.y() as u16) >> 8) & (dimensions.height() - 1);
        let cell = CellCoord::new(x, y);
        if dimensions.contains(cell) { Some(cell) } else { None }
    }

    /// Source Coord_Move / Move_Point. Both games use the same signed tables.
    pub fn moved(self, direction: DirType, distance: u16) -> Self {
        let angle = direction.0 as usize;
        let signed_distance = distance as i16;
        let dx = ((i32::from(COS_TABLE[angle]) * i32::from(signed_distance)) >> 7) as i16;
        let dy = -(((i32::from(SIN_TABLE[angle]) * i32::from(signed_distance)) >> 7) as i16);
        Self::from_leptons(self.x().wrapping_add(dx), self.y().wrapping_add(dy))
    }

    /// Source Coord_Mid: component-wise unsigned midpoint.
    pub const fn midpoint(self, other: Self) -> Self {
        let x = ((self.x() as u16 as u32 + other.x() as u16 as u32) >> 1) as u16 as i16;
        let y = ((self.y() as u16 as u32 + other.y() as u16 as u32) >> 1) as u16 as i16;
        Self::from_leptons(x, y)
    }

    /// Source Dragon-Strike distance: max(dx,dy) + min(dx,dy)/2.
    pub fn distance(self, other: Self) -> u32 {
        let dx = (i32::from(self.x()) - i32::from(other.x())).unsigned_abs();
        let dy = (i32::from(self.y()) - i32::from(other.y())).unsigned_abs();
        let (major, minor) = if dx > dy { (dx, dy) } else { (dy, dx) };
        major + minor / 2
    }

    /// Convert a randomized nearby coordinate to a cell center when lock=true.
    pub fn scatter(self, direction: DirType, distance: u16, lock: bool, variant: GameVariant) -> Self {
        let candidate = self.moved(direction, distance);
        let candidate = if candidate.x() < 0 || candidate.y() < 0 {
            self
        } else {
            candidate
        };
        if lock {
            candidate.snap()
        } else {
            let dimensions = variant.global_dimensions();
            let max_x = (dimensions.width() as i32) * Self::LEPTONS_PER_CELL;
            let max_y = (dimensions.height() as i32) * Self::LEPTONS_PER_CELL;
            if i32::from(candidate.x()) >= max_x || i32::from(candidate.y()) >= max_y {
                self
            } else {
                candidate
            }
        }
    }
}

/// Source Move_Point, exposed for callers that store signed component values.
pub fn move_point(x: &mut i16, y: &mut i16, direction: DirType, distance: u16) {
    let moved = Coordinate::from_leptons(*x, *y).moved(direction, distance);
    *x = moved.x();
    *y = moved.y();
}

/// Cell-space Dragon-Strike distance used by Red Alert's cell distance helper.
pub fn cell_distance(left: CellCoord, right: CellCoord) -> u32 {
    let dx = (i32::from(left.x) - i32::from(right.x)).unsigned_abs();
    let dy = (i32::from(left.y) - i32::from(right.y)).unsigned_abs();
    let (major, minor) = if dx > dy { (dx, dy) } else { (dy, dx) };
    major + minor / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packed_coordinate_round_trips_signed_components() {
        let coord = Coordinate::from_leptons(0x1234, -2);
        assert_eq!(coord.x(), 0x1234);
        assert_eq!(coord.y(), -2);
        assert_eq!(Coordinate::from_raw(coord.raw()), coord);
    }

    #[test]
    fn source_cell_center_uses_256_leptons_and_half_cell() {
        let coord = Coordinate::from_cell_center(CellCoord::new(7, 9));
        assert_eq!((coord.x(), coord.y()), (0x0780, 0x0980));
        assert_eq!(coord.snap(), coord);
    }

    #[test]
    fn source_snap_masks_low_byte_and_sets_half_cell() {
        let coord = Coordinate::from_leptons(0x0777, 0x12FF);
        assert_eq!(coord.snap(), Coordinate::from_leptons(0x0780, 0x1280));
    }

    #[test]
    fn exact_movement_tables_preserve_cardinal_motion() {
        let origin = Coordinate::from_leptons(0x2000, 0x2000);
        let north = origin.moved(DirType::NORTH, 128);
        let east = origin.moved(DirType::EAST, 128);
        let south = origin.moved(DirType::SOUTH, 128);
        let west = origin.moved(DirType::WEST, 128);
        assert_eq!((north.x(), north.y()), (0x2000, 0x1F81));
        assert_eq!((east.x(), east.y()), (0x2080, 0x2000));
        assert_eq!((south.x(), south.y()), (0x2000, 0x207F));
        assert_eq!((west.x(), west.y()), (0x1F80, 0x2000));
    }

    #[test]
    fn dragon_strike_distance_matches_source_integer_formula() {
        let a = Coordinate::from_leptons(0, 0);
        let b = Coordinate::from_leptons(100, 50);
        assert_eq!(a.distance(b), 125);
        assert_eq!(cell_distance(CellCoord::new(0, 0), CellCoord::new(5, 2)), 6);
    }

    #[test]
    fn cell_extraction_respects_game_variant_map_dimensions() {
        let td = Coordinate::from_cell_center(CellCoord::new(63, 63));
        let ra = Coordinate::from_cell_center(CellCoord::new(127, 127));
        assert_eq!(td.cell(GameVariant::TiberianDawn), Some(CellCoord::new(63, 63)));
        assert_eq!(ra.cell(GameVariant::RedAlert), Some(CellCoord::new(127, 127)));
    }
}
