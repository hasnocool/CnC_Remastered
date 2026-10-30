//! Platform-independent compatibility primitives for the two RTS variants.

pub mod bool_vector;
pub mod clock;
pub mod facing;
pub mod fixed16;
pub mod random;
pub mod random_class;
pub mod vector;
pub mod rect;
pub mod legacy_stub;
pub mod map;

pub use clock::{SimTick, TICKS_PER_MINUTE, TICKS_PER_SECOND};
pub use facing::{Facing, FACING_COUNT};
pub use map::{CellCoord, GameVariant, MapDimensions, MapError, REGION_HEIGHT, REGION_WIDTH};
