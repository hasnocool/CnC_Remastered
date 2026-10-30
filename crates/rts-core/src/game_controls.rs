//! Tiberian Dawn GameControlsClass from TIBERIANDAWN/GAMEDLG.CPP.
//!
//! The blocking modal loop is expressed as a frame-by-frame state machine.
//! All game callbacks, audio/UI subdialogs and persistent settings writes are
//! returned as effects for the runtime to execute.

pub const BUTTON_SPEED: i32 = 100;
pub const BUTTON_SCROLLRATE: i32 = 101;
pub const BUTTON_VISUAL: i32 = 102;
pub const BUTTON_SOUND: i32 = 103;
pub const BUTTON_OK: i32 = 104;
pub const BUTTON_FIRST: i32 = BUTTON_SPEED;
pub const BUTTON_COUNT: usize = 5;
pub const MAX_SPEED_SETTING: i32 = 7;
pub const MAX_SCROLL_SETTING: i32 = 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameControlsLayout {
    pub factor: i32,
    pub dialog_x: i32,
    pub dialog_y: i32,
    pub dialog_width: i32,
    pub dialog_height: i32,
    pub dialog_center_x: i32,
    pub top_margin: i32,
    pub text_height: i32,
    pub large_margin: i32,
    pub small_margin: i32,
    pub speed_x: i32,
    pub speed_y: i32,
    pub speed_width: i32,
    pub speed_height: i32,
    pub scroll_x: i32,
    pub scroll_y: i32,
    pub scroll_width: i32,
    pub scroll_height: i32,
    pub visual_x: i32,
    pub visual_y: i32,
    pub visual_width: i32,
    pub visual_height: i32,
    pub sound_x: i32,
    pub sound_y: i32,
    pub sound_width: i32,
    pub sound_height: i32,
    pub ok_x: i32,
    pub ok_y: i32,
    pub ok_width: i32,
    pub ok_height: i32,
}

impl GameControlsLayout {
    pub const fn new(seen_width: i32, seen_height: i32) -> Self {
        let factor = if seen_width == 320 { 1 } else { 2 };
        let dialog_width = 232 * factor;
        let dialog_height = 141 * factor;
        let dialog_x = (seen_width - dialog_width) / 2;
        let dialog_y = (seen_height - dialog_height) / 2;
        let dialog_center_x = dialog_x + dialog_width / 2;
        let top_margin = 30 * factor;
        let text_height = 7 * factor;
        let large_margin = 5 * factor;
        let small_margin = 2 * factor;
        let speed_width = dialog_width - 20 * factor;
        let speed_height = 6 * factor;
        let speed_x = dialog_x + 10 * factor;
        let speed_y = dialog_y + top_margin + large_margin + text_height;
        let scroll_width = dialog_width - 20 * factor;
        let scroll_height = 6 * factor;
        let scroll_x = dialog_x + 10 * factor;
        let scroll_y = speed_y + speed_height + text_height + 2 * large_margin + text_height;
        let visual_width = dialog_width - 40 * factor;
        let visual_height = 9 * factor;
        let visual_x = dialog_x + 20 * factor;
        let visual_y = scroll_y + scroll_height + text_height + 2 * large_margin;
        let sound_width = dialog_width - 40 * factor;
        let sound_height = 9 * factor;
        let sound_x = dialog_x + 20 * factor;
        let sound_y = visual_y + visual_height + large_margin;
        let ok_width = 20 * factor;
        let ok_height = 9 * factor;
        let ok_x = dialog_center_x - ok_width / 2;
        let ok_y = dialog_y + dialog_height - ok_height - large_margin;
        Self {
            factor, dialog_x, dialog_y, dialog_width, dialog_height, dialog_center_x,
            top_margin, text_height, large_margin, small_margin,
            speed_x, speed_y, speed_width, speed_height,
            scroll_x, scroll_y, scroll_width, scroll_height,
            visual_x, visual_y, visual_width, visual_height,
            sound_x, sound_y, sound_width, sound_height,
            ok_x, ok_y, ok_width, ok_height,
        }
    }

    /// The C++ constructor overwrites the OK button's x-coordinate using its
    /// text-derived Width, rather than the 20-pixel layout seed above.
    pub const fn actual_ok_x(&self, button_width: i32, seen_width: i32) -> i32 {
        (seen_width - button_width) / 2
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameOptions {
    pub game_speed: i32,
    pub scroll_rate: i32,
    pub sound_card_available: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameControlsInput {
    None,
    SpeedSliderClicked,
    ScrollSliderClicked,
    VisualButton,
    SoundButton,
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
pub enum GameControlsEffect {
    InvokeGameCallback,
    InvokeMainLoop,
    ClearSurfacesRestored,
    DrawDialogBackground(GameControlsLayout),
    DrawCaption(GameControlsLayout),
    DrawSpeedLabel { highlighted: bool, x: i32, y: i32 },
    DrawSpeedRangeLabels { x: i32, y: i32, width: i32 },
    DrawScrollRateLabel { highlighted: bool, x: i32, y: i32 },
    DrawScrollRangeLabels { x: i32, y: i32, width: i32 },
    DrawAllGadgets,
    SetGameSpeedSlider(i32),
    SetScrollRateSlider(i32),
    TurnButtonOff(i32),
    FlagButtonForRedraw(i32),
    TurnButtonOn(i32),
    QueueGameSpeedEvent(i32),
    SetScrollRateOption(i32),
    TemporarilySetGameSpeed(i32),
    SaveSettings,
    RestoreGameSpeedOption(i32),
    OpenVisualControls,
    OpenSoundControls,
    ShowNoSoundCardMessage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameControlsStep {
    pub effects: Vec<GameControlsEffect>,
    pub complete: bool,
    pub gamespeed: i32,
    pub scrollrate: i32,
    pub speed_slider: i32,
    pub scroll_slider: i32,
    pub focused_button: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameControlsDialog {
    pub layout: GameControlsLayout,
    pub options: GameOptions,
    pub gamespeed: i32,
    pub scrollrate: i32,
    pub speed_slider: i32,
    pub scroll_slider: i32,
    /// Index 0 speed, 1 scroll, 2 visual, 3 sound, 4 OK.
    pub current_button: usize,
    pub process: bool,
    pub display: bool,
    pub refresh: bool,
    pub complete: bool,
}

impl GameControlsDialog {
    pub fn new(seen_width: i32, seen_height: i32, options: GameOptions) -> Self {
        let speed_slider = (MAX_SPEED_SETTING - 1 - options.game_speed).clamp(0, MAX_SPEED_SETTING - 1);
        let scroll_slider = (MAX_SCROLL_SETTING - 1 - options.scroll_rate).clamp(0, MAX_SCROLL_SETTING - 1);
        Self {
            layout: GameControlsLayout::new(seen_width, seen_height),
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

    fn snapshot(&self, effects: Vec<GameControlsEffect>) -> GameControlsStep {
        GameControlsStep {
            effects,
            complete: self.complete,
            gamespeed: self.gamespeed,
            scrollrate: self.scrollrate,
            speed_slider: self.speed_slider,
            scroll_slider: self.scroll_slider,
            focused_button: self.current_button,
        }
    }

    fn button_id(index: usize) -> Option<i32> {
        match index {
            2 => Some(BUTTON_VISUAL),
            3 => Some(BUTTON_SOUND),
            4 => Some(BUTTON_OK),
            _ => None,
        }
    }

    fn focus_move(&mut self, up: bool, effects: &mut Vec<GameControlsEffect>) {
        if let Some(button) = Self::button_id(self.current_button) {
            effects.push(GameControlsEffect::TurnButtonOff(button));
            effects.push(GameControlsEffect::FlagButtonForRedraw(button));
        }
        if up {
            self.current_button = if self.current_button == 0 { BUTTON_COUNT - 1 } else { self.current_button - 1 };
        } else {
            self.current_button = (self.current_button + 1) % BUTTON_COUNT;
        }
        if let Some(button) = Self::button_id(self.current_button) {
            effects.push(GameControlsEffect::TurnButtonOn(button));
            effects.push(GameControlsEffect::FlagButtonForRedraw(button));
        }
        self.refresh = true;
    }

    fn draw(&mut self, effects: &mut Vec<GameControlsEffect>) {
        if self.display {
            effects.push(GameControlsEffect::DrawDialogBackground(self.layout));
            effects.push(GameControlsEffect::DrawCaption(self.layout));
            self.display = false;
            self.refresh = true;
        }
        if self.refresh {
            effects.push(GameControlsEffect::DrawSpeedLabel {
                highlighted: self.current_button == 0,
                x: self.layout.speed_x,
                y: self.layout.speed_y - self.layout.text_height,
            });
            effects.push(GameControlsEffect::DrawSpeedRangeLabels {
                x: self.layout.speed_x,
                y: self.layout.speed_y + self.layout.speed_height + 1,
                width: self.layout.speed_width,
            });
            effects.push(GameControlsEffect::DrawScrollRateLabel {
                highlighted: self.current_button == 1,
                x: self.layout.scroll_x,
                y: self.layout.scroll_y - self.layout.text_height,
            });
            effects.push(GameControlsEffect::DrawScrollRangeLabels {
                x: self.layout.scroll_x,
                y: self.layout.scroll_y + self.layout.scroll_height + 1,
                width: self.layout.scroll_width,
            });
            effects.push(GameControlsEffect::DrawAllGadgets);
            self.refresh = false;
        }
    }

    fn persist_options(&mut self, effects: &mut Vec<GameControlsEffect>) {
        let next_gamespeed = MAX_SPEED_SETTING - 1 - self.speed_slider;
        let next_scrollrate = MAX_SCROLL_SETTING - 1 - self.scroll_slider;
        if next_gamespeed != self.gamespeed {
            self.gamespeed = next_gamespeed;
            // Do not mutate the live game speed. Multiplayer applies this
            // value through the queued simulation event.
            effects.push(GameControlsEffect::QueueGameSpeedEvent(self.gamespeed));
        }
        if next_scrollrate != self.scrollrate {
            self.scrollrate = next_scrollrate;
            effects.push(GameControlsEffect::SetScrollRateOption(self.scrollrate));
        }

        let old_speed = self.options.game_speed;
        effects.push(GameControlsEffect::TemporarilySetGameSpeed(self.gamespeed));
        effects.push(GameControlsEffect::SaveSettings);
        effects.push(GameControlsEffect::RestoreGameSpeedOption(old_speed));
    }

    fn select(&mut self, selection: i32, effects: &mut Vec<GameControlsEffect>) {
        self.process = false;
        self.persist_options(effects);
        match selection {
            BUTTON_VISUAL => {
                effects.push(GameControlsEffect::OpenVisualControls);
                self.process = true;
                self.display = true;
                self.refresh = true;
            }
            BUTTON_SOUND if !self.options.sound_card_available => {
                effects.push(GameControlsEffect::ShowNoSoundCardMessage);
                self.process = true;
                self.display = true;
                self.refresh = true;
            }
            BUTTON_SOUND => {
                effects.push(GameControlsEffect::OpenSoundControls);
            }
            BUTTON_OK => {}
            _ => {}
        }
        self.complete = !self.process;
    }

    /// Process one iteration of GameControlsClass::Process without blocking.
    pub fn step(
        &mut self,
        game_normal: bool,
        main_loop_requested_exit: bool,
        surfaces_restored: bool,
        input: GameControlsInput,
    ) -> GameControlsStep {
        if self.complete {
            return self.snapshot(Vec::new());
        }

        let mut effects = Vec::new();
        if game_normal {
            effects.push(GameControlsEffect::InvokeGameCallback);
        } else {
            effects.push(GameControlsEffect::InvokeMainLoop);
            if main_loop_requested_exit {
                self.process = false;
            }
        }

        if surfaces_restored {
            effects.push(GameControlsEffect::ClearSurfacesRestored);
            self.display = true;
        }
        self.draw(&mut effects);

        let mut pressed_selection = None;
        match input {
            GameControlsInput::SpeedSliderClicked => {
                self.current_button = 0;
                self.refresh = true;
            }
            GameControlsInput::ScrollSliderClicked => {
                self.current_button = 1;
                self.refresh = true;
            }
            GameControlsInput::VisualButton => pressed_selection = Some(BUTTON_VISUAL),
            GameControlsInput::SoundButton => pressed_selection = Some(BUTTON_SOUND),
            GameControlsInput::OkButton => pressed_selection = Some(BUTTON_OK),
            GameControlsInput::Escape => {
                self.process = false;
                self.complete = true;
            }
            GameControlsInput::Left => {
                if self.current_button == 0 {
                    self.speed_slider = (self.speed_slider - 1).max(0);
                    effects.push(GameControlsEffect::SetGameSpeedSlider(self.speed_slider));
                } else if self.current_button == 1 {
                    self.scroll_slider = (self.scroll_slider - 1).max(0);
                    effects.push(GameControlsEffect::SetScrollRateSlider(self.scroll_slider));
                }
            }
            GameControlsInput::Right => {
                if self.current_button == 0 {
                    self.speed_slider = (self.speed_slider + 1).min(MAX_SPEED_SETTING - 1);
                    effects.push(GameControlsEffect::SetGameSpeedSlider(self.speed_slider));
                } else if self.current_button == 1 {
                    self.scroll_slider = (self.scroll_slider + 1).min(MAX_SCROLL_SETTING - 1);
                    effects.push(GameControlsEffect::SetScrollRateSlider(self.scroll_slider));
                }
            }
            GameControlsInput::Up => self.focus_move(true, &mut effects),
            GameControlsInput::Down => self.focus_move(false, &mut effects),
            GameControlsInput::Return => pressed_selection = Some(BUTTON_FIRST + self.current_button as i32),
            GameControlsInput::SpeedSliderChanged(value) => {
                self.speed_slider = value.clamp(0, MAX_SPEED_SETTING - 1);
            }
            GameControlsInput::ScrollSliderChanged(value) => {
                self.scroll_slider = value.clamp(0, MAX_SCROLL_SETTING - 1);
            }
            GameControlsInput::None => {}
        }

        if let Some(selection) = pressed_selection {
            self.select(selection, &mut effects);
        } else if !self.process {
            self.complete = true;
        }
        self.snapshot(effects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_options() -> GameOptions {
        GameOptions { game_speed: 3, scroll_rate: 2, sound_card_available: true }
    }

    #[test]
    fn layout_matches_320_and_640_source_geometry() {
        let low = GameControlsLayout::new(320, 200);
        assert_eq!((low.dialog_width, low.dialog_height), (232, 141));
        assert_eq!((low.dialog_x, low.dialog_y), (44, 29));
        assert_eq!((low.speed_x, low.speed_y, low.speed_width, low.speed_height), (54, 71, 212, 6));
        assert_eq!((low.scroll_x, low.scroll_y), (54, 101));
        assert_eq!((low.visual_x, low.visual_y, low.visual_width), (64, 130, 192));
        assert_eq!((low.sound_x, low.sound_y), (64, 144));
        assert_eq!((low.ok_x, low.ok_y, low.ok_width, low.ok_height), (150, 156, 20, 9));

        let high = GameControlsLayout::new(640, 400);
        assert_eq!((high.factor, high.dialog_width, high.dialog_height), (2, 464, 282));
        assert_eq!((high.dialog_x, high.dialog_y), (88, 59));
        assert_eq!((high.speed_x, high.speed_y), (108, 142));
    }

    #[test]
    fn constructor_inverts_game_and_scroll_speed_to_slider_values() {
        let dialog = GameControlsDialog::new(320, 200, default_options());
        assert_eq!(dialog.speed_slider, 3);
        assert_eq!(dialog.scroll_slider, 4);
        assert_eq!(dialog.current_button, 0);
    }

    #[test]
    fn left_and_right_bump_inverse_slider_values_by_one_step() {
        let mut dialog = GameControlsDialog::new(320, 200, default_options());
        let left = dialog.step(true, false, false, GameControlsInput::Left);
        assert_eq!(left.speed_slider, 2);
        let right = dialog.step(true, false, false, GameControlsInput::Right);
        assert_eq!(right.speed_slider, 3);
        dialog.current_button = 1;
        assert_eq!(dialog.step(true, false, false, GameControlsInput::Left).scroll_slider, 3);
    }

    #[test]
    fn speed_changes_are_queued_not_written_to_live_option_until_save() {
        let mut dialog = GameControlsDialog::new(320, 200, default_options());
        dialog.step(true, false, false, GameControlsInput::Left);
        let commit = dialog.step(true, false, false, GameControlsInput::OkButton);
        assert_eq!(commit.gamespeed, 4);
        let queue_index = commit.effects.iter().position(|effect| *effect == GameControlsEffect::QueueGameSpeedEvent(4)).unwrap();
        let temporary_index = commit.effects.iter().position(|effect| *effect == GameControlsEffect::TemporarilySetGameSpeed(4)).unwrap();
        let save_index = commit.effects.iter().position(|effect| *effect == GameControlsEffect::SaveSettings).unwrap();
        let restore_index = commit.effects.iter().position(|effect| *effect == GameControlsEffect::RestoreGameSpeedOption(3)).unwrap();
        assert!(queue_index < temporary_index && temporary_index < save_index && save_index < restore_index);
        assert_eq!(dialog.options.game_speed, 3);
    }

    #[test]
    fn scroll_rate_updates_options_and_is_saved_with_game_speed() {
        let mut dialog = GameControlsDialog::new(320, 200, default_options());
        dialog.current_button = 1;
        dialog.step(true, false, false, GameControlsInput::Right);
        let saved = dialog.step(true, false, false, GameControlsInput::OkButton);
        assert_eq!(saved.scrollrate, 1);
        assert!(saved.effects.contains(&GameControlsEffect::SetScrollRateOption(1)));
    }

    #[test]
    fn focus_navigation_wraps_across_two_sliders_and_three_buttons() {
        let mut dialog = GameControlsDialog::new(320, 200, default_options());
        assert_eq!(dialog.step(true, false, false, GameControlsInput::Up).focused_button, 4);
        assert_eq!(dialog.step(true, false, false, GameControlsInput::Down).focused_button, 0);
        dialog.current_button = 2;
        let down = dialog.step(true, false, false, GameControlsInput::Down);
        assert_eq!(down.focused_button, 3);
        assert!(down.effects.contains(&GameControlsEffect::TurnButtonOff(BUTTON_VISUAL)));
        assert!(down.effects.contains(&GameControlsEffect::TurnButtonOn(BUTTON_SOUND)));
        assert_eq!(dialog.step(true, false, false, GameControlsInput::Down).focused_button, 4);
    }

    #[test]
    fn visual_controls_return_to_parent_and_force_full_redraw() {
        let mut dialog = GameControlsDialog::new(320, 200, default_options());
        let opened = dialog.step(true, false, false, GameControlsInput::VisualButton);
        assert!(!opened.complete);
        assert!(opened.effects.contains(&GameControlsEffect::OpenVisualControls));
        let redraw = dialog.step(true, false, true, GameControlsInput::None);
        assert!(redraw.effects.contains(&GameControlsEffect::ClearSurfacesRestored));
        assert!(redraw.effects.contains(&GameControlsEffect::DrawDialogBackground(dialog.layout)));
    }

    #[test]
    fn unavailable_sound_card_shows_message_and_resumes_parent() {
        let mut opts = default_options();
        opts.sound_card_available = false;
        let mut dialog = GameControlsDialog::new(320, 200, opts);
        let opened = dialog.step(true, false, false, GameControlsInput::SoundButton);
        assert!(!opened.complete);
        assert!(opened.effects.contains(&GameControlsEffect::ShowNoSoundCardMessage));
        assert!(!opened.effects.contains(&GameControlsEffect::OpenSoundControls));
    }

    #[test]
    fn available_sound_controls_open_and_close_parent_dialog() {
        let mut dialog = GameControlsDialog::new(320, 200, default_options());
        let opened = dialog.step(true, false, false, GameControlsInput::SoundButton);
        assert!(opened.complete);
        assert!(opened.effects.contains(&GameControlsEffect::OpenSoundControls));
    }

    #[test]
    fn escape_exits_without_saving_or_mutating_options() {
        let mut dialog = GameControlsDialog::new(320, 200, default_options());
        let step = dialog.step(true, false, false, GameControlsInput::Escape);
        assert!(step.complete);
        assert!(!step.effects.contains(&GameControlsEffect::SaveSettings));
        assert!(!step.effects.iter().any(|effect| matches!(effect, GameControlsEffect::QueueGameSpeedEvent(_))));
    }

    #[test]
    fn restored_surfaces_force_both_background_and_label_refresh() {
        let mut dialog = GameControlsDialog::new(320, 200, default_options());
        let step = dialog.step(true, false, true, GameControlsInput::None);
        assert!(step.effects.contains(&GameControlsEffect::ClearSurfacesRestored));
        assert!(step.effects.contains(&GameControlsEffect::DrawDialogBackground(dialog.layout)));
        assert!(step.effects.contains(&GameControlsEffect::DrawSpeedLabel { highlighted: true, x: dialog.layout.speed_x, y: dialog.layout.speed_y - dialog.layout.text_height }));
        assert!(step.effects.contains(&GameControlsEffect::DrawScrollRateLabel { highlighted: false, x: dialog.layout.scroll_x, y: dialog.layout.scroll_y - dialog.layout.text_height }));
    }

    #[test]
    fn returning_from_speed_or_scroll_focus_keeps_source_exit_behavior() {
        let mut dialog = GameControlsDialog::new(320, 200, default_options());
        let result = dialog.step(true, false, false, GameControlsInput::Return);
        assert!(result.complete);
        assert!(result.effects.contains(&GameControlsEffect::SaveSettings));
    }
}
