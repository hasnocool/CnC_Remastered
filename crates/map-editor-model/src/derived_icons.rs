//! Pure source-derived icon lookup and connectivity helpers for placed overlays.

/// Resource icon lookup from Map.UpdateResourceOverlays.
pub const TIBERIUM_GOLD_RESOURCE_ICONS: [i32; 9] = [0, 1, 3, 4, 6, 7, 8, 10, 11];
/// Gem icon lookup from Map.UpdateResourceOverlays.
pub const GEM_RESOURCE_ICONS: [i32; 9] = [0, 0, 0, 1, 1, 1, 2, 2, 2];

/// Maps the count of adjacent resource overlays (0..=8) to the source icon index.
pub fn resource_icon(is_gem: bool, adjacent_resource_count: u8) -> Option<i32> {
    let index = usize::from(adjacent_resource_count);
    if index >= TIBERIUM_GOLD_RESOURCE_ICONS.len() {
        return None;
    }
    Some(if is_gem {
        GEM_RESOURCE_ICONS[index]
    } else {
        TIBERIUM_GOLD_RESOURCE_ICONS[index]
    })
}

/// Wall connection bits follow the original source order: N=1, E=2, S=4, W=8.
pub const fn wall_icon(north: bool, east: bool, south: bool, west: bool) -> i32 {
    (north as i32) | ((east as i32) << 1) | ((south as i32) << 2) | ((west as i32) << 3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_lookup_tables_match_source_exactly() {
        assert_eq!(resource_icon(false, 0), Some(0));
        assert_eq!(resource_icon(false, 8), Some(11));
        assert_eq!(resource_icon(true, 0), Some(0));
        assert_eq!(resource_icon(true, 8), Some(2));
        assert_eq!(resource_icon(false, 9), None);
    }

    #[test]
    fn wall_bits_match_source_cardinal_order() {
        assert_eq!(wall_icon(true, false, false, false), 1);
        assert_eq!(wall_icon(false, true, false, false), 2);
        assert_eq!(wall_icon(false, false, true, false), 4);
        assert_eq!(wall_icon(false, false, false, true), 8);
        assert_eq!(wall_icon(true, true, true, true), 15);
    }
}
