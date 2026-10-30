//! RandomStraw entropy mixer ported from REDALERT/RNDSTRAW.H and RNDSTRAW.CPP.
//!
//! This preserves the original 32 x 32-bit LCG state array, low-bit-first
//! entropy intake, 1,024-bit first-seed threshold, and evolving SHA-1
//! rehash/copy loop. It is retained for compatibility; it is not recommended
//! as a new cryptographic random-number generator.

use crate::random_class::RandomClass;
use crate::sha1::Sha1;

pub const RANDOM_GENERATORS: usize = 32;
pub const RANDOM_STATE_BYTES: usize = RANDOM_GENERATORS * 4;
pub const RANDOM_SEED_BITS: usize = RANDOM_STATE_BYTES * 8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RandomStraw {
    state: [u8; RANDOM_STATE_BYTES],
    seed_bits: usize,
    current: usize,
}

impl Default for RandomStraw {
    fn default() -> Self {
        Self::new()
    }
}

impl RandomStraw {
    pub const fn new() -> Self {
        Self {
            state: [0; RANDOM_STATE_BYTES],
            seed_bits: 0,
            current: 0,
        }
    }

    pub fn reset(&mut self) {
        self.state.fill(0);
        self.seed_bits = 0;
        self.current = 0;
    }

    pub const fn seed_bits_needed(&self) -> usize {
        RANDOM_SEED_BITS.saturating_sub(self.seed_bits)
    }

    pub const fn seed_bits_collected(&self) -> usize {
        self.seed_bits
    }

    pub const fn current_generator(&self) -> usize {
        self.current
    }

    /// Port of RandomStraw::Seed_Bit. The bit is XORed into the selected
    /// little-endian state byte, and the first complete 1,024-bit seed triggers
    /// the source's iterative SHA-1 scramble exactly once.
    pub fn seed_bit(&mut self, bit: i32) {
        let byte_index = (self.seed_bits / 8) % RANDOM_STATE_BYTES;
        let bit_mask = 1_u8 << (self.seed_bits & 7);

        if bit & 1 != 0 {
            self.state[byte_index] ^= bit_mask;
        }

        self.seed_bits = self.seed_bits.saturating_add(1);
        if self.seed_bits == RANDOM_SEED_BITS {
            self.scramble_seed();
        }
    }

    /// Submit eight bits, least-significant bit first.
    pub fn seed_byte(&mut self, mut seed: i8) {
        for _ in 0..8 {
            self.seed_bit(i32::from(seed) & 1);
            seed >>= 1;
        }
    }

    /// Submit sixteen bits, least-significant bit first.
    pub fn seed_short(&mut self, mut seed: i16) {
        for _ in 0..16 {
            self.seed_bit(i32::from(seed) & 1);
            seed >>= 1;
        }
    }

    /// Submit thirty-two bits, least-significant bit first.
    pub fn seed_long(&mut self, mut seed: i32) {
        for _ in 0..32 {
            self.seed_bit(seed & 1);
            seed >>= 1;
        }
    }

    /// The source keeps one SHA engine alive throughout this loop: each
    /// iteration hashes the current 128-byte generator state onto the
    /// accumulated SHA stream, takes a non-destructive digest snapshot, and
    /// copies up to 20 bytes back starting at that iteration's byte offset.
    fn scramble_seed(&mut self) {
        let mut sha = Sha1::new();

        for index in 0..RANDOM_STATE_BYTES {
            let digest = {
                sha.update(&self.state);
                sha.result()
            };

            let count = Sha1::DIGEST_SIZE.min(RANDOM_STATE_BYTES - index);
            self.state[index..index + count].copy_from_slice(&digest[..count]);
        }
    }

    /// Fill a caller-owned buffer using the 32 interleaved 15-bit LCGs.
    ///
    /// Each generator returns the low byte of RandomClass's 15-bit output;
    /// the current generator advances modulo 32 after every output byte.
    pub fn get(&mut self, destination: &mut [u8]) -> usize {
        for byte in destination.iter_mut() {
            let start = self.current * 4;
            let seed = u32::from_le_bytes([
                self.state[start],
                self.state[start + 1],
                self.state[start + 2],
                self.state[start + 3],
            ]);
            let mut generator = RandomClass::new(seed);
            *byte = generator.next() as u8;
            self.state[start..start + 4].copy_from_slice(&generator.seed.to_le_bytes());
            self.current = (self.current + 1) % RANDOM_GENERATORS;
        }
        destination.len()
    }

    pub fn get_vec(&mut self, count: usize) -> Vec<u8> {
        let mut result = vec![0; count];
        self.get(&mut result);
        result
    }

    /// Clear internal state to reduce the lifetime of seed material in memory.
    pub fn zeroize(&mut self) {
        self.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requires_exactly_1024_initial_seed_bits() {
        let mut random = RandomStraw::new();
        assert_eq!(random.seed_bits_needed(), 1024);
        for _ in 0..1023 {
            random.seed_bit(1);
        }
        assert_eq!(random.seed_bits_needed(), 1);
        random.seed_bit(1);
        assert_eq!(random.seed_bits_needed(), 0);
        assert_eq!(random.seed_bits_collected(), 1024);
    }

    #[test]
    fn seed_inputs_are_lsb_first_in_little_endian_state_bytes() {
        let mut random = RandomStraw::new();
        random.seed_byte(0b0000_0101);
        assert_eq!(random.state[0], 0x05);
        assert_eq!(random.seed_bits_collected(), 8);
    }

    #[test]
    fn same_seed_input_produces_same_scrambled_state_and_stream() {
        let mut first = RandomStraw::new();
        let mut second = RandomStraw::new();

        for value in 0_u16..128 {
            let byte = (value.wrapping_mul(73) ^ 0xA5) as u8 as i8;
            first.seed_byte(byte);
            second.seed_byte(byte);
        }

        assert_eq!(first.state, second.state);
        assert_eq!(first.get_vec(256), second.get_vec(256));
    }

    #[test]
    fn initial_unseeded_generators_match_default_lcg_states() {
        let mut random = RandomStraw::new();
        assert_eq!(random.get_vec(32), vec![12; 32]);
        assert_eq!(random.get_vec(1), [37]);
        assert_eq!(random.current_generator(), 1);
    }

    #[test]
    fn seed_bit_collection_after_first_scramble_continues_cycling_state_bytes() {
        let mut random = RandomStraw::new();
        for _ in 0..RANDOM_SEED_BITS {
            random.seed_bit(0);
        }
        let before = random.state;
        random.seed_bit(1);
        assert_ne!(random.state, before);
        assert_eq!(random.seed_bits_needed(), 0);
    }

    #[test]
    fn reset_clears_seed_bits_index_and_generator_state() {
        let mut random = RandomStraw::new();
        random.seed_byte(0x5A);
        let _ = random.get_vec(9);
        random.reset();

        assert_eq!(random.seed_bits_collected(), 0);
        assert_eq!(random.current_generator(), 0);
        assert_eq!(random.state, [0; RANDOM_STATE_BYTES]);
    }
}
