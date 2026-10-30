//! WINHIDE.CPP compatibility surface.
//!
//! In the pinned upstream source the only mouse-hide/show calls are commented
//! out. On non-IBM builds the implementation is excluded entirely. Therefore
//! both exported operations are deliberate no-ops, matching observable behavior.

#[inline]
pub const fn window_hide_mouse(_window: usize) {}

#[inline]
pub const fn window_show_mouse() {}
