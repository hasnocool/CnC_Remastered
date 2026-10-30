//! Tiberian Dawn-specific rules and runtime composition.
///
/// This crate is intentionally a variant boundary at this milestone. Game rules,
/// data tables, AI, campaigns, and rendering hooks remain to be ported.
use rts_core::{GameVariant, MapDimensions};

pub const VARIANT: GameVariant = GameVariant::TiberianDawn;
pub const UPSTREAM_SOURCE_DIRECTORY: &str = "TIBERIANDAWN";

pub const fn global_map_dimensions() -> MapDimensions {
    VARIANT.global_dimensions()
}
