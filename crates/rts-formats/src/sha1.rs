//! Streaming SHA-1 engine ported from REDALERT/SHA.H and SHA.CPP.
//!
//! The public API includes a non-destructive digest snapshot, matching the
//! original SHAEngine::Result behavior: callers may continue hashing after
//! requesting the current digest.

#[derive(Debug, Clone)]
pub struct Sha1 {
    state: [u32; 5],
    buffer: [u8; 64],
    buffer_len: usize,
    length_bytes: u64,
}

impl Default for Sha1 {
    fn default() -> Self {
        Self::new()
    }
}

impl Sha1 {
    pub const DIGEST_SIZE: usize = 20;
    pub const BLOCK_SIZE: usize = 64;

    pub const fn new() -> Self {
        Self {
            state: [
                0x6745_2301,
                0xEFCD_AB89,
                0x98BA_DCFE,
                0x1032_5476,
                0xC3D2_E1F0,
            ],
            buffer: [0; 64],
            buffer_len: 0,
            length_bytes: 0,
        }
    }

    pub fn init(&mut self) {
        *self = Self::new();
    }

    pub const fn length_bytes(&self) -> u64 {
        self.length_bytes
    }

    pub const fn pending_bytes(&self) -> usize {
        self.buffer_len
    }

    pub fn update(&mut self, mut input: &[u8]) {
        self.length_bytes = self.length_bytes.wrapping_add(input.len() as u64);

        if self.buffer_len != 0 {
            let count = (Self::BLOCK_SIZE - self.buffer_len).min(input.len());
            self.buffer[self.buffer_len..self.buffer_len + count]
                .copy_from_slice(&input[..count]);
            self.buffer_len += count;
            input = &input[count..];

            if self.buffer_len == Self::BLOCK_SIZE {
                let block = self.buffer;
                self.process_block(&block);
                self.buffer_len = 0;
            }
        }

        while input.len() >= Self::BLOCK_SIZE {
            let mut block = [0_u8; 64];
            block.copy_from_slice(&input[..64]);
            self.process_block(&block);
            input = &input[64..];
        }

        if !input.is_empty() {
            self.buffer[..input.len()].copy_from_slice(input);
            self.buffer_len = input.len();
        }
    }

    /// Return the digest as if the input stopped now, without mutating state.
    pub fn result(&self) -> [u8; Self::DIGEST_SIZE] {
        let mut snapshot = self.clone();
        let bit_length = snapshot.length_bytes.wrapping_mul(8);

        snapshot.buffer[snapshot.buffer_len] = 0x80;
        snapshot.buffer_len += 1;

        if snapshot.buffer_len > 56 {
            snapshot.buffer[snapshot.buffer_len..].fill(0);
            let block = snapshot.buffer;
            snapshot.process_block(&block);
            snapshot.buffer = [0; 64];
            snapshot.buffer_len = 0;
        }

        snapshot.buffer[snapshot.buffer_len..56].fill(0);
        snapshot.buffer[56..64].copy_from_slice(&bit_length.to_be_bytes());
        let block = snapshot.buffer;
        snapshot.process_block(&block);

        let mut digest = [0_u8; Self::DIGEST_SIZE];
        for (index, word) in snapshot.state.iter().enumerate() {
            digest[index * 4..index * 4 + 4].copy_from_slice(&word.to_be_bytes());
        }
        digest
    }

    pub fn digest(input: &[u8]) -> [u8; Self::DIGEST_SIZE] {
        let mut sha = Self::new();
        sha.update(input);
        sha.result()
    }

    fn process_block(&mut self, block: &[u8; 64]) {
        let mut schedule = [0_u32; 80];

        for (index, word) in schedule[..16].iter_mut().enumerate() {
            let start = index * 4;
            *word = u32::from_be_bytes([
                block[start],
                block[start + 1],
                block[start + 2],
                block[start + 3],
            ]);
        }

        for index in 16..80 {
            schedule[index] = (schedule[index - 3]
                ^ schedule[index - 8]
                ^ schedule[index - 14]
                ^ schedule[index - 16])
                .rotate_left(1);
        }

        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];
        let mut e = self.state[4];

        for (index, word) in schedule.iter().enumerate() {
            let (function, constant) = match index {
                0..=19 => ((b & c) | ((!b) & d), 0x5A82_7999),
                20..=39 => (b ^ c ^ d, 0x6ED9_EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1B_BCDC),
                _ => (b ^ c ^ d, 0xCA62_C1D6),
            };

            let next = a
                .rotate_left(5)
                .wrapping_add(function)
                .wrapping_add(e)
                .wrapping_add(constant)
                .wrapping_add(*word);

            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = next;
        }

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha1_known_abc_vector() {
        assert_eq!(
            Sha1::digest(b"abc"),
            [
                0xA9, 0x99, 0x3E, 0x36, 0x47, 0x06, 0x81, 0x6A, 0xBA, 0x3E,
                0x25, 0x71, 0x78, 0x50, 0xC2, 0x6C, 0x9C, 0xD0, 0xD8, 0x9D,
            ]
        );
    }

    #[test]
    fn sha1_known_multiblock_vector() {
        assert_eq!(
            Sha1::digest(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
            [
                0x84, 0x98, 0x3E, 0x44, 0x1C, 0x3B, 0xD2, 0x6E, 0xBA, 0xAE,
                0x4A, 0xA1, 0xF9, 0x51, 0x29, 0xE5, 0xE5, 0x46, 0x70, 0xF1,
            ]
        );
    }

    #[test]
    fn sha1_known_repeated_block_vector() {
        assert_eq!(
            Sha1::digest(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
            [
                0x34, 0xAA, 0x97, 0x3C, 0xD4, 0xC4, 0xDA, 0xA4, 0xF6, 0x1E,
                0xEB, 0x2B, 0xDB, 0xAD, 0x27, 0x31, 0x65, 0x34, 0x01, 0x6F,
            ]
        );
    }

    #[test]
    fn chunked_hashing_matches_single_update() {
        let input = b"streaming SHA-1 should be independent of chunk boundaries";
        let expected = Sha1::digest(input);

        for chunk_size in 1..=input.len() {
            let mut sha = Sha1::new();
            for chunk in input.chunks(chunk_size) {
                sha.update(chunk);
            }
            assert_eq!(sha.result(), expected, "chunk size {chunk_size}");
        }
    }

    #[test]
    fn result_is_a_non_destructive_snapshot() {
        let mut sha = Sha1::new();
        sha.update(b"abc");
        let first = sha.result();
        assert_eq!(first, Sha1::digest(b"abc"));

        sha.update(b"def");
        assert_eq!(sha.result(), Sha1::digest(b"abcdef"));
        assert_eq!(first, Sha1::digest(b"abc"));
    }
}
