//! DirectDraw compatibility state from both
//! REDALERT/WIN32LIB/DDRAW.CPP and TIBERIANDAWN/WIN32LIB/DDRAW.CPP.
//!
//! The pinned source files are byte-identical. Set_Video_Mode returns TRUE
//! immediately; DirectDrawCreate, SetCooperativeLevel, SetDisplayMode and
//! CreatePalette code is under #if (0). Active code still supports externally
//! supplied DirectDraw globals, caps queries, palette conversion, vertical
//! blank waits, overlapped-blit probing, and the 20-entry surface monitor.
//!
//! Wait_Blit is exposed as a poll step so callers can yield rather than spin
//! in the original blocking do/while loop.

pub const DD_OK: i32 = 0;
pub const E_NOTIMPL: i32 = 0x8000_4001u32 as i32;
pub const MAX_SURFACES: usize = 20;

pub const VIDEO_BLITTER: u32 = 1;
pub const VIDEO_BLITTER_ASYNC: u32 = 2;
pub const VIDEO_SYNC_PALETTE: u32 = 4;
pub const VIDEO_BANK_SWITCHED: u32 = 8;
pub const VIDEO_COLOR_FILL: u32 = 16;
pub const VIDEO_NO_HARDWARE_ASSIST: u32 = 32;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VideoCapabilities {
    pub blit: bool,
    pub asynchronous_blit: bool,
    pub palette_vsync: bool,
    pub bank_switched: bool,
    pub color_fill: bool,
    pub no_hardware_assist: bool,
    pub free_video_memory: u32,
}

impl VideoCapabilities {
    pub const fn to_legacy_bits(self) -> u32 {
        (if self.blit { VIDEO_BLITTER } else { 0 })
            | (if self.asynchronous_blit { VIDEO_BLITTER_ASYNC } else { 0 })
            | (if self.palette_vsync { VIDEO_SYNC_PALETTE } else { 0 })
            | (if self.bank_switched { VIDEO_BANK_SWITCHED } else { 0 })
            | (if self.color_fill { VIDEO_COLOR_FILL } else { 0 })
            | (if self.no_hardware_assist { VIDEO_NO_HARDWARE_ASSIST } else { 0 })
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PaletteEntry { pub red: u8, pub green: u8, pub blue: u8, pub flags: u8 }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlitStatus { Complete, SurfaceLost, Pending, Failed(i32) }
impl Default for BlitStatus { fn default() -> Self { Self::Pending } }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlitPoll { Complete, SurfaceLost, Pending, PendingWithError(i32) }

pub trait DirectDrawHost {
    fn get_caps(&mut self) -> Result<VideoCapabilities, i32>;
    fn restore_display_mode(&mut self) -> i32;
    fn release_direct_draw(&mut self) -> i32;
    fn wait_for_vertical_blank(&mut self) -> i32;
    fn attach_palette_to_surface(&mut self);
    fn set_palette_entries(&mut self, entries: &[PaletteEntry; 256]);
    fn get_blt_status(&mut self) -> BlitStatus;
    fn restore_surface(&mut self, surface: u64) -> i32;
    fn release_surface(&mut self, surface: u64);
    fn focus_loss(&mut self);
    fn focus_restore(&mut self);

    /// Returns the value at test_buffer.Get_Pixel(0,5) after the exact 64x64
    /// video-memory self-blit probe in Check_Overlapped_Blit_Capability.
    fn overlapped_blit_test_pixel(&mut self) -> u8;

    /// Maps an HRESULT to the DirectDraw SDK's Process_DD_Result text.
    fn direct_draw_error_message(&self, _result: i32) -> Option<&'static str> { None }
    fn message_box(&mut self, title: &str, message: &str);
}

/// Process_DD_Result: show the optional success text, or the SDK error text;
/// unknown results retain the original 16-bit diagnostic suffix.
pub fn process_dd_result(result: i32, display_ok_message: bool, host: &mut impl DirectDrawHost) {
    if result == DD_OK {
        if display_ok_message { host.message_box("Note", "Direct Draw request went ok."); }
        return;
    }
    let message = host.direct_draw_error_message(result)
        .map(str::to_owned)
        .unwrap_or_else(|| format!("Unrecognised Direct Draw result code: {}", (result as u32) & 0xffff));
    host.message_box("Note", &message);
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SurfaceMonitor {
    surfaces: [Option<u64>; MAX_SURFACES],
    in_focus: bool,
    pub surfaces_restored: bool,
}

impl Default for SurfaceMonitor {
    fn default() -> Self {
        Self { surfaces: [None; MAX_SURFACES], in_focus: false, surfaces_restored: false }
    }
}

impl SurfaceMonitor {
    pub fn new() -> Self { Self::default() }
    pub fn in_focus(&self) -> bool { self.in_focus }
    pub fn set_surface_focus(&mut self, in_focus: bool) { self.in_focus = in_focus; }
    pub fn surfaces(&self) -> &[Option<u64>; MAX_SURFACES] { &self.surfaces }

    /// Add_DD_Surface ignores duplicates and uses the first empty slot.
    pub fn add_dd_surface(&mut self, surface: u64) -> bool {
        if self.got_surface_already(surface) { return false; }
        let Some(slot) = self.surfaces.iter_mut().find(|slot| slot.is_none()) else { return false; };
        *slot = Some(surface);
        true
    }

    /// Remove_DD_Surface removes the first match.
    pub fn remove_dd_surface(&mut self, surface: u64) -> bool {
        let Some(slot) = self.surfaces.iter_mut().find(|slot| **slot == Some(surface)) else { return false; };
        *slot = None;
        true
    }

    pub fn got_surface_already(&self, surface: u64) -> bool {
        self.surfaces.iter().any(|entry| *entry == Some(surface))
    }

    /// Restore_Surfaces only runs while focused. First failure invokes focus
    /// loss and exits; success invokes focus restore and resets the palette.
    pub fn restore_surfaces(
        &mut self,
        current_palette: &[u8],
        manager: &mut DirectDrawManager,
        host: &mut impl DirectDrawHost,
    ) -> Result<bool, i32> {
        if !self.in_focus { return Ok(false); }
        for surface in self.surfaces.iter().flatten().copied() {
            let result = host.restore_surface(surface);
            if result != DD_OK {
                host.focus_loss();
                return Err(result);
            }
        }
        host.focus_restore();
        self.surfaces_restored = true;
        manager.set_dd_palette(Some(current_palette), host);
        Ok(true)
    }

    /// Release calls Release on each registered DirectDraw surface.
    pub fn release(&mut self, host: &mut impl DirectDrawHost) {
        for slot in &mut self.surfaces {
            if let Some(surface) = slot.take() { host.release_surface(surface); }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectDrawManager {
    direct_draw_object_present: bool,
    direct_draw2_interface_present: bool,
    palette_surface_present: bool,
    first_palette_set: bool,
    can_vblank_sync: bool,
    pub overlapped_video_blits: bool,
    pub system_to_video_blits: bool,
    pub video_to_system_blits: bool,
    pub system_to_system_blits: bool,
    screen_width: i32,
    palette_entries: [PaletteEntry; 256],
}

impl Default for DirectDrawManager {
    fn default() -> Self {
        Self {
            direct_draw_object_present: false,
            direct_draw2_interface_present: false,
            palette_surface_present: false,
            first_palette_set: false,
            can_vblank_sync: true,
            overlapped_video_blits: true,
            system_to_video_blits: false,
            video_to_system_blits: false,
            system_to_system_blits: false,
            screen_width: 320,
            palette_entries: [PaletteEntry::default(); 256],
        }
    }
}

impl DirectDrawManager {
    pub fn new() -> Self { Self::default() }

    /// Active Set_Video_Mode is an unconditional TRUE; the legacy hardware
    /// setup code is fully disabled under #if (0).
    pub const fn set_video_mode(&mut self, _window: usize, _width: i32, _height: i32, _bpp: i32) -> bool {
        true
    }

    /// Represents the COM globals and ScreenWidth supplied by other code.
    pub fn bind_direct_draw_object(&mut self, present: bool) { self.direct_draw_object_present = present; }
    pub fn bind_direct_draw2_interface(&mut self, present: bool) { self.direct_draw2_interface_present = present; }
    pub fn bind_palette_surface(&mut self, present: bool) { self.palette_surface_present = present; }
    pub fn set_screen_width(&mut self, width: i32) { self.screen_width = width; }

    pub fn direct_draw_object_present(&self) -> bool { self.direct_draw_object_present }
    pub fn direct_draw2_interface_present(&self) -> bool { self.direct_draw2_interface_present }
    pub fn first_palette_set(&self) -> bool { self.first_palette_set }
    pub fn can_vblank_sync(&self) -> bool { self.can_vblank_sync }
    pub fn palette_entries(&self) -> &[PaletteEntry; 256] { &self.palette_entries }

    /// Check_Overlapped_Blit_Capability assumes true, runs the source 64x64
    /// clear/plot/self-blit probe, and disables overlap when pixel (0,5)=255.
    pub fn check_overlapped_blit_capability(&mut self, host: &mut impl DirectDrawHost) {
        self.overlapped_video_blits = true;
        if host.overlapped_blit_test_pixel() == 255 { self.overlapped_video_blits = false; }
    }

    pub fn reset_video_mode(&mut self, host: &mut impl DirectDrawHost) {
        if self.direct_draw_object_present {
            let restore = host.restore_display_mode();
            process_dd_result(restore, false, host);
            let release = host.release_direct_draw();
            process_dd_result(release, false, host);
            self.direct_draw_object_present = false;
        }
    }

    pub fn get_free_video_memory(&self, host: &mut impl DirectDrawHost) -> u32 {
        if !self.direct_draw_object_present { return 0; }
        match host.get_caps() {
            Ok(caps) => caps.free_video_memory,
            Err(result) => { process_dd_result(result, false, host); 0 }
        }
    }

    pub fn get_video_hardware_capabilities(&self, host: &mut impl DirectDrawHost) -> u32 {
        if !self.direct_draw_object_present { return 0; }
        match host.get_caps() {
            Ok(caps) => caps.to_legacy_bits(),
            Err(result) => { process_dd_result(result, false, host); 0 }
        }
    }

    /// Wait_Vert_Blank skips 320-pixel modes. E_NOTIMPL permanently disables
    /// synchronization; all other HRESULTs are sent through Process_DD_Result.
    pub fn wait_vert_blank(&mut self, host: &mut impl DirectDrawHost) {
        if !self.direct_draw_object_present || self.screen_width == 320 || !self.can_vblank_sync { return; }
        let result = host.wait_for_vertical_blank();
        if result == E_NOTIMPL {
            self.can_vblank_sync = false;
            return;
        }
        process_dd_result(result, false, host);
    }

    /// Set_DD_Palette turns each 6-bit stored channel into an 8-bit entry by
    /// shifting left twice. Palette is attached to the surface only once.
    pub fn set_dd_palette(&mut self, palette: Option<&[u8]>, host: &mut impl DirectDrawHost) {
        let Some(bytes) = palette.and_then(|source| source.get(..768)) else { return; };
        if !self.direct_draw_object_present || !self.palette_surface_present { return; }
        for (index, entry) in self.palette_entries.iter_mut().enumerate() {
            let at = index * 3;
            entry.red = bytes[at] << 2;
            entry.green = bytes[at + 1] << 2;
            entry.blue = bytes[at + 2] << 2;
            entry.flags = 0;
        }
        if !self.first_palette_set {
            host.attach_palette_to_surface();
            self.first_palette_set = true;
        }
        host.set_palette_entries(&self.palette_entries);
    }

    /// Non-blocking Wait_Blit equivalent. The original spins until DD_OK or
    /// DDERR_SURFACELOST; the caller may poll Pending on a later frame.
    pub fn wait_blt_poll(&self, host: &mut impl DirectDrawHost) -> BlitPoll {
        if !self.palette_surface_present { return BlitPoll::Pending; }
        match host.get_blt_status() {
            BlitStatus::Complete => BlitPoll::Complete,
            BlitStatus::SurfaceLost => BlitPoll::SurfaceLost,
            BlitStatus::Pending => BlitPoll::Pending,
            BlitStatus::Failed(code) => BlitPoll::PendingWithError(code),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct FakeHost {
        caps: VideoCapabilities,
        restore_result: i32,
        release_result: i32,
        vblank_result: i32,
        restored: Vec<u64>,
        released: Vec<u64>,
        focus_lost: usize,
        focus_restored: usize,
        attached: usize,
        palette_sets: usize,
        palette: [PaletteEntry; 256],
        blit_status: BlitStatus,
        overlapped_pixel: u8,
        messages: Vec<(String, String)>,
    }

    impl DirectDrawHost for FakeHost {
        fn get_caps(&mut self) -> Result<VideoCapabilities, i32> { Ok(self.caps) }
        fn restore_display_mode(&mut self) -> i32 { self.restore_result }
        fn release_direct_draw(&mut self) -> i32 { self.release_result }
        fn wait_for_vertical_blank(&mut self) -> i32 { self.vblank_result }
        fn attach_palette_to_surface(&mut self) { self.attached += 1; }
        fn set_palette_entries(&mut self, entries: &[PaletteEntry; 256]) {
            self.palette = *entries;
            self.palette_sets += 1;
        }
        fn get_blt_status(&mut self) -> BlitStatus { self.blit_status }
        fn restore_surface(&mut self, surface: u64) -> i32 { self.restored.push(surface); self.restore_result }
        fn release_surface(&mut self, surface: u64) { self.released.push(surface); }
        fn focus_loss(&mut self) { self.focus_lost += 1; }
        fn focus_restore(&mut self) { self.focus_restored += 1; }
        fn overlapped_blit_test_pixel(&mut self) -> u8 { self.overlapped_pixel }
        fn message_box(&mut self, title: &str, message: &str) { self.messages.push((title.into(), message.into())); }
    }

    #[test]
    fn video_mode_active_branch_returns_true_without_initializing_directdraw() {
        let mut manager = DirectDrawManager::new();
        assert!(manager.set_video_mode(1, 640, 480, 8));
        assert!(!manager.direct_draw_object_present());
    }

    #[test]
    fn capability_flags_match_misc_header_bit_values() {
        let caps = VideoCapabilities {
            blit: true, asynchronous_blit: true, palette_vsync: false,
            bank_switched: true, color_fill: false, no_hardware_assist: true,
            free_video_memory: 4096,
        };
        assert_eq!(caps.to_legacy_bits(), VIDEO_BLITTER | VIDEO_BLITTER_ASYNC | VIDEO_BANK_SWITCHED | VIDEO_NO_HARDWARE_ASSIST);
    }

    #[test]
    fn palette_conversion_attaches_once_and_shifts_six_bit_channels() {
        let mut manager = DirectDrawManager::new();
        manager.bind_direct_draw_object(true);
        manager.bind_palette_surface(true);
        let mut host = FakeHost::default();
        let mut palette = vec![0u8; 768];
        palette[..3].copy_from_slice(&[63, 32, 1]);
        manager.set_dd_palette(Some(&palette), &mut host);
        manager.set_dd_palette(Some(&palette), &mut host);
        assert_eq!(manager.palette_entries()[0], PaletteEntry { red: 252, green: 128, blue: 4, flags: 0 });
        assert_eq!((host.attached, host.palette_sets), (1, 2));
    }

    #[test]
    fn surface_monitor_deduplicates_restores_and_releases_twenty_entries() {
        let mut monitor = SurfaceMonitor::new();
        let mut host = FakeHost::default();
        assert!(monitor.add_dd_surface(100));
        assert!(!monitor.add_dd_surface(100));
        assert!(monitor.add_dd_surface(200));
        monitor.set_surface_focus(true);
        let mut manager = DirectDrawManager::new();
        assert_eq!(monitor.restore_surfaces(&[0; 768], &mut manager, &mut host), Ok(true));
        assert_eq!(host.restored, vec![100, 200]);
        assert_eq!(host.focus_restored, 1);
        assert!(monitor.surfaces_restored);
        monitor.release(&mut host);
        assert_eq!(host.released, vec![100, 200]);
        assert!(monitor.surfaces().iter().all(Option::is_none));
    }

    #[test]
    fn failed_surface_restore_invokes_focus_loss_and_stops() {
        let mut monitor = SurfaceMonitor::new();
        let mut host = FakeHost { restore_result: -1, ..FakeHost::default() };
        monitor.add_dd_surface(7);
        monitor.add_dd_surface(8);
        monitor.set_surface_focus(true);
        let mut manager = DirectDrawManager::new();
        assert_eq!(monitor.restore_surfaces(&[], &mut manager, &mut host), Err(-1));
        assert_eq!(host.restored, vec![7]);
        assert_eq!(host.focus_lost, 1);
        assert_eq!(host.focus_restored, 0);
    }

    #[test]
    fn vertical_blank_not_implemented_is_remembered() {
        let mut manager = DirectDrawManager::new();
        manager.bind_direct_draw_object(true);
        manager.set_screen_width(640);
        let mut host = FakeHost { vblank_result: E_NOTIMPL, ..FakeHost::default() };
        manager.wait_vert_blank(&mut host);
        assert!(!manager.can_vblank_sync());
        manager.wait_vert_blank(&mut host);
        assert_eq!(host.messages.len(), 0);
    }

    #[test]
    fn overlapped_blit_test_defaults_true_and_disables_on_repeated_pixel() {
        let mut manager = DirectDrawManager::new();
        let mut host = FakeHost { overlapped_pixel: 255, ..FakeHost::default() };
        manager.check_overlapped_blit_capability(&mut host);
        assert!(!manager.overlapped_video_blits);
    }

    #[test]
    fn wait_blit_is_pollable_without_blocking() {
        let mut manager = DirectDrawManager::new();
        manager.bind_palette_surface(true);
        let mut host = FakeHost { blit_status: BlitStatus::Pending, ..FakeHost::default() };
        assert_eq!(manager.wait_blt_poll(&mut host), BlitPoll::Pending);
        host.blit_status = BlitStatus::SurfaceLost;
        assert_eq!(manager.wait_blt_poll(&mut host), BlitPoll::SurfaceLost);
        host.blit_status = BlitStatus::Complete;
        assert_eq!(manager.wait_blt_poll(&mut host), BlitPoll::Complete);
    }

    #[test]
    fn process_result_preserves_success_and_unknown_error_message() {
        let mut host = FakeHost::default();
        process_dd_result(DD_OK, true, &mut host);
        process_dd_result(0x8007_000eu32 as i32, false, &mut host);
        assert_eq!(host.messages[0].1, "Direct Draw request went ok.");
        assert_eq!(host.messages[1].1, "Unrecognised Direct Draw result code: 14");
    }

    #[test]
    fn reset_video_mode_only_releases_an_existing_object() {
        let mut manager = DirectDrawManager::new();
        let mut host = FakeHost::default();
        manager.reset_video_mode(&mut host);
        assert!(!manager.direct_draw_object_present());
        manager.bind_direct_draw_object(true);
        manager.reset_video_mode(&mut host);
        assert!(!manager.direct_draw_object_present());
    }
}
