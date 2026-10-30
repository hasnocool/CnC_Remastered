//! CreditClass from TIBERIANDAWN/CREDITS.CPP/H.

pub const CREDIT_MIN_ADDER: i32 = 1;
pub const CREDIT_MAX_ADDER: i32 = 143;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CreditSound { Up, Down }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CreditRenderRequest {
    pub x: i32,
    pub y: i32,
    pub value: i32,
    pub use_green_12_gradient: bool,
    pub centered: bool,
    pub use_gradient_palette: bool,
    pub text_color: i32,
}

pub trait CreditDisplayHost {
    fn resolution_factor(&self) -> u32;
    fn seen_buffer_width(&self) -> i32;
    fn play_credit_sound(&mut self, sound: CreditSound);
    fn draw_credits_tab(&mut self);
    fn draw_credit_value(&mut self, request: CreditRenderRequest);
    fn flag_map_for_redraw(&mut self, full_redraw: bool);
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreditClass {
    /// Target total obtained from HouseClass::Available_Money().
    pub credits: i32,
    /// Current value actually shown to the player.
    pub current: i32,
    pub is_to_redraw: bool,
    pub is_up: bool,
    pub is_audible: bool,
    countdown: i32,
}

impl Default for CreditClass {
    fn default() -> Self {
        Self {
            credits: 0,
            current: 0,
            is_to_redraw: false,
            is_up: false,
            is_audible: false,
            countdown: 0,
        }
    }
}

impl CreditClass {
    pub fn new() -> Self { Self::default() }
    pub const fn countdown(&self) -> i32 { self.countdown }

    /// Port of CreditClass::AI. None corresponds to a null HouseClass pointer.
    pub fn ai(
        &mut self,
        forced: bool,
        available_money: Option<i32>,
        logic_only: bool,
        host: &mut impl CreditDisplayHost,
    ) {
        let Some(money) = available_money else { return; };
        self.credits = money.max(0);

        if self.current == self.credits { return; }

        if forced {
            self.is_audible = false;
            self.current = self.credits;
        } else {
            if self.countdown != 0 { self.countdown -= 1; }
            if self.countdown != 0 { return; }

            // The source computes ABS(Credits - Current), then shifts right 5.
            let mut adder = self.credits.wrapping_sub(self.current).wrapping_abs();
            adder >>= 5;
            adder = adder.clamp(CREDIT_MIN_ADDER, CREDIT_MAX_ADDER);
            if self.current > self.credits { adder = adder.wrapping_neg(); }

            self.current = self.current.wrapping_add(adder);
            self.countdown = 1;

            if self.current.wrapping_sub(adder) != self.current {
                self.is_audible = true;
                self.is_up = adder > 0;
            }
        }

        self.is_to_redraw = true;
        if !logic_only {
            host.flag_map_for_redraw(false);
        }
    }

    /// Graphic_Logic performs sound and redraw only when forced or dirty.
    pub fn graphic_logic(&mut self, forced: bool, host: &mut impl CreditDisplayHost) {
        if !forced && !self.is_to_redraw { return; }

        if self.is_audible {
            host.play_credit_sound(if self.is_up { CreditSound::Up } else { CreditSound::Down });
        }

        let factor = host.resolution_factor();
        let x = host.seen_buffer_width().wrapping_sub(120i32.wrapping_shl(factor));
        host.draw_credits_tab();
        host.draw_credit_value(CreditRenderRequest {
            x,
            y: 0,
            value: self.current,
            use_green_12_gradient: true,
            centered: true,
            use_gradient_palette: true,
            text_color: 11,
        });

        self.is_to_redraw = false;
        self.is_audible = false;
    }

    /// Update delegates to AI; redraw=true is the source's forced update mode.
    pub fn update(
        &mut self,
        forced: bool,
        redraw: bool,
        available_money: Option<i32>,
        logic_only: bool,
        host: &mut impl CreditDisplayHost,
    ) {
        self.ai(forced || redraw, available_money, logic_only, host);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct FakeDisplay {
        factor: u32,
        width: i32,
        sounds: Vec<CreditSound>,
        tab_draws: usize,
        text_draws: Vec<CreditRenderRequest>,
        redraw_flags: Vec<bool>,
    }

    impl CreditDisplayHost for FakeDisplay {
        fn resolution_factor(&self) -> u32 { self.factor }
        fn seen_buffer_width(&self) -> i32 { self.width }
        fn play_credit_sound(&mut self, sound: CreditSound) { self.sounds.push(sound); }
        fn draw_credits_tab(&mut self) { self.tab_draws += 1; }
        fn draw_credit_value(&mut self, request: CreditRenderRequest) {
            self.text_draws.push(request);
        }
        fn flag_map_for_redraw(&mut self, full_redraw: bool) { self.redraw_flags.push(full_redraw); }
    }

    #[test]
    fn constructor_matches_legacy_null_display_state() {
        let credits = CreditClass::new();
        assert_eq!(credits.credits, 0);
        assert_eq!(credits.current, 0);
        assert!(!credits.is_to_redraw && !credits.is_up && !credits.is_audible);
        assert_eq!(credits.countdown(), 0);
    }

    #[test]
    fn ai_clamps_money_and_approaches_balance_by_clamped_thirty_second_step() {
        let mut credits = CreditClass::new();
        let mut host = FakeDisplay::default();
        credits.ai(false, Some(10_000), false, &mut host);
        assert_eq!(credits.credits, 10_000);
        assert_eq!(credits.current, 143);
        assert_eq!(credits.countdown(), 1);
        assert!(credits.is_audible && credits.is_up && credits.is_to_redraw);
        assert_eq!(host.redraw_flags, vec![false]);

        credits.ai(false, Some(10_000), false, &mut host);
        assert_eq!(credits.current, 286); // Countdown is reduced from 1 to 0 at call entry
        credits.ai(false, Some(10_000), false, &mut host);
        assert_eq!(credits.current, 429);
    }

    #[test]
    fn downward_animation_uses_negative_step_and_down_sound() {
        let mut credits = CreditClass { credits: 0, current: 3200, ..CreditClass::default() };
        let mut host = FakeDisplay::default();
        credits.ai(false, Some(0), true, &mut host);
        assert_eq!(credits.current, 3100);
        assert!(credits.is_audible);
        assert!(!credits.is_up);
        assert!(host.redraw_flags.is_empty());
        credits.graphic_logic(false, &mut host);
        assert_eq!(host.sounds, vec![CreditSound::Down]);
    }

    #[test]
    fn forced_update_snaps_to_total_without_sound() {
        let mut credits = CreditClass::new();
        let mut host = FakeDisplay::default();
        credits.ai(false, Some(500), false, &mut host);
        credits.ai(true, Some(45), false, &mut host);
        assert_eq!(credits.current, 45);
        assert!(!credits.is_audible);
        assert!(credits.is_to_redraw);
    }

    #[test]
    fn null_player_and_equal_balance_do_not_change_state() {
        let mut credits = CreditClass::new();
        let mut host = FakeDisplay::default();
        credits.ai(false, None, false, &mut host);
        assert_eq!(credits.credits, 0);
        credits.ai(false, Some(0), false, &mut host);
        assert!(host.redraw_flags.is_empty());
        assert!(!credits.is_to_redraw);
    }

    #[test]
    fn graphic_logic_uses_resolution_offset_and_clears_sound_dirty_flags() {
        let mut credits = CreditClass {
            credits: 100, current: 100, is_to_redraw: true, is_up: true,
            is_audible: true, countdown: 0,
        };
        let mut host = FakeDisplay { factor: 1, width: 640, ..FakeDisplay::default() };
        credits.graphic_logic(false, &mut host);
        assert_eq!(host.sounds, vec![CreditSound::Up]);
        assert_eq!(host.tab_draws, 1);
        assert_eq!(host.text_draws[0], CreditRenderRequest {
            x: 400, y: 0, value: 100, use_green_12_gradient: true,
            centered: true, use_gradient_palette: true, text_color: 11,
        });
        assert!(!credits.is_to_redraw && !credits.is_audible);
        credits.graphic_logic(false, &mut host);
        assert_eq!(host.tab_draws, 1);
        credits.graphic_logic(true, &mut host);
        assert_eq!(host.tab_draws, 2);
    }
}



/// Red Alert's separate frame-throttled credit display implementation.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RedAlertCreditAiContext {
    /// The original _last variable is static in CreditClass::AI and shared
    /// between every instance; pass one shared context to all instances.
    pub last_frame: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RedAlertCreditSound { MoneyUp, MoneyDown }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RedAlertCreditStyle {
    /// Non-Win32 path actively paints grey bright gradient text.
    GreyBrightGradient,
    /// Win32 Fancy_Text_Print call is commented out in the pinned source.
    Win32CreditTextDisabled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RedAlertCreditRenderRequest {
    pub x: i32,
    pub y: i32,
    pub value: i32,
    pub style: RedAlertCreditStyle,
}

pub trait RedAlertCreditDisplayHost {
    fn play_sound(&mut self, sound: RedAlertCreditSound);
    fn draw_credit_value(&mut self, request: RedAlertCreditRenderRequest);
    fn draw_mission_timer(&mut self, formatted_time: &str);
    fn flag_map_for_redraw(&mut self, full_redraw: bool);
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RedAlertCreditClass {
    pub credits: i32,
    pub current: i32,
    pub is_to_redraw: bool,
    pub is_up: bool,
    pub is_audible: bool,
    countdown: i32,
}

impl Default for RedAlertCreditClass {
    fn default() -> Self {
        Self {
            credits: 0,
            current: 0,
            is_to_redraw: false,
            is_up: false,
            is_audible: false,
            countdown: 0,
        }
    }
}

impl RedAlertCreditClass {
    pub fn new() -> Self { Self::default() }
    pub const fn countdown(&self) -> i32 { self.countdown }

    /// Port of REDALERT/CREDITS.CPP::AI. The caller supplies global Frame,
    /// money, and the mission-timer state; one shared context reproduces the
    /// function-static _last value.
    pub fn ai(
        &mut self,
        forced: bool,
        available_money: Option<i32>,
        logic_only: bool,
        frame: u32,
        mission_timer_active: bool,
        mission_timer_value: i32,
        context: &mut RedAlertCreditAiContext,
        host: &mut impl RedAlertCreditDisplayHost,
    ) {
        if !forced && !logic_only && frame == context.last_frame { return; }
        if !logic_only { context.last_frame = frame; }

        let Some(money) = available_money else { return; };
        self.credits = money.max(0);

        if mission_timer_active || mission_timer_value != 0 {
            self.is_to_redraw = true;
            host.flag_map_for_redraw(false);
        }

        if self.current == self.credits { return; }

        if forced {
            self.is_audible = false;
            self.current = self.credits;
        } else {
            if self.countdown != 0 { self.countdown -= 1; }
            if self.countdown != 0 { return; }

            let mut adder = self.credits.wrapping_sub(self.current);
            self.countdown = if adder > 0 { 1 } else { 3 };
            adder = adder.wrapping_abs();
            adder >>= 3;
            adder = adder.clamp(CREDIT_MIN_ADDER, CREDIT_MAX_ADDER);
            if self.current > self.credits { adder = adder.wrapping_neg(); }
            self.current = self.current.wrapping_add(adder);

            if self.current.wrapping_sub(adder) != self.current {
                self.is_audible = true;
                self.is_up = adder > 0;
            }
        }

        self.is_to_redraw = true;
        if !logic_only { host.flag_map_for_redraw(false); }
    }

    /// Red Alert's graphic path uses RESFACTOR and, under WIN32, adds an
    /// additional 80*RESFACTOR horizontal offset. The WIN32 credit text call
    /// is disabled; the mission timer text calls are also #if (0).
    pub fn graphic_logic(
        &mut self,
        forced: bool,
        seen_buffer_width: i32,
        resolution_factor: i32,
        win32: bool,
        mission_timer_active: bool,
        mission_timer_ticks: i32,
        ticks_per_second: i32,
        host: &mut impl RedAlertCreditDisplayHost,
    ) {
        if !forced && !self.is_to_redraw { return; }

        let x = seen_buffer_width
            .wrapping_sub(120i32.wrapping_mul(resolution_factor))
            .wrapping_add(if win32 { 80i32.wrapping_mul(resolution_factor) } else { 0 });

        if self.is_audible {
            host.play_sound(if self.is_up {
                RedAlertCreditSound::MoneyUp
            } else {
                RedAlertCreditSound::MoneyDown
            });
        }

        host.draw_credit_value(RedAlertCreditRenderRequest {
            x,
            y: 0,
            value: self.current,
            style: if win32 {
                RedAlertCreditStyle::Win32CreditTextDisabled
            } else {
                RedAlertCreditStyle::GreyBrightGradient
            },
        });

        // Timer formatting is active only in the non-Win32 branch. In Win32
        // both timer text calls are disabled under #if (0).
        if mission_timer_active && !win32 && ticks_per_second > 0 {
            let mut seconds = mission_timer_ticks / ticks_per_second;
            let mut minutes = seconds / 60;
            let hours = minutes / 60;
            seconds %= 60;
            minutes %= 60;
            let formatted = if hours != 0 {
                format!("{hours:02}:{minutes:02}:{seconds:02}")
            } else {
                format!("{minutes:02}:{seconds:02}")
            };
            host.draw_mission_timer(&formatted);
        }

        self.is_to_redraw = false;
        self.is_audible = false;
    }
}

#[cfg(test)]
mod red_alert_tests {
    use super::*;

    #[derive(Default)]
    struct FakeHost {
        sounds: Vec<RedAlertCreditSound>,
        credit_draws: Vec<RedAlertCreditRenderRequest>,
        timers: Vec<String>,
        redraws: Vec<bool>,
    }

    impl RedAlertCreditDisplayHost for FakeHost {
        fn play_sound(&mut self, sound: RedAlertCreditSound) { self.sounds.push(sound); }
        fn draw_credit_value(&mut self, request: RedAlertCreditRenderRequest) {
            self.credit_draws.push(request);
        }
        fn draw_mission_timer(&mut self, value: &str) { self.timers.push(value.to_owned()); }
        fn flag_map_for_redraw(&mut self, full_redraw: bool) { self.redraws.push(full_redraw); }
    }

    #[test]
    fn frame_throttle_is_shared_and_logic_only_bypasses_it() {
        let mut context = RedAlertCreditAiContext::default();
        let mut one = RedAlertCreditClass::new();
        let mut two = RedAlertCreditClass::new();
        let mut host = FakeHost::default();
        one.ai(false, Some(800), false, 10, false, 0, &mut context, &mut host);
        assert_eq!(one.current, 100);
        two.ai(false, Some(800), false, 10, false, 0, &mut context, &mut host);
        assert_eq!(two.current, 0);
        two.ai(false, Some(800), true, 10, false, 0, &mut context, &mut host);
        assert_eq!(two.current, 100);
    }

    #[test]
    fn up_and_down_changes_have_distinct_countdown_intervals() {
        let mut context = RedAlertCreditAiContext::default();
        let mut credits = RedAlertCreditClass::new();
        let mut host = FakeHost::default();

        credits.ai(false, Some(800), true, 0, false, 0, &mut context, &mut host);
        assert_eq!(credits.current, 100);
        assert_eq!(credits.countdown(), 1);
        credits.ai(false, Some(800), true, 1, false, 0, &mut context, &mut host);
        assert_eq!(credits.current, 200);

        credits.current = 800;
        credits.ai(false, Some(0), true, 2, false, 0, &mut context, &mut host);
        assert_eq!(credits.current, 700);
        assert_eq!(credits.countdown(), 3);
        credits.ai(false, Some(0), true, 3, false, 0, &mut context, &mut host);
        assert_eq!(credits.current, 700);
    }

    #[test]
    fn negative_money_is_clamped_and_mission_timer_marks_map_dirty() {
        let mut context = RedAlertCreditAiContext::default();
        let mut credits = RedAlertCreditClass::new();
        let mut host = FakeHost::default();
        credits.ai(false, Some(-500), false, 1, true, 1000, &mut context, &mut host);
        assert_eq!(credits.credits, 0);
        assert!(credits.is_to_redraw);
        assert_eq!(host.redraws, vec![false]);
    }

    #[test]
    fn forced_update_snaps_without_audible_effect() {
        let mut context = RedAlertCreditAiContext::default();
        let mut credits = RedAlertCreditClass::new();
        let mut host = FakeHost::default();
        credits.ai(false, Some(900), false, 1, false, 0, &mut context, &mut host);
        credits.ai(true, Some(55), false, 2, false, 0, &mut context, &mut host);
        assert_eq!(credits.current, 55);
        assert!(!credits.is_audible);
    }

    #[test]
    fn graphic_logic_formats_timer_only_in_non_win32_branch() {
        let mut credits = RedAlertCreditClass {
            credits: 100, current: 100, is_to_redraw: true, is_up: true,
            is_audible: true, countdown: 0,
        };
        let mut host = FakeHost::default();
        credits.graphic_logic(false, 640, 1, false, true, 125, 25, &mut host);
        assert_eq!(host.credit_draws[0].x, 520);
        assert_eq!(host.credit_draws[0].style, RedAlertCreditStyle::GreyBrightGradient);
        assert_eq!(host.timers, vec!["00:05"]);
        assert_eq!(host.sounds, vec![RedAlertCreditSound::MoneyUp]);

        credits.is_to_redraw = true;
        credits.graphic_logic(false, 640, 1, true, true, 125, 25, &mut host);
        assert_eq!(host.credit_draws[1].x, 600);
        assert_eq!(host.credit_draws[1].style, RedAlertCreditStyle::Win32CreditTextDisabled);
        assert_eq!(host.timers.len(), 1);
    }
}
