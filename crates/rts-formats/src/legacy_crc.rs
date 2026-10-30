//! Westwood's legacy rotate-and-add checksum engine (not a standards CRC).

/// Port of REDALERT/CRC.H and REDALERT/CRC.CPP.
///
/// The engine processes four-byte little-endian words:
/// `crc = rotate_left(crc, 1) + word`, with 32-bit wrapping arithmetic.
/// Incomplete trailing words are previewed by value() and committed only when
/// four bytes have accumulated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LegacyCrcEngine {
    crc: u32,
    staging: [u8; 4],
    index: u8,
}

impl Default for LegacyCrcEngine {
    fn default() -> Self {
        Self::new(0)
    }
}

impl LegacyCrcEngine {
    pub const fn new(initial: u32) -> Self {
        Self {
            crc: initial,
            staging: [0; 4],
            index: 0,
        }
    }

    pub const fn reset(&mut self, initial: u32) {
        self.crc = initial;
        self.staging = [0; 4];
        self.index = 0;
    }

    pub const fn committed_value(&self) -> u32 {
        self.crc
    }

    pub const fn pending_len(&self) -> usize {
        self.index as usize
    }

    /// Current observable value; a partial word is included without committing it.
    pub fn value(&self) -> u32 {
        if self.index != 0 {
            self.crc
                .rotate_left(1)
                .wrapping_add(u32::from_le_bytes(self.staging))
        } else {
            self.crc
        }
    }

    pub fn update_byte(&mut self, byte: u8) {
        self.staging[self.index as usize] = byte;
        self.index += 1;

        if self.index == 4 {
            self.crc = self.value();
            self.staging = [0; 4];
            self.index = 0;
        }
    }

    /// Bulk update equivalent to the original pointer/length overload.
    /// The implementation is safe for arbitrary slice alignment.
    pub fn update(&mut self, bytes: &[u8]) -> u32 {
        let mut offset = 0;

        while offset < bytes.len() && self.index != 0 {
            self.update_byte(bytes[offset]);
            offset += 1;
        }

        while bytes.len() - offset >= 4 {
            let word = u32::from_le_bytes([
                bytes[offset],
                bytes[offset + 1],
                bytes[offset + 2],
                bytes[offset + 3],
            ]);
            self.crc = self.crc.rotate_left(1).wrapping_add(word);
            offset += 4;
        }

        while offset < bytes.len() {
            self.update_byte(bytes[offset]);
            offset += 1;
        }

        self.value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_word_value_matches_little_endian_union() {
        let mut engine = LegacyCrcEngine::new(0);
        assert_eq!(engine.update(b"abc"), 0x0063_6261);
        assert_eq!(engine.pending_len(), 3);
        assert_eq!(engine.committed_value(), 0);
    }

    #[test]
    fn complete_word_commits_rotate_add_result() {
        let mut engine = LegacyCrcEngine::new(0);
        assert_eq!(engine.update(b"abcd"), 0x6463_6261);
        assert_eq!(engine.committed_value(), 0x6463_6261);
        assert_eq!(engine.pending_len(), 0);
    }

    #[test]
    fn split_updates_match_one_bulk_update() {
        let input = b"Westwood legacy checksum fixture";
        let mut bulk = LegacyCrcEngine::new(0x1234_5678);
        let expected = bulk.update(input);

        for split in 0..=input.len() {
            let mut streamed = LegacyCrcEngine::new(0x1234_5678);
            streamed.update(&input[..split]);
            let actual = streamed.update(&input[split..]);
            assert_eq!(actual, expected, "split at byte {split}");
        }
    }

    #[test]
    fn pending_bytes_can_be_observed_without_mutating_state() {
        let mut engine = LegacyCrcEngine::new(1);
        engine.update(b"x");
        let first = engine.value();
        assert_eq!(engine.value(), first);
        assert_eq!(engine.pending_len(), 1);
        engine.update(b"yzq");
        assert_eq!(engine.pending_len(), 0);
    }
}
