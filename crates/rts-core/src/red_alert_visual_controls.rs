//! Red Alert VisualControlsClass from REDALERT/VISUDLG.CPP.
//!
//! This is separate from Tiberian Dawn's visually similar dialog. Red Alert
//! writes four option values as slider_value / 256 and scales SliderClass
//! thumb units by RESFACTOR. The modal loop is represented as frame steps.

pub const BUTTON_BRIGHTNESS: i32 = 1;
pub const BUTTON_COLOR: i32 = 2;
pub const BUTTON_CONTRAST: i32 = 3;
pub const BUTTON_TINT: i32 = 4;
pub const BUTTON_RESET: i32 = 5;
pub const BUTTON_OPTIONS: i32 = 6;
pub const SLIDER_MAXIMUM: u16 = 255;
pub const THUMB_BASE_UNITS: u16 = 20;
pub const RESET_SLIDER_VALUE: u16 = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VisualSetting {
    Brightness,
    Color,
    Contrast,
    Tint,
}

impl VisualSetting {
    pub const ALL: [Self; 4] = [Self::Brightness, Self::Color, Self::Contrast, Self::Tint];
    pub const fn index(self) -> usize {
        match self {
            Self::Brightness => 0, Self::Color => 1,
            Self::Contrast => 2, Self::Tint => 3,
        }
    }
    pub const fn button_id(self) -> i32 { self.index() as i32 + BUTTON_BRIGHTNESS }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RedAlertVisualLayout {
    pub factor: i32,
    pub option_x: i32, pub option_y: i32,
    pub option_width: i32, pub option_height: i32,
    pub text_x: i32, pub text_y: i32,
    pub slider_x: i32, pub slider_y: i32,
    pub slider_width: i32, pub slider_height: i32,
    pub slider_y_spacing: i32,
    pub button_y: i32,
    pub reset_x: i32, pub reset_width: i32,
    pub options_x: i32, pub options_width: i32,
    pub thumb_units: u16,
    pub slider_maximum: u16,
}

impl RedAlertVisualLayout {
    pub const fn new(seen_width: i32, seen_height: i32) -> Self {
        let factor = if seen_width == 320 { 1 } else { 2 };
        let option_width = 216 * factor;
        let option_height = 122 * factor;
        let option_x = (seen_width - option_width) / 2;
        let option_y = (seen_height - option_height) / 2;
        let options_width = 60 * factor;
        let reset_width = 80 * factor;
        let thumb_units = THUMB_BASE_UNITS * factor as u16;
        Self {
            factor, option_x, option_y, option_width, option_height,
            text_x: option_x + 28 * factor,
            text_y: option_y + 30 * factor,
            slider_x: option_x + 105 * factor,
            slider_y: option_y + 30 * factor,
            slider_width: 70 * factor,
            slider_height: 5 * factor,
            slider_y_spacing: 11 * factor,
            button_y: option_y + 102 * factor,
            reset_x: option_x + 17 * factor,
            reset_width,
            options_x: option_x + option_width - (options_width + 17 * factor),
            options_width,
            thumb_units,
            slider_maximum: SLIDER_MAXIMUM,
        }
    }

    pub const fn slider_y_for(&self, setting: VisualSetting) -> i32 {
        self.slider_y + setting.index() as i32 * self.slider_y_spacing
    }

    pub const fn title_y_for(&self, setting: VisualSetting) -> i32 {
        self.text_y + setting.index() as i32 * self.slider_y_spacing
    }

    pub const fn effective_maximum(&self) -> u16 {
        if self.slider_maximum > self.thumb_units {
            self.slider_maximum - self.thumb_units
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VisualOptionValues {
    /// Fixed normalized option values represented as numerators over 256.
    pub brightness: u16,
    pub color: u16,
    pub contrast: u16,
    pub tint: u16,
}

impl VisualOptionValues {
    pub const HALF: Self = Self { brightness: 128, color: 128, contrast: 128, tint: 128 };
    pub const fn as_array(self) -> [u16; 4] {
        [self.brightness, self.color, self.contrast, self.tint]
    }
    pub fn set(&mut self, setting: VisualSetting, value: u16) {
        match setting {
            VisualSetting::Brightness => self.brightness = value,
            VisualSetting::Color => self.color = value,
            VisualSetting::Contrast => self.contrast = value,
            VisualSetting::Tint => self.tint = value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedAlertVisualSession {
    Normal,
    Skirmish,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedAlertVisualInput {
    None,
    SliderClicked(VisualSetting),
    SliderChanged(VisualSetting, u16),
    ResetButton,
    OptionsButton,
    BackgroundLeftClick,
    RightClick,
    Escape,
    Left,
    Right,
    Up,
    Down,
    Return,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualTitleStyle { BrightColor, NormalText }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedAlertVisualEffect {
    CallBack,
    MainLoop,
    ClearSurfacesRestored,
    HideMouse,
    DrawDialogBox(RedAlertVisualLayout),
    DrawCaption(RedAlertVisualLayout),
    ShowMouse,
    DrawTitle { setting: VisualSetting, x: i32, y: i32, style: VisualTitleStyle },
    DrawAllGadgets,
    SetSliderValue { setting: VisualSetting, value: u16 },
    SetOption { setting: VisualSetting, numerator: u16, denominator: u16 },
    TurnButtonOff(i32),
    FlagButtonForRedraw(i32),
    TurnButtonOn(i32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedAlertVisualStep {
    pub effects: Vec<RedAlertVisualEffect>,
    pub complete: bool,
    pub slider_values: [u16; 4],
    pub options: VisualOptionValues,
    pub current_button: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedAlertVisualDialog {
    pub layout: RedAlertVisualLayout,
    pub sliders: [u16; 4],
    pub options: VisualOptionValues,
    /// 0..3 sliders, 4 Reset, 5 Options.
    pub current_button: usize,
    pub display: bool,
    pub partial: bool,
    pub process: bool,
    pub complete: bool,
}

impl RedAlertVisualDialog {
    pub fn new(
        seen_width: i32,
        seen_height: i32,
        options: VisualOptionValues,
    ) -> Self {
        let layout = RedAlertVisualLayout::new(seen_width, seen_height);
        let sliders = options.as_array().map(|value| value.min(layout.effective_maximum()));
        Self {
            layout, sliders, options, current_button: 0,
            display: true, partial: true, process: true, complete: false,
        }
    }

    fn snapshot(&self, effects: Vec<RedAlertVisualEffect>) -> RedAlertVisualStep {
        RedAlertVisualStep {
            effects, complete: self.complete, slider_values: self.sliders,
            options: self.options, current_button: self.current_button,
        }
    }

    fn update_setting(
        &mut self,
        setting: VisualSetting,
        value: u16,
        effects: &mut Vec<RedAlertVisualEffect>,
    ) {
        let value = value.min(self.layout.effective_maximum());
        self.sliders[setting.index()] = value;
        self.options.set(setting, value);
        effects.push(RedAlertVisualEffect::SetSliderValue { setting, value });
        effects.push(RedAlertVisualEffect::SetOption {
            setting, numerator: value, denominator: 256,
        });
    }

    fn reset(&mut self, effects: &mut Vec<RedAlertVisualEffect>) {
        for setting in VisualSetting::ALL {
            self.update_setting(setting, RESET_SLIDER_VALUE, effects);
        }
    }

    fn draw(&mut self, effects: &mut Vec<RedAlertVisualEffect>) {
        if self.display {
            effects.push(RedAlertVisualEffect::HideMouse);
            effects.push(RedAlertVisualEffect::DrawDialogBox(self.layout));
            effects.push(RedAlertVisualEffect::DrawCaption(self.layout));
            effects.push(RedAlertVisualEffect::ShowMouse);
            self.display = false;
            self.partial = true;
        }
        if self.partial {
            effects.push(RedAlertVisualEffect::HideMouse);
            for setting in VisualSetting::ALL {
                effects.push(RedAlertVisualEffect::DrawTitle {
                    setting,
                    x: self.layout.slider_x - 8 * self.layout.factor,
                    y: self.layout.title_y_for(setting),
                    style: if self.current_button == setting.index() {
                        VisualTitleStyle::BrightColor
                    } else {
                        VisualTitleStyle::NormalText
                    },
                });
            }
            effects.push(RedAlertVisualEffect::DrawAllGadgets);
            effects.push(RedAlertVisualEffect::ShowMouse);
            self.partial = false;
        }
    }

    fn focus_button_id(index: usize) -> i32 {
        if index == 4 { BUTTON_RESET } else { BUTTON_OPTIONS }
    }

    fn horizontal_move(&mut self, right: bool, effects: &mut Vec<RedAlertVisualEffect>) {
        if self.current_button <= 3 {
            let setting = VisualSetting::ALL[self.current_button];
            let current = self.sliders[self.current_button] as i32;
            let step = self.layout.thumb_units as i32;
            let next = if right { current + step } else { current - step }.clamp(0, self.layout.effective_maximum() as i32) as u16;
            self.update_setting(setting, next, effects);
        } else {
            let old_id = Self::focus_button_id(self.current_button);
            effects.push(RedAlertVisualEffect::TurnButtonOff(old_id));
            effects.push(RedAlertVisualEffect::FlagButtonForRedraw(old_id));
            self.current_button = if self.current_button == 4 { 5 } else { 4 };
            let new_id = Self::focus_button_id(self.current_button);
            effects.push(RedAlertVisualEffect::TurnButtonOn(new_id));
            effects.push(RedAlertVisualEffect::FlagButtonForRedraw(new_id));
        }
    }

    fn vertical_move(&mut self, up: bool, effects: &mut Vec<RedAlertVisualEffect>) {
        let old = self.current_button;
        if old >= 4 {
            let button = Self::focus_button_id(old);
            effects.push(RedAlertVisualEffect::TurnButtonOff(button));
            effects.push(RedAlertVisualEffect::FlagButtonForRedraw(button));
        }
        if up {
            if self.current_button == 0 {
                self.current_button = 4;
            } else {
                self.current_button -= 1;
                if self.current_button == 4 {
                    self.current_button -= 1;
                }
            }
        } else {
            self.current_button += 1;
            if self.current_button > 4 {
                self.current_button = 0;
            }
        }
        if self.current_button <= 3 {
            self.partial = true;
        } else {
            let button = Self::focus_button_id(self.current_button);
            effects.push(RedAlertVisualEffect::TurnButtonOn(button));
            effects.push(RedAlertVisualEffect::FlagButtonForRedraw(button));
        }
    }

    /// Process one pass of VisualControlsClass::Process.
    pub fn step(
        &mut self,
        session: RedAlertVisualSession,
        main_loop_requested_exit: bool,
        surfaces_restored: bool,
        input: RedAlertVisualInput,
    ) -> RedAlertVisualStep {
        if self.complete { return self.snapshot(Vec::new()); }
        let mut effects = Vec::new();
        match session {
            RedAlertVisualSession::Normal | RedAlertVisualSession::Skirmish => {
                effects.push(RedAlertVisualEffect::CallBack);
            }
            RedAlertVisualSession::Other => {
                effects.push(RedAlertVisualEffect::MainLoop);
                if main_loop_requested_exit {
                    self.process = false;
                    self.complete = true;
                }
            }
        }
        if surfaces_restored {
            effects.push(RedAlertVisualEffect::ClearSurfacesRestored);
            self.display = true;
        }
        self.draw(&mut effects);

        match input {
            RedAlertVisualInput::SliderClicked(setting) => {
                self.current_button = setting.index();
                self.partial = true;
            }
            RedAlertVisualInput::SliderChanged(setting, value) => {
                self.update_setting(setting, value, &mut effects);
            }
            RedAlertVisualInput::ResetButton => self.reset(&mut effects),
            RedAlertVisualInput::OptionsButton
            | RedAlertVisualInput::BackgroundLeftClick
            | RedAlertVisualInput::RightClick
            | RedAlertVisualInput::Escape => {
                self.process = false;
                self.complete = true;
            }
            RedAlertVisualInput::Left => self.horizontal_move(false, &mut effects),
            RedAlertVisualInput::Right => self.horizontal_move(true, &mut effects),
            RedAlertVisualInput::Up => self.vertical_move(true, &mut effects),
            RedAlertVisualInput::Down => self.vertical_move(false, &mut effects),
            RedAlertVisualInput::Return => match self.current_button {
                4 => self.reset(&mut effects),
                5 => {
                    self.process = false;
                    self.complete = true;
                }
                _ => {}
            },
            RedAlertVisualInput::None => {}
        }
        self.snapshot(effects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_geometry_and_slider_thumb_scale_at_both_resolutions() {
        let low = RedAlertVisualLayout::new(320, 200);
        assert_eq!((low.option_x, low.option_y, low.option_width, low.option_height), (52, 39, 216, 122));
        assert_eq!((low.slider_x, low.slider_y, low.slider_width, low.slider_height), (157, 69, 70, 5));
        assert_eq!((low.reset_x, low.options_x, low.button_y), (69, 191, 141));
        assert_eq!((low.thumb_units, low.effective_maximum()), (20, 235));
        let high = RedAlertVisualLayout::new(640, 400);
        assert_eq!((high.option_x, high.option_y, high.option_width, high.option_height), (104, 78, 432, 244));
        assert_eq!((high.thumb_units, high.effective_maximum()), (40, 215));
        assert_eq!(high.slider_y_for(VisualSetting::Tint), 138);
    }

    #[test]
    fn constructor_maps_fixed_options_into_slider_units_and_clamps_by_thumb() {
        let options = VisualOptionValues { brightness: 200, color: 128, contrast: 255, tint: 64 };
        let dialog = RedAlertVisualDialog::new(320, 200, options);
        assert_eq!(dialog.sliders, [200, 128, 235, 64]);
        assert_eq!(dialog.options, options);
    }

    #[test]
    fn mouse_slider_updates_write_value_over_256_to_matching_option() {
        let mut dialog = RedAlertVisualDialog::new(320, 200, VisualOptionValues::HALF);
        let step = dialog.step(
            RedAlertVisualSession::Normal, false, false,
            RedAlertVisualInput::SliderChanged(VisualSetting::Color, 90),
        );
        assert_eq!(step.options.color, 90);
        assert!(step.effects.contains(&RedAlertVisualEffect::SetOption {
            setting: VisualSetting::Color, numerator: 90, denominator: 256,
        }));
    }

    #[test]
    fn horizontal_bump_uses_twenty_or_forty_units_by_resolution() {
        let mut low = RedAlertVisualDialog::new(320, 200, VisualOptionValues::HALF);
        assert_eq!(low.step(RedAlertVisualSession::Normal, false, false, RedAlertVisualInput::Left).slider_values[0], 108);
        let mut high = RedAlertVisualDialog::new(640, 400, VisualOptionValues::HALF);
        assert_eq!(high.step(RedAlertVisualSession::Normal, false, false, RedAlertVisualInput::Right).slider_values[0], 168);
    }

    #[test]
    fn slider_bump_clamps_to_source_maximum_minus_thumb() {
        let options = VisualOptionValues { brightness: 235, color: 128, contrast: 128, tint: 128 };
        let mut dialog = RedAlertVisualDialog::new(320, 200, options);
        assert_eq!(dialog.step(RedAlertVisualSession::Normal, false, false, RedAlertVisualInput::Right).slider_values[0], 235);
        let low = VisualOptionValues { brightness: 0, color: 0, contrast: 0, tint: 0 };
        dialog.options = low;
        dialog.sliders = [0; 4];
        assert_eq!(dialog.step(RedAlertVisualSession::Normal, false, false, RedAlertVisualInput::Left).slider_values[0], 0);
    }

    #[test]
    fn reset_sets_all_four_options_to_one_half_and_refreshes_slider_values() {
        let mut dialog = RedAlertVisualDialog::new(320, 200, VisualOptionValues {
            brightness: 1, color: 200, contrast: 3, tint: 250,
        });
        let result = dialog.step(RedAlertVisualSession::Normal, false, false, RedAlertVisualInput::ResetButton);
        assert_eq!(result.options, VisualOptionValues::HALF);
        assert_eq!(result.slider_values, [128; 4]);
        assert_eq!(result.effects.iter().filter(|op| matches!(op, RedAlertVisualEffect::SetOption { denominator: 256, numerator: 128, .. })).count(), 4);
    }

    #[test]
    fn focus_navigation_skips_reset_when_moving_up_from_options() {
        let mut dialog = RedAlertVisualDialog::new(320, 200, VisualOptionValues::HALF);
        assert_eq!(dialog.step(RedAlertVisualSession::Normal, false, false, RedAlertVisualInput::Up).current_button, 4);
        assert_eq!(dialog.step(RedAlertVisualSession::Normal, false, false, RedAlertVisualInput::Down).current_button, 0);
        dialog.current_button = 5;
        assert_eq!(dialog.step(RedAlertVisualSession::Normal, false, false, RedAlertVisualInput::Up).current_button, 3);
        assert_eq!(dialog.step(RedAlertVisualSession::Normal, false, false, RedAlertVisualInput::Down).current_button, 4);
    }

    #[test]
    fn reset_and_options_button_horizontal_focus_wraps() {
        let mut dialog = RedAlertVisualDialog::new(320, 200, VisualOptionValues::HALF);
        dialog.current_button = 4;
        let right = dialog.step(RedAlertVisualSession::Normal, false, false, RedAlertVisualInput::Right);
        assert_eq!(right.current_button, 5);
        assert!(right.effects.contains(&RedAlertVisualEffect::TurnButtonOff(BUTTON_RESET)));
        assert!(right.effects.contains(&RedAlertVisualEffect::TurnButtonOn(BUTTON_OPTIONS)));
    }

    #[test]
    fn options_escape_and_outside_click_exit_without_reset() {
        for input in [
            RedAlertVisualInput::OptionsButton,
            RedAlertVisualInput::BackgroundLeftClick,
            RedAlertVisualInput::RightClick,
            RedAlertVisualInput::Escape,
        ] {
            let mut dialog = RedAlertVisualDialog::new(320, 200, VisualOptionValues::HALF);
            assert!(dialog.step(RedAlertVisualSession::Normal, false, false, input).complete);
            assert_eq!(dialog.options, VisualOptionValues::HALF);
        }
    }

    #[test]
    fn callbacks_depend_on_session_and_main_loop_exit() {
        let mut normal = RedAlertVisualDialog::new(320, 200, VisualOptionValues::HALF);
        assert!(normal.step(RedAlertVisualSession::Normal, false, false, RedAlertVisualInput::None).effects.contains(&RedAlertVisualEffect::CallBack));
        let mut multiplayer = RedAlertVisualDialog::new(320, 200, VisualOptionValues::HALF);
        let result = multiplayer.step(RedAlertVisualSession::Other, true, false, RedAlertVisualInput::None);
        assert!(result.complete);
        assert!(result.effects.contains(&RedAlertVisualEffect::MainLoop));
    }

    #[test]
    fn focus_restoration_redraws_dialog_and_all_title_labels() {
        let mut dialog = RedAlertVisualDialog::new(320, 200, VisualOptionValues::HALF);
        let step = dialog.step(RedAlertVisualSession::Skirmish, false, true, RedAlertVisualInput::None);
        assert!(step.effects.contains(&RedAlertVisualEffect::ClearSurfacesRestored));
        assert!(step.effects.contains(&RedAlertVisualEffect::DrawDialogBox(dialog.layout)));
        assert_eq!(step.effects.iter().filter(|op| matches!(op, RedAlertVisualEffect::DrawTitle { .. })).count(), 4);
    }
}
