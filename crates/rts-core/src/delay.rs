//! Timer-based delay from REDALERT/WIN32LIB/DELAY.CPP.
//!
//! The original waits for the system timer to advance past Time()+1 once per
//! requested duration unit. A host supplies the same monotonically increasing
//! timer source; this keeps timing policy outside deterministic simulation.

/// Wait for one legacy delay unit using the source timer comparison.
pub fn delay_with_timer<F, W>(duration: i32, mut timer_time: F, mut wait: W)
where
    F: FnMut() -> u32,
    W: FnMut(),
{
    for _ in 0..duration.max(0) {
        let threshold = timer_time().wrapping_add(1);
        while threshold >= timer_time() {
            wait();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn waits_until_timer_exceeds_start_plus_one() {
        let tick = Rc::new(Cell::new(10u32));
        let read = tick.clone();
        let advance = tick.clone();
        let waits = Rc::new(Cell::new(0));
        let count = waits.clone();
        delay_with_timer(2, move || read.get(), move || {
            count.set(count.get() + 1);
            advance.set(advance.get() + 1);
        });
        assert_eq!(tick.get(), 14);
        assert_eq!(waits.get(), 4);
    }

    #[test]
    fn negative_duration_is_noop() {
        let mut calls = 0;
        delay_with_timer(-1, || { calls += 1; 0 }, || {});
        assert_eq!(calls, 0);
    }
}
