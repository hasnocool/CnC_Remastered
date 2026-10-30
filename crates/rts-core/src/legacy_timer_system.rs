//! Host-independent state from WIN32LIB/TIMERINI.CPP.
//!
//! Windows multimedia timer creation and resolution requests are host services;
//! this type preserves the shared flags, counters, callback updates and teardown.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LegacyTimerSystem {
    frequency: u32,
    timer_handle: u32,
    system_ticks: u32,
    user_ticks: u32,
    active: bool,
    owns_global_timer: bool,
}

impl LegacyTimerSystem {
    /// Mirrors WinTimerClass construction after the platform timer request.
    /// timer_handle is zero when timeSetEvent failed.
    pub const fn new(frequency: u32, timer_handle: u32, partial: bool) -> Self {
        Self {
            frequency,
            timer_handle,
            system_ticks: 0,
            user_ticks: 0,
            active: timer_handle != 0,
            owns_global_timer: timer_handle != 0 && !partial,
        }
    }

    /// Timer_Callback / Update_Tick_Count increment both counters per callback.
    pub fn update_tick_count(&mut self) {
        self.system_ticks = self.system_ticks.wrapping_add(1);
        self.user_ticks = self.user_ticks.wrapping_add(1);
    }

    pub const fn get_system_tick_count(&self) -> u32 { self.system_ticks }
    pub const fn get_user_tick_count(&self) -> u32 { self.user_ticks }
    pub const fn timer_system_on(&self) -> bool { self.active }
    pub const fn owns_global_timer(&self) -> bool { self.owns_global_timer }
    pub const fn frequency(&self) -> u32 { self.frequency }
    pub const fn timer_handle(&self) -> u32 { self.timer_handle }

    /// Mirrors the destructor after the host has killed the multimedia timer
    /// and restored timer resolution.
    pub fn remove(&mut self) {
        self.timer_handle = 0;
        self.active = false;
        self.owns_global_timer = false;
    }
}

impl Default for LegacyTimerSystem {
    fn default() -> Self { Self::new(60, 0, true) }
}
