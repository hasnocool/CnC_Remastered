//! Mouse cursor state and animation controller from both variants' MOUSE.CPP/H.
//!
//! Mouse shapes and timing are host-driven. The exact per-variant MouseControl
//! tables are embedded below; the host supplies shape extraction, cursor
//! installation, mouse-shape file loading, countdown timer state, and the
//! inherited ScrollClass behavior.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MouseVariant { RedAlert, TiberianDawn }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum RedAlertMouseType {
    MOUSE_NORMAL = 0,
    MOUSE_N = 1,
    MOUSE_NE = 2,
    MOUSE_E = 3,
    MOUSE_SE = 4,
    MOUSE_S = 5,
    MOUSE_SW = 6,
    MOUSE_W = 7,
    MOUSE_NW = 8,
    MOUSE_NO_N = 9,
    MOUSE_NO_NE = 10,
    MOUSE_NO_E = 11,
    MOUSE_NO_SE = 12,
    MOUSE_NO_S = 13,
    MOUSE_NO_SW = 14,
    MOUSE_NO_W = 15,
    MOUSE_NO_NW = 16,
    MOUSE_NO_MOVE = 17,
    MOUSE_CAN_MOVE = 18,
    MOUSE_ENTER = 19,
    MOUSE_DEPLOY = 20,
    MOUSE_CAN_SELECT = 21,
    MOUSE_CAN_ATTACK = 22,
    MOUSE_SELL_BACK = 23,
    MOUSE_SELL_UNIT = 24,
    MOUSE_REPAIR = 25,
    MOUSE_NO_REPAIR = 26,
    MOUSE_NO_SELL_BACK = 27,
    MOUSE_RADAR_CURSOR = 28,
    MOUSE_NUCLEAR_BOMB = 29,
    MOUSE_AIR_STRIKE = 30,
    MOUSE_DEMOLITIONS = 31,
    MOUSE_AREA_GUARD = 32,
    MOUSE_HEAL = 33,
    MOUSE_DAMAGE = 34,
    MOUSE_NO_DEPLOY = 35,
    MOUSE_NO_ENTER = 36,
    MOUSE_NO_GREPAIR = 37,
    MOUSE_CHRONO_SELECT = 38,
    MOUSE_CHRONO_DEST = 39,
}
pub const REDALERT_MOUSE_COUNT: usize = 40;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum TiberianDawnMouseType {
    MOUSE_NORMAL = 0,
    MOUSE_N = 1,
    MOUSE_NE = 2,
    MOUSE_E = 3,
    MOUSE_SE = 4,
    MOUSE_S = 5,
    MOUSE_SW = 6,
    MOUSE_W = 7,
    MOUSE_NW = 8,
    MOUSE_NO_N = 9,
    MOUSE_NO_NE = 10,
    MOUSE_NO_E = 11,
    MOUSE_NO_SE = 12,
    MOUSE_NO_S = 13,
    MOUSE_NO_SW = 14,
    MOUSE_NO_W = 15,
    MOUSE_NO_NW = 16,
    MOUSE_NO_MOVE = 17,
    MOUSE_CAN_MOVE = 18,
    MOUSE_ENTER = 19,
    MOUSE_DEPLOY = 20,
    MOUSE_CAN_SELECT = 21,
    MOUSE_CAN_ATTACK = 22,
    MOUSE_SELL_BACK = 23,
    MOUSE_SELL_UNIT = 24,
    MOUSE_REPAIR = 25,
    MOUSE_NO_REPAIR = 26,
    MOUSE_NO_SELL_BACK = 27,
    MOUSE_RADAR_CURSOR = 28,
    MOUSE_ION_CANNON = 29,
    MOUSE_NUCLEAR_BOMB = 30,
    MOUSE_AIR_STRIKE = 31,
    MOUSE_DEMOLITIONS = 32,
    MOUSE_AREA_GUARD = 33,
}
pub const TIBERIAN_DAWN_MOUSE_COUNT: usize = 34;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MouseControl {
    pub start_frame: i32,
    pub frame_count: i32,
    pub frame_rate: i32,
    pub small_frame: i32,
    pub hotspot_x: i32,
    pub hotspot_y: i32,
}

pub const REDALERT_MOUSE_CONTROL: [MouseControl; REDALERT_MOUSE_COUNT] = [
    MouseControl { start_frame: 0, frame_count: 1, frame_rate: 0, small_frame: 80, hotspot_x: 0, hotspot_y: 0 }, // MOUSE_NORMAL
    MouseControl { start_frame: 1, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 14, hotspot_y: 0 }, // MOUSE_N
    MouseControl { start_frame: 2, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 29, hotspot_y: 0 }, // MOUSE_NE
    MouseControl { start_frame: 3, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 29, hotspot_y: 11 }, // MOUSE_E
    MouseControl { start_frame: 4, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 29, hotspot_y: 23 }, // MOUSE_SE
    MouseControl { start_frame: 5, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 14, hotspot_y: 23 }, // MOUSE_S
    MouseControl { start_frame: 6, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 0, hotspot_y: 23 }, // MOUSE_SW
    MouseControl { start_frame: 7, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 0, hotspot_y: 11 }, // MOUSE_W
    MouseControl { start_frame: 8, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 0, hotspot_y: 0 }, // MOUSE_NW
    MouseControl { start_frame: 124, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 14, hotspot_y: 0 }, // MOUSE_NO_N
    MouseControl { start_frame: 125, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 29, hotspot_y: 0 }, // MOUSE_NO_NE
    MouseControl { start_frame: 126, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 29, hotspot_y: 11 }, // MOUSE_NO_E
    MouseControl { start_frame: 127, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 29, hotspot_y: 23 }, // MOUSE_NO_SE
    MouseControl { start_frame: 128, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 14, hotspot_y: 23 }, // MOUSE_NO_S
    MouseControl { start_frame: 129, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 0, hotspot_y: 23 }, // MOUSE_NO_SW
    MouseControl { start_frame: 130, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 0, hotspot_y: 11 }, // MOUSE_NO_W
    MouseControl { start_frame: 131, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 0, hotspot_y: 0 }, // MOUSE_NO_NW
    MouseControl { start_frame: 14, frame_count: 1, frame_rate: 0, small_frame: 33, hotspot_x: 14, hotspot_y: 11 }, // MOUSE_NO_MOVE
    MouseControl { start_frame: 10, frame_count: 4, frame_rate: 4, small_frame: 29, hotspot_x: 14, hotspot_y: 11 }, // MOUSE_CAN_MOVE
    MouseControl { start_frame: 113, frame_count: 3, frame_rate: 4, small_frame: 142, hotspot_x: 14, hotspot_y: 11 }, // MOUSE_ENTER
    MouseControl { start_frame: 59, frame_count: 9, frame_rate: 4, small_frame: -1, hotspot_x: 14, hotspot_y: 11 }, // MOUSE_DEPLOY
    MouseControl { start_frame: 15, frame_count: 6, frame_rate: 4, small_frame: -1, hotspot_x: 14, hotspot_y: 11 }, // MOUSE_CAN_SELECT
    MouseControl { start_frame: 21, frame_count: 8, frame_rate: 4, small_frame: 134, hotspot_x: 14, hotspot_y: 11 }, // MOUSE_CAN_ATTACK
    MouseControl { start_frame: 68, frame_count: 12, frame_rate: 2, small_frame: -1, hotspot_x: 14, hotspot_y: 11 }, // MOUSE_SELL_BACK
    MouseControl { start_frame: 148, frame_count: 12, frame_rate: 2, small_frame: -1, hotspot_x: 14, hotspot_y: 11 }, // MOUSE_SELL_UNIT
    MouseControl { start_frame: 35, frame_count: 24, frame_rate: 2, small_frame: -1, hotspot_x: 14, hotspot_y: 11 }, // MOUSE_REPAIR
    MouseControl { start_frame: 120, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 14, hotspot_y: 11 }, // MOUSE_NO_REPAIR
    MouseControl { start_frame: 119, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 14, hotspot_y: 11 }, // MOUSE_NO_SELL_BACK
    MouseControl { start_frame: 81, frame_count: 1, frame_rate: 0, small_frame: 145, hotspot_x: 14, hotspot_y: 11 }, // MOUSE_RADAR_CURSOR
    MouseControl { start_frame: 90, frame_count: 7, frame_rate: 4, small_frame: -1, hotspot_x: 14, hotspot_y: 11 }, // MOUSE_NUCLEAR_BOMB
    MouseControl { start_frame: 82, frame_count: 8, frame_rate: 2, small_frame: 213, hotspot_x: 14, hotspot_y: 11 }, // MOUSE_AIR_STRIKE
    MouseControl { start_frame: 116, frame_count: 3, frame_rate: 4, small_frame: 121, hotspot_x: 14, hotspot_y: 11 }, // MOUSE_DEMOLITIONS
    MouseControl { start_frame: 147, frame_count: 1, frame_rate: 0, small_frame: 146, hotspot_x: 14, hotspot_y: 11 }, // MOUSE_AREA_GUARD
    MouseControl { start_frame: 160, frame_count: 4, frame_rate: 4, small_frame: 194, hotspot_x: 14, hotspot_y: 11 }, // MOUSE_HEAL
    MouseControl { start_frame: 164, frame_count: 3, frame_rate: 4, small_frame: 167, hotspot_x: 14, hotspot_y: 11 }, // MOUSE_DAMAGE
    MouseControl { start_frame: 170, frame_count: 24, frame_rate: 2, small_frame: -1, hotspot_x: 14, hotspot_y: 11 }, // MOUSE_GREPAIR
    MouseControl { start_frame: 195, frame_count: 8, frame_rate: 4, small_frame: 203, hotspot_x: 14, hotspot_y: 11 }, // MOUSE_STAY_ATTACK
    MouseControl { start_frame: 211, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 14, hotspot_y: 11 }, // MOUSE_NO_DEPLOY
    MouseControl { start_frame: 212, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 14, hotspot_y: 11 }, // MOUSE_NO_ENTER
    MouseControl { start_frame: 213, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 14, hotspot_y: 11 }, // MOUSE_NO_GREPAIR
    MouseControl { start_frame: 97, frame_count: 8, frame_rate: 3, small_frame: -1, hotspot_x: 14, hotspot_y: 11 }, // MOUSE_CHRONO_SELECT
    MouseControl { start_frame: 105, frame_count: 8, frame_rate: 2, small_frame: -1, hotspot_x: 14, hotspot_y: 11 }, // MOUSE_CHRONO_DEST
];

pub const TIBERIAN_DAWN_MOUSE_CONTROL: [MouseControl; TIBERIAN_DAWN_MOUSE_COUNT] = [
    MouseControl { start_frame: 0, frame_count: 1, frame_rate: 0, small_frame: 86, hotspot_x: 0, hotspot_y: 0 }, // MOUSE_NORMAL
    MouseControl { start_frame: 1, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 15, hotspot_y: 0 }, // MOUSE_N
    MouseControl { start_frame: 2, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 29, hotspot_y: 0 }, // MOUSE_NE
    MouseControl { start_frame: 3, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 29, hotspot_y: 12 }, // MOUSE_E
    MouseControl { start_frame: 4, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 29, hotspot_y: 23 }, // MOUSE_SE
    MouseControl { start_frame: 5, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 15, hotspot_y: 23 }, // MOUSE_S
    MouseControl { start_frame: 6, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 0, hotspot_y: 23 }, // MOUSE_SW
    MouseControl { start_frame: 7, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 0, hotspot_y: 13 }, // MOUSE_W
    MouseControl { start_frame: 8, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 0, hotspot_y: 0 }, // MOUSE_NW
    MouseControl { start_frame: 130, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 15, hotspot_y: 0 }, // MOUSE_NO_N
    MouseControl { start_frame: 131, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 29, hotspot_y: 0 }, // MOUSE_NO_NE
    MouseControl { start_frame: 132, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 29, hotspot_y: 12 }, // MOUSE_NO_E
    MouseControl { start_frame: 133, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 29, hotspot_y: 23 }, // MOUSE_NO_SE
    MouseControl { start_frame: 134, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 15, hotspot_y: 23 }, // MOUSE_NO_S
    MouseControl { start_frame: 135, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 0, hotspot_y: 23 }, // MOUSE_NO_SW
    MouseControl { start_frame: 136, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 0, hotspot_y: 13 }, // MOUSE_NO_W
    MouseControl { start_frame: 137, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 0, hotspot_y: 0 }, // MOUSE_NO_NW
    MouseControl { start_frame: 11, frame_count: 1, frame_rate: 0, small_frame: 27, hotspot_x: 15, hotspot_y: 12 }, // MOUSE_NO_MOVE
    MouseControl { start_frame: 10, frame_count: 1, frame_rate: 0, small_frame: 26, hotspot_x: 15, hotspot_y: 12 }, // MOUSE_CAN_MOVE
    MouseControl { start_frame: 119, frame_count: 3, frame_rate: 4, small_frame: 148, hotspot_x: 15, hotspot_y: 12 }, // MOUSE_ENTER
    MouseControl { start_frame: 53, frame_count: 9, frame_rate: 4, small_frame: -1, hotspot_x: 15, hotspot_y: 12 }, // MOUSE_DEPLOY
    MouseControl { start_frame: 12, frame_count: 6, frame_rate: 4, small_frame: -1, hotspot_x: 15, hotspot_y: 12 }, // MOUSE_CAN_SELECT
    MouseControl { start_frame: 18, frame_count: 8, frame_rate: 4, small_frame: 140, hotspot_x: 15, hotspot_y: 12 }, // MOUSE_CAN_ATTACK
    MouseControl { start_frame: 62, frame_count: 24, frame_rate: 2, small_frame: -1, hotspot_x: 15, hotspot_y: 12 }, // MOUSE_SELL_BACK
    MouseControl { start_frame: 154, frame_count: 24, frame_rate: 2, small_frame: -1, hotspot_x: 15, hotspot_y: 12 }, // MOUSE_SELL_UNIT
    MouseControl { start_frame: 29, frame_count: 24, frame_rate: 2, small_frame: -1, hotspot_x: 15, hotspot_y: 12 }, // MOUSE_REPAIR
    MouseControl { start_frame: 126, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 15, hotspot_y: 12 }, // MOUSE_NO_REPAIR
    MouseControl { start_frame: 125, frame_count: 1, frame_rate: 0, small_frame: -1, hotspot_x: 15, hotspot_y: 12 }, // MOUSE_NO_SELL_BACK
    MouseControl { start_frame: 87, frame_count: 1, frame_rate: 0, small_frame: 151, hotspot_x: 0, hotspot_y: 0 }, // MOUSE_RADAR_CURSOR
    MouseControl { start_frame: 103, frame_count: 16, frame_rate: 2, small_frame: -1, hotspot_x: 15, hotspot_y: 12 }, // MOUSE_ION_CANNON
    MouseControl { start_frame: 96, frame_count: 7, frame_rate: 4, small_frame: -1, hotspot_x: 15, hotspot_y: 12 }, // MOUSE_NUCLEAR_BOMB
    MouseControl { start_frame: 88, frame_count: 8, frame_rate: 2, small_frame: -1, hotspot_x: 15, hotspot_y: 12 }, // MOUSE_AIR_STRIKE
    MouseControl { start_frame: 122, frame_count: 3, frame_rate: 4, small_frame: 127, hotspot_x: 15, hotspot_y: 12 }, // MOUSE_DEMOLITIONS
    MouseControl { start_frame: 153, frame_count: 1, frame_rate: 0, small_frame: 152, hotspot_x: 15, hotspot_y: 12 }, // MOUSE_AREA_GUARD
];

pub trait MouseHost {
    /// ScrollClass::One_Time inherited initialization.
    fn scroll_one_time(&mut self);
    /// ScrollClass::Init_Clear inherited reset.
    fn scroll_init_clear(&mut self);
    /// ScrollClass::AI inherited input processing.
    fn scroll_ai(&mut self, input: &mut u16, x: i32, y: i32);
    /// Load local override/container MOUSE.SHP. Red Alert attempts a loose
    /// override only in debug builds; Tiberian Dawn always probes the loose file.
    fn load_mouse_shapes(&mut self, variant: MouseVariant, probe_loose_file: bool) -> Option<usize>;
    /// Extract a shape pointer/token from the loaded SHP resource.
    fn extract_shape(&mut self, shape_file: usize, frame: i32) -> Option<usize>;
    /// Set cursor hotspot and selected shape.
    fn set_mouse_cursor(&mut self, hotspot_x: i32, hotspot_y: i32, shape: usize);
    /// Query/set the shared legacy animation timer.
    fn animation_timer_expired(&mut self) -> bool;
    fn set_animation_timer(&mut self, ticks: i32);
    /// Tiberian Dawn writes a legacy VTable pointer after its base class has
    /// been initialized; the host can bind its object-layout equivalent.
    fn bind_tiberian_dawn_vtable(&mut self) {}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MouseController {
    pub variant: MouseVariant,
    pub is_small: bool,
    pub current_mouse_shape: usize,
    pub normal_mouse_shape: usize,
    pub frame: i32,
    pub mouse_shapes: Option<usize>,
    startup_initialized: bool,
}

impl MouseController {
    pub fn new(variant: MouseVariant) -> Self {
        Self {
            variant,
            is_small: false,
            current_mouse_shape: 0,
            normal_mouse_shape: 0,
            frame: 0,
            mouse_shapes: None,
            startup_initialized: false,
        }
    }

    fn controls(&self) -> &'static [MouseControl] {
        match self.variant {
            MouseVariant::RedAlert => &REDALERT_MOUSE_CONTROL,
            MouseVariant::TiberianDawn => &TIBERIAN_DAWN_MOUSE_CONTROL,
        }
    }

    fn control(&self, mouse: usize) -> Option<MouseControl> {
        self.controls().get(mouse).copied()
    }

    fn normal_frame(&self) -> i32 {
        self.controls().first().map_or(0, |control| control.start_frame)
    }

    /// MouseClass::Set_Default_Mouse remembers the desired ordinary cursor,
    /// then invokes Override_Mouse_Shape.
    pub fn set_default_mouse(&mut self, host: &mut impl MouseHost, mouse: usize, small: bool) -> bool {
        if self.control(mouse).is_none() { return false; }
        self.normal_mouse_shape = mouse;
        self.override_mouse_shape(host, mouse, small)
    }

    /// MouseClass::Revert_Mouse_Shape restores NormalMouseShape in large mode.
    pub fn revert_mouse_shape(&mut self, host: &mut impl MouseHost) -> bool {
        self.override_mouse_shape(host, self.normal_mouse_shape, false)
    }

    /// Variant-sensitive Override_Mouse_Shape behavior. RA falls back to the
    /// large frame when a requested small frame is missing. TD forces small
    /// false before the choice, so the large frame is used as well.
    pub fn override_mouse_shape(&mut self, host: &mut impl MouseHost, mouse: usize, mut small: bool) -> bool {
        let Some(control) = self.control(mouse) else { return false };
        if control.small_frame == -1 { small = false; }

        if !self.startup_initialized
            || (self.mouse_shapes.is_some()
                && (mouse != self.current_mouse_shape || small != self.is_small))
        {
            self.startup_initialized = true;
            host.set_animation_timer(control.frame_rate);
            self.frame = 0;
            let mut base_frame = if small { control.small_frame } else { control.start_frame };
            if self.variant == MouseVariant::RedAlert && base_frame == -1 {
                base_frame = control.start_frame;
            }
            if let Some(shape_file) = self.mouse_shapes {
                if let Some(shape) = host.extract_shape(shape_file, base_frame + self.frame) {
                    host.set_mouse_cursor(control.hotspot_x, control.hotspot_y, shape);
                }
            }
            self.current_mouse_shape = mouse;
            self.is_small = small;
            return true;
        }
        false
    }

    /// Mouse_Small changes the active size without changing the remembered
    /// normal cursor shape. Small and animated frame offsets match the source.
    pub fn mouse_small(&mut self, host: &mut impl MouseHost, small: bool) {
        if self.is_small == small { return; }
        let Some(control) = self.control(self.current_mouse_shape) else { return };
        self.is_small = small;
        if let Some(shape_file) = self.mouse_shapes {
            if small {
                if control.small_frame != -1 {
                    if let Some(shape) = host.extract_shape(shape_file, control.small_frame + self.frame / 4) {
                        host.set_mouse_cursor(control.hotspot_x, control.hotspot_y, shape);
                    }
                } else if let Some(normal) = self.control(0) {
                    if let Some(shape) = host.extract_shape(shape_file, normal.start_frame) {
                        host.set_mouse_cursor(normal.hotspot_x, normal.hotspot_y, shape);
                    }
                }
            } else if let Some(shape) = host.extract_shape(shape_file, control.start_frame + self.frame / 4) {
                host.set_mouse_cursor(control.hotspot_x, control.hotspot_y, shape);
            }
        }
    }

    /// MouseClass::AI advances one animation frame when the shared timer
    /// expires, then always delegates to ScrollClass::AI.
    pub fn ai(&mut self, host: &mut impl MouseHost, input: &mut u16, x: i32, y: i32) {
        if let Some(control) = self.control(self.current_mouse_shape) {
            if control.frame_rate != 0 && host.animation_timer_expired() {
                if control.frame_count > 0 {
                    self.frame = self.frame.wrapping_add(1) % control.frame_count;
                }
                host.set_animation_timer(control.frame_rate);
                if !self.is_small || control.small_frame != -1 {
                    let base = if self.is_small { control.small_frame } else { control.start_frame };
                    let mut frame = base + self.frame;
                    if self.variant == MouseVariant::RedAlert && frame == -1 {
                        frame = control.start_frame;
                    }
                    if let Some(shape_file) = self.mouse_shapes {
                        if let Some(shape) = host.extract_shape(shape_file, frame) {
                            host.set_mouse_cursor(control.hotspot_x, control.hotspot_y, shape);
                        }
                    }
                }
            }
        }
        host.scroll_ai(input, x, y);
    }

    /// One_Time initializes the scroll base and resolves MOUSE.SHP according
    /// to the variant's source-specific file search behavior.
    pub fn one_time(&mut self, host: &mut impl MouseHost, debug_build: bool) {
        host.scroll_one_time();
        let probe_loose = self.variant == MouseVariant::TiberianDawn || debug_build;
        self.mouse_shapes = host.load_mouse_shapes(self.variant, probe_loose);
        if self.variant == MouseVariant::TiberianDawn {
            host.bind_tiberian_dawn_vtable();
        }
    }

    /// Init_Clear resets inherited scroll state, size override, and default
    /// mouse choice. Current animated cursor remains as in the original code.
    pub fn init_clear(&mut self, host: &mut impl MouseHost) {
        host.scroll_init_clear();
        self.is_small = false;
        self.normal_mouse_shape = 0;
    }

    pub fn get_mouse_shape(&self) -> usize { self.normal_mouse_shape }
    pub fn last_animation_frame(&self) -> i32 { self.frame }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct FakeHost {
        loaded: Option<usize>,
        extracted: Vec<i32>,
        cursor_calls: Vec<(i32, i32, usize)>,
        timer: i32,
        timer_expired: bool,
        scroll_one_time: usize,
        scroll_clear: usize,
        scroll_ai: usize,
        loose_probe: Vec<(MouseVariant, bool)>,
        vtable_bindings: usize,
    }

    impl MouseHost for FakeHost {
        fn scroll_one_time(&mut self) { self.scroll_one_time += 1; }
        fn scroll_init_clear(&mut self) { self.scroll_clear += 1; }
        fn scroll_ai(&mut self, _input: &mut u16, _x: i32, _y: i32) { self.scroll_ai += 1; }
        fn load_mouse_shapes(&mut self, variant: MouseVariant, probe_loose_file: bool) -> Option<usize> {
            self.loose_probe.push((variant, probe_loose_file));
            self.loaded
        }
        fn extract_shape(&mut self, shape_file: usize, frame: i32) -> Option<usize> {
            self.extracted.push(frame);
            Some(shape_file + frame as usize)
        }
        fn set_mouse_cursor(&mut self, x: i32, y: i32, shape: usize) {
            self.cursor_calls.push((x, y, shape));
        }
        fn animation_timer_expired(&mut self) -> bool { self.timer_expired }
        fn set_animation_timer(&mut self, ticks: i32) { self.timer = ticks; self.timer_expired = false; }
        fn bind_tiberian_dawn_vtable(&mut self) { self.vtable_bindings += 1; }
    }

    #[test]
    fn controls_and_enum_cardinalities_match_variant_data() {
        assert_eq!(REDALERT_MOUSE_COUNT, REDALERT_MOUSE_CONTROL.len());
        assert_eq!(TIBERIAN_DAWN_MOUSE_COUNT, TIBERIAN_DAWN_MOUSE_CONTROL.len());
        assert_eq!(REDALERT_MOUSE_COUNT, 42);
        assert_eq!(TIBERIAN_DAWN_MOUSE_COUNT, 34);
        assert_eq!(REDALERT_MOUSE_CONTROL[0].frame_rate, 0);
        assert_eq!(TIBERIAN_DAWN_MOUSE_CONTROL[0].frame_rate, 0);
    }

    #[test]
    fn override_load_fallback_and_mouse_size_switches_use_host_shapes() {
        let mut host = FakeHost { loaded: Some(1000), ..FakeHost::default() };
        let mut mouse = MouseController::new(MouseVariant::RedAlert);
        mouse.one_time(&mut host, true);
        assert!(mouse.set_default_mouse(&mut host, 1, true));
        assert!(!mouse.is_small); // compass cursor has no small frame.
        assert_eq!(host.cursor_calls.last().unwrap().2, 1001);
        mouse.set_default_mouse(&mut host, 18, true); // MOUSE_CAN_MOVE
        assert!(mouse.is_small);
        mouse.mouse_small(&mut host, false);
        assert!(!mouse.is_small);
    }

    #[test]
    fn animation_wraps_frame_and_delegates_scroll_ai() {
        let mut host = FakeHost { loaded: Some(100), timer_expired: true, ..FakeHost::default() };
        let mut mouse = MouseController::new(MouseVariant::RedAlert);
        mouse.one_time(&mut host, false);
        mouse.set_default_mouse(&mut host, 18, false);
        let mut input = 0;
        mouse.ai(&mut host, &mut input, 10, 20);
        assert_eq!(mouse.frame, 1);
        assert_eq!(host.timer, REDALERT_MOUSE_CONTROL[18].frame_rate);
        assert_eq!(host.scroll_ai, 1);
    }

    #[test]
    fn source_variant_file_probe_and_tiberian_dawn_binding_differ() {
        let mut host = FakeHost { loaded: Some(1), ..FakeHost::default() };
        MouseController::new(MouseVariant::RedAlert).one_time(&mut host, false);
        let mut td = MouseController::new(MouseVariant::TiberianDawn);
        td.one_time(&mut host, false);
        assert_eq!(host.loose_probe, vec![(MouseVariant::RedAlert, false), (MouseVariant::TiberianDawn, true)]);
        assert_eq!(host.vtable_bindings, 1);
        td.init_clear(&mut host);
        assert_eq!((host.scroll_one_time, host.scroll_clear), (2, 1));
    }
}
