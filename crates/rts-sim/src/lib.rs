//! Deterministic simulation scheduling primitives.

use std::collections::BTreeMap;

use rts_core::SimTick;

/// An event removed from the queue for processing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduledEvent<E> {
    pub tick: SimTick,
    pub sequence: u64,
    pub event: E,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleError {
    SequenceExhausted,
}

/// Stable event queue ordered by simulation tick, then insertion sequence.
///
/// This provides deterministic ordering infrastructure; it does not yet implement
/// original game event semantics or the engine's complete event identifier set.
#[derive(Debug, Clone)]
pub struct EventQueue<E> {
    pending: BTreeMap<(SimTick, u64), E>,
    next_sequence: u64,
}

impl<E> Default for EventQueue<E> {
    fn default() -> Self {
        Self { pending: BTreeMap::new(), next_sequence: 0 }
    }
}

impl<E> EventQueue<E> {
    pub fn schedule(&mut self, tick: SimTick, event: E) -> Result<u64, ScheduleError> {
        let sequence = self.next_sequence;
        self.next_sequence = self
            .next_sequence
            .checked_add(1)
            .ok_or(ScheduleError::SequenceExhausted)?;
        self.pending.insert((tick, sequence), event);
        Ok(sequence)
    }

    pub fn pop_next_due(&mut self, now: SimTick) -> Option<ScheduledEvent<E>> {
        let key = *self.pending.first_key_value()?.0;
        if key.0 > now {
            return None;
        }

        let event = self.pending.remove(&key)?;
        Some(ScheduledEvent { tick: key.0, sequence: key.1, event })
    }

    pub fn len(&self) -> usize {
        self.pending.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_tick_events_keep_insertion_order() {
        let mut queue = EventQueue::default();
        assert_eq!(queue.schedule(SimTick(5), "first"), Ok(0));
        assert_eq!(queue.schedule(SimTick(5), "second"), Ok(1));
        assert_eq!(queue.schedule(SimTick(4), "earlier"), Ok(2));

        assert_eq!(queue.pop_next_due(SimTick(5)).expect("due").event, "earlier");
        assert_eq!(queue.pop_next_due(SimTick(5)).expect("due").event, "first");
        assert_eq!(queue.pop_next_due(SimTick(5)).expect("due").event, "second");
        assert!(queue.is_empty());
    }

    #[test]
    fn future_events_are_not_released_early() {
        let mut queue = EventQueue::default();
        queue.schedule(SimTick(10), 42).expect("sequence available");
        assert!(queue.pop_next_due(SimTick(9)).is_none());
        assert_eq!(queue.pop_next_due(SimTick(10)).expect("due").event, 42);
    }
}
