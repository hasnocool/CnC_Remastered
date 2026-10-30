//! Win32 platform stubs and callbacks from both WINSTUB.CPP variants.
//!
//! Platform work is expressed as deterministic events or host callbacks.
//! In the pinned sources Create_Main_Window leaves MainWindow null and
//! returns immediately; Window_Dialog_Box and Check_For_Focus_Loss are
//! no-ops because their implementation bodies are #if (0).

pub const GAME_PALETTE_BYTES: usize = 256 * 3;
pub const PALETTE_INTERPOLATION_BYTES: usize = 256 * 256;
pub const WM_DESTROY: u32 = 0x0002;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WinStubVariant { RedAlert, TiberianDawn }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ViewLockCounts { pub seen_buffer: u32, pub hidden_page: u32 }
impl ViewLockCounts {
    pub const fn any_locked(self) -> bool { self.seen_buffer != 0 || self.hidden_page != 0 }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FocusState {
    pub sound_on: bool,
    pub old_theme: Option<u32>,
    pub currently_playing_theme: Option<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WinStubEvent {
    SuspendTheme,
    StopTheme,
    RememberOldTheme(u32),
    StopPrimarySound,
    StartPrimarySound { force: bool },
    ClearMouseCursorClip,
    SetMouseCursorClip,
    RestoreCachedIcons,
    FlagMapToRedraw { full: bool },
    ClearVisiblePage,
    ClearHiddenPage,
    ShowMouse,
    DebugPrint(String),
    ClearVisiblePageBeforeMemoryError,
    RestoreCurrentPalette,
    ShowOutOfMemoryMessage { title: &'static str, body: &'static str, abort: bool },
    InvalidateCachedIcons,
    SetReadyToQuit(u32),
    PostWindowMessage { window: usize, message: u32 },
    PollKeyboardUntilReadyToQuitChanges,
    ExitProcess(i32),
    EnableMono,
    ClearMonoScreen,
    PrintMono(String),
    AppendAssertLog(String),
    AssertMessageBox(String),
    ProgramEnd { message: String, flag: bool },
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MainWindowState {
    pub main_window: Option<usize>,
    pub instance: Option<usize>,
    pub show_command: Option<i32>,
}
impl MainWindowState {
    /// Both Create_Main_Window implementations set MainWindow=NULL and return.
    pub fn create_main_window(&mut self, _instance: usize, _show: i32, _width: i32, _height: i32) {
        self.main_window = None;
    }
    /// DialogBox/message-pump implementation is entirely #if (0).
    pub const fn window_dialog_box(&self) {}
    /// Check_For_Focus_Loss implementation is entirely #if (0).
    pub const fn check_for_focus_loss(&self) {}
}

pub fn focus_loss(variant: WinStubVariant, state: &mut FocusState) -> Vec<WinStubEvent> {
    let mut events = Vec::with_capacity(4);
    match variant {
        WinStubVariant::RedAlert => events.push(WinStubEvent::SuspendTheme),
        WinStubVariant::TiberianDawn => {
            if state.sound_on && state.old_theme.is_none() {
                if let Some(theme) = state.currently_playing_theme {
                    state.old_theme = Some(theme);
                    events.push(WinStubEvent::RememberOldTheme(theme));
                }
            }
            events.push(WinStubEvent::StopTheme);
        }
    }
    events.extend([WinStubEvent::StopPrimarySound, WinStubEvent::ClearMouseCursorClip]);
    events
}

pub fn focus_restore() -> Vec<WinStubEvent> {
    vec![
        WinStubEvent::RestoreCachedIcons,
        WinStubEvent::FlagMapToRedraw { full: true },
        WinStubEvent::StartPrimarySound { force: true },
        WinStubEvent::SetMouseCursorClip,
        WinStubEvent::ClearVisiblePage,
        WinStubEvent::ClearHiddenPage,
    ]
}

pub const fn any_locked(counts: ViewLockCounts) -> bool { counts.any_locked() }

/// VQA palette updates are staged by Flag_To_Set_Palette and consumed by
/// Check_VQ_Palette_Set when the video/audio layer is ready.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct VqPaletteQueue {
    palette: Option<Vec<u8>>,
    num_bytes: i32,
    slow_palette: u32,
    changed: bool,
    palette_counter: usize,
}
pub trait WinStubPaletteHost {
    fn palettes_read(&self) -> bool;
    fn next_interpolated_palette(&mut self) -> Option<[u8; PALETTE_INTERPOLATION_BYTES]>;
    fn set_interpolation_table(&mut self, table: &[u8; PALETTE_INTERPOLATION_BYTES]);
    fn set_palette(&mut self, palette: &[u8; GAME_PALETTE_BYTES]);
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PaletteQueueError {
    PaletteTooShort { supplied: usize, required: usize },
    MissingInterpolatedPalette,
}
impl VqPaletteQueue {
    pub fn flag_to_set_palette(&mut self, palette: &[u8], num_bytes: i32, slow_palette: u32) {
        self.palette = Some(palette.to_vec());
        self.num_bytes = num_bytes;
        self.slow_palette = slow_palette;
        self.changed = true;
    }
    pub fn is_changed(&self) -> bool { self.changed }
    pub fn num_bytes(&self) -> i32 { self.num_bytes }
    pub fn slow_palette(&self) -> u32 { self.slow_palette }
    pub fn palette_counter(&self) -> usize { self.palette_counter }

    /// SetPalette masks 768 channels to six bits, boosts luminance by 15%
    /// with a cap of 63, optionally consumes one precomputed interpolation
    /// table, then submits the palette. numbytes/slowpal are stored but ignored.
    pub fn check_vq_palette_set(&mut self, host: &mut impl WinStubPaletteHost) -> Result<bool, PaletteQueueError> {
        if !self.changed { return Ok(false); }
        let source = self.palette.as_ref().ok_or(PaletteQueueError::PaletteTooShort {
            supplied: 0, required: GAME_PALETTE_BYTES,
        })?;
        if source.len() < GAME_PALETTE_BYTES {
            return Err(PaletteQueueError::PaletteTooShort { supplied: source.len(), required: GAME_PALETTE_BYTES });
        }
        let mut palette = [0u8; GAME_PALETTE_BYTES];
        palette.copy_from_slice(&source[..GAME_PALETTE_BYTES]);
        for channel in &mut palette {
            let gun = (*channel & 63) as u32;
            *channel = (gun + gun * 15 / 100).min(63) as u8;
        }
        if host.palettes_read() {
            let table = host.next_interpolated_palette().ok_or(PaletteQueueError::MissingInterpolatedPalette)?;
            host.set_interpolation_table(&table);
            self.palette_counter = self.palette_counter.wrapping_add(1);
        }
        host.set_palette(&palette);
        self.changed = false;
        Ok(true)
    }
}

pub trait WinStubTitleScreenHost {
    fn read_pcx_file(&mut self, name: &str, palette: &mut [u8; GAME_PALETTE_BYTES]) -> Option<u64>;
    fn blit_loaded_title_screen(&mut self, loaded_buffer: u64, viewport: usize);
    fn release_loaded_title_screen(&mut self, loaded_buffer: u64);
}
/// Load_Title_Screen blits and releases the temporary buffer only after a
/// successful Read_PCX_File call.
pub fn load_title_screen(name: &str, viewport: usize, palette: &mut [u8; GAME_PALETTE_BYTES],
    host: &mut impl WinStubTitleScreenHost) -> bool {
    let Some(buffer) = host.read_pcx_file(name, palette) else { return false; };
    host.blit_loaded_title_screen(buffer, viewport);
    host.release_loaded_title_screen(buffer);
    true
}

/// Variant-specific Memory_Error_Handler sequences. mouse_state is the count
/// returned by successive Get_Mouse_State calls before the cursor is visible.
pub fn memory_error_events(variant: WinStubVariant, main_window: usize, mouse_state: u32) -> Vec<WinStubEvent> {
    let mut events = Vec::new();
    match variant {
        WinStubVariant::RedAlert => {
            events.push(WinStubEvent::ClearVisiblePageBeforeMemoryError);
            events.push(WinStubEvent::RestoreCurrentPalette);
            for _ in 0..mouse_state { events.push(WinStubEvent::ShowMouse); }
            events.push(WinStubEvent::ShowOutOfMemoryMessage { title: "TEXT_MEMORY_ERROR", body: "TEXT_ABORT", abort: false });
            events.push(WinStubEvent::SetReadyToQuit(1));
            events.push(WinStubEvent::PostWindowMessage { window: main_window, message: WM_DESTROY });
            events.push(WinStubEvent::PollKeyboardUntilReadyToQuitChanges);
            events.push(WinStubEvent::ExitProcess(0));
        }
        WinStubVariant::TiberianDawn => {
            events.push(WinStubEvent::DebugPrint("Error - out of memory.".into()));
            events.push(WinStubEvent::ClearVisiblePageBeforeMemoryError);
            events.push(WinStubEvent::RestoreCurrentPalette);
            for _ in 0..mouse_state { events.push(WinStubEvent::ShowMouse); }
            events.push(WinStubEvent::ShowOutOfMemoryMessage { title: "Error - out of memory.", body: "Abort", abort: false });
            events.push(WinStubEvent::InvalidateCachedIcons);
        }
    }
    events
}

/// Assert_Failure is present only in Red Alert debug builds (#ifndef NDEBUG).
pub fn red_alert_assert_failure(expression: &str, line: u32, file: &str, timestamp: &str, debug_build: bool)
    -> Option<(String, Vec<WinStubEvent>)> {
    if !debug_build { return None; }
    let message = format!("assert '{expression}' failed at line {line} in module {file}.\n");
    let log = format!("{timestamp} - {message}");
    let events = vec![
        WinStubEvent::EnableMono,
        WinStubEvent::ClearMonoScreen,
        WinStubEvent::PrintMono(message.clone()),
        WinStubEvent::AppendAssertLog(log.clone()),
        WinStubEvent::AssertMessageBox(message.clone()),
        WinStubEvent::ProgramEnd { message, flag: false },
        WinStubEvent::InvalidateCachedIcons,
    ];
    Some((log, events))
}

/// WWDebugString (Red Alert) and CCDebugString (Tiberian Dawn) are no-op stubs.
pub const fn debug_string(_message: &str) {}
/// Colour_Debug is empty/commented out for both variants.
pub const fn colour_debug(_call_number: i32) {}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Default)]
    struct FakePaletteHost {
        palettes_read: bool,
        interpolation: Option<[u8; PALETTE_INTERPOLATION_BYTES]>,
        table: Option<[u8; PALETTE_INTERPOLATION_BYTES]>,
        palette: Option<[u8; GAME_PALETTE_BYTES]>,
    }
    impl WinStubPaletteHost for FakePaletteHost {
        fn palettes_read(&self) -> bool { self.palettes_read }
        fn next_interpolated_palette(&mut self) -> Option<[u8; PALETTE_INTERPOLATION_BYTES]> { self.interpolation.take() }
        fn set_interpolation_table(&mut self, table: &[u8; PALETTE_INTERPOLATION_BYTES]) { self.table = Some(*table); }
        fn set_palette(&mut self, palette: &[u8; GAME_PALETTE_BYTES]) { self.palette = Some(*palette); }
    }
    #[test]
    fn focus_handling_keeps_variant_specific_theme_behavior() {
        let mut red = FocusState { sound_on:true, old_theme:None, currently_playing_theme:Some(7) };
        assert_eq!(focus_loss(WinStubVariant::RedAlert,&mut red),vec![WinStubEvent::SuspendTheme,WinStubEvent::StopPrimarySound,WinStubEvent::ClearMouseCursorClip]);
        let mut td = FocusState { sound_on:true, old_theme:None, currently_playing_theme:Some(9) };
        assert_eq!(focus_loss(WinStubVariant::TiberianDawn,&mut td),vec![WinStubEvent::RememberOldTheme(9),WinStubEvent::StopTheme,WinStubEvent::StopPrimarySound,WinStubEvent::ClearMouseCursorClip]);
        assert_eq!(td.old_theme,Some(9));
        assert_eq!(focus_restore().len(),6);
    }
    #[test]
    fn any_locked_checks_seen_and_hidden_page_counts() {
        assert!(!any_locked(ViewLockCounts{seen_buffer:0,hidden_page:0}));
        assert!(any_locked(ViewLockCounts{seen_buffer:1,hidden_page:0}));
        assert!(any_locked(ViewLockCounts{seen_buffer:0,hidden_page:2}));
    }
    #[test]
    fn window_creation_and_dialog_pump_keep_noop_stubs() {
        let mut state=MainWindowState::default();
        state.create_main_window(1,5,640,480);
        assert_eq!(state.main_window,None);
        state.check_for_focus_loss();
        state.window_dialog_box();
    }
    #[test]
    fn palette_queue_masks_channels_and_increases_luminance() {
        let mut queue=VqPaletteQueue::default();
        queue.flag_to_set_palette(&vec![255;GAME_PALETTE_BYTES],768,10);
        let mut host=FakePaletteHost::default();
        assert!(queue.check_vq_palette_set(&mut host).unwrap());
        assert!(host.palette.unwrap().iter().all(|channel|*channel==63));
        assert!(!queue.is_changed());
    }
    #[test]
    fn palette_queue_uses_precomputed_interpolation_table() {
        let mut queue=VqPaletteQueue::default();
        queue.flag_to_set_palette(&vec![63;GAME_PALETTE_BYTES],768,0);
        let table=[7u8;PALETTE_INTERPOLATION_BYTES];
        let mut host=FakePaletteHost{palettes_read:true,interpolation:Some(table),..FakePaletteHost::default()};
        queue.check_vq_palette_set(&mut host).unwrap();
        assert_eq!(queue.palette_counter(),1);
        assert_eq!(host.table.unwrap()[0],7);
    }
    #[test]
    fn title_screen_load_blits_and_releases_only_on_success() {
        struct Fake { success:bool, calls:Vec<&'static str> }
        impl WinStubTitleScreenHost for Fake {
            fn read_pcx_file(&mut self,_:&str,_:&mut [u8;GAME_PALETTE_BYTES])->Option<u64>{self.calls.push("read");self.success.then_some(42)}
            fn blit_loaded_title_screen(&mut self,_:u64,_:usize){self.calls.push("blit");}
            fn release_loaded_title_screen(&mut self,_:u64){self.calls.push("release");}
        }
        let mut palette=[0;GAME_PALETTE_BYTES];let mut host=Fake{success:true,calls:Vec::new()};
        assert!(load_title_screen("TITLE.PCX",9,&mut palette,&mut host));
        assert_eq!(host.calls,vec!["read","blit","release"]);
        let mut failed=Fake{success:false,calls:Vec::new()};
        assert!(!load_title_screen("MISSING.PCX",9,&mut palette,&mut failed));
        assert_eq!(failed.calls,vec!["read"]);
    }
    #[test]
    fn memory_error_events_are_variant_specific() {
        let red=memory_error_events(WinStubVariant::RedAlert,123,2);
        assert!(red.contains(&WinStubEvent::PostWindowMessage{window:123,message:WM_DESTROY}));
        assert!(red.contains(&WinStubEvent::ExitProcess(0)));
        let td=memory_error_events(WinStubVariant::TiberianDawn,0,1);
        assert!(td.contains(&WinStubEvent::DebugPrint("Error - out of memory.".into())));
        assert!(!td.contains(&WinStubEvent::ExitProcess(0)));
    }
    #[test]
    fn red_alert_assert_handler_is_debug_only() {
        assert!(red_alert_assert_failure("x",4,"unit.cpp","10/30/2026 00:00:00",false).is_none());
        let (log,events)=red_alert_assert_failure("x",4,"unit.cpp","10/30/2026 00:00:00",true).unwrap();
        assert!(log.contains("assert 'x' failed at line 4 in module unit.cpp"));
        assert!(events.contains(&WinStubEvent::ProgramEnd{message:format!("assert 'x' failed at line 4 in module unit.cpp.\n"),flag:false}));
    }
}
