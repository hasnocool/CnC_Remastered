//! Tiberian Dawn ending and Nod target-selection sequences from ENDING.CPP.

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EndingEvent {
    FadeToBlack,
    FadeToPalette,
    LoadTitleScreen(&'static str),
    BlitHiddenPageToSeen,
    ClearVisiblePage,
    PlayMovie { name: String, theme_none: bool, preserve_screen: bool },
    ScorePresentation,
    CreatePseudoSeenBuffer { width: i32, height: i32 },
    CreateTextPrintBuffer { width: i32, height: i32 },
    ClearBlitList,
    ClearSeenBuffer,
    ClearHiddenPage,
    ClearPseudoSeenBuffer,
    LoadPalette(&'static str),
    LoadUncompressed(&'static str),
    CopyHiddenPixelsToSatSelection { bytes: usize },
    CopySatSelectionToSeen { bytes: usize },
    BlitSystemPageToPseudo,
    HideMouse,
    ShowMouse,
    WaitVerticalBlank,
    SetPalette,
    SetInterpolationPalette,
    IncreasePaletteLuminance { red: u8, green: u8, blue: u8, maximum: u8 },
    ReadInterpolationPalette(&'static str),
    InterpolateDoubleScale(&'static str),
    ClearKeyboard,
    LoadSample(&'static str),
    PlaySample { name: &'static str, volume: u8, pan: u8 },
    ShowTargetText { x: i32, y: i32 },
    CallbackDelay(u32),
    WaitForKey,
    ClearKeyBuffer,
    DeleteSatSelectionPixels,
    DeletePseudoSeenBuffer,
    DeleteScoreObjects,
    FillSeenRectangle { left: i32, top: i32, right: i32, bottom: i32, color: u8 },
    FillTextRectangle { left: i32, top: i32, right: i32, bottom: i32, color: u8 },
    RestoreFont,
    RestoreFontSpacing,
    FreeSample(&'static str),
    PreserveVqaScreen,
    ScaleAttractScreen { source_width: i32, source_height: i32, dest_width: i32, dest_height: i32 },
    WaitCallbacks { seconds: u32 },
    FreeLocalPalette,
    DeleteTextPrintBuffer,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NodInput {
    None,
    OtherKey,
    MouseDown { x: i32, y: i32 },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodEndingState {
    pub selection: u8,
    pub printed_text: bool,
    pub mouse_shown: bool,
    pub done: bool,
    pub not_for_win95: bool,
    pub trailer_available: bool,
}

impl NodEndingState {
    /// Emit the setup and presentation prefix through the start of the
    /// interactive target-selection loop.
    pub fn begin(not_for_win95: bool, trailer_available: bool) -> (Self, Vec<EndingEvent>) {
        let state = Self {
            selection: 1, printed_text: false, mouse_shown: false, done: false,
            not_for_win95, trailer_available,
        };
        let mut events = vec![
            EndingEvent::ScorePresentation,
            EndingEvent::CreatePseudoSeenBuffer { width: 320, height: 200 },
            EndingEvent::CreateTextPrintBuffer { width: 640, height: 400 },
            EndingEvent::ClearBlitList,
            EndingEvent::ClearSeenBuffer,
            EndingEvent::ClearHiddenPage,
            EndingEvent::ClearPseudoSeenBuffer,
            EndingEvent::LoadPalette("SATSEL.PAL"),
            EndingEvent::LoadUncompressed("SATSEL.CPS"),
        ];
        if not_for_win95 {
            events.push(EndingEvent::CopyHiddenPixelsToSatSelection { bytes: 64000 });
        } else {
            events.push(EndingEvent::BlitSystemPageToPseudo);
        }
        events.extend([
            EndingEvent::LoadSample("KANEFINL.AUD"),
            EndingEvent::LoadSample("LOOPIE6M.AUD"),
            EndingEvent::PlayMovie { name: "NODFINAL".into(), theme_none: true, preserve_screen: false },
            EndingEvent::HideMouse,
            EndingEvent::WaitVerticalBlank,
            EndingEvent::SetPalette,
        ]);
        if not_for_win95 {
            events.push(EndingEvent::CopySatSelectionToSeen { bytes: 64000 });
        }
        events.extend([
            EndingEvent::ShowMouse,
            EndingEvent::SetInterpolationPalette,
            EndingEvent::IncreasePaletteLuminance { red: 30, green: 30, blue: 30, maximum: 63 },
            EndingEvent::ReadInterpolationPalette("SATSELIN.PAL"),
            EndingEvent::InterpolateDoubleScale("SATSELIN.PAL"),
            EndingEvent::ClearKeyboard,
            EndingEvent::PlaySample { name: "KANEFINL.AUD", volume: 255, pan: 128 },
            EndingEvent::PlaySample { name: "LOOPIE6M.AUD", volume: 255, pan: 128 },
        ]);
        (state, events)
    }

    /// One loop iteration. The host reports sound playback and input state;
    /// a click inside the target-selection rectangle completes the sequence.
    pub fn step(
        &mut self, input: NodInput, kanefinl_playing: bool, loopie6m_playing: bool,
    ) -> Vec<EndingEvent> {
        if self.done { return Vec::new(); }
        let mut events = Vec::new();
        if !self.printed_text && !kanefinl_playing {
            self.printed_text = true;
            self.mouse_shown = true;
            events.push(EndingEvent::ShowTargetText { x: 0, y: 180 });
            events.push(EndingEvent::ShowMouse);
        }
        events.push(EndingEvent::CallbackDelay(1));
        match input {
            NodInput::None => {
                if !loopie6m_playing {
                    events.push(EndingEvent::PlaySample { name: "LOOPIE6M.AUD", volume: 255, pan: 128 });
                }
            }
            _ if kanefinl_playing => events.push(EndingEvent::ClearKeyBuffer),
            NodInput::MouseDown { x, y } => {
                let (top, bottom, split_x, split_y) = if self.not_for_win95 {
                    (44, 354, 320, 200)
                } else {
                    (44, 354, 320, 200)
                };
                if y >= top && y <= bottom {
                    self.done = true;
                    if x < split_x && y < split_y { self.selection = 2; }
                    if x < split_x && y >= split_y { self.selection = 3; }
                    if x >= split_x && y >= split_y { self.selection = 4; }
                }
            }
            NodInput::OtherKey => {}
        }
        events
    }

    pub fn finish(&self) -> Vec<EndingEvent> {
        if !self.done { return Vec::new(); }
        let mut events = Vec::new();
        if self.mouse_shown { events.push(EndingEvent::HideMouse); }
        if self.not_for_win95 {
            events.push(EndingEvent::DeleteSatSelectionPixels);
        } else {
            events.push(EndingEvent::DeletePseudoSeenBuffer);
        }
        events.extend([
            EndingEvent::DeleteScoreObjects,
            EndingEvent::FillSeenRectangle { left: 0, top: 360, right: 638, bottom: 398, color: 0 },
            EndingEvent::FillTextRectangle { left: 0, top: 360, right: 638, bottom: 398, color: 0 },
            EndingEvent::HideMouse,
            EndingEvent::ClearKeyboard,
            EndingEvent::RestoreFont,
            EndingEvent::RestoreFontSpacing,
            EndingEvent::FreeSample("KANEFINL.AUD"),
            EndingEvent::FreeSample("LOOPIE6M.AUD"),
            EndingEvent::PreserveVqaScreen,
            EndingEvent::PlayMovie {
                name: format!("NODEND{}", self.selection),
                theme_none: false,
                preserve_screen: true,
            },
        ]);
        if self.trailer_available {
            append_attract_sequence(&mut events, true);
            events.push(EndingEvent::PlayMovie { name: "TRAILER".into(), theme_none: false, preserve_screen: false });
        }
        append_attract_sequence(&mut events, false);
        events.push(EndingEvent::PlayMovie { name: "CC2TEASE".into(), theme_none: false, preserve_screen: false });
        events.push(EndingEvent::FreeLocalPalette);
        events.push(EndingEvent::DeleteTextPrintBuffer);
        events.push(EndingEvent::ClearBlitList);
        events
    }
}

fn append_attract_sequence(events: &mut Vec<EndingEvent>, _trailer: bool) {
    events.extend([
        EndingEvent::FadeToBlack,
        EndingEvent::LoadUncompressed("ATTRACT2.CPS"),
        EndingEvent::ScaleAttractScreen { source_width: 320, source_height: 199, dest_width: 640, dest_height: 398 },
        EndingEvent::FadeToPalette,
        EndingEvent::ClearKeyboard,
        EndingEvent::WaitCallbacks { seconds: 3 },
        EndingEvent::FadeToBlack,
    ]);
}

/// GDI ending flow. The demo branch is optional and mirrors the source's
/// compile-time DEMO selection.
pub fn gdi_ending(demo: bool, temple_ioned: bool, trailer_available: bool) -> Vec<EndingEvent> {
    if demo {
        return vec![
            EndingEvent::FadeToBlack,
            EndingEvent::LoadTitleScreen("DEMOPIC.PCX"),
            EndingEvent::BlitHiddenPageToSeen,
            EndingEvent::FadeToPalette,
            EndingEvent::ClearKeyBuffer,
            EndingEvent::WaitForKey,
            EndingEvent::FadeToBlack,
            EndingEvent::ClearVisiblePage,
        ];
    }
    let mut events = vec![
        EndingEvent::PlayMovie {
            name: if temple_ioned { "GDIFINB" } else { "GDIFINA" }.into(),
            theme_none: false, preserve_screen: false,
        },
        EndingEvent::ScorePresentation,
        EndingEvent::PlayMovie {
            name: if temple_ioned { "GDIEND2" } else { "GDIEND1" }.into(),
            theme_none: false, preserve_screen: false,
        },
    ];
    if trailer_available {
        append_attract_sequence(&mut events, true);
        events.push(EndingEvent::PlayMovie { name: "TRAILER".into(), theme_none: false, preserve_screen: false });
    }
    append_attract_sequence(&mut events, false);
    events.push(EndingEvent::PlayMovie { name: "CC2TEASE".into(), theme_none: false, preserve_screen: false });
    events
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gdi_ending_selects_temple_movie_and_optional_trailer() {
        let events = gdi_ending(false, true, true);
        assert!(events.contains(&EndingEvent::PlayMovie {
            name: "GDIFINB".into(), theme_none: false, preserve_screen: false,
        }));
        assert!(events.contains(&EndingEvent::PlayMovie {
            name: "GDIEND2".into(), theme_none: false, preserve_screen: false,
        }));
        assert!(events.contains(&EndingEvent::PlayMovie {
            name: "TRAILER".into(), theme_none: false, preserve_screen: false,
        }));
        assert_eq!(events.last(), Some(&EndingEvent::PlayMovie {
            name: "CC2TEASE".into(), theme_none: false, preserve_screen: false,
        }));
    }

    #[test]
    fn nod_ending_waits_for_intro_audio_and_selects_quadrant() {
        let (mut state, start) = NodEndingState::begin(false, false);
        assert!(start.contains(&EndingEvent::PlayMovie {
            name: "NODFINAL".into(), theme_none: true, preserve_screen: false,
        }));
        assert!(!state.step(NodInput::MouseDown { x: 10, y: 100 }, true, false).contains(&EndingEvent::ClearKeyBuffer));
        assert!(!state.done);
        let events = state.step(NodInput::MouseDown { x: 10, y: 100 }, false, false);
        assert!(state.done);
        assert_eq!(state.selection, 3);
        assert!(events.contains(&EndingEvent::ShowTargetText { x: 0, y: 180 }));
        let finish = state.finish();
        assert!(finish.contains(&EndingEvent::PlayMovie {
            name: "NODEND3".into(), theme_none: false, preserve_screen: true,
        }));
    }

    #[test]
    fn nod_selection_uses_right_bottom_quadrant_and_trailer_condition() {
        let (mut state, _) = NodEndingState::begin(true, true);
        state.step(NodInput::MouseDown { x: 500, y: 300 }, false, true);
        assert_eq!(state.selection, 4);
        let finish = state.finish();
        assert!(finish.iter().any(|e| matches!(e, EndingEvent::PlayMovie { name, .. } if name == "TRAILER")));
        assert_eq!(finish.last(), Some(&EndingEvent::ClearBlitList));
    }

    #[test]
    fn demo_gdi_ending_uses_title_screen_path() {
        let events = gdi_ending(true, false, false);
        assert_eq!(events[1], EndingEvent::LoadTitleScreen("DEMOPIC.PCX"));
        assert!(events.contains(&EndingEvent::ClearVisiblePage));
    }
}
