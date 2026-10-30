//! Red Alert TabClass from REDALERT/TAB.CPP.
//!
//! Red Alert uses an 8*RESFACTOR input band even though One_Time sets the
//! visible tab height to zero. The top-row interaction, money flash and
//! credit/mission timer pulse are preserved separately from the disabled
//! tab-bar drawing block.

use crate::credits::{
    RedAlertCreditAiContext, RedAlertCreditClass, RedAlertCreditDisplayHost,
};

pub const EVA_WIDTH: i32 = 80;
pub const TAB_HEIGHT: i32 = 8;
pub const MONEY_FLASH_DURATION: i32 = 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedAlertTabInput {
    None,
    LeftMouse,
    Other(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedAlertTabLabel {
    Controls,
    Sidebar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedAlertTabTextStyle {
    Win32MetalGradient,
    NonWin32GreyBright,
}

pub trait RedAlertTabRuntime: RedAlertCreditDisplayHost {
    fn seen_buffer_height(&self) -> i32;
    fn debug_map(&self) -> bool;
    fn is_win32(&self) -> bool;
    fn resolution_factor(&self) -> i32;

    fn sidebar_one_time(&mut self);
    fn sidebar_draw(&mut self, complete: bool);
    fn sidebar_flag_to_redraw(&mut self, full_redraw: bool);
    fn sidebar_ai(&mut self, input: &mut RedAlertTabInput, x: i32, y: i32);

    fn retrieve_tab_shape(&mut self, filename: &'static str) -> bool;
    fn draw_tab_shape(&mut self, frame: i32, x: i32, y: i32);
    fn draw_tab_text(
        &mut self,
        label: RedAlertTabLabel,
        x: i32,
        y: i32,
        style: RedAlertTabTextStyle,
    );
    fn set_metal_scheme_color(&mut self, color: u8);

    fn queue_options(&mut self);
    fn activate_map_sidebar(&mut self, control: i32);
    fn override_mouse_normal(&mut self, force: bool);

    fn player_available_money(&self) -> Option<i32>;
    fn current_frame(&self) -> u32;
    fn money_flash_timer_value(&self) -> i32;
    fn set_money_flash_timer(&mut self, ticks: i32);
    fn mission_timer_active(&self) -> bool;
    fn mission_timer_value(&self) -> i32;
    fn timer_warning_ticks(&self) -> i32;
    fn map_flasher_timer_active(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedAlertTabClass {
    pub is_to_redraw: bool,
    pub eva_width: i32,
    /// One_Time intentionally sets this to zero; AI uses TAB_HEIGHT instead.
    pub tab_height: i32,
    pub flasher_timer: i32,
    pub money_flash_timer: i32,
    pub credits: RedAlertCreditClass,
    tab_shape_loaded: bool,
}

impl Default for RedAlertTabClass {
    fn default() -> Self { Self::new() }
}

impl RedAlertTabClass {
    pub fn new() -> Self {
        Self {
            is_to_redraw: false,
            eva_width: 0,
            tab_height: 0,
            flasher_timer: 0,
            money_flash_timer: 0,
            credits: RedAlertCreditClass::new(),
            tab_shape_loaded: false,
        }
    }

    pub fn one_time(&mut self, runtime: &mut impl RedAlertTabRuntime) {
        let factor = runtime.resolution_factor();
        self.eva_width = EVA_WIDTH * factor;
        self.tab_height = 0;
        runtime.sidebar_one_time();
        self.tab_shape_loaded = runtime.retrieve_tab_shape("TABS.SHP");
    }

    pub fn redraw_tab(&mut self, runtime: &mut impl RedAlertTabRuntime) {
        self.is_to_redraw = true;
        runtime.sidebar_flag_to_redraw(false);
    }

    /// The source's tab/credit paint body is disabled by #if (0).
    pub fn draw_it(&mut self, complete: bool, runtime: &mut impl RedAlertTabRuntime) {
        runtime.sidebar_draw(complete);
        if runtime.debug_map() { return; }
        self.is_to_redraw = false;
    }

    /// Port of Draw_Credits_Tab with platform-specific frame sets.
    pub fn draw_credits_tab(&self, runtime: &mut impl RedAlertTabRuntime) {
        if !self.tab_shape_loaded { return; }
        let factor = runtime.resolution_factor();
        if runtime.is_win32() {
            let frame = if runtime.money_flash_timer_value() > 1 { 8 } else { 6 };
            runtime.draw_tab_shape(frame, (320 - EVA_WIDTH) * factor, 0);
        } else {
            runtime.draw_tab_shape(4, (320 - EVA_WIDTH * 2) * factor, 0);
        }

        if runtime.mission_timer_active() {
            let light = runtime.mission_timer_value() < runtime.timer_warning_ticks()
                || runtime.map_flasher_timer_active();
            if runtime.is_win32() {
                runtime.draw_tab_shape(if light { 4 } else { 2 }, 320 * factor, 0);
            } else {
                runtime.draw_tab_shape(
                    if light { 6 } else { 5 },
                    EVA_WIDTH * factor,
                    0,
                );
            }
        }
    }

    pub fn hilite_tab(&self, tab: i32, runtime: &mut impl RedAlertTabRuntime) {
        if !self.tab_shape_loaded { return; }
        let factor = runtime.resolution_factor();
        let selected_sidebar = tab != 0;
        let x = if selected_sidebar { (320 - EVA_WIDTH) * factor } else { 0 };
        if runtime.is_win32() {
            runtime.draw_tab_shape(1, x, 0);
            runtime.set_metal_scheme_color(134);
            // The Win32 source prints the Controls title even when tab != 0.
            runtime.draw_tab_text(
                RedAlertTabLabel::Controls,
                (EVA_WIDTH / 2) * factor,
                0,
                RedAlertTabTextStyle::Win32MetalGradient,
            );
            runtime.set_metal_scheme_color(128);
        } else {
            runtime.draw_tab_shape(if selected_sidebar { 1 } else { 3 }, x, 0);
            runtime.draw_tab_text(
                if selected_sidebar { RedAlertTabLabel::Sidebar } else { RedAlertTabLabel::Controls },
                if selected_sidebar {
                    (320 - EVA_WIDTH / 2) * factor
                } else {
                    (EVA_WIDTH / 2) * factor
                },
                0,
                RedAlertTabTextStyle::NonWin32GreyBright,
            );
        }
    }

    fn set_active(selection: i32, runtime: &mut impl RedAlertTabRuntime) {
        match selection {
            0 => runtime.queue_options(),
            1 => runtime.activate_map_sidebar(-1),
            _ => {}
        }
    }

    /// Top-row handling follows the Red Alert source: the visible click band
    /// remains active, but y==0 never activates a tab. The sidebar/right tab
    /// is absent on WIN32 because the sidebar is always active there.
    pub fn ai(
        &mut self,
        input: &mut RedAlertTabInput,
        x: i32,
        y: i32,
        runtime: &mut impl RedAlertTabRuntime,
        credit_context: &mut RedAlertCreditAiContext,
    ) {
        let width = runtime.seen_buffer_width();
        let factor = runtime.resolution_factor();
        if y >= 0 && y < TAB_HEIGHT * factor && x < width - 1 && x > 0 {
            if y > 0 {
                if *input == RedAlertTabInput::LeftMouse {
                    let mut selection = -1;
                    if x < EVA_WIDTH * factor { selection = 0; }
                    if !runtime.is_win32() && x > (320 - EVA_WIDTH) * factor {
                        selection = 1;
                    }
                    if selection >= 0 {
                        Self::set_active(selection, runtime);
                        *input = RedAlertTabInput::None;
                    }
                }
                runtime.override_mouse_normal(false);
            }
        }

        if runtime.money_flash_timer_value() == 1 {
            self.is_to_redraw = true;
            runtime.sidebar_flag_to_redraw(false);
        }

        self.credits.ai(
            false,
            runtime.player_available_money(),
            false,
            runtime.current_frame(),
            runtime.mission_timer_active(),
            runtime.mission_timer_value(),
            credit_context,
            runtime,
        );
        runtime.sidebar_ai(input, x, y);
    }

    /// Port of Flash_Money: mark the tab/sidebar for redraw and start the
    /// seven-tick money-flash timer.
    pub fn flash_money(&mut self, runtime: &mut impl RedAlertTabRuntime) {
        self.is_to_redraw = true;
        runtime.sidebar_flag_to_redraw(false);
        self.money_flash_timer = MONEY_FLASH_DURATION;
        runtime.set_money_flash_timer(MONEY_FLASH_DURATION);
    }

    pub const fn tab_shape_loaded(&self) -> bool { self.tab_shape_loaded }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::credits::{
        RedAlertCreditRenderRequest, RedAlertCreditSound, RedAlertCreditStyle,
    };

    #[derive(Default)]
    struct MockRuntime {
        width: i32,
        height: i32,
        factor: i32,
        win32: bool,
        debug: bool,
        shape_available: bool,
        timer_money: i32,
        mission_active: bool,
        mission_value: i32,
        warning_ticks: i32,
        map_flasher: bool,
        frame: u32,
        money: Option<i32>,
        calls: Vec<String>,
        shape_loads: Vec<&'static str>,
        shapes: Vec<(i32, i32, i32)>,
        labels: Vec<(RedAlertTabLabel, i32, i32, RedAlertTabTextStyle)>,
        metal_colors: Vec<u8>,
        sidebar_draws: Vec<bool>,
        sidebar_flags: Vec<bool>,
        sidebar_inputs: Vec<(RedAlertTabInput, i32, i32)>,
        options_queued: usize,
        map_activations: Vec<i32>,
        mouse_overrides: Vec<bool>,
        credit_sounds: Vec<RedAlertCreditSound>,
        credit_draws: Vec<RedAlertCreditRenderRequest>,
        timers: Vec<String>,
        redraws: Vec<bool>,
    }

    impl RedAlertCreditDisplayHost for MockRuntime {
        fn play_sound(&mut self, sound: RedAlertCreditSound) { self.credit_sounds.push(sound); }
        fn draw_credit_value(&mut self, request: RedAlertCreditRenderRequest) { self.credit_draws.push(request); }
        fn draw_mission_timer(&mut self, value: &str) { self.timers.push(value.to_owned()); }
        fn flag_map_for_redraw(&mut self, full_redraw: bool) { self.redraws.push(full_redraw); }
    }

    impl RedAlertTabRuntime for MockRuntime {
        fn seen_buffer_height(&self) -> i32 { self.height }
        fn debug_map(&self) -> bool { self.debug }
        fn is_win32(&self) -> bool { self.win32 }
        fn resolution_factor(&self) -> i32 { self.factor }
        fn sidebar_one_time(&mut self) { self.calls.push("sidebar-one-time".into()); }
        fn sidebar_draw(&mut self, complete: bool) { self.sidebar_draws.push(complete); }
        fn sidebar_flag_to_redraw(&mut self, full_redraw: bool) { self.sidebar_flags.push(full_redraw); }
        fn sidebar_ai(&mut self, input: &mut RedAlertTabInput, x: i32, y: i32) {
            self.sidebar_inputs.push((*input, x, y));
        }
        fn retrieve_tab_shape(&mut self, filename: &'static str) -> bool {
            self.shape_loads.push(filename);
            self.shape_available
        }
        fn draw_tab_shape(&mut self, frame: i32, x: i32, y: i32) { self.shapes.push((frame, x, y)); }
        fn draw_tab_text(&mut self, label: RedAlertTabLabel, x: i32, y: i32, style: RedAlertTabTextStyle) {
            self.labels.push((label, x, y, style));
        }
        fn set_metal_scheme_color(&mut self, color: u8) { self.metal_colors.push(color); }
        fn queue_options(&mut self) { self.options_queued += 1; }
        fn activate_map_sidebar(&mut self, control: i32) { self.map_activations.push(control); }
        fn override_mouse_normal(&mut self, force: bool) { self.mouse_overrides.push(force); }
        fn player_available_money(&self) -> Option<i32> { self.money }
        fn current_frame(&self) -> u32 { self.frame }
        fn money_flash_timer_value(&self) -> i32 { self.timer_money }
        fn set_money_flash_timer(&mut self, ticks: i32) { self.timer_money = ticks; }
        fn mission_timer_active(&self) -> bool { self.mission_active }
        fn mission_timer_value(&self) -> i32 { self.mission_value }
        fn timer_warning_ticks(&self) -> i32 { self.warning_ticks }
        fn map_flasher_timer_active(&self) -> bool { self.map_flasher }
    }

    fn runtime(win32: bool) -> MockRuntime {
        MockRuntime {
            width: 320, height: 200, factor: 1, win32,
            shape_available: true,
            ..MockRuntime::default()
        }
    }

    #[test]
    fn constructor_and_one_time_set_resolution_width_and_zero_tab_height() {
        let mut tabs = RedAlertTabClass::new();
        assert!(!tabs.is_to_redraw);
        assert_eq!(tabs.credits.current, 0);
        let mut host = runtime(false);
        tabs.one_time(&mut host);
        assert_eq!((tabs.eva_width, tabs.tab_height), (80, 0));
        assert_eq!(host.shape_loads, vec!["TABS.SHP"]);
        let mut high = MockRuntime { width: 640, height: 400, factor: 2, ..runtime(true) };
        tabs.one_time(&mut high);
        assert_eq!((tabs.eva_width, tabs.tab_height), (160, 0));
    }

    #[test]
    fn draw_it_delegates_to_sidebar_and_retains_dirty_flag_on_debug_map() {
        let mut tabs = RedAlertTabClass::new();
        let mut host = runtime(true);
        tabs.is_to_redraw = true;
        host.debug = true;
        tabs.draw_it(true, &mut host);
        assert_eq!(host.sidebar_draws, vec![true]);
        assert!(tabs.is_to_redraw);
        host.debug = false;
        tabs.draw_it(false, &mut host);
        assert!(!tabs.is_to_redraw);
        assert_eq!(host.sidebar_draws, vec![true, false]);
    }

    #[test]
    fn credit_tab_win32_uses_money_flash_frames_and_mission_timer_warning() {
        let tabs = RedAlertTabClass::new();
        let mut host = runtime(true);
        tabs.draw_credits_tab(&mut host);
        assert_eq!(host.shapes, vec![(6, 240, 0)]);
        host.shapes.clear();
        host.timer_money = 2;
        host.mission_active = true;
        host.mission_value = 50;
        host.warning_ticks = 100;
        tabs.draw_credits_tab(&mut host);
        assert_eq!(host.shapes, vec![(8, 240, 0), (4, 320, 0)]);
    }

    #[test]
    fn credit_tab_non_win32_frame_selection_uses_map_flasher_state() {
        let tabs = RedAlertTabClass::new();
        let mut host = runtime(false);
        host.mission_active = true;
        host.mission_value = 500;
        host.warning_ticks = 100;
        tabs.draw_credits_tab(&mut host);
        assert_eq!(host.shapes, vec![(4, 160, 0), (5, 80, 0)]);
        host.shapes.clear();
        host.map_flasher = true;
        tabs.draw_credits_tab(&mut host);
        assert_eq!(host.shapes, vec![(4, 160, 0), (6, 80, 0)]);
    }

    #[test]
    fn hilite_win32_always_uses_controls_text_and_restores_metal_color() {
        let tabs = RedAlertTabClass::new();
        let mut host = runtime(true);
        tabs.hilite_tab(1, &mut host);
        assert_eq!(host.shapes, vec![(1, 240, 0)]);
        assert_eq!(host.labels, vec![(RedAlertTabLabel::Controls, 40, 0, RedAlertTabTextStyle::Win32MetalGradient)]);
        assert_eq!(host.metal_colors, vec![134, 128]);
    }

    #[test]
    fn hilite_non_win32_selects_sidebar_frame_and_label() {
        let tabs = RedAlertTabClass::new();
        let mut host = runtime(false);
        tabs.hilite_tab(1, &mut host);
        assert_eq!(host.shapes, vec![(1, 240, 0)]);
        assert_eq!(host.labels, vec![(RedAlertTabLabel::Sidebar, 280, 0, RedAlertTabTextStyle::NonWin32GreyBright)]);
        tabs.hilite_tab(0, &mut host);
        assert_eq!(host.shapes[1], (3, 0, 0));
        assert_eq!(host.labels[1].0, RedAlertTabLabel::Controls);
    }

    #[test]
    fn mouse_top_band_requires_y_positive_and_left_click_opens_options() {
        let mut tabs = RedAlertTabClass::new();
        let mut host = runtime(true);
        tabs.one_time(&mut host);
        let mut top = RedAlertTabInput::LeftMouse;
        tabs.ai(&mut top, 20, 0, &mut host, &mut RedAlertCreditAiContext::default());
        assert_eq!(top, RedAlertTabInput::LeftMouse);
        assert!(host.options_queued == 0);
        let mut click = RedAlertTabInput::LeftMouse;
        tabs.ai(&mut click, 20, 1, &mut host, &mut RedAlertCreditAiContext::default());
        assert_eq!(click, RedAlertTabInput::None);
        assert_eq!(host.options_queued, 1);
    }

    #[test]
    fn right_sidebar_click_is_ignored_on_win32_but_active_on_non_win32() {
        let mut tabs = RedAlertTabClass::new();
        let mut win = runtime(true);
        let mut click = RedAlertTabInput::LeftMouse;
        tabs.ai(&mut click, 300, 1, &mut win, &mut RedAlertCreditAiContext::default());
        assert_eq!(click, RedAlertTabInput::LeftMouse);
        assert!(win.map_activations.is_empty());

        let mut dos = runtime(false);
        let mut click = RedAlertTabInput::LeftMouse;
        tabs.ai(&mut click, 300, 1, &mut dos, &mut RedAlertCreditAiContext::default());
        assert_eq!(click, RedAlertTabInput::None);
        assert_eq!(dos.map_activations, vec![-1]);
    }

    #[test]
    fn money_flash_timer_one_flags_redraw_and_flash_money_sets_seven_ticks() {
        let mut tabs = RedAlertTabClass::new();
        let mut host = runtime(true);
        host.timer_money = 1;
        tabs.ai(&mut RedAlertTabInput::None, 100, 100, &mut host, &mut RedAlertCreditAiContext::default());
        assert!(tabs.is_to_redraw);
        assert_eq!(host.sidebar_flags, vec![false]);

        tabs.flash_money(&mut host);
        assert_eq!(tabs.money_flash_timer, 7);
        assert_eq!(host.timer_money, 7);
        assert!(tabs.is_to_redraw);
        assert_eq!(host.sidebar_flags, vec![false, false]);
    }

    #[test]
    fn credit_ai_uses_shared_frame_context_and_available_money() {
        let mut tabs = RedAlertTabClass::new();
        let mut host = runtime(false);
        host.money = Some(800);
        host.frame = 5;
        let mut context = RedAlertCreditAiContext::default();
        tabs.ai(&mut RedAlertTabInput::None, 150, 100, &mut host, &mut context);
        assert_eq!(tabs.credits.credits, 800);
        assert_eq!(tabs.credits.current, 100);
        assert_eq!(context.last_frame, 5);
        assert_eq!(host.redraws, vec![false]);
    }

    #[test]
    fn draw_credits_tab_does_nothing_when_tab_shape_is_missing() {
        let tabs = RedAlertTabClass::new();
        let mut host = MockRuntime { shape_available: false, ..runtime(true) };
        let mut tabs = tabs;
        tabs.one_time(&mut host);
        tabs.draw_credits_tab(&mut host);
        assert!(host.shapes.is_empty());
    }
}
