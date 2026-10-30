//! Tiberian Dawn TabClass from TIBERIANDAWN/TAB.CPP.
//!
//! The 2019 upstream change disables the main tab-bar painting block with
//! #if 0. Draw_It still delegates to SidebarClass, respects Debug_Map, and
//! clears its dirty bit. The remaining active click/highlight/credits and
//! one-time initialization behavior is represented through TabRuntime.

use crate::credits::{CreditClass, CreditDisplayHost};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabInput {
    None,
    LeftMouse,
    Other(i32),
}

pub trait TabRuntime: CreditDisplayHost {
    fn seen_buffer_height(&self) -> i32;
    fn debug_map(&self) -> bool;
    fn special_scroll_mod_active(&self) -> bool;

    fn sidebar_one_time(&mut self);
    fn sidebar_draw(&mut self, complete: bool);
    fn sidebar_flag_to_redraw(&mut self, full_redraw: bool);
    fn sidebar_ai(&mut self, input: &mut TabInput, x: i32, y: i32);

    fn retrieve_hires_shape(&mut self, filename: &'static str) -> bool;
    fn draw_tab_shape(&mut self, frame: i32, x: i32, y: i32);
    fn draw_tab_controls_title(&mut self, x: i32, y: i32);

    fn queue_options(&mut self);
    fn activate_map_sidebar(&mut self, control: i32);
    fn override_mouse_normal(&mut self, force: bool);

    /// Equivalent to PlayerPtr->Available_Money(); None models a null PlayerPtr.
    fn player_available_money(&self) -> Option<i32>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabClass {
    pub is_to_redraw: bool,
    pub eva_width: i32,
    pub tab_height: i32,
    pub credits: CreditClass,
    tab_shape_loaded: bool,
}

impl Default for TabClass {
    fn default() -> Self {
        Self::new()
    }
}

impl TabClass {
    pub fn new() -> Self {
        Self {
            is_to_redraw: false,
            eva_width: 0,
            tab_height: 0,
            credits: CreditClass::new(),
            tab_shape_loaded: false,
        }
    }

    /// Port of TabClass::One_Time.
    pub fn one_time(&mut self, runtime: &mut impl TabRuntime) {
        let factor = if runtime.seen_buffer_width() == 320 { 1 } else { 2 };
        self.eva_width = 80 * factor;
        // The active source intentionally sets Tab_Height to zero.
        self.tab_height = 0;
        runtime.sidebar_one_time();
        self.tab_shape_loaded = runtime.retrieve_hires_shape("TABS.SHP");
    }

    /// Port of TabClass::Redraw_Tab.
    pub fn redraw_tab(&mut self, runtime: &mut impl TabRuntime) {
        self.is_to_redraw = true;
        runtime.sidebar_flag_to_redraw(false);
    }

    /// Port of Draw_It with the source's inactive #if 0 block omitted.
    pub fn draw_it(&mut self, complete: bool, runtime: &mut impl TabRuntime) {
        runtime.sidebar_draw(complete);
        if runtime.debug_map() {
            return;
        }
        self.is_to_redraw = false;
    }

    /// Port of Draw_Credits_Tab. The TabShape pointer is shared in C++; the
    /// runtime owns the corresponding shared asset and this instance records
    /// whether One_Time successfully retrieved it.
    pub fn draw_credits_tab(&self, runtime: &mut impl TabRuntime) {
        if self.tab_shape_loaded {
            runtime.draw_tab_shape(0, 320, 0);
        }
    }

    /// Port of Hilite_Tab. The source does not use its tab argument.
    pub fn hilite_tab(&self, _tab: i32, runtime: &mut impl TabRuntime) {
        if self.tab_shape_loaded {
            runtime.draw_tab_shape(1, 0, 0);
            runtime.draw_tab_controls_title(80, 0);
        }
    }

    fn set_active(selection: i32, runtime: &mut impl TabRuntime) {
        match selection {
            0 => runtime.queue_options(),
            1 => runtime.activate_map_sidebar(-1),
            _ => {}
        }
    }

    /// Port of TabClass::AI. A top-left click opens the options menu; a
    /// top-right click activates the map sidebar. The source clears consumed
    /// mouse input before passing it to SidebarClass::AI.
    pub fn ai(
        &mut self,
        input: &mut TabInput,
        x: i32,
        y: i32,
        runtime: &mut impl TabRuntime,
    ) {
        let width = runtime.seen_buffer_width();

        if y >= 0 && y < self.tab_height && x < width - 1 && x > 0 {
            let mut allowed = false;
            if y > 0
                || (runtime.special_scroll_mod_active()
                    && ((x > 3 && x < self.eva_width)
                        || (x < width - 3 && x > width - self.eva_width)))
            {
                allowed = true;
            }

            if allowed {
                if *input == TabInput::LeftMouse {
                    let mut selection = -1;
                    if x < self.eva_width {
                        selection = 0;
                    }
                    if x > width - self.eva_width {
                        selection = 1;
                    }
                    if selection >= 0 {
                        Self::set_active(selection, runtime);
                        *input = TabInput::None;
                    }
                }
                runtime.override_mouse_normal(false);
            }
        }

        self.credits.ai(false, runtime.player_available_money(), false, runtime);
        runtime.sidebar_ai(input, x, y);
    }

    pub const fn tab_shape_loaded(&self) -> bool {
        self.tab_shape_loaded
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::credits::{CreditRenderRequest, CreditSound};

    #[derive(Default)]
    struct MockRuntime {
        width: i32,
        height: i32,
        debug: bool,
        scroll_mod: bool,
        shape_available: bool,
        shape_loads: Vec<&'static str>,
        sidebar_one_time_calls: usize,
        sidebar_draw_calls: Vec<bool>,
        redraw_flags: Vec<bool>,
        sidebar_inputs: Vec<(TabInput, i32, i32)>,
        shape_draws: Vec<(i32, i32, i32)>,
        title_draws: Vec<(i32, i32)>,
        option_activations: usize,
        map_activations: Vec<i32>,
        mouse_overrides: Vec<bool>,
        money: Option<i32>,
        credit_sounds: Vec<CreditSound>,
        credit_text_draws: Vec<CreditRenderRequest>,
        credit_tab_draws: usize,
    }

    impl CreditDisplayHost for MockRuntime {
        fn resolution_factor(&self) -> u32 {
            if self.width == 320 { 0 } else { 1 }
        }
        fn seen_buffer_width(&self) -> i32 { self.width }
        fn play_credit_sound(&mut self, sound: CreditSound) { self.credit_sounds.push(sound); }
        fn draw_credits_tab(&mut self) { self.credit_tab_draws += 1; }
        fn draw_credit_value(&mut self, request: CreditRenderRequest) { self.credit_text_draws.push(request); }
        fn flag_map_for_redraw(&mut self, full_redraw: bool) { self.redraw_flags.push(full_redraw); }
    }

    impl TabRuntime for MockRuntime {
        fn seen_buffer_height(&self) -> i32 { self.height }
        fn debug_map(&self) -> bool { self.debug }
        fn special_scroll_mod_active(&self) -> bool { self.scroll_mod }
        fn sidebar_one_time(&mut self) { self.sidebar_one_time_calls += 1; }
        fn sidebar_draw(&mut self, complete: bool) { self.sidebar_draw_calls.push(complete); }
        fn sidebar_flag_to_redraw(&mut self, full_redraw: bool) { self.redraw_flags.push(full_redraw); }
        fn sidebar_ai(&mut self, input: &mut TabInput, x: i32, y: i32) {
            self.sidebar_inputs.push((*input, x, y));
        }
        fn retrieve_hires_shape(&mut self, filename: &'static str) -> bool {
            self.shape_loads.push(filename);
            self.shape_available
        }
        fn draw_tab_shape(&mut self, frame: i32, x: i32, y: i32) {
            self.shape_draws.push((frame, x, y));
        }
        fn draw_tab_controls_title(&mut self, x: i32, y: i32) { self.title_draws.push((x, y)); }
        fn queue_options(&mut self) { self.option_activations += 1; }
        fn activate_map_sidebar(&mut self, control: i32) { self.map_activations.push(control); }
        fn override_mouse_normal(&mut self, force: bool) { self.mouse_overrides.push(force); }
        fn player_available_money(&self) -> Option<i32> { self.money }
    }

    fn runtime() -> MockRuntime {
        MockRuntime {
            width: 320,
            height: 200,
            shape_available: true,
            ..MockRuntime::default()
        }
    }

    #[test]
    fn constructor_matches_default_dirty_and_credit_state() {
        let tabs = TabClass::new();
        assert!(!tabs.is_to_redraw);
        assert_eq!((tabs.eva_width, tabs.tab_height), (0, 0));
        assert!(!tabs.tab_shape_loaded());
        assert_eq!(tabs.credits.current, 0);
    }

    #[test]
    fn one_time_uses_resolution_factor_disables_bar_and_loads_shape() {
        let mut tabs = TabClass::new();
        let mut low = runtime();
        tabs.one_time(&mut low);
        assert_eq!((tabs.eva_width, tabs.tab_height), (80, 0));
        assert_eq!(low.sidebar_one_time_calls, 1);
        assert_eq!(low.shape_loads, vec!["TABS.SHP"]);
        assert!(tabs.tab_shape_loaded());

        let mut high = MockRuntime { width: 640, height: 400, shape_available: true, ..MockRuntime::default() };
        tabs.one_time(&mut high);
        assert_eq!((tabs.eva_width, tabs.tab_height), (160, 0));
    }

    #[test]
    fn one_time_keeps_shape_unavailable_when_asset_lookup_fails() {
        let mut tabs = TabClass::new();
        let mut host = MockRuntime { width: 320, shape_available: false, ..MockRuntime::default() };
        tabs.one_time(&mut host);
        assert!(!tabs.tab_shape_loaded());
        tabs.draw_credits_tab(&mut host);
        tabs.hilite_tab(4, &mut host);
        assert!(host.shape_draws.is_empty());
    }

    #[test]
    fn draw_it_delegates_and_clears_dirty_unless_debug_map_returns_early() {
        let mut tabs = TabClass::new();
        let mut host = runtime();
        tabs.redraw_tab(&mut host);
        assert!(tabs.is_to_redraw);
        assert_eq!(host.redraw_flags, vec![false]);

        host.debug = true;
        tabs.draw_it(true, &mut host);
        assert!(tabs.is_to_redraw);
        assert_eq!(host.sidebar_draw_calls, vec![true]);

        host.debug = false;
        tabs.draw_it(false, &mut host);
        assert!(!tabs.is_to_redraw);
        assert_eq!(host.sidebar_draw_calls, vec![true, false]);
    }

    #[test]
    fn draw_credits_tab_and_highlight_use_the_source_frames_and_positions() {
        let mut tabs = TabClass::new();
        let mut host = runtime();
        tabs.one_time(&mut host);
        tabs.draw_credits_tab(&mut host);
        tabs.hilite_tab(99, &mut host);
        assert_eq!(host.shape_draws, vec![(0, 320, 0), (1, 0, 0)]);
        assert_eq!(host.title_draws, vec![(80, 0)]);
    }

    #[test]
    fn disabled_zero_height_tab_region_does_not_consume_mouse_clicks() {
        let mut tabs = TabClass::new();
        let mut host = runtime();
        tabs.one_time(&mut host);
        let mut input = TabInput::LeftMouse;
        tabs.ai(&mut input, 20, 0, &mut host);
        assert_eq!(input, TabInput::LeftMouse);
        assert!(host.mouse_overrides.is_empty());
        assert!(host.option_activations == 0 && host.map_activations.is_empty());
        assert_eq!(host.sidebar_inputs.last(), Some(&(TabInput::LeftMouse, 20, 0)));
    }

    #[test]
    fn left_and_right_tabs_activate_options_and_map_and_clear_consumed_input() {
        let mut tabs = TabClass::new();
        let mut host = runtime();
        tabs.one_time(&mut host);
        // The active One_Time sets height to zero; set a hit region to verify the
        // still-live AI code path independently of disabled tab painting.
        tabs.tab_height = 8;

        let mut left = TabInput::LeftMouse;
        tabs.ai(&mut left, 30, 1, &mut host);
        assert_eq!(left, TabInput::None);
        assert_eq!(host.option_activations, 1);

        let mut right = TabInput::LeftMouse;
        tabs.ai(&mut right, 300, 1, &mut host);
        assert_eq!(right, TabInput::None);
        assert_eq!(host.map_activations, vec![-1]);
        assert_eq!(host.mouse_overrides, vec![false, false]);
    }

    #[test]
    fn scroll_mod_allows_top_row_tab_hits_but_without_it_they_are_ignored() {
        let mut tabs = TabClass::new();
        let mut host = runtime();
        tabs.one_time(&mut host);
        tabs.tab_height = 8;

        let mut blocked = TabInput::LeftMouse;
        tabs.ai(&mut blocked, 30, 0, &mut host);
        assert_eq!(blocked, TabInput::LeftMouse);
        assert!(host.option_activations == 0);

        host.scroll_mod = true;
        let mut allowed = TabInput::LeftMouse;
        tabs.ai(&mut allowed, 30, 0, &mut host);
        assert_eq!(allowed, TabInput::None);
        assert_eq!(host.option_activations, 1);
    }

    #[test]
    fn credits_ai_runs_before_sidebar_ai_and_updates_credit_state() {
        let mut tabs = TabClass::new();
        let mut host = runtime();
        host.money = Some(1_000);
        let mut input = TabInput::Other(12);
        tabs.ai(&mut input, 160, 100, &mut host);
        assert_eq!(tabs.credits.credits, 1_000);
        assert_eq!(tabs.credits.current, 31);
        assert!(tabs.credits.is_to_redraw);
        assert_eq!(host.redraw_flags, vec![false]);
        assert_eq!(host.sidebar_inputs, vec![(TabInput::Other(12), 160, 100)]);
    }

    #[test]
    fn selected_hit_region_overwrites_right_edge_selection_like_cpp() {
        let mut tabs = TabClass::new();
        let mut host = runtime();
        host.width = 320;
        tabs.eva_width = 200;
        tabs.tab_height = 8;
        let mut input = TabInput::LeftMouse;
        tabs.ai(&mut input, 180, 1, &mut host);
        // Both x comparisons match when the two tab regions overlap; the
        // second (right-side) assignment wins in the original source.
        assert_eq!(host.map_activations, vec![-1]);
    }
}
