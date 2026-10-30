//! Red Alert's second deterministic RNG, ported from RANDOM.H/RANDOM.CPP.

/// Port of the upstream RandomClass linear-congruential generator.
///
/// The original project uses 32-bit unsigned arithmetic on Win32 and returns
/// the 15 bits starting at bit 10. The range overload uses rejection sampling
/// over the smallest low-bit mask that covers the requested range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RandomClass {
    /// Public seed state in the original C++ class.
    pub seed: u32,
}

impl Default for RandomClass {
    fn default() -> Self {
        Self::new(0)
    }
}

impl RandomClass {
    pub const MULT_CONSTANT: u32 = 0x41C6_4E6D;
    pub const ADD_CONSTANT: u32 = 0x0000_3039;
    pub const THROW_AWAY_BITS: u32 = 10;
    pub const SIGNIFICANT_BITS: u32 = 15;
    pub const MAX_VALUE: u32 = (1 << Self::SIGNIFICANT_BITS) - 1;

    pub const fn new(seed: u32) -> Self {
        Self { seed }
    }

    /// Fetch the next 15-bit value, preserving the original 32-bit seed wrap.
    pub fn next(&mut self) -> u16 {
        self.seed = self
            .seed
            .wrapping_mul(Self::MULT_CONSTANT)
            .wrapping_add(Self::ADD_CONSTANT);

        ((self.seed >> Self::THROW_AWAY_BITS) & Self::MAX_VALUE) as u16
    }

    /// Port of RandomClass::operator()(minval, maxval).
    ///
    /// Equal bounds return immediately without advancing the random sequence.
    /// Reversed bounds are swapped. As in the original implementation, ranges
    /// larger than 15 significant bits do not provide a uniform full-range
    /// result; callers are expected to keep the magnitude within 0..32767.
    pub fn next_inclusive(&mut self, mut minval: i32, mut maxval: i32) -> i32 {
        if minval == maxval {
            return minval;
        }

        if minval > maxval {
            core::mem::swap(&mut minval, &mut maxval);
        }

        let magnitude = maxval.wrapping_sub(minval);
        let mut highbit = Self::SIGNIFICANT_BITS as i32 - 1;

        while highbit > 0 && (magnitude & (1_i32 << highbit)) == 0 {
            highbit -= 1;
        }

        let mask = (1_i32 << (highbit + 1)) - 1;
        let mut pick = magnitude.wrapping_add(1);

        while pick > magnitude {
            pick = i32::from(self.next()) & mask;
        }

        pick.wrapping_add(minval)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lcg_sequence_matches_upstream_constants_and_bit_extraction() {
        let mut random = RandomClass::new(0);
        assert_eq!(
            (0..5).map(|_| random.next()).collect::<Vec<_>>(),
            [12, 30_469, 16_649, 6_471, 27_301]
        );
        assert_eq!(random.seed, 229_283_573);
    }

    #[test]
    fn seed_arithmetic_wraps_at_32_bits() {
        let mut random = RandomClass::new(u32::MAX);
        assert_eq!(random.next(), 3_704);
        assert_eq!(random.seed, 571_035_320);
    }

    #[test]
    fn equal_range_does_not_advance_seed() {
        let mut random = RandomClass::new(123);
        assert_eq!(random.next_inclusive(7, 7), 7);
        assert_eq!(random.seed, 123);
    }

    #[test]
    fn reversed_range_is_normalized() {
        let mut forward = RandomClass::new(0);
        let mut reverse = RandomClass::new(0);
        assert_eq!(
            forward.next_inclusive(10, 100),
            reverse.next_inclusive(100, 10)
        );
    }

    #[test]
    fn range_results_stay_inside_requested_interval() {
        let mut random = RandomClass::new(0x1234_5678);
        for _ in 0..10_000 {
            let value = random.next_inclusive(-12, 37);
            assert!((-12..=37).contains(&value));
        }
    }

    #[test]
    fn maximum_supported_significance_is_fifteen_bits() {
        let mut random = RandomClass::new(1);
        for _ in 0..1_000 {
            assert!(u32::from(random.next()) <= Self::MAX_VALUE);
        }
    }
}
