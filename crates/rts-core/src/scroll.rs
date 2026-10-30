//! ScrollClass active behavior from both variants' SCROLL.CPP.
//!
//! The tactical-map scrolling algorithm is enclosed in #if 0 in both source
//! files. The compiled AI path delegates directly to HelpClass::AI.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScrollVariant { RedAlert, TiberianDawn }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScrollState {
    pub variant: ScrollVariant,
    pub is_auto_scroll: bool,
    pub inertia: i32,
    pub counter: i32,
    pub counter_started: bool,
    pub scroll_delay: i32,
}

impl ScrollState {
    pub fn new(variant: ScrollVariant, win32: bool) -> Self {
        let delay = match variant {
            ScrollVariant::RedAlert if !win32 => 2,
            _ => 1,
        };
        Self {
            variant, is_auto_scroll: true, inertia: 0, counter: delay,
            counter_started: variant == ScrollVariant::TiberianDawn,
            scroll_delay: delay,
        }
    }

    /// Returns the previous autoscroll state. -1 toggles; all other values
    /// are converted to bool exactly as the C++ assignment does.
    pub fn set_autoscroll(&mut self, control: i32) -> bool {
        let old = self.is_auto_scroll;
        if control == -1 { self.is_auto_scroll = !self.is_auto_scroll; }
        else { self.is_auto_scroll = control != 0; }
        old
    }

    /// ScrollClass::AI's active path delegates to HelpClass::AI. The
    /// disabled edge-scroll block has no runtime effect in the pinned source.
    pub fn ai<R>(&self, input: u16, x: i32, y: i32, help_ai: impl FnOnce(u16, i32, i32) -> R) -> R {
        help_ai(input, x, y)
    }

    /// Init_IO resets the shared countdown before invoking HelpClass::Init_IO.
    pub fn init_io<R>(&mut self, help_init_io: impl FnOnce() -> R) -> R {
        self.counter = 0;
        help_init_io()
    }
}

impl Default for ScrollState {
    fn default() -> Self { Self::new(ScrollVariant::RedAlert, true) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors_preserve_variant_timer_setup() {
        let ra = ScrollState::new(ScrollVariant::RedAlert, true);
        assert_eq!((ra.counter, ra.scroll_delay, ra.inertia), (1, 1, 0));
        assert!(!ra.counter_started);
        let dos_ra = ScrollState::new(ScrollVariant::RedAlert, false);
        assert_eq!(dos_ra.counter, 2);
        let td = ScrollState::new(ScrollVariant::TiberianDawn, false);
        assert_eq!(td.counter, 1);
        assert!(td.counter_started);
    }

    #[test]
    fn autoscroll_returns_previous_state_and_handles_toggle_sentinel() {
        let mut state = ScrollState::default();
        assert!(state.set_autoscroll(0));
        assert!(!state.is_auto_scroll);
        assert!(!state.set_autoscroll(-1));
        assert!(state.is_auto_scroll);
        assert!(state.set_autoscroll(2));
        assert!(state.is_auto_scroll);
    }

    #[test]
    fn ai_delegates_without_running_disabled_map_scroll_logic() {
        let state = ScrollState::default();
        let result = state.ai(42, 100, 200, |input, x, y| (input, x, y));
        assert_eq!(result, (42, 100, 200));
    }

    #[test]
    fn init_io_resets_counter_before_base_callback() {
        let mut state = ScrollState::default();
        state.counter = 9;
        let result = state.init_io(|| 123);
        assert_eq!(result, 123);
        assert_eq!(state.counter, 0);
    }
}
