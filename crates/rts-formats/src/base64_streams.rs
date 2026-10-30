//! Base64 Pipe/Straw adapters ported from REDALERT/B64PIPE.CPP
//! and REDALERT/B64STRAW.CPP.

use crate::base64::{decode_into, encode_into};
use crate::pipe::PipeSink;
use crate::straw::StrawSource;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Base64Mode {
    Encode,
    Decode,
}

/// Push-oriented stream adapter equivalent to Base64Pipe.
#[derive(Debug, Clone)]
pub struct Base64Pipe {
    mode: Base64Mode,
    staged: Vec<u8>,
}

impl Base64Pipe {
    pub fn new(mode: Base64Mode) -> Self {
        Self {
            mode,
            staged: Vec::new(),
        }
    }

    pub fn pending_len(&self) -> usize {
        self.staged.len()
    }

    pub fn put(&mut self, source: &[u8]) -> Vec<u8> {
        self.staged.extend_from_slice(source);

        let (input_quantum, output_quantum) = match self.mode {
            Base64Mode::Encode => (3, 4),
            Base64Mode::Decode => (4, 3),
        };

        let mut output = Vec::new();
        while self.staged.len() >= input_quantum {
            let block: Vec<u8> = self.staged.drain(..input_quantum).collect();
            let mut converted = vec![0_u8; output_quantum];
            let written = match self.mode {
                Base64Mode::Encode => encode_into(&block, &mut converted).written,
                Base64Mode::Decode => decode_into(&block, &mut converted),
            };
            output.extend_from_slice(&converted[..written]);
        }
        output
    }

    pub fn flush(&mut self) -> Vec<u8> {
        if self.staged.is_empty() {
            return Vec::new();
        }

        let pending = core::mem::take(&mut self.staged);
        let capacity = match self.mode {
            Base64Mode::Encode => 4,
            Base64Mode::Decode => 3,
        };
        let mut converted = vec![0_u8; capacity];
        let written = match self.mode {
            Base64Mode::Encode => encode_into(&pending, &mut converted).written,
            Base64Mode::Decode => decode_into(&pending, &mut converted),
        };
        converted.truncate(written);
        converted
    }

    pub fn put_to_sink(
        &mut self,
        source: &[u8],
        sink: &mut impl PipeSink,
    ) -> usize {
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

/// Pull-oriented stream adapter equivalent to Base64Straw.
pub struct Base64Straw<S> {
    source: S,
    mode: Base64Mode,
    pending: Vec<u8>,
    pending_offset: usize,
    done: bool,
}

impl<S: StrawSource> Base64Straw<S> {
    pub fn new(source: S, mode: Base64Mode) -> Self {
        Self {
            source,
            mode,
            pending: Vec::new(),
            pending_offset: 0,
            done: false,
        }
    }

    pub fn source(&self) -> &S {
        &self.source
    }

    pub fn source_mut(&mut self) -> &mut S {
        &mut self.source
    }

    fn fill_pending(&mut self) -> bool {
        self.pending.clear();
        self.pending_offset = 0;
        if self.done {
            return false;
        }

        let (input_quantum, output_quantum) = match self.mode {
            Base64Mode::Encode => (3, 4),
            Base64Mode::Decode => (4, 3),
        };

        let mut input = vec![0_u8; input_quantum];
        let read = self.source.get(&mut input).min(input_quantum);
        if read == 0 {
            self.done = true;
            return false;
        }
        input.truncate(read);

        let mut converted = vec![0_u8; output_quantum];
        let written = match self.mode {
            Base64Mode::Encode => encode_into(&input, &mut converted).written,
            Base64Mode::Decode => decode_into(&input, &mut converted),
        };
        converted.truncate(written);
        self.pending = converted;

        // Like Straw::Get, a short read is treated as the end of this source.
        if read < input_quantum {
            self.done = true;
        }
        !self.pending.is_empty()
    }

    pub fn get(&mut self, destination: &mut [u8]) -> usize {
        if destination.is_empty() {
            return 0;
        }

        let mut total = 0_usize;
        while total < destination.len() {
            if self.pending_offset >= self.pending.len() && !self.fill_pending() {
                break;
            }

            let available = self.pending.len() - self.pending_offset;
            let count = available.min(destination.len() - total);
            destination[total..total + count].copy_from_slice(
                &self.pending[self.pending_offset..self.pending_offset + count],
            );
            total += count;
            self.pending_offset += count;
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::straw::SliceStraw;

    #[test]
    fn pipe_encodes_partial_final_block_on_flush() {
        let mut pipe = Base64Pipe::new(Base64Mode::Encode);
        let mut output = pipe.put(b"f");
        output.extend_from_slice(&pipe.flush());
        assert_eq!(output, b"Zg==");
    }

    #[test]
    fn pipe_decodes_padding_and_invalid_characters() {
        let mut pipe = Base64Pipe::new(Base64Mode::Decode);
        let mut output = pipe.put(b"Zg==");
        output.extend_from_slice(&pipe.flush());
        assert_eq!(output, b"f");
    }

    #[test]
    fn pipe_stream_round_trip_across_input_boundaries() {
        let input = b"Westwood Base64 stream adapter";
        let mut encoder = Base64Pipe::new(Base64Mode::Encode);
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&encoder.put(&input[..4]));
        encoded.extend_from_slice(&encoder.put(&input[4..11]));
        encoded.extend_from_slice(&encoder.put(&input[11..]));
        encoded.extend_from_slice(&encoder.flush());

        let mut decoder = Base64Pipe::new(Base64Mode::Decode);
        let mut decoded = Vec::new();
        for block in encoded.chunks(5) {
            decoded.extend_from_slice(&decoder.put(block));
        }
        decoded.extend_from_slice(&decoder.flush());
        assert_eq!(decoded, input);
    }

    #[test]
    fn straw_stream_encodes_and_decodes() {
        let input = b"Base64 pull stream";
        let mut encoder = Base64Straw::new(
            SliceStraw::new(input.to_vec()),
            Base64Mode::Encode,
        );
        let mut encoded = Vec::new();
        let mut chunk = [0_u8; 5];
        loop {
            let count = encoder.get(&mut chunk);
            if count == 0 { break; }
            encoded.extend_from_slice(&chunk[..count]);
        }

        let mut decoder = Base64Straw::new(
            SliceStraw::new(encoded),
            Base64Mode::Decode,
        );
        let mut decoded = Vec::new();
        let mut chunk = [0_u8; 4];
        loop {
            let count = decoder.get(&mut chunk);
            if count == 0 { break; }
            decoded.extend_from_slice(&chunk[..count]);
        }
        assert_eq!(decoded, input);
    }

    #[test]
    fn decoder_straw_does_not_require_full_request_size() {
        let mut decoder = Base64Straw::new(
            SliceStraw::new(b"Zm8=".to_vec()),
            Base64Mode::Decode,
        );
        let mut output = [0_u8; 8];
        assert_eq!(decoder.get(&mut output), 2);
        assert_eq!(&output[..2], b"fo");
    }
}
