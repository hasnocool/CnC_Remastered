/// Eight-way facing identifiers in the order used by the upstream FacingType enum.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Facing {
    North = 0,
    NorthEast = 1,
    East = 2,
    SouthEast = 3,
    South = 4,
    SouthWest = 5,
    West = 6,
    NorthWest = 7,
}

pub const FACING_COUNT: u8 = 8;

impl Facing {
    pub const ALL: [Self; FACING_COUNT as usize] = [
        Self::North,
        Self::NorthEast,
        Self::East,
        Self::SouthEast,
        Self::South,
        Self::SouthWest,
        Self::West,
        Self::NorthWest,
    ];

    pub const fn from_ordinal(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::North),
            1 => Some(Self::NorthEast),
            2 => Some(Self::East),
            3 => Some(Self::SouthEast),
            4 => Some(Self::South),
            5 => Some(Self::SouthWest),
            6 => Some(Self::West),
            7 => Some(Self::NorthWest),
            _ => None,
        }
    }

    pub const fn ordinal(self) -> u8 {
        self as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordinal_round_trips_for_all_eight_directions() {
        for (ordinal, facing) in Facing::ALL.into_iter().enumerate() {
            assert_eq!(facing.ordinal(), ordinal as u8);
            assert_eq!(Facing::from_ordinal(ordinal as u8), Some(facing));
        }
        assert_eq!(Facing::from_ordinal(8), None);
    }
}


/// Eight-bit angle representation used by the original `DirType`.
///
/// The eight cardinal/intercardinal directions occupy multiples of 32;
/// movement and turret rotation can use the full 0..=255 angular range.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DirType(pub u8);

impl DirType {
    pub const NORTH: Self = Self(0);
    pub const NORTH_EAST: Self = Self(32);
    pub const EAST: Self = Self(64);
    pub const SOUTH_EAST: Self = Self(96);
    pub const SOUTH: Self = Self(128);
    pub const SOUTH_WEST: Self = Self(160);
    pub const SOUTH_WEST_X1: Self = Self(152);
    pub const SOUTH_WEST_X2: Self = Self(144);
    pub const WEST: Self = Self(192);
    pub const NORTH_WEST: Self = Self(224);

    pub const fn from_facing(facing: Facing) -> Self {
        Self((facing as u8) << 5)
    }

    pub const fn as_facing(self) -> Facing {
        // The source Dir_Facing adds 0x10 before taking the high three bits.
        match self.0.wrapping_add(0x10) >> 5 {
            0 => Facing::North,
            1 => Facing::NorthEast,
            2 => Facing::East,
            3 => Facing::SouthEast,
            4 => Facing::South,
            5 => Facing::SouthWest,
            6 => Facing::West,
            _ => Facing::NorthWest,
        }
    }

    /// Signed shortest angular difference, equivalent to the source's
    /// signed-char subtraction.
    pub const fn signed_difference(self, current: Self) -> i8 {
        self.0.wrapping_sub(current.0) as i8
    }

    pub const fn wrapping_add(self, amount: i32) -> Self {
        Self(self.0.wrapping_add(amount as u8))
    }

    pub const fn wrapping_sub(self, amount: i32) -> Self {
        Self(self.0.wrapping_sub(amount as u8))
    }

    /// Map a 0..255 direction into its 1/32 rotation zone.
    pub const fn to_32_zone(self) -> u8 {
        self.0 >> 3
    }
}

/// Stateful facing controller ported from both variants' FACING.H/FACING.CPP.
///
/// Current and desired headings are stored as full byte angles, not merely the
/// eight-value `FacingType`. Rotation uses the source's signed 8-bit shortest
/// difference, 127-rate cap, strict snap comparison, and wrapping byte updates.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FacingClass {
    current: DirType,
    desired: DirType,
}

impl FacingClass {
    pub const fn new() -> Self {
        Self {
            current: DirType::NORTH,
            desired: DirType::NORTH,
        }
    }

    pub const fn with_direction(direction: DirType) -> Self {
        Self {
            current: direction,
            desired: direction,
        }
    }

    pub const fn current(self) -> DirType {
        self.current
    }

    pub const fn desired(self) -> DirType {
        self.desired
    }

    pub const fn get(self) -> DirType {
        self.current
    }

    pub const fn is_rotating(self) -> bool {
        self.current.0 != self.desired.0
    }

    pub const fn difference(self) -> i32 {
        self.desired.signed_difference(self.current) as i32
    }

    pub const fn difference_to(self, direction: DirType) -> i32 {
        direction.signed_difference(self.current) as i32
    }

    pub fn set_desired(&mut self, direction: DirType) -> bool {
        if self.desired == direction {
            return false;
        }
        self.desired = direction;
        true
    }

    pub fn set_current(&mut self, direction: DirType) -> bool {
        if self.current == direction {
            return false;
        }
        self.current = direction;
        true
    }

    pub fn set(&mut self, direction: DirType) {
        self.set_current(direction);
        self.set_desired(direction);
    }

    /// Advance the current facing toward the desired facing.
    ///
    /// Returns true when the adjustment enters a different 1/32 rotation
    /// zone, the exact condition used by the original implementation to
    /// request a sprite redraw.
    pub fn rotation_adjust(&mut self, rate: i32) -> bool {
        if !self.is_rotating() {
            return false;
        }

        let rate = rate.min(127);
        let old_zone = self.current.to_32_zone();
        let difference = self.difference();

        if difference.abs() < rate {
            self.current = self.desired;
        } else if difference < 0 {
            self.current = self.current.wrapping_sub(rate);
        } else {
            self.current = self.current.wrapping_add(rate);
        }

        self.current.to_32_zone() != old_zone
    }
}

#[cfg(test)]
mod dir_type_tests {
    use super::*;

    #[test]
    fn cardinal_angles_convert_to_eight_way_facing() {
        for facing in Facing::ALL {
            assert_eq!(DirType::from_facing(facing).as_facing(), facing);
        }
    }

    #[test]
    fn westwood_special_harvester_facings_are_preserved() {
        assert_eq!(DirType::SOUTH_WEST_X1.0, (5 << 5) - 8);
        assert_eq!(DirType::SOUTH_WEST_X2.0, (5 << 5) - 16);
        assert_eq!(DirType::SOUTH_WEST_X1.as_facing(), Facing::SouthWest);
        assert_eq!(DirType::SOUTH_WEST_X2.as_facing(), Facing::SouthWest);
    }

    #[test]
    fn signed_difference_wraps_at_the_byte_boundary() {
        assert_eq!(DirType(0).signed_difference(DirType(255)), 1);
        assert_eq!(DirType(255).signed_difference(DirType(0)), -1);
        assert_eq!(DirType(128).signed_difference(DirType(0)), -128);
    }

    #[test]
    fn facing_class_initializes_north_and_tracks_changes() {
        let mut facing = FacingClass::new();
        assert_eq!(facing.current(), DirType::NORTH);
        assert_eq!(facing.desired(), DirType::NORTH);
        assert!(!facing.set_desired(DirType::NORTH_EAST));
        assert!(facing.set_desired(DirType::EAST));
        assert!(facing.is_rotating());
        assert!(!facing.set_current(DirType::NORTH));
        assert!(facing.set_current(DirType::SOUTH));
    }

    #[test]
    fn rotation_takes_shortest_path_across_zero() {
        let mut facing = FacingClass::with_direction(DirType(250));
        facing.set_desired(DirType(5));

        assert_eq!(facing.difference(), 11);
        facing.rotation_adjust(4);
        assert_eq!(facing.current(), DirType(254));
        facing.rotation_adjust(4);
        assert_eq!(facing.current(), DirType(2));
        facing.rotation_adjust(4);
        assert_eq!(facing.current(), DirType(5));
        assert!(!facing.is_rotating());
    }

    #[test]
    fn rotation_caps_rate_and_reports_sprite_zone_changes() {
        let mut facing = FacingClass::new();
        facing.set_desired(DirType(128));
        assert!(facing.rotation_adjust(127));
        assert_eq!(facing.current(), DirType(127));
        assert!(!facing.rotation_adjust(127));
        assert_eq!(facing.current(), DirType(127));
    }

    #[test]
    fn rotation_snap_uses_strict_less_than_comparison() {
        let mut facing = FacingClass::with_direction(DirType(0));
        facing.set_desired(DirType(10));
        facing.rotation_adjust(10);
        assert_eq!(facing.current(), DirType(10));
    }

    #[test]
    fn instant_rotation_uses_rate_127() {
        let mut facing = FacingClass::with_direction(DirType(0));
        facing.set_desired(DirType(200));
        facing.rotation_adjust(127);
        assert_eq!(facing.current(), DirType(127));
        facing.rotation_adjust(127);
        assert_eq!(facing.current(), DirType(200));
    }
}
