//! LCW compressor and framed block-stream ports for REDALERT/LCWCOMP.ASM,
//! LCWPIPE.CPP, and LCWSTRAW.CPP.
//!
//! LCWPIPE and LCWSTRAW use a four-byte little-endian block header:
//! compressed byte count followed by uncompressed byte count.

use crate::lcw::{decompress_expected, LcwError};
use crate::pipe::PipeSink;
use crate::straw::StrawSource;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LcwMode {
    Compress,
    Decompress,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LcwBlockHeader {
    pub compressed_count: u16,
    pub uncompressed_count: u16,
}

impl LcwBlockHeader {
    pub const SIZE: usize = 4;

    pub fn read(bytes: &[u8]) -> Result<Self, LcwError> {
        if bytes.len() < Self::SIZE {
            return Err(LcwError::UnexpectedEndOfInput);
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

fn source_size_error(size: usize) -> LcwError {
    LcwError::LengthOutOfRange { size }
}

/// Encode one uncompressed block using the same LCW command families as the
/// Westwood 32-bit assembly compressor.
///
/// The encoder favors longest matches, uses 0xFE for byte runs of 65+ bytes,
/// short relative references for lengths 3..10 within 4095 bytes, medium
/// absolute references for lengths 3..64, long absolute references for longer
/// matches, and 0x10..0x3F literal-length runs. It always ends with 0x80.
pub fn compress(input: &[u8]) -> Result<Vec<u8>, LcwError> {
    if input.len() > usize::from(u16::MAX) {
        return Err(source_size_error(input.len()));
    }

    if input.is_empty() {
        return Ok(vec![0x80]);
    }

    let mut output = Vec::with_capacity(input.len() + (input.len() / 63) + 8);
    let mut literal_header = 0_usize;
    let mut in_literal = true;

    // The original assembly always begins with a one-byte literal command.
    output.push(0x81);
    output.push(input[0]);

    let mut position = 1_usize;
    while position < input.len() {
        let current = input[position];

        // The original encoder checks for a 65-byte run before searching for
        // a back-reference, then emits the complete run as a 0xFE command.
        let mut run_count = 1_usize;
        while position + run_count < input.len()
            && input[position + run_count] == current
        {
            run_count += 1;
        }

        if run_count >= 65 {
            output.push(0xFE);
            output.extend_from_slice(&(run_count as u16).to_le_bytes());
            output.push(current);
            position += run_count;
            in_literal = false;
            continue;
        }

        // Scan previous occurrences in ascending order. On equal-length
        // matches the original assembly overwrites the prior candidate, so
        // the last (nearest) equal-length match wins.
        let mut best_count = 1_usize;
        let mut best_offset = 0_usize;

        for candidate in 0..position {
            if input[candidate] != current {
                continue;
            }

            let mut count = 0_usize;
            while position + count < input.len()
                && candidate + count < input.len()
                && input[candidate + count] == input[position + count]
            {
                count += 1;
            }

            if count >= best_count {
                best_count = count;
                best_offset = candidate;
            }

            if count == input.len() - position {
                break;
            }
        }

        if best_count >= 3 {
            let distance = position - best_offset;

            if best_count <= 10 && distance <= 0x0FFF {
                // 0xxx yyyy, yyyyyyyy: short relative copy.
                let opcode = (((best_count - 3) as u8) << 4)
                    | (((distance >> 8) as u8) & 0x0F);
                output.push(opcode);
                output.push(distance as u8);
            } else if best_count <= 64 {
                // 11xxxxxx, offset16: medium absolute copy.
                output.push(0xC0 | ((best_count - 3) as u8));
                output.extend_from_slice(&(best_offset as u16).to_le_bytes());
            } else {
                // 0xFF, length16, offset16: long absolute copy.
                output.push(0xFF);
                output.extend_from_slice(&(best_count as u16).to_le_bytes());
                output.extend_from_slice(&(best_offset as u16).to_le_bytes());
            }

            position += best_count;
            in_literal = false;
            continue;
        }

        // Literal byte. Literal command headers contain the number of
        // following bytes and are capped at 63 (0xBF).
        if !in_literal {
            output.push(0x80);
            literal_header = output.len() - 1;
            in_literal = true;
        }
        if output[literal_header] == 0xBF {
            output.push(0x80);
            literal_header = output.len() - 1;
        }
        output[literal_header] += 1;
        output.push(current);
        position += 1;
    }

    output.push(0x80);
    Ok(output)
}

/// Compress each input chunk into a framed LCW block.
pub fn compress_blocks(input: &[u8], block_size: usize) -> Result<Vec<u8>, LcwError> {
    if block_size == 0 || block_size > usize::from(u16::MAX) {
        return Err(LcwError::InvalidBlockSize(block_size));
    }

    let mut output = Vec::new();
    for chunk in input.chunks(block_size) {
        let compressed = compress(chunk)?;
        if compressed.len() > usize::from(u16::MAX) {
            return Err(source_size_error(compressed.len()));
        }
        LcwBlockHeader {
            compressed_count: compressed.len() as u16,
            uncompressed_count: chunk.len() as u16,
        }
        .write(&mut output);
        output.extend_from_slice(&compressed);
    }
    Ok(output)
}

/// Decompress a complete sequence of framed LCW blocks.
pub fn decompress_blocks(
    input: &[u8],
    output_limit: usize,
) -> Result<Vec<u8>, LcwError> {
    let mut cursor = 0_usize;
    let mut output = Vec::new();

    while cursor < input.len() {
        let header = LcwBlockHeader::read(&input[cursor..])?;
        cursor += LcwBlockHeader::SIZE;

        let compressed_count = usize::from(header.compressed_count);
        let uncompressed_count = usize::from(header.uncompressed_count);
        let end = cursor
            .checked_add(compressed_count)
            .ok_or(LcwError::UnexpectedEndOfInput)?;
        let compressed = input
            .get(cursor..end)
            .ok_or(LcwError::UnexpectedEndOfInput)?;
        cursor = end;

        let block = decompress_expected(compressed, uncompressed_count)?;
        let requested = output
            .len()
            .checked_add(block.len())
            .unwrap_or(usize::MAX);
        if requested > output_limit {
            return Err(LcwError::OutputLimitExceeded {
                requested,
                limit: output_limit,
            });
        }
        output.extend_from_slice(&block);
    }

    Ok(output)
}

/// Push-oriented framed LCW transformer. Compression buffers until full blocks
/// are available; decompression accepts arbitrary input fragments and emits
/// only complete decoded blocks. Flush emits a final compressed partial block
/// or passes through a truncated decompression tail, matching LCWPipe::Flush.
#[derive(Debug, Clone)]
pub struct LcwPipe {
    mode: LcwMode,
    block_size: usize,
    staged: Vec<u8>,
}

impl LcwPipe {
    pub fn new(mode: LcwMode, block_size: usize) -> Result<Self, LcwError> {
        if block_size == 0 || block_size > usize::from(u16::MAX) {
            return Err(LcwError::InvalidBlockSize(block_size));
        }
        Ok(Self {
            mode,
            block_size,
            staged: Vec::new(),
        })
    }

    pub fn mode(&self) -> LcwMode {
        self.mode
    }

    pub fn pending_len(&self) -> usize {
        self.staged.len()
    }

    pub fn put(&mut self, source: &[u8]) -> Result<Vec<u8>, LcwError> {
        match self.mode {
            LcwMode::Compress => {
                self.staged.extend_from_slice(source);
                let mut output = Vec::new();

                while self.staged.len() >= self.block_size {
                    let block: Vec<u8> = self.staged.drain(..self.block_size).collect();
                    output.extend_from_slice(&compress_blocks(&block, self.block_size)?);
                }
                Ok(output)
            }
            LcwMode::Decompress => {
                self.staged.extend_from_slice(source);
                let mut output = Vec::new();

                loop {
                    if self.staged.len() < LcwBlockHeader::SIZE {
                        break;
                    }

                    let header = LcwBlockHeader::read(&self.staged)?;
                    let compressed_count = usize::from(header.compressed_count);
                    let uncompressed_count = usize::from(header.uncompressed_count);
                    if compressed_count == 0 {
                        return Err(LcwError::InvalidBlockHeader);
                    }

                    let frame_size = LcwBlockHeader::SIZE
                        .checked_add(compressed_count)
                        .ok_or(LcwError::UnexpectedEndOfInput)?;
                    if self.staged.len() < frame_size {
                        break;
                    }

                    let block = decompress_expected(
                        &self.staged[LcwBlockHeader::SIZE..frame_size],
                        uncompressed_count,
                    )?;
                    output.extend_from_slice(&block);
                    self.staged.drain(..frame_size);
                }

                Ok(output)
            }
        }
    }

    pub fn flush(&mut self) -> Result<Vec<u8>, LcwError> {
        match self.mode {
            LcwMode::Compress => {
                if self.staged.is_empty() {
                    return Ok(Vec::new());
                }
                let block = core::mem::take(&mut self.staged);
                compress_blocks(&block, self.block_size)
            }
            LcwMode::Decompress => {
                if self.staged.len() < LcwBlockHeader::SIZE {
                    return Ok(core::mem::take(&mut self.staged));
                }

                let header = LcwBlockHeader::read(&self.staged)?;
                // LCWPipe::Flush only emits the header plus data when there is
                // an incomplete compressed body. A complete header with no
                // body bytes leaves Counter==0 and is discarded.
                if self.staged.len() == LcwBlockHeader::SIZE {
                    self.staged.clear();
                    return Ok(Vec::new());
                }
                let _ = header;
                Ok(core::mem::take(&mut self.staged))
            }
        }
    }
}

/// Pull-oriented framed LCW adapter corresponding to LCWStraw.
///
/// A single source read may be shorter than requested; this adapter repeatedly
/// reads until the requested header/body is complete or the source returns EOF.
pub struct LcwStraw<S> {
    source: S,
    mode: LcwMode,
    block_size: usize,
    pending: Vec<u8>,
    pending_offset: usize,
    done: bool,
}

impl<S: StrawSource> LcwStraw<S> {
    pub fn new(source: S, mode: LcwMode, block_size: usize) -> Result<Self, LcwError> {
        if block_size == 0 || block_size > usize::from(u16::MAX) {
            return Err(LcwError::InvalidBlockSize(block_size));
        }
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
            let count = self.source.get(&mut destination[offset..]);
            if count == 0 {
                return false;
            }
            offset += count.min(destination.len() - offset);
        }
        true
    }

    fn fill_pending(&mut self) -> Result<bool, LcwError> {
        self.pending.clear();
        self.pending_offset = 0;
        if self.done {
            return Ok(false);
        }

        match self.mode {
            LcwMode::Compress => {
                let mut raw = vec![0_u8; self.block_size];
                let count = self.source.get(&mut raw).min(self.block_size);
                if count == 0 {
                    self.done = true;
                    return Ok(false);
                }
                raw.truncate(count);
                self.pending = compress_blocks(&raw, self.block_size)?;
                Ok(true)
            }
            LcwMode::Decompress => {
                let mut header_bytes = [0_u8; LcwBlockHeader::SIZE];
                if !self.read_exact_source(&mut header_bytes) {
                    self.done = true;
                    return Ok(false);
                }
                let header = LcwBlockHeader::read(&header_bytes)?;
                let compressed_count = usize::from(header.compressed_count);
                let uncompressed_count = usize::from(header.uncompressed_count);
                if compressed_count == 0 {
                    return Err(LcwError::InvalidBlockHeader);
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

    pub fn get(&mut self, destination: &mut [u8]) -> Result<usize, LcwError> {
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
            self.pending_offset += count;
            total += count;
        }
        Ok(total)
    }
}

impl LcwPipe {
    /// Forward a completed transformed payload to a downstream sink.
    pub fn put_to_sink(
        &mut self,
        source: &[u8],
        sink: &mut impl PipeSink,
    ) -> Result<usize, LcwError> {
        let output = self.put(source)?;
        Ok(sink.put(&output))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::straw::SliceStraw;

    #[test]
    fn compressor_starts_with_a_literal_and_ends_with_marker() {
        assert_eq!(compress(b"abc").unwrap(), [0x83, b'a', b'b', b'c', 0x80]);
        assert_eq!(compress(b"").unwrap(), [0x80]);
    }

    #[test]
    fn compressor_uses_short_and_medium_back_references() {
        let short = compress(b"ABCAABC").unwrap();
        assert_eq!(decompress_expected(&short, 7).unwrap(), b"ABCAABC");

        let medium_input: Vec<u8> = (0..40).map(|index| (index * 37) as u8).collect();
        let mut medium_repeated = medium_input.clone();
        medium_repeated.extend_from_slice(&medium_input[..20]);
        let encoded = compress(&medium_repeated).unwrap();
        assert_eq!(decompress_expected(&encoded, medium_repeated.len()).unwrap(), medium_repeated);
    }

    #[test]
    fn compressor_uses_long_repeat_run() {
        let input = vec![b'Q'; 300];
        let encoded = compress(&input).unwrap();
        assert!(encoded.contains(&0xFE));
        assert_eq!(decompress_expected(&encoded, input.len()).unwrap(), input);
    }

    #[test]
    fn compression_blocks_round_trip() {
        let input = b"westwood LCW block stream: repeated repeated repeated";
        let framed = compress_blocks(input, 13).unwrap();
        assert_eq!(decompress_blocks(&framed, input.len()).unwrap(), input);
    }

    #[test]
    fn pipe_accepts_fragmented_input_and_flushes_partial_compression_block() {
        let input = b"0123456789abcdef";
        let mut encoder = LcwPipe::new(LcwMode::Compress, 6).unwrap();
        let mut framed = Vec::new();
        framed.extend_from_slice(&encoder.put(&input[..3]).unwrap());
        framed.extend_from_slice(&encoder.put(&input[3..11]).unwrap());
        framed.extend_from_slice(&encoder.put(&input[11..]).unwrap());
        framed.extend_from_slice(&encoder.flush().unwrap());
        assert_eq!(decompress_blocks(&framed, input.len()).unwrap(), input);
    }

    #[test]
    fn pipe_accepts_fragmented_framed_input() {
        let input = b"small block boundary";
        let framed = compress_blocks(input, 5).unwrap();
        let mut decoder = LcwPipe::new(LcwMode::Decompress, 5).unwrap();
        let mut output = Vec::new();
        for chunk in framed.chunks(2) {
            output.extend_from_slice(&decoder.put(chunk).unwrap());
        }
        assert!(decoder.flush().unwrap().is_empty());
        assert_eq!(output, input);
    }

    #[test]
    fn pipe_flush_passes_through_incomplete_decompression_tail() {
        let mut decoder = LcwPipe::new(LcwMode::Decompress, 32).unwrap();
        let incomplete_header = [0x02, 0x00];
        assert!(decoder.put(&incomplete_header).unwrap().is_empty());
        assert_eq!(decoder.flush().unwrap(), incomplete_header);
    }

    #[test]
    fn straw_decodes_framed_blocks_across_small_reads() {
        let input = b"one two three four five";
        let framed = compress_blocks(input, 7).unwrap();
        let source = SliceStraw::new(framed);
        let mut straw = LcwStraw::new(source, LcwMode::Decompress, 7).unwrap();
        let mut output = Vec::new();
        let mut chunk = [0_u8; 3];
        loop {
            let count = straw.get(&mut chunk).unwrap();
            if count == 0 {
                break;
            }
            output.extend_from_slice(&chunk[..count]);
        }
        assert_eq!(output, input);
    }

    #[test]
    fn straw_compresses_source_in_blocks() {
        let input = b"straw compression data";
        let source = SliceStraw::new(input.to_vec());
        let mut straw = LcwStraw::new(source, LcwMode::Compress, 6).unwrap();
        let mut framed = Vec::new();
        let mut chunk = [0_u8; 5];
        loop {
            let count = straw.get(&mut chunk).unwrap();
            if count == 0 {
                break;
            }
            framed.extend_from_slice(&chunk[..count]);
        }
        assert_eq!(decompress_blocks(&framed, input.len()).unwrap(), input);
    }
}
