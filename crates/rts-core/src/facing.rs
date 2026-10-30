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
