//! Westwood TimerClass and CountDownTimerClass tick accounting.
//!
//! Platform timer installation and Windows callback scheduling are supplied by
//! the host. This module preserves the original accumulated/start-marker logic.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseTimer {
    System,
    User,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimerClass {
    tick_type: BaseTimer,
    started: i32,
    accumulated: i32,
}

impl TimerClass {
    pub const fn new(tick_type: BaseTimer) -> Self {
        Self { tick_type, started: 0, accumulated: 0 }
    }

    pub const fn tick_type(&self) -> BaseTimer { self.tick_type }
    pub const fn is_running(&self) -> bool { self.started != 0 }
    pub const fn accumulated(&self) -> i32 { self.accumulated }

    fn ticks<'a>(&self, system: i32, user: i32) -> i32 {
        match self.tick_type { BaseTimer::System => system, BaseTimer::User => user }
    }

    /// Constructor's optional start is ignored until the timer system is live.
    pub fn initialize(&mut self, on: bool, timer_system_on: bool, system: i32, user: i32) {
        self.accumulated = 0;
        self.started = 0;
        if on && timer_system_on { self.start(system, user); }
    }

    pub fn time(&mut self, system: i32, user: i32) -> i32 {
        if self.started != 0 {
            let ticks = self.ticks(system, user);
            self.accumulated = self.accumulated.wrapping_add(ticks.wrapping_sub(self.started.wrapping_sub(1)));
            self.started = ticks.wrapping_add(1);
        }
        self.accumulated
    }

    pub fn stop(&mut self, system: i32, user: i32) -> i32 {
        let elapsed = self.time(system, user);
        self.started = 0;
        elapsed
    }

    pub fn start(&mut self, system: i32, user: i32) -> i32 {
        if self.started == 0 {
            self.started = self.ticks(system, user).wrapping_add(1);
        }
        self.time(system, user)
    }

    pub fn set(&mut self, value: i32, start: bool, system: i32, user: i32) -> i32 {
        self.started = 0;
        self.accumulated = value;
        if start { self.start(system, user) } else { self.time(system, user) }
    }

    pub fn reset(&mut self, start: bool, system: i32, user: i32) -> i32 {
        self.set(0, start, system, user)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CountDownTimerClass {
    timer: TimerClass,
    delay_time: i32,
}

impl CountDownTimerClass {
    pub const fn new(tick_type: BaseTimer) -> Self {
        Self { timer: TimerClass::new(tick_type), delay_time: 0 }
    }

    pub fn initialize(
        &mut self, delay: i32, on: bool, timer_system_on: bool, system: i32, user: i32,
    ) -> i32 {
        self.timer.initialize(on, timer_system_on, system, user);
        self.set(delay, on, system, user)
    }

    pub fn time(&mut self, system: i32, user: i32) -> i32 {
        self.delay_time.wrapping_sub(self.timer.time(system, user)).max(0)
    }

    pub fn set(&mut self, value: i32, start: bool, system: i32, user: i32) -> i32 {
        self.delay_time = value;
        self.timer.reset(start, system, user);
        self.time(system, user)
    }

    pub fn stop(&mut self, system: i32, user: i32) -> i32 {
        self.timer.stop(system, user);
        self.time(system, user)
    }

    pub fn start(&mut self, system: i32, user: i32) -> i32 {
        self.timer.start(system, user);
        self.time(system, user)
    }

    pub fn reset(&mut self, start: bool, system: i32, user: i32) -> i32 {
        self.timer.reset(start, system, user);
        self.time(system, user)
    }

    pub const fn delay_time(&self) -> i32 { self.delay_time }
}
