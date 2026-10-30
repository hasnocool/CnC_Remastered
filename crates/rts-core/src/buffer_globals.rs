//! Global callback and viewport slots from both variants' BUFFGLBL.CPP.

use core::ffi::{c_char, c_void};

pub type BlitToVvpc = unsafe extern "C" fn(*mut c_void, *mut c_void, i32, i32, i32, i32, i32, i32, bool) -> bool;
pub type ScaleToVvpc = unsafe extern "C" fn(*mut c_void, *mut c_void, i32, i32, i32, i32, i32, i32, i32, i32, bool, *mut c_char) -> bool;

/// Default state of the legacy graphics-buffer globals before a video mode is set.
#[derive(Default)]
pub struct BufferGlobalState {
    /// Installed blit implementation; null in the upstream static initializer.
    pub blit_to_vvpc: Option<BlitToVvpc>,
    /// Installed scale implementation; null in the upstream static initializer.
    pub scale_to_vvpc: Option<ScaleToVvpc>,
    /// Logic viewport pointer; null until the graphics system assigns it.
    pub logic_page: Option<core::ptr::NonNull<c_void>>,
    /// The source initializes icon caching as enabled.
    pub icon_cache_allowed: bool,
    /// Optional callback invoked by the graphics system on focus loss.
    pub focus_loss: Option<unsafe extern "C" fn()>,
}

impl BufferGlobalState {
    /// Construct the exact upstream static defaults.
    pub const fn initial() -> Self {
        Self { blit_to_vvpc: None, scale_to_vvpc: None, logic_page: None,
            icon_cache_allowed: true, focus_loss: None }
    }
}
