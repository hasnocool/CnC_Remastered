//! Red Alert SoundControlsClass and MusicListClass from REDALERT/SOUNDDLG.CPP.
//!
//! This variant mutates Options immediately as inputs arrive; unlike the
//! Tiberian Dawn dialog it does not save an aggregate settings structure on
//! exit. The modal loop is one step per frame, and all audio/render operations
//! are returned as effects.

use crate::sound_controls::{MusicListEntry, SoundTrack};

pub const BUTTON_STOP: i32 = 605;
pub const BUTTON_PLAY: i32 = 606;
pub const BUTTON_SHUFFLE: i32 = 607;
pub const BUTTON_REPEAT: i32 = 608;
pub const BUTTON_OPTIONS: i32 = 609;
pub const SLIDER_MUSIC: i32 = 610;
pub const SLIDER_SOUND: i32 = 611;
pub const BUTTON_LISTBOX: i32 = 612;
pub const THEME_QUIET: u8 = 0;
pub const VOLUME_SCALE: u16 = 256;
pub const SLIDER_MAXIMUM: u16 = 255;
pub const SLIDER_THUMB: u16 = 16;
pub const SLIDER_EFFECTIVE_MAXIMUM: u16 = SLIDER_MAXIMUM - SLIDER_THUMB;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedAlertSoundLocale { Default, German, French }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RedAlertSoundFeatures {
    pub win32: bool,
    pub fixit_csii: bool,
    pub fixit_version_3: bool,
}

impl RedAlertSoundFeatures {
    pub const fn default_win32() -> Self {
        Self { win32: true, fixit_csii: false, fixit_version_3: false }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RedAlertSoundLayout {
    pub factor: i32,
    pub option_width: i32,
    pub option_height: i32,
    pub option_x: i32,
    pub option_y: i32,
    pub listbox_x: i32,
    pub listbox_y: i32,
    pub listbox_width: i32,
    pub listbox_height: i32,
    pub return_button_x: i32,
    pub return_button_y: i32,
    pub return_button_width: i32,
    pub return_button_height: i32,
    pub stop_x: i32,
    pub stop_y: i32,
    pub play_x: i32,
    pub play_y: i32,
    pub onoff_width: i32,
    pub shuffle_x: i32,
    pub shuffle_y: i32,
    pub repeat_x: i32,
    pub repeat_y: i32,
    pub music_slider_x: i32,
    pub music_slider_y: i32,
    pub music_slider_width: i32,
    pub music_slider_height: i32,
    pub sound_slider_x: i32,
    pub sound_slider_y: i32,
    pub sound_slider_width: i32,
    pub sound_slider_height: i32,
    pub music_title_x: i32,
    pub music_title_y: i32,
    pub sound_title_x: i32,
    pub sound_title_y: i32,
    pub shuffle_title_x: i32,
    pub shuffle_title_y: i32,
    pub repeat_title_x: i32,
    pub repeat_title_y: i32,
    pub list_tabs: [i32; 3],
}

impl RedAlertSoundLayout {
    pub const fn new(
        seen_width: i32,
        seen_height: i32,
        locale: RedAlertSoundLocale,
        features: RedAlertSoundFeatures,
    ) -> Self {
        let factor = if seen_width == 320 { 1 } else { 2 };
        let option_width = if matches!(locale, RedAlertSoundLocale::French) { 308 * factor } else { 292 * factor };
        let option_height = 146 * factor;
        let option_x = (seen_width - option_width) / 2;
        let option_y = (seen_height - option_height) / 2;
        let listbox_x = 17 * factor;
        let listbox_y = 54 * factor;
        let listbox_width = option_width - listbox_x * 2;
        let listbox_height = 72 * factor + if features.fixit_csii && features.win32 { 2 } else { 0 };
        let button_width = 70 * factor;
        let button_x = option_width - (button_width + 17 * factor);
        let button_y = 128 * factor;
        let stop_x = 17 * factor;
        let stop_y = 128 * factor;
        let play_x = 35 * factor;
        let play_y = 128 * factor;
        let onoff_width = 25 * factor;
        let shuffle_x = match locale {
            RedAlertSoundLocale::German => 79 * factor,
            RedAlertSoundLocale::French => 99 * factor,
            RedAlertSoundLocale::Default => 97 * factor,
        };
        let repeat_x = if matches!(locale, RedAlertSoundLocale::French) { 169 * factor } else { 164 * factor };
        let music_slider_x = 147 * factor;
        let music_slider_y = 28 * factor;
        let music_slider_width = 108 * factor;
        let music_slider_height = 5 * factor;
        let sound_slider_x = 147 * factor;
        let sound_slider_y = 40 * factor;
        let sound_slider_width = 108 * factor;
        let sound_slider_height = 5 * factor;
        let return_button_x = option_x + button_x;
        let return_button_y = option_y + button_y;
        let title_adjust = 5 * factor;
        let locale_adjust = if matches!(locale, RedAlertSoundLocale::French) { 4 } else { 0 };
        Self {
            factor, option_width, option_height, option_x, option_y,
            listbox_x, listbox_y, listbox_width, listbox_height,
            return_button_x, return_button_y, return_button_width: button_width, return_button_height: 9 * factor,
            stop_x: option_x + stop_x, stop_y: option_y + stop_y,
            play_x: option_x + play_x, play_y: option_y + play_y,
            onoff_width,
            shuffle_x: option_x + shuffle_x, shuffle_y: option_y + 130 * factor,
            repeat_x: option_x + repeat_x, repeat_y: option_y + 130 * factor,
            music_slider_x: option_x + music_slider_x, music_slider_y: option_y + music_slider_y,
            music_slider_width, music_slider_height,
            sound_slider_x: option_x + sound_slider_x, sound_slider_y: option_y + sound_slider_y,
            sound_slider_width, sound_slider_height,
            music_title_x: option_x + music_slider_x - title_adjust,
            music_title_y: option_y + music_slider_y - 2 * factor,
            sound_title_x: option_x + sound_slider_x - title_adjust,
            sound_title_y: option_y + sound_slider_y - 2 * factor,
            shuffle_title_x: option_x + shuffle_x + locale_adjust - title_adjust,
            shuffle_title_y: option_y + 130 * factor + factor,
            repeat_title_x: option_x + repeat_x - title_adjust,
            repeat_title_y: option_y + 130 * factor + factor,
            list_tabs: [55 * factor, 72 * factor, 90 * factor],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RedAlertSoundOptions {
    /// Fixed-point values with denominator 256, mirroring Options.ScoreVolume and Options.Volume.
    pub score_volume_256: u16,
    pub sound_volume_256: u16,
    pub shuffle: bool,
    pub repeat: bool,
    pub multiplayer_score_volume_256: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedAlertThemeEntry {
    pub theme_id: u8,
    pub duration_seconds: u32,
    pub full_name: String,
    pub allowed: bool,
    pub currently_playing: bool,
}

pub fn build_theme_entries(themes: &[RedAlertThemeEntry]) -> (Vec<MusicListEntry>, Option<usize>) {
    let mut entries = Vec::new();
    let mut selected = None;
    for theme in themes.iter().filter(|theme| theme.allowed) {
        let number = entries.len() + 1;
        let display_text = format!(
            "Track {}\t{}:{:02}\t{}",
            number,
            theme.duration_seconds / 60,
            theme.duration_seconds % 60,
            theme.full_name,
        );
        if theme.currently_playing { selected = Some(entries.len()); }
        entries.push(MusicListEntry {
            theme_id_prefix: theme.theme_id,
            display_text,
            track_id: theme.theme_id,
        });
    }
    (entries, selected)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedAlertSoundSession { Normal, Skirmish, Other }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedAlertSoundInput {
    None,
    Escape,
    ReturnButton,
    MusicSliderChanged(u16),
    SoundSliderChanged(u16),
    ListboxPressed,
    SelectTrack(Option<usize>),
    StopButton,
    PlayButton,
    Space,
    ShuffleToggled(bool),
    RepeatToggled(bool),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedAlertSoundEffect {
    CallBack,
    MainLoop,
    ClearSurfacesRestored,
    DrawDialogBox(RedAlertSoundLayout),
    DrawCaption(RedAlertSoundLayout),
    DrawVolumeLabel { music: bool, x: i32, y: i32, right_aligned: bool },
    DrawShuffleLabel { x: i32, y: i32, right_aligned: bool },
    DrawRepeatLabel { x: i32, y: i32, right_aligned: bool },
    DrawAllGadgets,
    SetScoreVolume { numerator: u16, denominator: u16, apply_immediately: bool },
    SetSoundVolume { numerator: u16, denominator: u16, apply_immediately: bool },
    SetMultiplayerScoreVolume { numerator: u16, denominator: u16 },
    SetShuffle(bool),
    SetRepeat(bool),
    StopTheme,
    QueueTheme(u8),
    CleanupListEntries,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedAlertSoundStep {
    pub effects: Vec<RedAlertSoundEffect>,
    pub complete: bool,
    pub options: RedAlertSoundOptions,
    pub score_slider: u16,
    pub sound_slider: u16,
    pub selected_track: Option<usize>,
    pub entries: Vec<MusicListEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedAlertSoundDialog {
    pub layout: RedAlertSoundLayout,
    pub features: RedAlertSoundFeatures,
    pub options: RedAlertSoundOptions,
    pub score_slider: u16,
    pub sound_slider: u16,
    pub entries: Vec<MusicListEntry>,
    pub selected_track: Option<usize>,
    pub display: bool,
    pub process: bool,
    pub complete: bool,
    pub cleanup_emitted: bool,
}

impl RedAlertSoundDialog {
    pub fn new(
        seen_width: i32,
        seen_height: i32,
        locale: RedAlertSoundLocale,
        features: RedAlertSoundFeatures,
        options: RedAlertSoundOptions,
        themes: &[RedAlertThemeEntry],
    ) -> Self {
        let (entries, selected_track) = build_theme_entries(themes);
        let layout = RedAlertSoundLayout::new(seen_width, seen_height, locale, features);
        Self {
            layout,
            features,
            options,
            score_slider: options.score_volume_256.min(SLIDER_EFFECTIVE_MAXIMUM),
            sound_slider: options.sound_volume_256.min(SLIDER_EFFECTIVE_MAXIMUM),
            entries,
            selected_track,
            display: true,
            process: true,
            complete: false,
            cleanup_emitted: false,
        }
    }

    fn snapshot(&self, effects: Vec<RedAlertSoundEffect>) -> RedAlertSoundStep {
        RedAlertSoundStep {
            effects,
            complete: self.complete,
            options: self.options,
            score_slider: self.score_slider,
            sound_slider: self.sound_slider,
            selected_track: self.selected_track,
            entries: self.entries.clone(),
        }
    }

    fn finish(&mut self, effects: &mut Vec<RedAlertSoundEffect>) {
        if self.options.score_volume_256 == 0 {
            effects.push(RedAlertSoundEffect::StopTheme);
        }
        if !self.cleanup_emitted {
            effects.push(RedAlertSoundEffect::CleanupListEntries);
            self.cleanup_emitted = true;
        }
        self.process = false;
        self.complete = true;
    }

    pub fn step(
        &mut self,
        session: RedAlertSoundSession,
        main_loop_requested_exit: bool,
        surfaces_restored: bool,
        input: RedAlertSoundInput,
    ) -> RedAlertSoundStep {
        if self.complete { return self.snapshot(Vec::new()); }

        let mut effects = Vec::new();
        match session {
            RedAlertSoundSession::Normal | RedAlertSoundSession::Skirmish => {
                effects.push(RedAlertSoundEffect::CallBack);
            }
            RedAlertSoundSession::Other => {
                effects.push(RedAlertSoundEffect::MainLoop);
                if main_loop_requested_exit { self.process = false; }
            }
        }

        if surfaces_restored && self.features.win32 {
            effects.push(RedAlertSoundEffect::ClearSurfacesRestored);
            self.display = true;
        }

        if self.display {
            effects.extend([
                RedAlertSoundEffect::DrawDialogBox(self.layout),
                RedAlertSoundEffect::DrawCaption(self.layout),
                RedAlertSoundEffect::DrawVolumeLabel {
                    music: true,
                    x: self.layout.music_title_x,
                    y: self.layout.music_title_y,
                    right_aligned: true,
                },
                RedAlertSoundEffect::DrawVolumeLabel {
                    music: false,
                    x: self.layout.sound_title_x,
                    y: self.layout.sound_title_y,
                    right_aligned: true,
                },
                RedAlertSoundEffect::DrawShuffleLabel {
                    x: self.layout.shuffle_title_x,
                    y: self.layout.shuffle_title_y,
                    right_aligned: true,
                },
                RedAlertSoundEffect::DrawRepeatLabel {
                    x: self.layout.repeat_title_x,
                    y: self.layout.repeat_title_y,
                    right_aligned: true,
                },
                RedAlertSoundEffect::DrawAllGadgets,
            ]);
            self.display = false;
        }

        match input {
            RedAlertSoundInput::Escape | RedAlertSoundInput::ReturnButton => {
                self.process = false;
            }
            RedAlertSoundInput::MusicSliderChanged(value) => {
                let value = value.min(SLIDER_EFFECTIVE_MAXIMUM);
                self.score_slider = value;
                self.options.score_volume_256 = value;
                effects.push(RedAlertSoundEffect::SetScoreVolume {
                    numerator: value,
                    denominator: VOLUME_SCALE,
                    apply_immediately: true,
                });
                if self.features.fixit_version_3 && session != RedAlertSoundSession::Normal {
                    self.options.multiplayer_score_volume_256 = value;
                    effects.push(RedAlertSoundEffect::SetMultiplayerScoreVolume {
                        numerator: value,
                        denominator: VOLUME_SCALE,
                    });
                }
            }
            RedAlertSoundInput::SoundSliderChanged(value) => {
                let value = value.min(SLIDER_EFFECTIVE_MAXIMUM);
                self.sound_slider = value;
                self.options.sound_volume_256 = value;
                effects.push(RedAlertSoundEffect::SetSoundVolume {
                    numerator: value,
                    denominator: VOLUME_SCALE,
                    apply_immediately: true,
                });
            }
            RedAlertSoundInput::SelectTrack(index) => {
                self.selected_track = index.filter(|value| *value < self.entries.len());
            }
            RedAlertSoundInput::StopButton => {
                effects.push(RedAlertSoundEffect::StopTheme);
                effects.push(RedAlertSoundEffect::QueueTheme(THEME_QUIET));
            }
            RedAlertSoundInput::PlayButton | RedAlertSoundInput::Space => {
                if let Some(index) = self.selected_track {
                    if let Some(entry) = self.entries.get(index) {
                        effects.push(RedAlertSoundEffect::QueueTheme(entry.track_id));
                    }
                }
            }
            RedAlertSoundInput::ShuffleToggled(value) => {
                self.options.shuffle = value;
                effects.push(RedAlertSoundEffect::SetShuffle(value));
            }
            RedAlertSoundInput::RepeatToggled(value) => {
                self.options.repeat = value;
                effects.push(RedAlertSoundEffect::SetRepeat(value));
            }
            RedAlertSoundInput::ListboxPressed | RedAlertSoundInput::None => {}
        }

        if !self.process {
            self.finish(&mut effects);
        }
        self.snapshot(effects)
    }

    pub fn draw_music_entry(
        &self,
        renderer: &mut impl crate::sound_controls::MusicListRenderer,
        index: usize,
        x: i32,
        y: i32,
        width: i32,
        line_height: i32,
        selected: bool,
        flags: u32,
        masks: crate::sound_controls::MusicTextFlagMask,
        tabs: &[i32],
    ) {
        if let Some(entry) = self.entries.get(index) {
            crate::sound_controls::draw_music_entry(
                renderer,
                entry,
                x,
                y,
                width,
                line_height,
                selected,
                flags,
                masks,
                tabs,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn options() -> RedAlertSoundOptions {
        RedAlertSoundOptions {
            score_volume_256: 255,
            sound_volume_256: 180,
            shuffle: true,
            repeat: false,
            multiplayer_score_volume_256: 200,
        }
    }

    fn themes() -> Vec<RedAlertThemeEntry> {
        vec![
            RedAlertThemeEntry { theme_id: 0, duration_seconds: 181, full_name: "Act on Instinct".into(), allowed: true, currently_playing: false },
            RedAlertThemeEntry { theme_id: 1, duration_seconds: 62, full_name: "No Mercy".into(), allowed: false, currently_playing: false },
            RedAlertThemeEntry { theme_id: 2, duration_seconds: 125, full_name: "C&C Thang".into(), allowed: true, currently_playing: true },
        ]
    }

    fn dialog(locale: RedAlertSoundLocale, features: RedAlertSoundFeatures) -> RedAlertSoundDialog {
        RedAlertSoundDialog::new(320, 200, locale, features, options(), &themes())
    }

    #[test]
    fn default_layout_matches_red_alert_header_coordinates() {
        let layout = RedAlertSoundLayout::new(320, 200, RedAlertSoundLocale::Default, RedAlertSoundFeatures::default_win32());
        assert_eq!((layout.option_width, layout.option_height), (292, 146));
        assert_eq!((layout.option_x, layout.option_y), (14, 27));
        assert_eq!((layout.listbox_x, layout.listbox_y, layout.listbox_width, layout.listbox_height), (17, 54, 258, 72));
        assert_eq!((layout.return_button_x, layout.return_button_y, layout.return_button_width), (171, 155, 70));
        assert_eq!((layout.stop_x, layout.play_x, layout.stop_y, layout.play_y), (31, 155, 155, 155));
        assert_eq!((layout.music_slider_x, layout.music_slider_y, layout.music_slider_width), (161, 55, 108));
        assert_eq!((layout.sound_slider_x, layout.sound_slider_y), (161, 67));
        assert_eq!(layout.list_tabs, [55, 72, 90]);
    }

    #[test]
    fn french_layout_changes_width_button_shuffle_and_repeat_positions() {
        let french = RedAlertSoundLayout::new(320, 200, RedAlertSoundLocale::French, RedAlertSoundFeatures::default_win32());
        assert_eq!((french.option_width, french.option_x), (308, 6));
        assert_eq!(french.shuffle_x, 109);
        assert_eq!(french.shuffle_title_x, 111);
        assert_eq!(french.repeat_x, 175);
    }

    #[test]
    fn german_layout_uses_german_shuffle_coordinate() {
        let german = RedAlertSoundLayout::new(320, 200, RedAlertSoundLocale::German, RedAlertSoundFeatures::default_win32());
        assert_eq!(german.shuffle_x, 93);
    }

    #[test]
    fn fixit_csii_adds_two_pixels_to_win32_listbox_height_only() {
        let enabled = RedAlertSoundFeatures { fixit_csii: true, ..RedAlertSoundFeatures::default_win32() };
        let disabled = RedAlertSoundFeatures::default_win32();
        assert_eq!(RedAlertSoundLayout::new(320, 200, RedAlertSoundLocale::Default, enabled).listbox_height, 74);
        assert_eq!(RedAlertSoundLayout::new(320, 200, RedAlertSoundLocale::Default, RedAlertSoundFeatures { win32: false, fixit_csii: true, fixit_version_3: false }).listbox_height, 72);
        assert_eq!(RedAlertSoundLayout::new(320, 200, RedAlertSoundLocale::Default, disabled).listbox_height, 72);
    }

    #[test]
    fn theme_list_filters_allowed_items_and_preserves_prefix_track_ids() {
        let (entries, selected) = build_theme_entries(&themes());
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].theme_id_prefix, 0);
        assert_eq!(entries[0].display_text, "Track 1\t3:01\tAct on Instinct");
        assert_eq!(entries[1].theme_id_prefix, 2);
        assert_eq!(selected, Some(1));
    }

    #[test]
    fn sliders_clamp_to_maximum_minus_thumb_and_update_settings_immediately() {
        let mut dialog = dialog(RedAlertSoundLocale::Default, RedAlertSoundFeatures::default_win32());
        assert_eq!((dialog.score_slider, dialog.sound_slider), (239, 180));
        let music = dialog.step(RedAlertSoundSession::Normal, false, false, false, RedAlertSoundInput::MusicSliderChanged(250));
        assert_eq!(music.options.score_volume_256, 239);
        assert!(music.effects.contains(&RedAlertSoundEffect::SetScoreVolume { numerator: 239, denominator: 256, apply_immediately: true }));
        let sound = dialog.step(RedAlertSoundSession::Normal, false, false, false, RedAlertSoundInput::SoundSliderChanged(90));
        assert_eq!(sound.options.sound_volume_256, 90);
        assert!(sound.effects.contains(&RedAlertSoundEffect::SetSoundVolume { numerator: 90, denominator: 256, apply_immediately: true }));
    }

    #[test]
    fn fixit_version_three_mirrors_multiplayer_score_volume_only_outside_normal_game() {
        let features = RedAlertSoundFeatures { fixit_version_3: true, ..RedAlertSoundFeatures::default_win32() };
        let mut dialog = dialog(RedAlertSoundLocale::Default, features);
        let multiplayer = dialog.step(RedAlertSoundSession::Other, false, false, false, RedAlertSoundInput::MusicSliderChanged(111));
        assert_eq!(multiplayer.options.multiplayer_score_volume_256, 111);
        assert!(multiplayer.effects.contains(&RedAlertSoundEffect::SetMultiplayerScoreVolume { numerator: 111, denominator: 256 }));
        let normal = dialog.step(RedAlertSoundSession::Normal, false, false, false, RedAlertSoundInput::MusicSliderChanged(100));
        assert_eq!(normal.options.multiplayer_score_volume_256, 111);
        assert!(!normal.effects.iter().any(|effect| matches!(effect, RedAlertSoundEffect::SetMultiplayerScoreVolume { .. })));
    }

    #[test]
    fn stop_queues_quiet_theme_and_play_uses_selected_theme_id() {
        let mut dialog = dialog(RedAlertSoundLocale::Default, RedAlertSoundFeatures::default_win32());
        let stop = dialog.step(RedAlertSoundSession::Normal, false, false, false, RedAlertSoundInput::StopButton);
        assert!(stop.effects.contains(&RedAlertSoundEffect::StopTheme));
        assert!(stop.effects.contains(&RedAlertSoundEffect::QueueTheme(THEME_QUIET)));
        let play = dialog.step(RedAlertSoundSession::Normal, false, false, false, RedAlertSoundInput::PlayButton);
        assert!(play.effects.contains(&RedAlertSoundEffect::QueueTheme(2)));
        let space = dialog.step(RedAlertSoundSession::Normal, false, false, false, RedAlertSoundInput::Space);
        assert!(space.effects.contains(&RedAlertSoundEffect::QueueTheme(2)));
    }

    #[test]
    fn shuffle_and_repeat_are_toggle_state_and_apply_options_immediately() {
        let mut dialog = dialog(RedAlertSoundLocale::Default, RedAlertSoundFeatures::default_win32());
        let shuffle = dialog.step(RedAlertSoundSession::Normal, false, false, false, RedAlertSoundInput::ShuffleToggled(false));
        assert!(!shuffle.options.shuffle);
        assert!(shuffle.effects.contains(&RedAlertSoundEffect::SetShuffle(false)));
        let repeat = dialog.step(RedAlertSoundSession::Normal, false, false, false, RedAlertSoundInput::RepeatToggled(true));
        assert!(repeat.options.repeat);
        assert!(repeat.effects.contains(&RedAlertSoundEffect::SetRepeat(true)));
    }

    #[test]
    fn exit_stops_theme_when_score_volume_zero_and_frees_list_once() {
        let mut dialog = dialog(RedAlertSoundLocale::Default, RedAlertSoundFeatures::default_win32());
        dialog.step(RedAlertSoundSession::Normal, false, false, false, RedAlertSoundInput::MusicSliderChanged(0));
        let done = dialog.step(RedAlertSoundSession::Normal, false, false, false, RedAlertSoundInput::Escape);
        assert!(done.complete);
        assert!(done.effects.contains(&RedAlertSoundEffect::StopTheme));
        assert!(done.effects.contains(&RedAlertSoundEffect::CleanupListEntries));
        let second = dialog.step(RedAlertSoundSession::Normal, false, false, false, RedAlertSoundInput::None);
        assert!(second.effects.is_empty());
    }

    #[test]
    fn win32_surface_restore_requests_redraw_while_non_win32_does_not() {
        let mut win = dialog(RedAlertSoundLocale::Default, RedAlertSoundFeatures::default_win32());
        let result = win.step(RedAlertSoundSession::Normal, false, true, false, RedAlertSoundInput::None);
        assert!(result.effects.contains(&RedAlertSoundEffect::ClearSurfacesRestored));
        let features = RedAlertSoundFeatures { win32: false, ..RedAlertSoundFeatures::default_win32() };
        let mut dos = dialog(RedAlertSoundLocale::Default, features);
        let result = dos.step(RedAlertSoundSession::Normal, false, true, false, RedAlertSoundInput::None);
        assert!(!result.effects.contains(&RedAlertSoundEffect::ClearSurfacesRestored));
    }

    #[test]
    fn main_loop_exit_runs_final_cleanup_once() {
        let mut dialog = dialog(RedAlertSoundLocale::Default, RedAlertSoundFeatures::default_win32());
        let result = dialog.step(RedAlertSoundSession::Other, true, false, RedAlertSoundInput::None);
        assert!(result.complete);
        assert!(result.effects.contains(&RedAlertSoundEffect::MainLoop));
        assert!(result.effects.contains(&RedAlertSoundEffect::CleanupListEntries));
    }

    #[test]
    fn empty_theme_list_does_not_queue_unselected_music() {
        let mut dialog = RedAlertSoundDialog::new(320, 200, RedAlertSoundLocale::Default, RedAlertSoundFeatures::default_win32(), options(), &[]);
        let result = dialog.step(RedAlertSoundSession::Normal, false, false, false, RedAlertSoundInput::PlayButton);
        assert!(!result.effects.iter().any(|effect| matches!(effect, RedAlertSoundEffect::QueueTheme(_))));
    }
}
