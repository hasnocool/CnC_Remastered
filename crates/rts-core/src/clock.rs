/// Simulation rate defined by both upstream game variants.
pub const TICKS_PER_SECOND: u64 = 15;
pub const TICKS_PER_MINUTE: u64 = TICKS_PER_SECOND * 60;

/// An authoritative simulation time value. Wall-clock time must never be stored here.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SimTick(pub u64);

impl SimTick {
    pub const ZERO: Self = Self(0);

    pub const fn checked_add(self, ticks: u64) -> Option<Self> {
        match self.0.checked_add(ticks) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    pub const fn checked_elapsed_since(self, earlier: Self) -> Option<u64> {
        self.0.checked_sub(earlier.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upstream_tick_rate_is_fifteen_per_second() {
        assert_eq!(TICKS_PER_SECOND, 15);
        assert_eq!(TICKS_PER_MINUTE, 900);
    }

    #[test]
    fn tick_arithmetic_is_checked() {
        assert_eq!(SimTick(41).checked_add(1), Some(SimTick(42)));
        assert_eq!(SimTick(1).checked_elapsed_since(SimTick(3)), None);
        assert_eq!(SimTick(9).checked_elapsed_since(SimTick(4)), Some(5));
        assert_eq!(SimTick(u64::MAX).checked_add(1), None);
    }
}
