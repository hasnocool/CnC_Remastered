//! Source-compatible deterministic Sim_Random / Sim_IRandom routines.

use crate::map::GameVariant;

/// The 256-byte lookup sequence shared by both original game variants.
pub const SIM_RANDOM_VALUES: [u8; 256] = [
    0x47, 0xce, 0xc6, 0x6e, 0xd7, 0x9f, 0x98, 0x29, 0x92, 0x0c, 0x74, 0xa2, 0x65, 0x20, 0x4b, 0x4f,
    0x1e, 0xed, 0x3a, 0xdf, 0xa5, 0x7d, 0xb5, 0xc8, 0x86, 0x01, 0x81, 0xca, 0xf1, 0x17, 0xd6, 0x23,
    0xe1, 0xbd, 0x0e, 0xe4, 0x62, 0xfa, 0xd9, 0x5c, 0x68, 0xf5, 0x7f, 0xdc, 0xe7, 0xb9, 0xc4, 0xb3,
    0x7a, 0xd8, 0x06, 0x3e, 0xeb, 0x09, 0x1a, 0x31, 0x3f, 0x46, 0x28, 0x12, 0xf0, 0x10, 0x84, 0x76,
    0x3b, 0xc5, 0x53, 0x18, 0x14, 0x73, 0x7e, 0x59, 0x48, 0x93, 0xaa, 0x1d, 0x5d, 0x79, 0x24, 0x61,
    0x1b, 0xfd, 0x2b, 0xa8, 0xc2, 0xdb, 0xe8, 0x2a, 0xb0, 0x25, 0x95, 0xab, 0x96, 0x83, 0xfc, 0x5f,
    0x9c, 0x32, 0x78, 0x9a, 0x9e, 0xe2, 0x8e, 0x35, 0x4c, 0x41, 0xa1, 0x69, 0x5a, 0xfe, 0xa7, 0xa4,
    0xf6, 0x6d, 0xc1, 0x58, 0x0a, 0xcf, 0xea, 0xc3, 0xba, 0x85, 0x99, 0x8d, 0x36, 0xb6, 0xdd, 0xd3,
    0x04, 0xe6, 0x45, 0x0d, 0x60, 0xae, 0xa3, 0x22, 0x4d, 0xe9, 0xc9, 0x9b, 0xb7, 0x0f, 0x02, 0x42,
    0xf9, 0x0b, 0x8f, 0x43, 0x44, 0x87, 0x70, 0xbe, 0xe3, 0xf8, 0xee, 0xa9, 0xbc, 0xc0, 0x67, 0x33,
    0x16, 0x37, 0x57, 0xad, 0x5e, 0x9d, 0x64, 0x40, 0x54, 0x05, 0x2c, 0xe0, 0xb2, 0x97, 0x08, 0xaf,
    0x75, 0x8a, 0x5b, 0xfb, 0x4e, 0xbf, 0x91, 0xf3, 0xcb, 0x7c, 0x63, 0xef, 0x89, 0x52, 0x6c, 0x2f,
    0x21, 0x4a, 0xf7, 0xcd, 0x2e, 0xf4, 0xc7, 0x6f, 0x19, 0xb1, 0x66, 0xcc, 0x90, 0x8c, 0x50, 0x51,
    0x26, 0x7b, 0xda, 0x49, 0x80, 0x30, 0x55, 0x1f, 0xd2, 0xb4, 0xd1, 0xd5, 0x6b, 0xf2, 0x72, 0xbb,
    0x13, 0x3d, 0xff, 0x15, 0x38, 0xe5, 0xd4, 0xde, 0x2d, 0x27, 0x94, 0xa0, 0xd0, 0x39, 0x82, 0x8b,
    0x03, 0xac, 0x3c, 0x34, 0x77, 0xb8, 0xec, 0x00, 0x07, 0x1c, 0x88, 0xa6, 0x56, 0x11, 0x71, 0x6a,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixedFractionFormat {
    /// Tiberian Dawn's COORDA.ASM uses an 8-bit fractional component.
    TiberianDawn8_8,
    /// Red Alert's COORDA.ASM uses a 16-bit fractional component.
    RedAlert16_16,
}

impl FixedFractionFormat {
    pub const fn for_variant(variant: GameVariant) -> Self {
        match variant {
            GameVariant::TiberianDawn => Self::TiberianDawn8_8,
            GameVariant::RedAlert => Self::RedAlert16_16,
        }
    }

    /// Port of Cardinal_To_Fixed from each variant's COORDA.ASM.
    ///
    /// Division by zero uses the variant's original sentinel. The multiply
    /// stage wraps to 32 bits exactly as the Win32 assembly does.
    pub const fn cardinal_to_fixed(self, base: u32, cardinal: u32) -> u32 {
        if base == 0 {
            return match self {
                Self::TiberianDawn8_8 => 0xFFFF,
                Self::RedAlert16_16 => 0xFFFF_FFFF,
            };
        }

        let shifted = match self {
            Self::TiberianDawn8_8 => cardinal.wrapping_shl(8),
            Self::RedAlert16_16 => cardinal.wrapping_shl(16),
        };
        shifted / base
    }

    /// Exact unsigned 32-bit arithmetic used by each variant's COORDA.ASM helper.
    pub const fn fixed_to_cardinal(self, base: u32, fraction: u32) -> u32 {
        match self {
            Self::TiberianDawn8_8 => {
                let product = base.wrapping_mul(fraction).wrapping_add(0x80);
                if product & 0xFF00_0000 != 0 {
                    0xFFFF
                } else {
                    product >> 8
                }
            }
            Self::RedAlert16_16 => {
                base.wrapping_mul(fraction).wrapping_add(0x8000) >> 16
            }
        }
    }
}

/// Per-game simulation random state.
///
/// The C++ source increments the low byte of the global SimRandIndex before
/// indexing the lookup table. Thus the first result is table entry 1 (0xCE),
/// not entry 0 (0x47).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimRandom {
    index: u8,
}

impl Default for SimRandom {
    fn default() -> Self {
        Self::new()
    }
}

impl SimRandom {
    pub const fn new() -> Self {
        Self { index: 0 }
    }

    /// Restore the low-byte state used by the original SimRandIndex global.
    pub const fn with_index(index: u8) -> Self {
        Self { index }
    }

    pub const fn index(self) -> u8 {
        self.index
    }

    pub fn next_u8(&mut self) -> u8 {
        self.index = self.index.wrapping_add(1);
        SIM_RANDOM_VALUES[self.index as usize]
    }

    pub fn next_i32(&mut self) -> i32 {
        i32::from(self.next_u8())
    }

    /// Reproduces Sim_IRandom(minval, maxval), including the per-variant
    /// Fixed_To_Cardinal scaling and 32-bit wrap behavior.
    pub fn next_inclusive(&mut self, variant: GameVariant, minval: i32, maxval: i32) -> i32 {
        let random = u32::from(self.next_u8());
        let span = maxval.wrapping_sub(minval) as u32;
        let scaled = FixedFractionFormat::for_variant(variant)
            .fixed_to_cardinal(span, random);
        (minval as u32).wrapping_add(scaled) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_values_match_both_upstream_rand_cpp_tables() {
        let mut random = SimRandom::new();
        let actual: Vec<u8> = (0..8).map(|_| random.next_u8()).collect();
        assert_eq!(actual, [0xCE, 0xC6, 0x6E, 0xD7, 0x9F, 0x98, 0x29, 0x92]);
    }

    #[test]
    fn table_is_a_permutation_of_all_byte_values() {
        let mut values = SIM_RANDOM_VALUES;
        values.sort_unstable();
        assert_eq!(values, core::array::from_fn(|index| index as u8));
    }

    #[test]
    fn low_byte_index_wraps_before_table_lookup() {
        let mut random = SimRandom::new();
        for _ in 0..255 {
            random.next_u8();
        }
        assert_eq!(random.index(), 255);
        assert_eq!(random.next_u8(), 0x47);
        assert_eq!(random.index(), 0);
    }

    #[test]
    fn cardinal_to_fixed_uses_the_variant_specific_fractional_width() {
        assert_eq!(FixedFractionFormat::TiberianDawn8_8.cardinal_to_fixed(100, 50), 128);
        assert_eq!(FixedFractionFormat::RedAlert16_16.cardinal_to_fixed(100, 50), 32_768);
        assert_eq!(FixedFractionFormat::TiberianDawn8_8.cardinal_to_fixed(0, 1), 0xFFFF);
        assert_eq!(FixedFractionFormat::RedAlert16_16.cardinal_to_fixed(0, 1), u32::MAX);
    }

    #[test]
    fn tiberian_dawn_range_scaling_matches_8_bit_assembly() {
        let mut random = SimRandom::new();
        assert_eq!(random.next_inclusive(GameVariant::TiberianDawn, 10, 110), 90);
    }

    #[test]
    fn red_alert_range_scaling_matches_16_bit_assembly() {
        let mut random = SimRandom::new();
        // This is the literal upstream helper behavior: Sim_Random supplies
        // an 8-bit value while Red Alert's helper treats its second operand as
        // a 16.16 fraction. Preserve it rather than silently rescaling it.
        assert_eq!(random.next_inclusive(GameVariant::RedAlert, 10, 110), 10);
    }
}
