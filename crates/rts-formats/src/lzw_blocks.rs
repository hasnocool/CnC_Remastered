//! Framed LZW Pipe/Straw adapters ported from REDALERT/LZWPIPE.CPP
//! and REDALERT/LZWSTRAW.CPP.

use crate::lzw::{compress, decompress_expected, LzwError};
use crate::pipe::PipeSink;
use crate::straw::StrawSource;

/// The four-byte block prefix shared with LCW streams.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LzwBlockHeader {
    pub compressed_count: u16,
    pub uncompressed_count: u16,
}

impl LzwBlockHeader {
    pub const SIZE: usize = 4;

    pub fn read(bytes: &[u8]) -> Result<Self, LzwError> {
        if bytes.len() < Self::SIZE {
            return Err(LzwError::UnexpectedEndOfInput);
        }
        Ok(Self {
            compressed_count: u16::from_le_bytes([bytes[0], bytes[1]]),
            uncompressed_count: u16::from_le_bytes([bytes[2], bytes[3]]),
        })
    }

    pub fn write(self, output: &mut Vec<u8>) {
        output.extend_from_slice(&self.compressed_count.to_le_bytes());
        output.extend_from_slice(&self.uncompressed_count.to_le_bytes());
    }
}

fn validate_block_size(block_size: usize) -> Result<(), LzwError> {
    if block_size == 0 || block_size > usize::from(u16::MAX) {
        return Err(LzwError::InvalidBlockSize(block_size));
    }
    Ok(())
}

fn encode_frame(input: &[u8]) -> Result<Vec<u8>, LzwError> {
    if input.len() > usize::from(u16::MAX) {
        return Err(LzwError::LengthOutOfRange(input.len()));
    }
    let compressed = compress(input)?;
    if compressed.len() > usize::from(u16::MAX) {
        return Err(LzwError::LengthOutOfRange(compressed.len()));
    }

    let mut frame = Vec::with_capacity(LzwBlockHeader::SIZE + compressed.len());
    LzwBlockHeader {
        compressed_count: compressed.len() as u16,
        uncompressed_count: input.len() as u16,
    }
    .write(&mut frame);
    frame.extend_from_slice(&compressed);
    Ok(frame)
}

/// Compress a byte slice into independent LZW frames of at most block_size.
pub fn compress_blocks(input: &[u8], block_size: usize) -> Result<Vec<u8>, LzwError> {
    validate_block_size(block_size)?;
    let mut output = Vec::new();

    for block in input.chunks(block_size) {
        output.extend_from_slice(&encode_frame(block)?);
    }

    Ok(output)
}

/// Decompress complete LZW frames, enforcing a total decoded output limit.
pub fn decompress_blocks(input: &[u8], output_limit: usize) -> Result<Vec<u8>, LzwError> {
    let mut cursor = 0_usize;
    let mut output = Vec::new();

    while cursor < input.len() {
        let header = LzwBlockHeader::read(&input[cursor..])?;
        cursor += LzwBlockHeader::SIZE;

        let compressed_count = usize::from(header.compressed_count);
        let uncompressed_count = usize::from(header.uncompressed_count);
        if compressed_count == 0 {
            return Err(LzwError::InvalidBlockHeader);
        }

        let end = cursor
            .checked_add(compressed_count)
            .ok_or(LzwError::UnexpectedEndOfInput)?;
        let compressed = input
            .get(cursor..end)
            .ok_or(LzwError::UnexpectedEndOfInput)?;
        cursor = end;

        let decoded = decompress_expected(compressed, uncompressed_count)?;
        let requested = output
            .len()
            .checked_add(decoded.len())
            .unwrap_or(usize::MAX);
        if requested > output_limit {
            return Err(LzwError::OutputLimitExceeded {
                requested,
                limit: output_limit,
            });
        }
        output.extend_from_slice(&decoded);
    }

    Ok(output)
}

/// Push transformer equivalent to LZWPipe.
///
/// In compression mode it emits full framed blocks as they are completed and
/// a short final block on flush. In decompression mode arbitrary fragments are
/// staged until a complete block is available. Truncated tails pass through on
/// flush just as the legacy Pipe implementation did.
#[derive(Debug, Clone)]
pub struct LzwPipe {
    mode: LzwMode,
    block_size: usize,
    staged: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LzwMode {
    Compress,
    Decompress,
}

impl LzwPipe {
    pub fn new(mode: LzwMode, block_size: usize) -> Result<Self, LzwError> {
        validate_block_size(block_size)?;
        Ok(Self {
            mode,
            block_size,
            staged: Vec::new(),
        })
    }

    pub fn pending_len(&self) -> usize {
        self.staged.len()
    }

    pub fn put(&mut self, source: &[u8]) -> Result<Vec<u8>, LzwError> {
        match self.mode {
            LzwMode::Compress => {
                self.staged.extend_from_slice(source);
                let mut output = Vec::new();

                while self.staged.len() >= self.block_size {
                    let block: Vec<u8> = self.staged.drain(..self.block_size).collect();
                    output.extend_from_slice(&encode_frame(&block)?);
                }
                Ok(output)
            }
            LzwMode::Decompress => {
                self.staged.extend_from_slice(source);
                let mut output = Vec::new();

                loop {
                    if self.staged.len() < LzwBlockHeader::SIZE {
                        break;
                    }

                    let header = LzwBlockHeader::read(&self.staged)?;
                    let compressed_count = usize::from(header.compressed_count);
                    let uncompressed_count = usize::from(header.uncompressed_count);
                    if compressed_count == 0 {
                        return Err(LzwError::InvalidBlockHeader);
                    }

                    let frame_size = LzwBlockHeader::SIZE
                        .checked_add(compressed_count)
                        .ok_or(LzwError::UnexpectedEndOfInput)?;
                    if self.staged.len() < frame_size {
                        break;
                    }

                    let decoded = decompress_expected(
                        &self.staged[LzwBlockHeader::SIZE..frame_size],
                        uncompressed_count,
                    )?;
                    output.extend_from_slice(&decoded);
                    self.staged.drain(..frame_size);
                }

                Ok(output)
            }
        }
    }

    pub fn flush(&mut self) -> Result<Vec<u8>, LzwError> {
        match self.mode {
            LzwMode::Compress => {
                if self.staged.is_empty() {
                    return Ok(Vec::new());
                }
                encode_frame(&core::mem::take(&mut self.staged))
            }
            LzwMode::Decompress => {
                if self.staged.len() < LzwBlockHeader::SIZE {
                    return Ok(core::mem::take(&mut self.staged));
                }
                if self.staged.len() == LzwBlockHeader::SIZE {
                    self.staged.clear();
                    return Ok(Vec::new());
                }
                Ok(core::mem::take(&mut self.staged))
            }
        }
    }

    pub fn put_to_sink(
        &mut self,
        source: &[u8],
        sink: &mut impl PipeSink,
    ) -> Result<usize, LzwError> {
        let output = self.put(source)?;
        Ok(sink.put(&output))
    }

    pub fn flush_to_sink(&mut self, sink: &mut impl PipeSink) -> Result<usize, LzwError> {
        let output = self.flush()?;
        let mut total = sink.put(&output);
        total += sink.flush();
        Ok(total)
    }
}

/// Demand-driven LZW adapter equivalent to LZWStraw.
pub struct LzwStraw<S> {
    source: S,
    mode: LzwMode,
    block_size: usize,
    pending: Vec<u8>,
    pending_offset: usize,
    done: bool,
}

impl<S: StrawSource> LzwStraw<S> {
    pub fn new(source: S, mode: LzwMode, block_size: usize) -> Result<Self, LzwError> {
        validate_block_size(block_size)?;
        Ok(Self {
            source,
            mode,
            block_size,
            pending: Vec::new(),
            pending_offset: 0,
            done: false,
        })
    }

    pub fn source(&self) -> &S {
        &self.source
    }

    pub fn source_mut(&mut self) -> &mut S {
        &mut self.source
    }

    fn read_exact_source(&mut self, destination: &mut [u8]) -> bool {
        let mut offset = 0_usize;
        while offset < destination.len() {
            let read = self.source.get(&mut destination[offset..]);
            if read == 0 {
                return false;
            }
            offset += read.min(destination.len() - offset);
        }
        true
    }

    fn fill_pending(&mut self) -> Result<bool, LzwError> {
        self.pending.clear();
        self.pending_offset = 0;
        if self.done {
            return Ok(false);
        }

        match self.mode {
            LzwMode::Compress => {
                let mut raw = vec![0_u8; self.block_size];
                let count = self.source.get(&mut raw).min(self.block_size);
                if count == 0 {
                    self.done = true;
                    return Ok(false);
                }
                raw.truncate(count);
                self.pending = encode_frame(&raw)?;
                Ok(true)
            }
            LzwMode::Decompress => {
                let mut header_bytes = [0_u8; LzwBlockHeader::SIZE];
                if !self.read_exact_source(&mut header_bytes) {
                    self.done = true;
                    return Ok(false);
                }

                let header = LzwBlockHeader::read(&header_bytes)?;
                let compressed_count = usize::from(header.compressed_count);
                let uncompressed_count = usize::from(header.uncompressed_count);
                if compressed_count == 0 {
                    return Err(LzwError::InvalidBlockHeader);
                }

                let mut compressed = vec![0_u8; compressed_count];
                if !self.read_exact_source(&mut compressed) {
                    self.done = true;
                    return Ok(false);
                }
                self.pending = decompress_expected(&compressed, uncompressed_count)?;
                Ok(true)
            }
        }
    }

    pub fn get(&mut self, destination: &mut [u8]) -> Result<usize, LzwError> {
        if destination.is_empty() {
            return Ok(0);
        }

        let mut total = 0_usize;
        while total < destination.len() {
            if self.pending_offset >= self.pending.len() && !self.fill_pending()? {
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
        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::straw::SliceStraw;

    #[test]
    fn block_stream_round_trips() {
        let input = b"Westwood LZW blocks: dictionary phrases repeat repeat repeat";
        let encoded = compress_blocks(input, 11).unwrap();
        assert_eq!(decompress_blocks(&encoded, input.len()).unwrap(), input);
    }

    #[test]
    fn pipe_buffers_compression_and_flushes_final_block() {
        let input = b"0123456789abcdef";
        let mut encoder = LzwPipe::new(LzwMode::Compress, 6).unwrap();
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&encoder.put(&input[..3]).unwrap());
        encoded.extend_from_slice(&encoder.put(&input[3..10]).unwrap());
        encoded.extend_from_slice(&encoder.put(&input[10..]).unwrap());
        encoded.extend_from_slice(&encoder.flush().unwrap());
        assert_eq!(decompress_blocks(&encoded, input.len()).unwrap(), input);
    }

    #[test]
    fn pipe_decodes_fragmented_headers_and_codewords() {
        let input = b"Fragmented block input with repeated repeated tokens";
        let encoded = compress_blocks(input, 9).unwrap();
        let mut decoder = LzwPipe::new(LzwMode::Decompress, 9).unwrap();
        let mut decoded = Vec::new();

        for part in encoded.chunks(3) {
            decoded.extend_from_slice(&decoder.put(part).unwrap());
        }

        assert!(decoder.flush().unwrap().is_empty());
        assert_eq!(decoded, input);
    }

    #[test]
    fn incomplete_pipe_tail_is_passed_through_on_flush() {
        let mut decoder = LzwPipe::new(LzwMode::Decompress, 32).unwrap();
        let tail = [0x03, 0x00];
        assert!(decoder.put(&tail).unwrap().is_empty());
        assert_eq!(decoder.flush().unwrap(), tail);
    }

    #[test]
    fn straw_decodes_framed_blocks_across_small_reads() {
        let input = b"LZW straw decoder data with repeated words";
        let encoded = compress_blocks(input, 8).unwrap();
        let mut straw = LzwStraw::new(SliceStraw::new(encoded), LzwMode::Decompress, 8).unwrap();
        let mut decoded = Vec::new();
        let mut chunk = [0_u8; 5];

        loop {
            let count = straw.get(&mut chunk).unwrap();
            if count == 0 {
                break;
            }
            decoded.extend_from_slice(&chunk[..count]);
        }

        assert_eq!(decoded, input);
    }

    #[test]
    fn straw_encodes_input_in_frames() {
        let input = b"LZW source bytes";
        let mut straw = LzwStraw::new(
            SliceStraw::new(input.to_vec()),
            LzwMode::Compress,
            5,
        )
        .unwrap();
        let mut encoded = Vec::new();
        let mut chunk = [0_u8; 7];

        loop {
            let count = straw.get(&mut chunk).unwrap();
            if count == 0 {
                break;
            }
            encoded.extend_from_slice(&chunk[..count]);
        }

        assert_eq!(decompress_blocks(&encoded, input.len()).unwrap(), input);
    }

    #[test]
    fn truncated_frame_reports_unexpected_end() {
        assert_eq!(
            decompress_blocks(&[0x03, 0x00, 0x01], 100),
            Err(LzwError::UnexpectedEndOfInput)
        );
    }
}
