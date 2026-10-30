//! Win95 trace compatibility from REDALERT/W95TRACE.CPP.
//!
//! The entire implementation body in the pinned source is guarded by #if 0;
//! the compiled function intentionally performs no work.

/// Legacy variadic debug trace sink: intentionally disabled upstream.
#[inline]
pub const fn output_debug_string_w95() {}
