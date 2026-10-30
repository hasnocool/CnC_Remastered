//! Active Red Alert map-selection support from REDALERT/MAPSEL.CPP.
//!
//! The upstream Map_Selection body is behind #if (0) and the compiled
//! function returns NULL. The coordinate table, Mouse_Over_Spot, pulse helper
//! and Set_Mouse remain active and are ported below.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapPoint { pub x: i16, pub y: i16 }

pub const MAP_COORDS: [[[MapPoint; 3]; 14]; 2] = [
  [
    [MapPoint { x: 185, y: 123 }, MapPoint { x: -1, y: -1 }, MapPoint { x: -1, y: -1 }],
    [MapPoint { x: 173, y: 112 }, MapPoint { x: -1, y: -1 }, MapPoint { x: -1, y: -1 }],
    [MapPoint { x: 196, y: 100 }, MapPoint { x: 200, y: 112 }, MapPoint { x: -1, y: -1 }],
    [MapPoint { x: 175, y: 113 }, MapPoint { x: -1, y: -1 }, MapPoint { x: -1, y: -1 }],
    [MapPoint { x: 187, y: 91 }, MapPoint { x: 202, y: 93 }, MapPoint { x: 206, y: 105 }],
    [MapPoint { x: 207, y: 161 }, MapPoint { x: 212, y: 172 }, MapPoint { x: -1, y: -1 }],
    [MapPoint { x: 172, y: 92 }, MapPoint { x: -1, y: -1 }, MapPoint { x: -1, y: -1 }],
    [MapPoint { x: 132, y: 119 }, MapPoint { x: 146, y: 125 }, MapPoint { x: -1, y: -1 }],
    [MapPoint { x: 199, y: 73 }, MapPoint { x: 205, y: 86 }, MapPoint { x: -1, y: -1 }],
    [MapPoint { x: 236, y: 114 }, MapPoint { x: -1, y: -1 }, MapPoint { x: -1, y: -1 }],
    [MapPoint { x: 219, y: 64 }, MapPoint { x: 225, y: 76 }, MapPoint { x: -1, y: -1 }],
    [MapPoint { x: 256, y: 69 }, MapPoint { x: -1, y: -1 }, MapPoint { x: -1, y: -1 }],
    [MapPoint { x: 262, y: 77 }, MapPoint { x: -1, y: -1 }, MapPoint { x: -1, y: -1 }],
    [MapPoint { x: 249, y: 97 }, MapPoint { x: -1, y: -1 }, MapPoint { x: -1, y: -1 }]
  ],
  [
    [MapPoint { x: 178, y: 105 }, MapPoint { x: -1, y: -1 }, MapPoint { x: -1, y: -1 }],
    [MapPoint { x: 163, y: 101 }, MapPoint { x: 163, y: 113 }, MapPoint { x: -1, y: -1 }],
    [MapPoint { x: 160, y: 89 }, MapPoint { x: -1, y: -1 }, MapPoint { x: -1, y: -1 }],
    [MapPoint { x: 142, y: 101 }, MapPoint { x: 142, y: 117 }, MapPoint { x: -1, y: -1 }],
    [MapPoint { x: 212, y: 163 }, MapPoint { x: -1, y: -1 }, MapPoint { x: -1, y: -1 }],
    [MapPoint { x: 155, y: 133 }, MapPoint { x: 171, y: 144 }, MapPoint { x: -1, y: -1 }],
    [MapPoint { x: 216, y: 103 }, MapPoint { x: -1, y: -1 }, MapPoint { x: -1, y: -1 }],
    [MapPoint { x: 132, y: 145 }, MapPoint { x: 154, y: 154 }, MapPoint { x: -1, y: -1 }],
    [MapPoint { x: 122, y: 117 }, MapPoint { x: -1, y: -1 }, MapPoint { x: -1, y: -1 }],
    [MapPoint { x: 117, y: 130 }, MapPoint { x: -1, y: -1 }, MapPoint { x: -1, y: -1 }],
    [MapPoint { x: 99, y: 107 }, MapPoint { x: 109, y: 146 }, MapPoint { x: -1, y: -1 }],
    [MapPoint { x: 134, y: 125 }, MapPoint { x: -1, y: -1 }, MapPoint { x: -1, y: -1 }],
    [MapPoint { x: 32, y: 156 }, MapPoint { x: 46, y: 171 }, MapPoint { x: -1, y: -1 }],
    [MapPoint { x: 108, y: 97 }, MapPoint { x: -1, y: -1 }, MapPoint { x: -1, y: -1 }]
  ]
];

pub const ANT_MISSION_FILES: [Option<&'static str>; 5] = [
    None, Some("SCA01EA.INI"), Some("SCA02EA.INI"),
    Some("SCA03EA.INI"), Some("SCA04EA.INI"),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapMouseShape { Normal, CanAttack, Other(i32) }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapMouseAnimation {
    pub start: i32,
    pub count: i32,
    pub delay: i32,
    pub hotspot_x: i32,
    pub hotspot_y: i32,
}

pub const fn set_mouse(shape: MapMouseShape) -> MapMouseAnimation {
    match shape {
        MapMouseShape::Normal => MapMouseAnimation {
            start: 0, count: 1, delay: 0, hotspot_x: 0, hotspot_y: 0,
        },
        MapMouseShape::CanAttack | MapMouseShape::Other(_) => MapMouseAnimation {
            start: 21, count: 8, delay: 4, hotspot_x: 14, hotspot_y: 11,
        },
    }
}

/// Screen coordinates are divided by RESFACTOR; hit-box edges are inclusive.
pub fn mouse_over_spot(
    house: usize,
    scenario: usize,
    mouse_x: i32,
    mouse_y: i32,
    resolution_factor: i32,
) -> Option<usize> {
    if house >= 2 || scenario >= 14 || resolution_factor <= 0 { return None; }
    let x = mouse_x / resolution_factor;
    let y = mouse_y / resolution_factor;
    for (index, point) in MAP_COORDS[house][scenario].iter().enumerate() {
        if point.x < 0 || point.y < 0 { break; }
        let left = i32::from(point.x);
        let top = i32::from(point.y);
        if x >= left && y >= top && x <= left + 11 && y <= top + 9 {
            return Some(index);
        }
    }
    None
}

/// The original Map_Selection body is disabled by #if (0); the compiled
/// function therefore always returns NULL.
pub const fn map_selection() -> Option<&'static str> { None }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RgbColor { pub red: u8, pub green: u8, pub blue: u8 }

pub trait MapPulseHost {
    fn timer_expired(&mut self) -> bool;
    fn set_timer(&mut self, ticks: u32);
    fn white_color(&self) -> RgbColor;
    fn black_color(&self) -> RgbColor;
    fn adjust_color(&mut self, color: RgbColor, amount: u8, toward: RgbColor) -> RgbColor;
    fn set_palette_color(&mut self, index: usize, color: RgbColor);
    fn set_palette(&mut self);
    fn call_back_delay(&mut self, ticks: i32);
}

pub const PULSE_PALETTE_INDEX: usize = 254;
pub const PULSE_STEP_RATE: i32 = 20;
pub const PULSE_MINIMUM: i32 = 0x20;
pub const PULSE_MAXIMUM: i32 = 150;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapPulseState { pub moving_up: bool, pub value: i32 }

impl Default for MapPulseState {
    fn default() -> Self { Self { moving_up: false, value: 255 } }
}

impl MapPulseState {
    /// One iteration of Cycle_Call_Back_Delay. TIMER_SECOND/6 is supplied
    /// by the host; calling per game tick avoids a blocking wait loop.
    pub fn cycle_callback_delay_step(&mut self, timer_period: u32, host: &mut impl MapPulseHost) {
        if host.timer_expired() {
            host.set_timer(timer_period);
            if self.moving_up {
                self.value += PULSE_STEP_RATE;
                if self.value > PULSE_MAXIMUM {
                    self.value = PULSE_MAXIMUM;
                    self.moving_up = false;
                }
            } else {
                self.value -= PULSE_STEP_RATE;
                if self.value < PULSE_MINIMUM {
                    self.value = PULSE_MINIMUM;
                    self.moving_up = true;
                }
            }
            let white = host.white_color();
            let black = host.black_color();
            let color = host.adjust_color(white, self.value as u8, black);
            host.set_palette_color(PULSE_PALETTE_INDEX, color);
            host.set_palette();
        }
        host.call_back_delay(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct FakePulseHost {
        expired: bool,
        timers: Vec<u32>,
        palette_colors: Vec<(usize, RgbColor)>,
        palette_sets: usize,
        callback_delays: Vec<i32>,
        adjustments: Vec<(RgbColor, u8, RgbColor)>,
    }

    impl MapPulseHost for FakePulseHost {
        fn timer_expired(&mut self) -> bool {
            let expired = self.expired;
            self.expired = false;
            expired
        }
        fn set_timer(&mut self, ticks: u32) { self.timers.push(ticks); }
        fn white_color(&self) -> RgbColor { RgbColor { red: 255, green: 255, blue: 255 } }
        fn black_color(&self) -> RgbColor { RgbColor::default() }
        fn adjust_color(&mut self, color: RgbColor, amount: u8, toward: RgbColor) -> RgbColor {
            self.adjustments.push((color, amount, toward));
            RgbColor { red: amount, green: amount, blue: amount }
        }
        fn set_palette_color(&mut self, index: usize, color: RgbColor) {
            self.palette_colors.push((index, color));
        }
        fn set_palette(&mut self) { self.palette_sets += 1; }
        fn call_back_delay(&mut self, ticks: i32) { self.callback_delays.push(ticks); }
    }

    #[test]
    fn map_coordinate_table_matches_shape_and_known_gdi_soviet_points() {
        assert_eq!(MAP_COORDS.len(), 2);
        assert_eq!(MAP_COORDS[0].len(), 14);
        assert_eq!(MAP_COORDS[1].len(), 14);
        assert_eq!(MAP_COORDS[0][0][0], MapPoint { x: 185, y: 123 });
        assert_eq!(MAP_COORDS[0][2][1], MapPoint { x: 200, y: 112 });
        assert_eq!(MAP_COORDS[1][12][0], MapPoint { x: 32, y: 156 });
        assert_eq!(MAP_COORDS[1][12][1], MapPoint { x: 46, y: 171 });
        assert_eq!(MAP_COORDS[0][0][1], MapPoint { x: -1, y: -1 });
    }

    #[test]
    fn mouse_hit_test_uses_inclusive_rectangle_boundaries() {
        let point = MAP_COORDS[0][0][0];
        assert_eq!(mouse_over_spot(0, 0, i32::from(point.x), i32::from(point.y), 1), Some(0));
        assert_eq!(mouse_over_spot(0, 0, i32::from(point.x)+11, i32::from(point.y)+9, 1), Some(0));
        assert_eq!(mouse_over_spot(0, 0, i32::from(point.x)+12, i32::from(point.y)+9, 1), None);
    }

    #[test]
    fn mouse_hit_test_scales_coordinates_and_rejects_invalid_arguments() {
        let point = MAP_COORDS[1][12][1];
        assert_eq!(mouse_over_spot(1, 12, i32::from(point.x)*2+22, i32::from(point.y)*2+18, 2), Some(1));
        assert_eq!(mouse_over_spot(2, 0, 100, 100, 1), None);
        assert_eq!(mouse_over_spot(0, 14, 100, 100, 1), None);
        assert_eq!(mouse_over_spot(0, 0, 100, 100, 0), None);
    }

    #[test]
    fn sentinel_stops_scenario_coordinate_search() {
        assert_eq!(MAP_COORDS[0][0][1], MapPoint { x: -1, y: -1 });
        assert_eq!(mouse_over_spot(0, 0, -1, -1, 1), None);
    }

    #[test]
    fn ant_mission_table_preserves_one_based_filename_slots() {
        assert_eq!(ANT_MISSION_FILES, [
            None, Some("SCA01EA.INI"), Some("SCA02EA.INI"),
            Some("SCA03EA.INI"), Some("SCA04EA.INI"),
        ]);
    }

    #[test]
    fn cursor_shapes_keep_animation_frames_and_hotspots() {
        assert_eq!(set_mouse(MapMouseShape::Normal), MapMouseAnimation {
            start: 0, count: 1, delay: 0, hotspot_x: 0, hotspot_y: 0,
        });
        assert_eq!(set_mouse(MapMouseShape::CanAttack), MapMouseAnimation {
            start: 21, count: 8, delay: 4, hotspot_x: 14, hotspot_y: 11,
        });
    }

    #[test]
    fn disabled_map_selection_returns_null() {
        assert_eq!(map_selection(), None);
    }

    #[test]
    fn pulse_state_starts_at_255_and_decrements_by_twenty() {
        let mut pulse = MapPulseState::default();
        let mut host = FakePulseHost { expired: true, ..FakePulseHost::default() };
        pulse.cycle_callback_delay_step(10, &mut host);
        assert_eq!(pulse.value, 235);
        assert!(!pulse.moving_up);
        assert_eq!(host.timers, vec![10]);
        assert_eq!(host.palette_colors[0].0, 254);
        assert_eq!(host.adjustments[0].1, 235);
        assert_eq!(host.palette_sets, 1);
        assert_eq!(host.callback_delays, vec![1]);
    }

    #[test]
    fn pulse_state_reverses_at_both_limits_and_skips_running_timer() {
        let mut pulse = MapPulseState { moving_up: false, value: 35 };
        let mut host = FakePulseHost { expired: true, ..FakePulseHost::default() };
        pulse.cycle_callback_delay_step(100, &mut host);
        assert_eq!(pulse.value, 32);
        assert!(pulse.moving_up);
        host.expired = true;
        pulse.value = 145;
        pulse.cycle_callback_delay_step(100, &mut host);
        assert_eq!(pulse.value, 150);
        assert!(!pulse.moving_up);
        host.expired = false;
        let previous = host.palette_sets;
        pulse.cycle_callback_delay_step(100, &mut host);
        assert_eq!(host.palette_sets, previous);
        assert_eq!(host.callback_delays, vec![1, 1, 1]);
    }
}
