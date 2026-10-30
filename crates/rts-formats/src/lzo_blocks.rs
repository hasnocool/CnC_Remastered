//! Four-byte framed LZO block streams ported from REDALERT/LZOPIPE.CPP
//! and REDALERT/LZOSTRAW.CPP.

use crate::lzo1x::{compress, decompress_expected, LzoError};
use crate::pipe::PipeSink;
use crate::straw::StrawSource;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LzoBlockHeader {
    pub compressed_count: u16,
    pub uncompressed_count: u16,
}

impl LzoBlockHeader {
    pub const SIZE: usize = 4;

    pub fn read(bytes: &[u8]) -> Result<Self, LzoError> {
        if bytes.len() < Self::SIZE {
            return Err(LzoError::UnexpectedEndOfInput);
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

fn validate_block_size(block_size: usize) -> Result<(), LzoError> {
    if block_size == 0 || block_size > usize::from(u16::MAX) {
        return Err(LzoError::InvalidBlockSize(block_size));
    }
    Ok(())
}

fn encode_frame(input: &[u8]) -> Result<Vec<u8>, LzoError> {
    if input.len() > usize::from(u16::MAX) {
        return Err(LzoError::LengthOverflow);
    }
    let compressed = compress(input)?;
    if compressed.len() > usize::from(u16::MAX) {
        return Err(LzoError::LengthOverflow);
    }

    let mut frame = Vec::with_capacity(LzoBlockHeader::SIZE + compressed.len());
    LzoBlockHeader {
        compressed_count: compressed.len() as u16,
        uncompressed_count: input.len() as u16,
    }
    .write(&mut frame);
    frame.extend_from_slice(&compressed);
    Ok(frame)
}

pub fn compress_blocks(input: &[u8], block_size: usize) -> Result<Vec<u8>, LzoError> {
    validate_block_size(block_size)?;
    let mut output = Vec::new();
    for block in input.chunks(block_size) {
        output.extend_from_slice(&encode_frame(block)?);
    }
    Ok(output)
}

pub fn decompress_blocks(input: &[u8], output_limit: usize) -> Result<Vec<u8>, LzoError> {
    let mut cursor = 0_usize;
    let mut output = Vec::new();

    while cursor < input.len() {
        let header = LzoBlockHeader::read(&input[cursor..])?;
        cursor += LzoBlockHeader::SIZE;

        let compressed_count = usize::from(header.compressed_count);
        let uncompressed_count = usize::from(header.uncompressed_count);
        if compressed_count == 0 {
            return Err(LzoError::InvalidBlockHeader);
        }

        let end = cursor
            .checked_add(compressed_count)
            .ok_or(LzoError::LengthOverflow)?;
        let compressed = input
            .get(cursor..end)
            .ok_or(LzoError::UnexpectedEndOfInput)?;
        cursor = end;

        let decoded = decompress_expected(compressed, uncompressed_count)?;
        let requested = output
            .len()
            .checked_add(decoded.len())
            .ok_or(LzoError::LengthOverflow)?;
        if requested > output_limit {
            return Err(LzoError::OutputLimitExceeded {
                requested,
                limit: output_limit,
            });
        }
        output.extend_from_slice(&decoded);
    }

    Ok(output)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LzoMode {
    Compress,
    Decompress,
}

/// Push adapter equivalent to LZOPipe.
#[derive(Debug, Clone)]
pub struct LzoPipe {
    mode: LzoMode,
    block_size: usize,
    raw: Vec<u8>,
    header: Option<LzoBlockHeader>,
    compressed: Vec<u8>,
}

impl LzoPipe {
    pub fn new(mode: LzoMode, block_size: usize) -> Result<Self, LzoError> {
        validate_block_size(block_size)?;
        Ok(Self {
            mode,
            block_size,
            raw: Vec::new(),
            header: None,
            compressed: Vec::new(),
        })
    }

    pub fn pending_len(&self) -> usize {
        self.raw.len() + self.compressed.len()
            + self.header.map_or(0, |_| LzoBlockHeader::SIZE)
    }

    pub fn put(&mut self, source: &[u8]) -> Result<Vec<u8>, LzoError> {
        match self.mode {
            LzoMode::Compress => {
                self.raw.extend_from_slice(source);
                let mut output = Vec::new();
                while self.raw.len() >= self.block_size {
                    let block: Vec<u8> = self.raw.drain(..self.block_size).collect();
                    output.extend_from_slice(&encode_frame(&block)?);
                }
                Ok(output)
            }
            LzoMode::Decompress => {
                self.raw.extend_from_slice(source);
                let mut output = Vec::new();

                loop {
                    if self.header.is_none() {
                        if self.raw.len() < LzoBlockHeader::SIZE {
                            break;
                        }
                        let header = LzoBlockHeader::read(&self.raw)?;
                        if header.compressed_count == 0 || header.compressed_count == u16::MAX {
                            return Err(LzoError::InvalidBlockHeader);
                        }
                        self.raw.drain(..LzoBlockHeader::SIZE);
                        self.header = Some(header);
                    }

                    let header = self.header.expect("header was checked above");
                    let required = usize::from(header.compressed_count);
                    if self.raw.len() < required {
                        break;
                    }

                    let block: Vec<u8> = self.raw.drain(..required).collect();
                    output.extend_from_slice(&decompress_expected(
                        &block,
                        usize::from(header.uncompressed_count),
                    )?);
                    self.header = None;
                }

                Ok(output)
            }
        }
    }

    pub fn flush(&mut self) -> Result<Vec<u8>, LzoError> {
        match self.mode {
            LzoMode::Compress => {
                if self.raw.is_empty() {
                    return Ok(Vec::new());
                }
                encode_frame(&core::mem::take(&mut self.raw))
            }
            LzoMode::Decompress => {
                let mut output = Vec::new();

                if let Some(header) = self.header.take() {
                    // Source LZOPipe flushes an incomplete compressed block
                    // as its original header followed by the incomplete body.
                    header.write(&mut output);
                    output.extend_from_slice(&self.raw);
                    self.raw.clear();
                } else {
                    // A truncated header is passed through as raw data.
                    output = core::mem::take(&mut self.raw);
                }

                Ok(output)
            }
        }
    }

    pub fn put_to_sink(
        &mut self,
        source: &[u8],
        sink: &mut impl PipeSink,
    ) -> Result<usize, LzoError> {
        let output = self.put(source)?;
        Ok(sink.put(&output))
    }

    pub fn flush_to_sink(&mut self, sink: &mut impl PipeSink) -> Result<usize, LzoError> {
        let output = self.flush()?;
        let mut total = sink.put(&output);
        total += sink.flush();
        Ok(total)
    }
}

/// Pull adapter equivalent to LZOStraw.
pub struct LzoStraw<S> {
    source: S,
    mode: LzoMode,
    block_size: usize,
    pending: Vec<u8>,
    pending_offset: usize,
    done: bool,
}

impl<S: StrawSource> LzoStraw<S> {
    pub fn new(source: S, mode: LzoMode, block_size: usize) -> Result<Self, LzoError> {
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

    fn fill_pending(&mut self) -> Result<bool, LzoError> {
        self.pending.clear();
        self.pending_offset = 0;
        if self.done {
            return Ok(false);
        }

        match self.mode {
            LzoMode::Compress => {
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
            LzoMode::Decompress => {
                let mut header_bytes = [0_u8; LzoBlockHeader::SIZE];
                if !self.read_exact_source(&mut header_bytes) {
                    self.done = true;
                    return Ok(false);
                }

                let header = LzoBlockHeader::read(&header_bytes)?;
                let compressed_count = usize::from(header.compressed_count);
                let uncompressed_count = usize::from(header.uncompressed_count);
                if compressed_count == 0 || compressed_count > self.block_size * 2 + 1024 {
                    return Err(LzoError::InvalidBlockHeader);
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

    pub fn get(&mut self, destination: &mut [u8]) -> Result<usize, LzoError> {
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
    fn framed_blocks_round_trip() {
        let input = b"LZO framed stream: Westwood Westwood Westwood repeated";
        let encoded = compress_blocks(input, 13).unwrap();
        assert_eq!(decompress_blocks(&encoded, input.len()).unwrap(), input);
    }

    #[test]
    fn pipe_compression_buffers_and_flushes_partial_block() {
        let input = b"0123456789abcdef";
        let mut encoder = LzoPipe::new(LzoMode::Compress, 6).unwrap();
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&encoder.put(&input[..3]).unwrap());
        encoded.extend_from_slice(&encoder.put(&input[3..10]).unwrap());
        encoded.extend_from_slice(&encoder.put(&input[10..]).unwrap());
        encoded.extend_from_slice(&encoder.flush().unwrap());
        assert_eq!(decompress_blocks(&encoded, input.len()).unwrap(), input);
    }

    #[test]
    fn pipe_decodes_fragmented_header_and_body() {
        let input = b"LZO fragmented decoding with repeated words";
        let encoded = compress_blocks(input, 9).unwrap();
        let mut decoder = LzoPipe::new(LzoMode::Decompress, 9).unwrap();
        let mut decoded = Vec::new();
        for part in encoded.chunks(2) {
            decoded.extend_from_slice(&decoder.put(part).unwrap());
        }
        assert!(decoder.flush().unwrap().is_empty());
        assert_eq!(decoded, input);
    }

    #[test]
    fn incomplete_decompression_frame_is_passed_through_on_flush() {
        let mut decoder = LzoPipe::new(LzoMode::Decompress, 64).unwrap();
        let tail = [0x04, 0x00, 0x03, 0x00, 0x11];
        assert!(decoder.put(&tail).unwrap().is_empty());
        assert_eq!(decoder.flush().unwrap(), tail);
    }

    #[test]
    fn straw_decompresses_blocks_across_small_reads() {
        let input = b"LZO straw: a stream of bytes over multiple blocks";
        let encoded = compress_blocks(input, 8).unwrap();
        let mut straw = LzoStraw::new(
            SliceStraw::new(encoded),
            LzoMode::Decompress,
            8,
        ).unwrap();

        let mut decoded = Vec::new();
        let mut chunk = [0_u8; 4];
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
    fn straw_compresses_source_into_frames() {
        let input = b"LZO compression through a pull chain";
        let mut straw = LzoStraw::new(
            SliceStraw::new(input.to_vec()),
            LzoMode::Compress,
            7,
        ).unwrap();

        let mut encoded = Vec::new();
        let mut chunk = [0_u8; 5];
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
    fn invalid_block_lengths_are_rejected() {
        assert_eq!(
            decompress_blocks(&[0x04, 0x00, 0x03, 0x00, 0x11], 64),
            Err(LzoError::UnexpectedEndOfInput)
        );
    }
}
