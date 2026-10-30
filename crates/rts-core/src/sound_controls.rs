//! Tiberian Dawn SoundControlsClass and MusicListClass from SOUNDDLG.CPP.
//!
//! The blocking Process loop is represented as one bounded step per frame.
//! Audio playback, settings persistence, game callbacks and drawing remain
//! host effects; the selection and option state lives in this module.

pub const BUTTON_STOP: i32 = 605;
pub const BUTTON_PLAY: i32 = 606;
pub const BUTTON_SHUFFLE: i32 = 607;
pub const BUTTON_REPEAT: i32 = 608;
pub const BUTTON_OPTIONS: i32 = 609;
pub const SLIDER_MUSIC: i32 = 610;
pub const SLIDER_SOUND: i32 = 611;
pub const BUTTON_LISTBOX: i32 = 612;
pub const MUSIC_SLIDER_MAXIMUM: u8 = 255;
pub const MUSIC_SLIDER_THUMB: u8 = 16;
pub const MUSIC_SLIDER_EFFECTIVE_MAXIMUM: u8 = MUSIC_SLIDER_MAXIMUM - MUSIC_SLIDER_THUMB;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundControlsLocale { Default, French, German }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SoundControlsLayout {
    pub factor: i32,
    pub option_width: i32,
    pub option_height: i32,
    pub option_x: i32,
    pub option_y: i32,
    pub listbox_x: i32,
    pub listbox_y: i32,
    pub listbox_width: i32,
    pub listbox_height: i32,
    pub button_width: i32,
    pub button_x: i32,
    pub button_y: i32,
    pub return_button_x: i32,
    pub return_button_width: i32,
    pub stop_x: i32,
    pub stop_y: i32,
    pub play_x: i32,
    pub play_y: i32,
    pub on_off_width: i32,
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
    pub list_tabs: [i32; 3],
}

impl SoundControlsLayout {
    /// Port of SoundControlsClass::Init plus Process's French button adjustment.
    pub const fn new(width: i32, height: i32, locale: SoundControlsLocale) -> Self {
        let factor = if width == 320 { 1 } else { 2 };
        let option_width = 292 * factor;
        let option_height = 146 * factor;
        let option_x = (width - option_width) / 2;
        let option_y = (height - option_height) / 2;
        let button_width = 85 * factor;
        let button_x = option_width - (button_width + 7 * factor);
        let french = matches!(locale, SoundControlsLocale::French);
        let return_button_x = option_x + button_x - if french { 16 } else { 0 };
        let return_button_width = button_width + if french { 22 } else { 0 };
        Self {
            factor,
            option_width,
            option_height,
            option_x,
            option_y,
            listbox_x: factor,
            listbox_y: 54 * factor,
            listbox_width: 290 * factor,
            listbox_height: 73 * factor,
            button_width,
            button_x,
            button_y: 130 * factor,
            return_button_x,
            return_button_width,
            stop_x: 5 * factor,
            stop_y: 129 * factor,
            play_x: 23 * factor,
            play_y: 129 * factor,
            on_off_width: 25 * factor,
            shuffle_x: if matches!(locale, SoundControlsLocale::German) { 79 * factor } else { 91 * factor },
            shuffle_y: 130 * factor,
            repeat_x: 166 * factor,
            repeat_y: 130 * factor,
            music_slider_x: 147 * factor,
            music_slider_y: 28 * factor,
            music_slider_width: 108 * factor,
            music_slider_height: 5 * factor,
            sound_slider_x: 147 * factor,
            sound_slider_y: 40 * factor,
            sound_slider_width: 108 * factor,
            sound_slider_height: 5 * factor,
            list_tabs: [55 * factor, 72 * factor, 90 * factor],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoundTrack {
    pub id: u8,
    pub duration_seconds: u32,
    pub full_name: String,
    pub allowed: bool,
    pub currently_playing: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MusicListEntry {
    /// C++ stores the ThemeType as byte zero and renders text from byte one.
    pub theme_id_prefix: u8,
    pub display_text: String,
    pub track_id: u8,
}

pub fn build_music_list(tracks: &[SoundTrack]) -> (Vec<MusicListEntry>, Option<usize>) {
    let mut entries = Vec::new();
    let mut selected = None;
    for track in tracks.iter().filter(|track| track.allowed) {
        let number = entries.len() + 1;
        let minutes = track.duration_seconds / 60;
        let seconds = track.duration_seconds % 60;
        let display_text = format!("Track {}\t{}:{:02}\t{}", number, minutes, seconds, track.full_name);
        if track.currently_playing {
            selected = Some(entries.len());
        }
        entries.push(MusicListEntry {
            theme_id_prefix: track.id,
            display_text,
            track_id: track.id,
        });
    }
    (entries, selected)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SoundSettings {
    pub score_volume: u8,
    pub sound_volume: u8,
    pub shuffle: bool,
    pub repeat: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundControlsInput {
    None,
    Escape,
    OptionsButton,
    MusicSlider(u8),
    SoundSlider(u8),
    ListboxPressed,
    StopButton,
    PlayButton,
    Space,
    ShuffleToggled(bool),
    RepeatToggled(bool),
    Other(i32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SoundControlsEffect {
    InvokeGameCallback,
    InvokeMainLoop,
    ClearSurfacesRestored,
    DrawBackground(SoundControlsLayout),
    DrawCaption(SoundControlsLayout),
    DrawVolumeAndToggleLabels(SoundControlsLayout),
    DrawAllGadgets,
    SetScoreVolume(u8),
    SetSoundVolume { value: u8, apply_immediately: bool },
    SetShuffle(bool),
    SetRepeat(bool),
    QueueSong(Option<u8>),
    StopTheme,
    SaveSettings,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoundControlsStep {
    pub effects: Vec<SoundControlsEffect>,
    pub complete: bool,
    pub settings: SoundSettings,
    pub selected_track: Option<usize>,
    pub track_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoundControlsDialog {
    pub layout: SoundControlsLayout,
    pub settings: SoundSettings,
    pub music_slider_value: u8,
    pub sound_slider_value: u8,
    pub entries: Vec<MusicListEntry>,
    pub selected_track: Option<usize>,
    pub display: bool,
    pub process: bool,
    pub finalized: bool,
}

impl SoundControlsDialog {
    pub fn new(
        width: i32,
        height: i32,
        locale: SoundControlsLocale,
        settings: SoundSettings,
        tracks: &[SoundTrack],
    ) -> Self {
        let (entries, selected_track) = build_music_list(tracks);
        Self {
            layout: SoundControlsLayout::new(width, height, locale),
            settings,
            // SliderClass::Set_Value caps to MaxValue - Thumb.
            music_slider_value: settings.score_volume.min(MUSIC_SLIDER_EFFECTIVE_MAXIMUM),
            sound_slider_value: settings.sound_volume.min(MUSIC_SLIDER_EFFECTIVE_MAXIMUM),
            entries,
            selected_track,
            display: true,
            process: true,
            finalized: false,
        }
    }

    fn snapshot(&self, effects: Vec<SoundControlsEffect>) -> SoundControlsStep {
        SoundControlsStep {
            effects,
            complete: !self.process,
            settings: self.settings,
            selected_track: self.selected_track,
            track_count: self.entries.len(),
        }
    }

    fn finalize(&mut self, effects: &mut Vec<SoundControlsEffect>) {
        if self.finalized { return; }
        if self.settings.score_volume == 0 {
            effects.push(SoundControlsEffect::StopTheme);
        }
        effects.push(SoundControlsEffect::SaveSettings);
        self.finalized = true;
    }

    /// Processes one iteration of SoundControlsClass::Process.
    ///
    /// The host supplies game mode, callback/exit and surface-restoration
    /// signals. No blocking loop, audio I/O or disk operation is performed.
    pub fn step(
        &mut self,
        game_normal: bool,
        main_loop_requested_exit: bool,
        surfaces_restored: bool,
        input: SoundControlsInput,
    ) -> SoundControlsStep {
        if self.finalized {
            return self.snapshot(Vec::new());
        }

        let mut effects = Vec::new();
        if game_normal {
            effects.push(SoundControlsEffect::InvokeGameCallback);
        } else {
            effects.push(SoundControlsEffect::InvokeMainLoop);
            if main_loop_requested_exit {
                self.process = false;
            }
        }

        if surfaces_restored {
            effects.push(SoundControlsEffect::ClearSurfacesRestored);
            self.display = true;
        }
        if self.display {
            effects.push(SoundControlsEffect::DrawBackground(self.layout));
            effects.push(SoundControlsEffect::DrawCaption(self.layout));
            effects.push(SoundControlsEffect::DrawVolumeAndToggleLabels(self.layout));
            effects.push(SoundControlsEffect::DrawAllGadgets);
            self.display = false;
        }

        match input {
            SoundControlsInput::Escape | SoundControlsInput::OptionsButton => {
                self.process = false;
            }
            SoundControlsInput::MusicSlider(value) => {
                let value = value.min(MUSIC_SLIDER_EFFECTIVE_MAXIMUM);
                self.music_slider_value = value;
                self.settings.score_volume = value;
                effects.push(SoundControlsEffect::SetScoreVolume(value));
            }
            SoundControlsInput::SoundSlider(value) => {
                let value = value.min(MUSIC_SLIDER_EFFECTIVE_MAXIMUM);
                self.sound_slider_value = value;
                self.settings.sound_volume = value;
                effects.push(SoundControlsEffect::SetSoundVolume {
                    value,
                    apply_immediately: true,
                });
            }
            SoundControlsInput::ListboxPressed => {}
            SoundControlsInput::StopButton => {
                effects.push(SoundControlsEffect::QueueSong(None));
            }
            SoundControlsInput::PlayButton | SoundControlsInput::Space => {
                if let Some(index) = self.selected_track {
                    if let Some(entry) = self.entries.get(index) {
                        effects.push(SoundControlsEffect::QueueSong(Some(entry.track_id)));
                    }
                }
            }
            SoundControlsInput::ShuffleToggled(value) => {
                self.settings.shuffle = value;
                effects.push(SoundControlsEffect::SetShuffle(value));
            }
            SoundControlsInput::RepeatToggled(value) => {
                self.settings.repeat = value;
                effects.push(SoundControlsEffect::SetRepeat(value));
            }
            SoundControlsInput::None | SoundControlsInput::Other(_) => {}
        }

        if !self.process {
            self.finalize(&mut effects);
        }
        self.snapshot(effects)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MusicTextColor { Green, Blue, White, Black, GreenShadow }

pub trait MusicListRenderer {
    fn fill_rect(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: MusicTextColor);
    fn clipped_text(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        foreground: MusicTextColor,
        background: MusicTextColor,
        flags: u32,
        width: i32,
        tabs: &[i32],
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MusicTextFlagMask {
    pub six_point_gradient: u32,
    pub use_gradient_palette: u32,
    pub bright_color: u32,
    pub medium_color: u32,
}

/// Port of MusicListClass::Draw_Entry; the first theme-id byte is not printed.
pub fn draw_music_entry(
    renderer: &mut impl MusicListRenderer,
    entry: &MusicListEntry,
    x: i32,
    y: i32,
    width: i32,
    line_height: i32,
    selected: bool,
    text_flags: u32,
    masks: MusicTextFlagMask,
    tabs: &[i32],
) {
    let flags = if text_flags & masks.six_point_gradient != 0 {
        let mut flags = text_flags;
        if selected {
            flags |= masks.bright_color;
            renderer.fill_rect(
                x, y, x + width - 1, y + line_height - 1,
                MusicTextColor::GreenShadow,
            );
        } else if text_flags & masks.use_gradient_palette == 0 {
            flags |= masks.medium_color;
        }
        flags
    } else {
        text_flags
    };
    let foreground = if text_flags & masks.six_point_gradient != 0 {
        MusicTextColor::Green
    } else if selected {
        MusicTextColor::Blue
    } else {
        MusicTextColor::White
    };
    renderer.clipped_text(
        &entry.display_text, x, y, foreground, MusicTextColor::Black, flags, width, tabs,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settings() -> SoundSettings {
        SoundSettings { score_volume: 255, sound_volume: 180, shuffle: true, repeat: false }
    }

    fn tracks() -> Vec<SoundTrack> {
        vec![
            SoundTrack { id: 0, duration_seconds: 181, full_name: "Act on Instinct".into(), allowed: true, currently_playing: false },
            SoundTrack { id: 1, duration_seconds: 62, full_name: "No Mercy".into(), allowed: false, currently_playing: false },
            SoundTrack { id: 2, duration_seconds: 125, full_name: "C&C Thang".into(), allowed: true, currently_playing: true },
        ]
    }

    #[test]
    fn layout_matches_low_and_high_resolution_and_locale_offsets() {
        let standard = SoundControlsLayout::new(320, 200, SoundControlsLocale::Default);
        assert_eq!((standard.option_width, standard.option_height), (292, 146));
        assert_eq!((standard.option_x, standard.option_y), (14, 27));
        assert_eq!((standard.listbox_x, standard.listbox_y, standard.listbox_width, standard.listbox_height), (1, 54, 290, 73));
        assert_eq!((standard.music_slider_x, standard.music_slider_y, standard.music_slider_width), (147, 28, 108));
        assert_eq!(standard.list_tabs, [55, 72, 90]);
        let high = SoundControlsLayout::new(640, 400, SoundControlsLocale::German);
        assert_eq!((high.factor, high.option_width, high.option_height), (2, 584, 292));
        assert_eq!((high.option_x, high.option_y), (28, 54));
        assert_eq!(high.shuffle_x, 158);
        let french = SoundControlsLayout::new(320, 200, SoundControlsLocale::French);
        assert_eq!(french.return_button_x, french.option_x + french.button_x - 16);
        assert_eq!(french.return_button_width, french.button_width + 22);
    }

    #[test]
    fn theme_catalogue_filters_unavailable_tracks_and_formats_duration() {
        let (entries, selected) = build_music_list(&tracks());
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].track_id, 0);
        assert_eq!(entries[0].display_text, "Track 1\t3:01\tAct on Instinct");
        assert_eq!(entries[1].display_text, "Track 2\t2:05\tC&C Thang");
        assert_eq!(selected, Some(1));
    }

    #[test]
    fn sliders_start_at_source_maximum_minus_thumb_size() {
        let dialog = SoundControlsDialog::new(320, 200, SoundControlsLocale::Default, settings(), &tracks());
        assert_eq!(dialog.music_slider_value, 239);
        assert_eq!(dialog.sound_slider_value, 180);
        assert_eq!(dialog.settings.score_volume, 255);
        assert_eq!(dialog.selected_track, Some(1));
    }

    #[test]
    fn music_and_sound_slider_events_update_independent_settings() {
        let mut dialog = SoundControlsDialog::new(320, 200, SoundControlsLocale::Default, settings(), &tracks());
        let music = dialog.step(true, false, false, SoundControlsInput::MusicSlider(210));
        assert_eq!(music.settings.score_volume, 210);
        assert!(music.effects.contains(&SoundControlsEffect::SetScoreVolume(210)));
        let sound = dialog.step(true, false, false, SoundControlsInput::SoundSlider(250));
        assert_eq!(sound.settings.sound_volume, 239);
        assert!(sound.effects.contains(&SoundControlsEffect::SetSoundVolume { value: 239, apply_immediately: true }));
    }

    #[test]
    fn shuffle_and_repeat_toggle_events_update_options() {
        let mut dialog = SoundControlsDialog::new(320, 200, SoundControlsLocale::Default, settings(), &tracks());
        let shuffle = dialog.step(true, false, false, SoundControlsInput::ShuffleToggled(false));
        let repeat = dialog.step(true, false, false, SoundControlsInput::RepeatToggled(true));
        assert!(!shuffle.settings.shuffle);
        assert!(shuffle.effects.contains(&SoundControlsEffect::SetShuffle(false)));
        assert!(repeat.settings.repeat);
        assert!(repeat.effects.contains(&SoundControlsEffect::SetRepeat(true)));
    }

    #[test]
    fn stop_play_and_space_queue_the_source_song_identifiers() {
        let mut dialog = SoundControlsDialog::new(320, 200, SoundControlsLocale::Default, settings(), &tracks());
        assert!(dialog.step(true, false, false, SoundControlsInput::StopButton).effects.contains(&SoundControlsEffect::QueueSong(None)));
        assert!(dialog.step(true, false, false, SoundControlsInput::PlayButton).effects.contains(&SoundControlsEffect::QueueSong(Some(2))));
        assert!(dialog.step(true, false, false, SoundControlsInput::Space).effects.contains(&SoundControlsEffect::QueueSong(Some(2))));
    }

    #[test]
    fn play_with_empty_or_unselected_catalogue_does_not_queue_a_song() {
        let mut dialog = SoundControlsDialog::new(320, 200, SoundControlsLocale::Default, settings(), &[]);
        assert!(!dialog.step(true, false, false, SoundControlsInput::PlayButton).effects.iter().any(|effect| matches!(effect, SoundControlsEffect::QueueSong(_))));
    }

    #[test]
    fn first_step_and_surface_restore_redraw_the_dialog() {
        let mut dialog = SoundControlsDialog::new(320, 200, SoundControlsLocale::Default, settings(), &tracks());
        let first = dialog.step(true, false, false, SoundControlsInput::None);
        assert!(first.effects.contains(&SoundControlsEffect::InvokeGameCallback));
        assert!(first.effects.contains(&SoundControlsEffect::DrawBackground(dialog.layout)));
        assert!(first.effects.contains(&SoundControlsEffect::DrawAllGadgets));
        let restored = dialog.step(true, false, true, SoundControlsInput::None);
        assert!(restored.effects.contains(&SoundControlsEffect::ClearSurfacesRestored));
        assert!(restored.effects.contains(&SoundControlsEffect::DrawCaption(dialog.layout)));
    }

    #[test]
    fn escape_saves_settings_and_stops_theme_when_music_is_muted() {
        let mut dialog = SoundControlsDialog::new(320, 200, SoundControlsLocale::Default, settings(), &tracks());
        dialog.step(true, false, false, SoundControlsInput::MusicSlider(0));
        let done = dialog.step(true, false, false, SoundControlsInput::Escape);
        assert!(done.complete);
        assert!(done.effects.contains(&SoundControlsEffect::StopTheme));
        assert!(done.effects.contains(&SoundControlsEffect::SaveSettings));
        let second = dialog.step(true, false, false, SoundControlsInput::None);
        assert!(second.effects.is_empty());
    }

    #[test]
    fn main_loop_exit_finalizes_after_current_frame_and_save_occurs_once() {
        let mut dialog = SoundControlsDialog::new(320, 200, SoundControlsLocale::Default, settings(), &tracks());
        let step = dialog.step(false, true, false, SoundControlsInput::None);
        assert!(step.complete);
        assert!(step.effects.contains(&SoundControlsEffect::InvokeMainLoop));
        assert_eq!(step.effects.iter().filter(|effect| **effect == SoundControlsEffect::SaveSettings).count(), 1);
        assert!(!dialog.step(false, false, false, SoundControlsInput::None).effects.iter().any(|effect| matches!(effect, SoundControlsEffect::SaveSettings)));
    }

    struct Renderer {
        fills: Vec<(i32, i32, i32, i32, MusicTextColor)>,
        prints: Vec<(String, MusicTextColor, u32)>,
    }

    impl MusicListRenderer for Renderer {
        fn fill_rect(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: MusicTextColor) {
            self.fills.push((x1, y1, x2, y2, color));
        }
        fn clipped_text(&mut self, text: &str, _x: i32, _y: i32, foreground: MusicTextColor, _background: MusicTextColor, flags: u32, _width: i32, _tabs: &[i32]) {
            self.prints.push((text.to_owned(), foreground, flags));
        }
    }

    #[test]
    fn music_entry_renderer_preserves_gradient_highlight_and_non_gradient_path() {
        let entry = build_music_list(&tracks()).0.remove(0);
        let masks = MusicTextFlagMask { six_point_gradient: 1, use_gradient_palette: 2, bright_color: 4, medium_color: 8 };
        let mut renderer = Renderer { fills: Vec::new(), prints: Vec::new() };
        draw_music_entry(&mut renderer, &entry, 3, 4, 20, 8, true, 1, masks, &[55,72,90]);
        assert_eq!(renderer.fills, vec![(3,4,22,11,MusicTextColor::GreenShadow)]);
        assert_eq!(renderer.prints[0], (entry.display_text.clone(), MusicTextColor::Green, 5));
        draw_music_entry(&mut renderer, &entry, 3, 4, 20, 8, false, 0, masks, &[]);
        assert_eq!(renderer.prints[1].1, MusicTextColor::White);
    }
}
