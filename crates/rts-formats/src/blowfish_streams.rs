//! Blowfish Pipe/Straw adapters ported from REDALERT/BLOWPIPE.H/.CPP
//! and BLWSTRAW.H/.CPP.

use crate::blowfish::{BlowfishEngine, BlowfishError, BLOCK_SIZE};
use crate::pipe::PipeSink;
use crate::straw::StrawSource;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlowfishMode {
    Encrypt,
    Decrypt,
}

/// Push-oriented eight-byte Blowfish block adapter.
#[derive(Debug, Clone)]
pub struct BlowPipe {
    mode: BlowfishMode,
    engine: Option<BlowfishEngine>,
    buffer: [u8; BLOCK_SIZE],
    count: usize,
}

impl BlowPipe {
    pub const fn new(mode: BlowfishMode) -> Self {
        Self {
            mode,
            engine: None,
            buffer: [0; BLOCK_SIZE],
            count: 0,
        }
    }

    pub const fn is_keyed(&self) -> bool {
        self.engine.is_some()
    }

    pub const fn pending_len(&self) -> usize {
        self.count
    }

    /// Create the engine on first use and submit the new key. A zero-length
    /// key leaves an engine object present but unkeyed, matching the original
    /// difference between BF==NULL and BF->IsKeyed==false.
    pub fn key(&mut self, key: &[u8]) -> Result<(), BlowfishError> {
        self.engine
            .get_or_insert_with(BlowfishEngine::new)
            .submit_key(key)
    }

    pub fn clear_engine(&mut self) {
        self.engine = None;
    }

    /// Feed a chunk and return complete transformed blocks. With no engine,
    /// bytes pass through unchanged. When an engine exists, partial bytes are
    /// held until a complete eight-byte block arrives.
    pub fn put(&mut self, mut source: &[u8]) -> Vec<u8> {
        if self.engine.is_none() {
            return source.to_vec();
        }

        let mut output = Vec::new();

        if self.count > 0 {
            let count = (BLOCK_SIZE - self.count).min(source.len());
            self.buffer[self.count..self.count + count].copy_from_slice(&source[..count]);
            self.count += count;
            source = &source[count..];

            if self.count == BLOCK_SIZE {
                output.extend_from_slice(&self.transform(&self.buffer));
                self.count = 0;
            }
        }

        while source.len() >= BLOCK_SIZE {
            let mut block = [0_u8; BLOCK_SIZE];
            block.copy_from_slice(&source[..BLOCK_SIZE]);
            output.extend_from_slice(&self.transform(&block));
            source = &source[BLOCK_SIZE..];
        }

        if !source.is_empty() {
            self.buffer[..source.len()].copy_from_slice(source);
            self.count = source.len();
        }

        output
    }

    fn transform(&self, block: &[u8; BLOCK_SIZE]) -> [u8; BLOCK_SIZE] {
        let Some(engine) = self.engine.as_ref() else {
            return *block;
        };
        match self.mode {
            BlowfishMode::Encrypt => engine.encrypt_block(block),
            BlowfishMode::Decrypt => engine.decrypt_block(block),
        }
    }

    /// Source BlowPipe::Flush passes a trailing partial block unchanged,
    /// resets its counter, and then flushes the downstream chain.
    pub fn flush(&mut self) -> Vec<u8> {
        if self.engine.is_none() || self.count == 0 {
            self.count = 0;
            return Vec::new();
        }

        let result = self.buffer[..self.count].to_vec();
        self.count = 0;
        result
    }

    pub fn put_to_sink(&mut self, source: &[u8], sink: &mut impl PipeSink) -> usize {
        let output = self.put(source);
        sink.put(&output)
    }

    pub fn flush_to_sink(&mut self, sink: &mut impl PipeSink) -> usize {
        let output = self.flush();
        let mut total = sink.put(&output);
        total += sink.flush();
        total
    }
}

/// Pull-oriented Blowfish adapter. Only full eight-byte blocks are
/// transformed; an upstream partial block is passed through unchanged.
pub struct BlowStraw<S> {
    source: S,
    mode: BlowfishMode,
    engine: Option<BlowfishEngine>,
    buffer: [u8; BLOCK_SIZE],
    count: usize,
    exhausted: bool,
}

impl<S: StrawSource> BlowStraw<S> {
    pub fn new(source: S, mode: BlowfishMode) -> Self {
        Self {
            source,
            mode,
            engine: None,
            buffer: [0; BLOCK_SIZE],
            count: 0,
            exhausted: false,
        }
    }

    pub fn source(&self) -> &S {
        &self.source
    }

    pub fn source_mut(&mut self) -> &mut S {
        &mut self.source
    }

    pub const fn is_keyed(&self) -> bool {
        self.engine.is_some()
    }

    pub const fn pending_len(&self) -> usize {
        self.count
    }

    pub fn key(&mut self, key: &[u8]) -> Result<(), BlowfishError> {
        if self.engine.is_none() {
            self.engine = Some(BlowfishEngine::new());
        }
        self.engine.as_mut().expect("engine created").submit_key(key)
    }

    pub fn clear_engine(&mut self) {
        self.engine = None;
        self.count = 0;
    }

    fn transform(&self, block: &[u8; BLOCK_SIZE]) -> [u8; BLOCK_SIZE] {
        let Some(engine) = self.engine.as_ref() else {
            return *block;
        };
        match self.mode {
            BlowfishMode::Encrypt => engine.encrypt_block(block),
            BlowfishMode::Decrypt => engine.decrypt_block(block),
        }
    }

    pub fn get(&mut self, destination: &mut [u8]) -> usize {
        if destination.is_empty() {
            return 0;
        }

        if self.engine.is_none() {
            return self.source.get(destination).min(destination.len());
        }

        let mut total = 0_usize;
        while total < destination.len() {
            // Drain any buffered bytes first. The source class stores them
            // right-aligned at Buffer[8-Counter..].
            if self.count > 0 {
                let count = (destination.len() - total).min(self.count);
                let start = BLOCK_SIZE - self.count;
                destination[total..total + count]
                    .copy_from_slice(&self.buffer[start..start + count]);
                self.count -= count;
                total += count;
                continue;
            }

            if self.exhausted {
                break;
            }

            let incoming = self.source.get(&mut self.buffer).min(BLOCK_SIZE);
            if incoming == 0 {
                self.exhausted = true;
                break;
            }

            if incoming == BLOCK_SIZE {
                let transformed = self.transform(&self.buffer);
                self.buffer = transformed;
                self.count = BLOCK_SIZE;
            } else {
                // Partial blocks are moved to the tail and returned unchanged.
                self.buffer.copy_within(0..incoming, BLOCK_SIZE - incoming);
                self.count = incoming;
                self.exhausted = true;
            }
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::straw::SliceStraw;

    #[test]
    fn pipe_round_trip_complete_blocks() {
        let input = b"12345678ABCDEFGH";
        let mut encoder = BlowPipe::new(BlowfishMode::Encrypt);
        let mut decoder = BlowPipe::new(BlowfishMode::Decrypt);
        encoder.key(b"stream-key").unwrap();
        decoder.key(b"stream-key").unwrap();

        let encoded = encoder.put(input);
        let decoded = decoder.put(&encoded);
        assert_eq!(encoded.len(), input.len());
        assert_eq!(decoded, input);
    }

    #[test]
    fn pipe_stages_partial_input_and_flushes_tail_unchanged() {
        let mut pipe = BlowPipe::new(BlowfishMode::Encrypt);
        pipe.key(b"key").unwrap();
        assert!(pipe.put(b"12345").is_empty());
        assert_eq!(pipe.pending_len(), 5);
        assert_eq!(pipe.flush(), b"12345");
        assert_eq!(pipe.pending_len(), 0);
    }

    #[test]
    fn no_engine_passes_data_through_immediately() {
        let mut pipe = BlowPipe::new(BlowfishMode::Encrypt);
        assert_eq!(pipe.put(b"partial"), b"partial");
        assert_eq!(pipe.pending_len(), 0);
    }

    #[test]
    fn zero_length_key_keeps_engine_but_is_passthrough() {
        let mut pipe = BlowPipe::new(BlowfishMode::Encrypt);
        pipe.key(&[]).unwrap();
        assert!(pipe.is_keyed());
        assert!(pipe.put(b"12345").is_empty());
        assert_eq!(pipe.flush(), b"12345");
    }

    #[test]
    fn straw_round_trip_complete_blocks() {
        let input = b"12345678ABCDEFGH87654321";
        let mut encrypt = BlowStraw::new(
            SliceStraw::new(input.to_vec()),
            BlowfishMode::Encrypt,
        );
        encrypt.key(b"stream-key").unwrap();

        let mut encoded = Vec::new();
        let mut chunk = [0_u8; 5];
        loop {
            let count = encrypt.get(&mut chunk);
            if count == 0 { break; }
            encoded.extend_from_slice(&chunk[..count]);
        }

        let mut decrypt = BlowStraw::new(
            SliceStraw::new(encoded),
            BlowfishMode::Decrypt,
        );
        decrypt.key(b"stream-key").unwrap();

        let mut decoded = Vec::new();
        loop {
            let count = decrypt.get(&mut chunk);
            if count == 0 { break; }
            decoded.extend_from_slice(&chunk[..count]);
        }
        assert_eq!(decoded, input);
    }

    #[test]
    fn straw_passes_final_partial_block_unchanged() {
        let input = b"12345678tail";
        let mut straw = BlowStraw::new(
            SliceStraw::new(input.to_vec()),
            BlowfishMode::Encrypt,
        );
        straw.key(b"key").unwrap();

        let mut output = Vec::new();
        let mut chunk = [0_u8; 5];
        loop {
            let count = straw.get(&mut chunk);
            if count == 0 { break; }
            output.extend_from_slice(&chunk[..count]);
        }
        assert_eq!(&output[8..], b"tail");
    }

    #[test]
    fn straw_without_engine_delegates_directly() {
        let mut straw = BlowStraw::new(
            SliceStraw::new(b"unchanged".to_vec()),
            BlowfishMode::Decrypt,
        );
        let mut output = [0_u8; 16];
        assert_eq!(straw.get(&mut output), 9);
        assert_eq!(&output[..9], b"unchanged");
    }
}
