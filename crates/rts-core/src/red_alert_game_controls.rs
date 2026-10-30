//! Red Alert GameControlsClass from REDALERT/GAMEDLG.CPP.
//!
//! This is distinct from Tiberian Dawn's options dialog. The optional WOLAPI
//! integration changes dialog size and the final button index. Processing is
//! one frame at a time; the host executes the returned UI, event and save
//! effects without blocking the game loop.

pub const BUTTON_SPEED: i32 = 100;
pub const BUTTON_SCROLLRATE: i32 = 101;
pub const BUTTON_VISUAL: i32 = 102;
pub const BUTTON_SOUND: i32 = 103;
pub const BUTTON_WOLAPI: i32 = 104;
pub const BUTTON_OK: i32 = 105;
pub const BUTTON_FIRST: i32 = BUTTON_SPEED;
pub const MAX_SPEED_SETTING: i32 = 7;
pub const MAX_SCROLL_SETTING: i32 = 7;
pub const GERMAN_OFFSET_Y: i32 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedAlertLocale { Default, German }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RedAlertGameControlsFeatures {
    pub wolapi_compiled: bool,
    pub wolapi_connected: bool,
}

impl RedAlertGameControlsFeatures {
    pub const fn standard() -> Self { Self { wolapi_compiled: false, wolapi_connected: false } }
    pub const fn wolapi(connected: bool) -> Self { Self { wolapi_compiled: true, wolapi_connected: connected } }
    pub const fn show_wolapi(self) -> bool { self.wolapi_compiled && self.wolapi_connected }
    pub const fn button_count(self) -> usize { if self.wolapi_compiled { 6 } else { 5 } }
    pub const fn last_focus_index(self) -> usize { self.button_count() - 1 }
    pub const fn ok_button_id(self) -> i32 { if self.wolapi_compiled { BUTTON_OK } else { BUTTON_WOLAPI } }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RedAlertGameControlsLayout {
    pub factor: i32,
    pub dialog_x: i32, pub dialog_y: i32,
    pub dialog_width: i32, pub dialog_height: i32,
    pub dialog_center_x: i32,
    pub top_margin: i32, pub text_height: i32,
    pub margin_large: i32, pub margin_small: i32,
    pub speed_x: i32, pub speed_y: i32, pub speed_width: i32, pub speed_height: i32,
    pub scroll_x: i32, pub scroll_y: i32, pub scroll_width: i32, pub scroll_height: i32,
    pub visual_x: i32, pub visual_y: i32, pub visual_width: i32, pub visual_height: i32,
    pub sound_x: i32, pub sound_y: i32, pub sound_width: i32, pub sound_height: i32,
    pub ok_x: i32, pub ok_y: i32, pub ok_width: i32, pub ok_height: i32,
    pub wol_x: Option<i32>, pub wol_y: Option<i32>, pub wol_width: Option<i32>, pub wol_height: Option<i32>,
}

impl RedAlertGameControlsLayout {
    pub const fn new(
        seen_width: i32,
        seen_height: i32,
        locale: RedAlertLocale,
        features: RedAlertGameControlsFeatures,
        ok_text_width: i32,
    ) -> Self {
        let factor = if seen_width == 320 { 1 } else { 2 };
        let dialog_width = 232 * factor;
        let mut dialog_height = 141 * factor;
        let mut dialog_x = (seen_width - dialog_width) / 2;
        let mut dialog_y = (seen_height - dialog_height) / 2;
        let dialog_center_x = dialog_x + dialog_width / 2;
        let top_margin = 25 * factor;
        let text_height = 6 * factor + 1;
        let margin_large = 5 * factor;
        let margin_small = 2 * factor;
        let speed_width = dialog_width - 34 * factor;
        let speed_height = 6 * factor;
        let speed_x = dialog_x + 17 * factor;
        let german_adjust = if matches!(locale, RedAlertLocale::German) { GERMAN_OFFSET_Y } else { 0 };
        let speed_y = dialog_y + top_margin + margin_large + text_height - german_adjust;
        let scroll_width = dialog_width - 34 * factor;
        let scroll_height = 6 * factor;
        let scroll_x = dialog_x + 17 * factor;
        let scroll_y = speed_y + speed_height + text_height + (margin_large * 2) + text_height;
        let visual_width = dialog_width - 40 * factor;
        let visual_height = 9 * factor;
        let visual_x = dialog_x + 20 * factor;
        let visual_y = scroll_y + scroll_height + text_height + (margin_large * 2);
        let sound_width = dialog_width - 40 * factor;
        let sound_height = 9 * factor;
        let sound_x = dialog_x + 20 * factor;
        let sound_y = visual_y + visual_height + margin_large;
        let ok_width = 20 * factor;
        let ok_height = 9 * factor;
        let ok_x = (seen_width - ok_text_width) / 2;
        let mut ok_y = dialog_y + dialog_height - ok_height - margin_large - (4 * factor);

        let wol_x;
        let wol_y;
        let wol_width;
        let wol_height;
        if features.show_wolapi() {
            wol_x = Some(sound_x);
            wol_y = Some(sound_y + sound_height + margin_large);
            wol_width = Some(sound_width);
            wol_height = Some(sound_height);
            dialog_height += sound_height + margin_large;
            dialog_y = (seen_height - dialog_height) / 2;
            ok_y = dialog_y + dialog_height - ok_height - margin_large - (4 * factor);
            dialog_x = (seen_width - dialog_width) / 2;
        } else {
            wol_x = None;
            wol_y = None;
            wol_width = None;
            wol_height = None;
        }

        Self {
            factor, dialog_x, dialog_y, dialog_width, dialog_height, dialog_center_x,
            top_margin, text_height, margin_large, margin_small,
            speed_x, speed_y, speed_width, speed_height,
            scroll_x, scroll_y, scroll_width, scroll_height,
            visual_x, visual_y, visual_width, visual_height,
            sound_x, sound_y, sound_width, sound_height,
            ok_x, ok_y, ok_width, ok_height,
            wol_x, wol_y, wol_width, wol_height,
        }
    }

    pub const fn actual_dialog_center_x(&self) -> i32 { self.dialog_x + self.dialog_width / 2 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RedAlertGameOptions {
    pub game_speed: i32,
    pub scroll_rate: i32,
    pub sound_card_available: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedAlertGameSession { Normal, Skirmish, Other }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedAlertGameInput {
    None,
    SpeedSliderClicked,
    ScrollSliderClicked,
    VisualButton,
    SoundButton,
    WolapiButton,
    OkButton,
    Escape,
    Left,
    Right,
    Up,
    Down,
    Return,
    SpeedSliderChanged(i32),
    ScrollSliderChanged(i32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedAlertGameEffect {
    CallBack,
    MainLoop,
    ClearSurfacesRestored,
    HideMouse,
    FlagMapToRedraw(bool),
    RenderMap,
    DrawDialogBox(RedAlertGameControlsLayout),
    DrawCaption(RedAlertGameControlsLayout),
    ShowMouse,
    DrawSpeedLabel { highlighted: bool, x: i32, y: i32 },
    DrawSpeedRangeLabels { x: i32, y: i32, width: i32, left_right_aligned: bool },
    DrawScrollLabel { highlighted: bool, x: i32, y: i32 },
    DrawScrollRangeLabels { x: i32, y: i32, width: i32, left_right_aligned: bool },
    DrawAllGadgets,
    SetSpeedSlider(i32),
    SetScrollSlider(i32),
    TurnButtonOff(i32),
    FlagButtonForRedraw(i32),
    TurnButtonOn(i32),
    QueueGameSpeedEvent(i32),
    SetScrollRate(i32),
    TemporarilySetGameSpeed(i32),
    SaveSettings,
    RestoreGameSpeed(i32),
    OpenVisualControls,
    OpenSoundControls,
    ShowNoSoundCardMessage,
    OpenWolapiOptions,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedAlertGameStep {
    pub effects: Vec<RedAlertGameEffect>,
    pub complete: bool,
    pub gamespeed: i32,
    pub scrollrate: i32,
    pub speed_slider: i32,
    pub scroll_slider: i32,
    pub current_button: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedAlertGameControlsDialog {
    pub features: RedAlertGameControlsFeatures,
    pub layout: RedAlertGameControlsLayout,
    pub options: RedAlertGameOptions,
    pub gamespeed: i32,
    pub scrollrate: i32,
    pub speed_slider: i32,
    pub scroll_slider: i32,
    /// 0 speed; 1 scroll; 2 visual; 3 sound; optional 4 WOLAPI; final OK.
    pub current_button: usize,
    pub process: bool,
    pub display: bool,
    pub refresh: bool,
    pub complete: bool,
}

impl RedAlertGameControlsDialog {
    pub fn new(
        seen_width: i32,
        seen_height: i32,
        ok_text_width: i32,
        locale: RedAlertLocale,
        features: RedAlertGameControlsFeatures,
        options: RedAlertGameOptions,
    ) -> Self {
        let speed_slider = (MAX_SPEED_SETTING - 1 - options.game_speed).clamp(0, MAX_SPEED_SETTING - 1);
        let scroll_slider = (MAX_SCROLL_SETTING - 1 - options.scroll_rate).clamp(0, MAX_SCROLL_SETTING - 1);
        Self {
            features,
            layout: RedAlertGameControlsLayout::new(seen_width, seen_height, locale, features, ok_text_width),
            options,
            gamespeed: options.game_speed,
            scrollrate: options.scroll_rate,
            speed_slider,
            scroll_slider,
            current_button: 0,
            process: true,
            display: true,
            refresh: true,
            complete: false,
        }
    }

    fn snapshot(&self, effects: Vec<RedAlertGameEffect>) -> RedAlertGameStep {
        RedAlertGameStep {
            effects, complete: self.complete, gamespeed: self.gamespeed,
            scrollrate: self.scrollrate, speed_slider: self.speed_slider,
            scroll_slider: self.scroll_slider, current_button: self.current_button,
        }
    }

    fn button_id(&self, index: usize) -> Option<i32> {
        match index {
            2 => Some(BUTTON_VISUAL),
            3 => Some(BUTTON_SOUND),
            4 if self.features.wolapi_compiled && self.features.show_wolapi() => Some(BUTTON_WOLAPI),
            value if value == self.features.last_focus_index() => Some(self.features.ok_button_id()),
            _ => None,
        }
    }

    fn focus_move(&mut self, up: bool, effects: &mut Vec<RedAlertGameEffect>) {
        if let Some(id) = self.button_id(self.current_button) {
            effects.push(RedAlertGameEffect::TurnButtonOff(id));
            effects.push(RedAlertGameEffect::FlagButtonForRedraw(id));
        }
        if up {
            self.current_button = if self.current_button == 0 {
                self.features.last_focus_index()
            } else {
                self.current_button - 1
            };
            if self.features.wolapi_compiled && !self.features.show_wolapi() && self.current_button == 4 {
                self.current_button = 3;
            }
        } else {
            self.current_button += 1;
            if self.features.wolapi_compiled && !self.features.show_wolapi() && self.current_button == 4 {
                self.current_button = 5;
            }
            if self.current_button > self.features.last_focus_index() {
                self.current_button = 0;
            }
        }
        if let Some(id) = self.button_id(self.current_button) {
            effects.push(RedAlertGameEffect::TurnButtonOn(id));
            effects.push(RedAlertGameEffect::FlagButtonForRedraw(id));
        }
        self.refresh = true;
    }

    fn draw(&mut self, effects: &mut Vec<RedAlertGameEffect>) {
        if self.display {
            effects.extend([
                RedAlertGameEffect::HideMouse,
                RedAlertGameEffect::FlagMapToRedraw(true),
                RedAlertGameEffect::RenderMap,
                RedAlertGameEffect::DrawDialogBox(self.layout),
                RedAlertGameEffect::DrawCaption(self.layout),
                RedAlertGameEffect::ShowMouse,
            ]);
            self.display = false;
            self.refresh = true;
        }
        if self.refresh {
            effects.push(RedAlertGameEffect::HideMouse);
            effects.push(RedAlertGameEffect::DrawSpeedLabel {
                highlighted: self.current_button == 0,
                x: self.layout.speed_x,
                y: self.layout.speed_y - self.layout.text_height,
            });
            effects.push(RedAlertGameEffect::DrawSpeedRangeLabels {
                x: self.layout.speed_x,
                y: self.layout.speed_y + self.layout.speed_height + self.layout.factor,
                width: self.layout.speed_width,
                left_right_aligned: true,
            });
            effects.push(RedAlertGameEffect::DrawScrollLabel {
                highlighted: self.current_button == 1,
                x: self.layout.scroll_x,
                y: self.layout.scroll_y - self.layout.text_height,
            });
            effects.push(RedAlertGameEffect::DrawScrollRangeLabels {
                x: self.layout.scroll_x,
                y: self.layout.scroll_y + self.layout.scroll_height + self.layout.factor,
                width: self.layout.scroll_width,
                left_right_aligned: true,
            });
            effects.push(RedAlertGameEffect::DrawAllGadgets);
            effects.push(RedAlertGameEffect::ShowMouse);
            self.refresh = false;
        }
    }

    fn persist_and_select(
        &mut self,
        selected: i32,
        session: RedAlertGameSession,
        effects: &mut Vec<RedAlertGameEffect>,
        wolapi_game_ended: bool,
    ) {
        let next_speed = (MAX_SPEED_SETTING - 1) - self.speed_slider;
        let next_scroll = (MAX_SCROLL_SETTING - 1) - self.scroll_slider;
        if next_speed != self.gamespeed {
            self.gamespeed = next_speed;
            effects.push(RedAlertGameEffect::QueueGameSpeedEvent(self.gamespeed));
        }
        if next_scroll != self.scrollrate {
            self.scrollrate = next_scroll;
            effects.push(RedAlertGameEffect::SetScrollRate(self.scrollrate));
        }
        self.process = false;

        if session == RedAlertGameSession::Normal {
            effects.push(RedAlertGameEffect::TemporarilySetGameSpeed(self.gamespeed));
            effects.push(RedAlertGameEffect::SaveSettings);
        } else {
            effects.push(RedAlertGameEffect::TemporarilySetGameSpeed(self.gamespeed));
            effects.push(RedAlertGameEffect::SaveSettings);
            effects.push(RedAlertGameEffect::RestoreGameSpeed(self.options.game_speed));
        }

        match selected {
            BUTTON_VISUAL => {
                effects.push(RedAlertGameEffect::OpenVisualControls);
                self.process = true;
                self.display = true;
                self.refresh = true;
            }
            BUTTON_SOUND if !self.options.sound_card_available => {
                effects.push(RedAlertGameEffect::ShowNoSoundCardMessage);
                self.process = true;
                self.display = true;
                self.refresh = true;
            }
            BUTTON_SOUND => {
                effects.push(RedAlertGameEffect::OpenSoundControls);
                self.process = true;
                self.display = true;
                self.refresh = true;
            }
            BUTTON_WOLAPI if self.features.show_wolapi() => {
                effects.push(RedAlertGameEffect::OpenWolapiOptions);
                if wolapi_game_ended {
                    self.process = false;
                } else {
                    self.process = true;
                    self.display = true;
                    self.refresh = true;
                }
            }
            _ => {}
        }
        self.complete = !self.process;
    }

    /// Process a single frame of the Red Alert options dialog.
    pub fn step(
        &mut self,
        session: RedAlertGameSession,
        main_loop_requested_exit: bool,
        surfaces_restored: bool,
        input: RedAlertGameInput,
        wolapi_game_ended: bool,
    ) -> RedAlertGameStep {
        if self.complete { return self.snapshot(Vec::new()); }
        let mut effects = Vec::new();
        match session {
            RedAlertGameSession::Normal | RedAlertGameSession::Skirmish => {
                effects.push(RedAlertGameEffect::CallBack);
            }
            RedAlertGameSession::Other => {
                effects.push(RedAlertGameEffect::MainLoop);
                if main_loop_requested_exit {
                    self.process = false;
                    self.complete = true;
                }
            }
        }
        if surfaces_restored {
            effects.push(RedAlertGameEffect::ClearSurfacesRestored);
            self.display = true;
        }
        self.draw(&mut effects);

        let mut pressed: Option<i32> = None;
        match input {
            RedAlertGameInput::SpeedSliderClicked => {
                self.current_button = 0;
                self.refresh = true;
            }
            RedAlertGameInput::ScrollSliderClicked => {
                self.current_button = 1;
                self.refresh = true;
            }
            RedAlertGameInput::VisualButton => pressed = Some(BUTTON_VISUAL),
            RedAlertGameInput::SoundButton => pressed = Some(BUTTON_SOUND),
            RedAlertGameInput::WolapiButton if self.features.show_wolapi() => pressed = Some(BUTTON_WOLAPI),
            RedAlertGameInput::OkButton => pressed = Some(self.features.ok_button_id()),
            RedAlertGameInput::Escape => {
                self.process = false;
                self.complete = true;
            }
            RedAlertGameInput::Left => {
                if self.current_button == 0 {
                    self.speed_slider = (self.speed_slider - 1).max(0);
                    effects.push(RedAlertGameEffect::SetSpeedSlider(self.speed_slider));
                } else if self.current_button == 1 {
                    self.scroll_slider = (self.scroll_slider - 1).max(0);
                    effects.push(RedAlertGameEffect::SetScrollSlider(self.scroll_slider));
                }
            }
            RedAlertGameInput::Right => {
                if self.current_button == 0 {
                    self.speed_slider = (self.speed_slider + 1).min(MAX_SPEED_SETTING - 1);
                    effects.push(RedAlertGameEffect::SetSpeedSlider(self.speed_slider));
                } else if self.current_button == 1 {
                    self.scroll_slider = (self.scroll_slider + 1).min(MAX_SCROLL_SETTING - 1);
                    effects.push(RedAlertGameEffect::SetScrollSlider(self.scroll_slider));
                }
            }
            RedAlertGameInput::Up => self.focus_move(true, &mut effects),
            RedAlertGameInput::Down => self.focus_move(false, &mut effects),
            RedAlertGameInput::Return => pressed = Some(BUTTON_FIRST + self.current_button as i32),
            RedAlertGameInput::SpeedSliderChanged(value) => self.speed_slider = value.clamp(0, MAX_SPEED_SETTING - 1),
            RedAlertGameInput::ScrollSliderChanged(value) => self.scroll_slider = value.clamp(0, MAX_SCROLL_SETTING - 1),
            RedAlertGameInput::None | RedAlertGameInput::WolapiButton => {}
        }

        if let Some(selection) = pressed {
            self.persist_and_select(selection, session, &mut effects, wolapi_game_ended);
        }
        self.complete = !self.process;
        self.snapshot(effects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn options() -> RedAlertGameOptions {
        RedAlertGameOptions { game_speed: 3, scroll_rate: 2, sound_card_available: true }
    }

    fn dialog(locale: RedAlertLocale, features: RedAlertGameControlsFeatures) -> RedAlertGameControlsDialog {
        RedAlertGameControlsDialog::new(320, 200, 40, locale, features, options())
    }

    #[test]
    fn base_layout_matches_source_coordinates() {
        let layout = RedAlertGameControlsLayout::new(320, 200, RedAlertLocale::Default, RedAlertGameControlsFeatures::standard(), 40);
        assert_eq!((layout.dialog_x, layout.dialog_y, layout.dialog_width, layout.dialog_height), (44, 29, 232, 141));
        assert_eq!((layout.speed_x, layout.speed_y, layout.speed_width, layout.speed_height), (61, 66, 198, 6));
        assert_eq!((layout.scroll_x, layout.scroll_y), (61, 97));
        assert_eq!((layout.visual_x, layout.visual_y, layout.visual_width), (64, 126, 192));
        assert_eq!((layout.sound_x, layout.sound_y), (64, 140));
        assert_eq!((layout.ok_x, layout.ok_y), (140, 148));
    }

    #[test]
    fn german_layout_shifts_only_the_two_slider_label_rows_by_four_pixels() {
        let normal = RedAlertGameControlsLayout::new(320, 200, RedAlertLocale::Default, RedAlertGameControlsFeatures::standard(), 40);
        let german = RedAlertGameControlsLayout::new(320, 200, RedAlertLocale::German, RedAlertGameControlsFeatures::standard(), 40);
        assert_eq!(german.speed_y, normal.speed_y - 4);
        assert_eq!(german.scroll_y, normal.scroll_y - 8);
        assert_eq!(german.visual_y, normal.visual_y - 8);
        assert_eq!(german.sound_y, normal.sound_y - 8);
    }

    #[test]
    fn wolapi_presence_expands_dialog_and_adds_optional_button() {
        let features = RedAlertGameControlsFeatures::wolapi(true);
        let layout = RedAlertGameControlsLayout::new(320, 200, RedAlertLocale::Default, features, 40);
        assert_eq!(features.button_count(), 6);
        assert_eq!(layout.dialog_height, 155);
        assert_eq!(layout.dialog_y, 22);
        assert_eq!(layout.wol_x, Some(64));
        assert_eq!(layout.wol_y, Some(154));
        assert_eq!(features.ok_button_id(), BUTTON_OK);
        let disconnected = RedAlertGameControlsFeatures::wolapi(false);
        assert_eq!(disconnected.button_count(), 6);
        assert_eq!(disconnected.last_focus_index(), 5);
        assert_eq!(disconnected.ok_button_id(), BUTTON_OK);
        assert_eq!(RedAlertGameControlsLayout::new(320, 200, RedAlertLocale::Default, disconnected, 40).wol_y, None);
    }

    #[test]
    fn constructor_maps_game_speed_and_scroll_rate_to_inverse_slider_values() {
        let dialog = dialog(RedAlertLocale::Default, RedAlertGameControlsFeatures::standard());
        assert_eq!((dialog.speed_slider, dialog.scroll_slider), (3, 4));
    }

    #[test]
    fn slider_click_focus_and_left_right_update_values_by_one() {
        let mut dialog = dialog(RedAlertLocale::Default, RedAlertGameControlsFeatures::standard());
        assert_eq!(dialog.step(RedAlertGameSession::Normal, false, false, RedAlertGameInput::SpeedSliderClicked, false).current_button, 0);
        assert_eq!(dialog.step(RedAlertGameSession::Normal, false, false, RedAlertGameInput::Left, false).speed_slider, 2);
        assert_eq!(dialog.step(RedAlertGameSession::Normal, false, false, RedAlertGameInput::Right, false).speed_slider, 3);
        dialog.step(RedAlertGameSession::Normal, false, false, RedAlertGameInput::ScrollSliderClicked, false);
        assert_eq!(dialog.step(RedAlertGameSession::Normal, false, false, RedAlertGameInput::Left, false).scroll_slider, 3);
    }

    #[test]
    fn event_and_save_order_is_multiplayer_safe() {
        let mut dialog = dialog(RedAlertLocale::Default, RedAlertGameControlsFeatures::standard());
        dialog.step(RedAlertGameSession::Normal, false, false, RedAlertGameInput::Left, false);
        let result = dialog.step(RedAlertGameSession::Other, false, false, RedAlertGameInput::OkButton, false);
        let event = result.effects.iter().position(|e| *e == RedAlertGameEffect::QueueGameSpeedEvent(4)).unwrap();
        let temp = result.effects.iter().position(|e| *e == RedAlertGameEffect::TemporarilySetGameSpeed(4)).unwrap();
        let save = result.effects.iter().position(|e| *e == RedAlertGameEffect::SaveSettings).unwrap();
        let restore = result.effects.iter().position(|e| *e == RedAlertGameEffect::RestoreGameSpeed(3)).unwrap();
        assert!(event < temp && temp < save && save < restore);
    }

    #[test]
    fn normal_session_saves_without_temporary_restore() {
        let mut dialog = dialog(RedAlertLocale::Default, RedAlertGameControlsFeatures::standard());
        let result = dialog.step(RedAlertGameSession::Normal, false, false, RedAlertGameInput::OkButton, false);
        assert!(result.effects.contains(&RedAlertGameEffect::SaveSettings));
        assert!(!result.effects.iter().any(|e| matches!(e, RedAlertGameEffect::RestoreGameSpeed(_))));
    }

    #[test]
    fn visual_dialog_returns_to_parent_and_forces_map_and_dialog_redraw() {
        let mut dialog = dialog(RedAlertLocale::Default, RedAlertGameControlsFeatures::standard());
        let result = dialog.step(RedAlertGameSession::Normal, false, false, RedAlertGameInput::VisualButton, false);
        assert!(!result.complete);
        assert!(result.effects.contains(&RedAlertGameEffect::OpenVisualControls));
        let redraw = dialog.step(RedAlertGameSession::Normal, false, true, RedAlertGameInput::None, false);
        assert!(redraw.effects.contains(&RedAlertGameEffect::ClearSurfacesRestored));
        assert!(redraw.effects.contains(&RedAlertGameEffect::FlagMapToRedraw(true)));
        assert!(redraw.effects.contains(&RedAlertGameEffect::DrawDialogBox(dialog.layout)));
    }

    #[test]
    fn sound_without_card_shows_warning_and_resumes_dialog() {
        let mut options = options();
        options.sound_card_available = false;
        let mut dialog = RedAlertGameControlsDialog::new(320, 200, 40, RedAlertLocale::Default, RedAlertGameControlsFeatures::standard(), options);
        let result = dialog.step(RedAlertGameSession::Normal, false, false, RedAlertGameInput::SoundButton, false);
        assert!(!result.complete);
        assert!(result.effects.contains(&RedAlertGameEffect::ShowNoSoundCardMessage));
        assert!(!result.effects.contains(&RedAlertGameEffect::OpenSoundControls));
    }

    #[test]
    fn sound_dialog_returns_to_parent_when_audio_device_exists() {
        let mut dialog = dialog(RedAlertLocale::Default, RedAlertGameControlsFeatures::standard());
        let result = dialog.step(RedAlertGameSession::Normal, false, false, RedAlertGameInput::SoundButton, false);
        assert!(!result.complete);
        assert!(result.effects.contains(&RedAlertGameEffect::OpenSoundControls));
    }

    #[test]
    fn wolapi_button_can_end_game_or_resume_parent_dialog() {
        let mut dialog = dialog(RedAlertLocale::Default, RedAlertGameControlsFeatures::wolapi(true));
        let returned = dialog.step(RedAlertGameSession::Normal, false, false, RedAlertGameInput::WolapiButton, false);
        assert!(!returned.complete);
        assert!(returned.effects.contains(&RedAlertGameEffect::OpenWolapiOptions));
        let mut ended = dialog(RedAlertLocale::Default, RedAlertGameControlsFeatures::wolapi(true));
        assert!(ended.step(RedAlertGameSession::Normal, false, false, RedAlertGameInput::WolapiButton, true).complete);
    }

    #[test]
    fn navigation_skips_missing_wolapi_and_wraps_focus() {
        let mut standard = dialog(RedAlertLocale::Default, RedAlertGameControlsFeatures::standard());
        standard.current_button = 3;
        assert_eq!(standard.step(RedAlertGameSession::Normal, false, false, RedAlertGameInput::Down, false).current_button, 4);
        assert_eq!(standard.step(RedAlertGameSession::Normal, false, false, RedAlertGameInput::Down, false).current_button, 0);
        standard.current_button = 0;
        assert_eq!(standard.step(RedAlertGameSession::Normal, false, false, RedAlertGameInput::Up, false).current_button, 4);
        let disconnected = RedAlertGameControlsFeatures::wolapi(false);
        let mut hidden_wol = dialog(RedAlertLocale::Default, disconnected);
        hidden_wol.current_button = 3;
        assert_eq!(hidden_wol.step(RedAlertGameSession::Normal, false, false, RedAlertGameInput::Down, false).current_button, 5);
        assert_eq!(hidden_wol.step(RedAlertGameSession::Normal, false, false, RedAlertGameInput::Up, false).current_button, 3);
        let mut with_wol = dialog(RedAlertLocale::Default, RedAlertGameControlsFeatures::wolapi(true));
        with_wol.current_button = 3;
        assert_eq!(with_wol.step(RedAlertGameSession::Normal, false, false, RedAlertGameInput::Down, false).current_button, 4);
    }

    #[test]
    fn escape_exits_without_committing_option_changes() {
        let mut dialog = dialog(RedAlertLocale::Default, RedAlertGameControlsFeatures::standard());
        let result = dialog.step(RedAlertGameSession::Normal, false, false, RedAlertGameInput::Escape, false);
        assert!(result.complete);
        assert!(!result.effects.contains(&RedAlertGameEffect::SaveSettings));
    }

    #[test]
    fn main_loop_exit_is_honored_for_noninteractive_session_modes() {
        let mut dialog = dialog(RedAlertLocale::Default, RedAlertGameControlsFeatures::standard());
        let result = dialog.step(RedAlertGameSession::Other, true, false, RedAlertGameInput::None, false);
        assert!(result.complete);
        assert!(result.effects.contains(&RedAlertGameEffect::MainLoop));
    }
}
