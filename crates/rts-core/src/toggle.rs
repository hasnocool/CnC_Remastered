//! ToggleClass state and action dispatch from both variants' TOGGLE.CPP.

pub const LEFT_PRESS: u32 = 0x0001;
pub const LEFT_RELEASE: u32 = 0x0004;
pub const KN_NONE: u16 = 0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ToggleVariant { RedAlert, TiberianDawn }

pub trait ToggleHost {
    fn sticky_process(&mut self, flags: u32);
    fn flag_to_redraw(&mut self);
    fn control_action(&mut self, flags: u32, key: &mut u16) -> bool;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ToggleState {
    pub variant: ToggleVariant,
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub action_flags: u32,
    pub enabled: bool,
    pub is_pressed: bool,
    pub is_on: bool,
    pub is_toggle_type: bool,
}

impl ToggleState {
    pub fn new(variant: ToggleVariant, id: u32, x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            variant, id, x, y, width, height,
            action_flags: LEFT_PRESS | LEFT_RELEASE, enabled: true,
            is_pressed: false, is_on: false, is_toggle_type: false,
        }
    }

    pub fn turn_on(&mut self, host: &mut impl ToggleHost) {
        self.is_on = true;
        host.flag_to_redraw();
    }

    pub fn turn_off(&mut self, host: &mut impl ToggleHost) {
        self.is_on = false;
        host.flag_to_redraw();
    }

    fn over_button(&self, mouse_x: i32, mouse_y: i32) -> bool {
        let dx = mouse_x.wrapping_sub(self.x);
        let dy = mouse_y.wrapping_sub(self.y);
        match self.variant {
            // Red Alert's signed comparisons omit the lower-bound checks.
            ToggleVariant::RedAlert => dx < self.width && dy < self.height,
            // Tiberian Dawn casts the deltas to unsigned before comparison.
            ToggleVariant::TiberianDawn => dx >= 0 && dy >= 0 && dx < self.width && dy < self.height,
        }
    }

    /// Returns ControlClass::Action's result, except LEFTPRESS consumes the
    /// key and returns true exactly as the source does.
    pub fn action(
        &mut self,
        mut flags: u32,
        key: &mut u16,
        mouse_x: i32,
        mouse_y: i32,
        host: &mut impl ToggleHost,
    ) -> bool {
        let overbutton = self.over_button(mouse_x, mouse_y);
        if flags == 0 {
            if overbutton {
                if !self.is_pressed {
                    self.is_pressed = true;
                    host.flag_to_redraw();
                }
            } else if self.is_pressed {
                self.is_pressed = false;
                host.flag_to_redraw();
            }
        }

        host.sticky_process(flags);

        if flags & LEFT_PRESS != 0 {
            self.is_pressed = true;
            host.flag_to_redraw();
            flags &= !LEFT_PRESS;
            let _ = host.control_action(flags, key);
            *key = KN_NONE;
            return true;
        }

        if flags & LEFT_RELEASE != 0 {
            if self.is_pressed {
                let toggle = self.is_toggle_type
                    && (self.variant == ToggleVariant::TiberianDawn || overbutton);
                if toggle { self.is_on = !self.is_on; }
                self.is_pressed = false;
                if self.variant == ToggleVariant::RedAlert {
                    host.flag_to_redraw();
                }
            } else {
                flags &= !LEFT_RELEASE;
            }
        }
        host.control_action(flags, key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct FakeHost { sticky: Vec<u32>, redraws: usize, base: Vec<u32>, base_result: bool }
    impl ToggleHost for FakeHost {
        fn sticky_process(&mut self, flags: u32) { self.sticky.push(flags); }
        fn flag_to_redraw(&mut self) { self.redraws += 1; }
        fn control_action(&mut self, flags: u32, _key: &mut u16) -> bool {
            self.base.push(flags); self.base_result
        }
    }

    #[test]
    fn constructor_and_turn_on_off_match_toggle_state() {
        let mut state = ToggleState::new(ToggleVariant::RedAlert, 5, 10, 20, 30, 12);
        assert_eq!(state.action_flags, LEFT_PRESS | LEFT_RELEASE);
        assert!(!state.is_pressed && !state.is_on && !state.is_toggle_type);
        let mut host = FakeHost::default();
        state.turn_on(&mut host);
        assert!(state.is_on);
        state.turn_off(&mut host);
        assert!(!state.is_on);
        assert_eq!(host.redraws, 2);
    }

    #[test]
    fn left_press_captures_focus_consumes_key_and_delegates_without_press_flag() {
        let mut state = ToggleState::new(ToggleVariant::RedAlert, 1, 0, 0, 20, 20);
        let mut host = FakeHost { base_result: false, ..Default::default() };
        let mut key = 0x41;
        assert!(state.action(LEFT_PRESS, &mut key, 5, 5, &mut host));
        assert!(state.is_pressed);
        assert_eq!(key, KN_NONE);
        assert_eq!(host.base, [0]);
        assert_eq!(host.sticky, [LEFT_PRESS]);
    }

    #[test]
    fn red_alert_only_toggles_when_release_is_over_button() {
        let mut state = ToggleState::new(ToggleVariant::RedAlert, 1, 10, 10, 20, 20);
        state.is_toggle_type = true;
        state.is_pressed = true;
        let mut host = FakeHost::default();
        let mut key = 0;
        state.action(LEFT_RELEASE, &mut key, 100, 100, &mut host);
        assert!(!state.is_on);
        assert!(!state.is_pressed);
        assert_eq!(host.redraws, 1);
    }

    #[test]
    fn tiberian_dawn_toggles_on_release_even_outside_button() {
        let mut state = ToggleState::new(ToggleVariant::TiberianDawn, 1, 10, 10, 20, 20);
        state.is_toggle_type = true;
        state.is_pressed = true;
        let mut host = FakeHost::default();
        state.action(LEFT_RELEASE, &mut 0, 100, 100, &mut host);
        assert!(state.is_on);
        assert!(!state.is_pressed);
        assert_eq!(host.redraws, 0);
    }

    #[test]
    fn forced_call_uses_variant_specific_negative_coordinate_test() {
        let mut ra = ToggleState::new(ToggleVariant::RedAlert, 1, 10, 10, 20, 20);
        let mut td = ToggleState::new(ToggleVariant::TiberianDawn, 1, 10, 10, 20, 20);
        let mut host = FakeHost::default();
        let mut key = 0;
        ra.action(0, &mut key, 0, 0, &mut host);
        td.action(0, &mut key, 0, 0, &mut host);
        assert!(ra.is_pressed);
        assert!(!td.is_pressed);
    }

    #[test]
    fn release_without_prior_press_is_removed_before_base_action() {
        let mut state = ToggleState::new(ToggleVariant::RedAlert, 1, 0, 0, 20, 20);
        let mut host = FakeHost { base_result: true, ..Default::default() };
        let mut key = 4;
        assert!(state.action(LEFT_RELEASE, &mut key, 5, 5, &mut host));
        assert_eq!(host.base, [0]);
        assert_eq!(key, 4);
    }
}
