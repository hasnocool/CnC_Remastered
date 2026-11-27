// Filename: crates/rts-formats/src/shp_retail.rs
//! Bounded reader and decoder for retail Tiberian Dawn / Red Alert SHP files.
//!
//! This is deliberately separate from shp::ShpBlock, which parses the
//! in-memory ShapeBlock_Type used by the legacy drawing routines. The retail
//! file uses a 14-byte header, 8-byte frame records, and Format80/40/20 data.

use crate::lcw;
use std::{collections::HashMap, fmt};

const HEADER_SIZE: usize = 14;
const FRAME_RECORD_SIZE: usize = 8;
const TRAILER_RECORDS: usize = 2;
const MAX_FRAMES: usize = 8192;
const MAX_PIXELS: usize = 4 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RetailShpFormat { Lcw, XorReference, XorPrevious }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetailShpHeader {
    pub frame_count: u16,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub largest_frame_size: u16,
    /// Preserved verbatim. Retail TD/RA SHP readers do not treat this field
    /// as an embedded-palette flag; palette extraction belongs to keyframe
    /// animation containers and is implemented separately in keyframe.rs.
    pub flags: u16,
}
impl RetailShpHeader {
    pub fn pixel_count(self) -> usize { usize::from(self.width) * usize::from(self.height) }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetailShpFrame<'a> {
    /// Encoded bytes bounded by the next greater payload offset (or EOF).
    pub data: &'a [u8],
    pub format: RetailShpFormat,
    /// Absolute 24-bit file offset from the packed frame record.
    pub file_offset: u32,
    /// The retail frame record stores these as two separate little-endian
    /// u16 values, not as a packed 24-bit offset / 8-bit format word.
    pub reference_offset: u16,
    pub reference_format: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RetailShpError {
    Truncated { needed: usize, available: usize },
    ZeroFrameCount,
    TooManyFrames(usize),
    FrameTooLarge(usize),
    InvalidFormat { frame: usize, format: u8 },
    InvalidOffset { frame: usize, offset: usize, bound: usize },
    InvalidEofOffset { declared: usize, actual: usize },
    InvalidSentinel,
    FirstFrameMustBeKeyframe,
    InvalidFrameIndex { requested: usize, count: usize },
    MissingReference { frame: usize, offset: u16 },
    ReferenceCycle { frame: usize },
    MissingPreviousFrame { frame: usize },
    InvalidDelta(&'static str),
    Lcw(lcw::LcwError),
}
impl fmt::Display for RetailShpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{self:?}") }
}
impl std::error::Error for RetailShpError {}
impl From<lcw::LcwError> for RetailShpError {
    fn from(value: lcw::LcwError) -> Self { Self::Lcw(value) }
}

#[derive(Clone, Debug)]
pub struct RetailShpFile<'a> {
    pub header: RetailShpHeader,
    pub frames: Vec<RetailShpFrame<'a>>,
    frame_by_offset: HashMap<u32, usize>,
    bytes: &'a [u8],
    decoded_len: usize,
}

#[derive(Clone, Copy)]
struct OffsetEntry {
    offset: u32,
    format: u8,
    reference_offset: u16,
    reference_format: u16,
}

fn read_u16(bytes: &[u8], at: usize) -> Result<u16, RetailShpError> {
    let end = at.checked_add(2).ok_or(RetailShpError::Truncated {
        needed: usize::MAX, available: bytes.len(),
    })?;
    let b = bytes.get(at..end).ok_or(RetailShpError::Truncated {
        needed: end, available: bytes.len(),
    })?;
    Ok(u16::from_le_bytes([b[0], b[1]]))
}
fn read_u32(bytes: &[u8], at: usize) -> Result<u32, RetailShpError> {
    let end = at.checked_add(4).ok_or(RetailShpError::Truncated {
        needed: usize::MAX, available: bytes.len(),
    })?;
    let b = bytes.get(at..end).ok_or(RetailShpError::Truncated {
        needed: end, available: bytes.len(),
    })?;
    Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

impl<'a> RetailShpFile<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self, RetailShpError> {
        if bytes.len() < HEADER_SIZE {
            return Err(RetailShpError::Truncated { needed: HEADER_SIZE, available: bytes.len() });
        }
        let header = RetailShpHeader {
            frame_count: read_u16(bytes, 0)?,
            x: read_u16(bytes, 2)?,
            y: read_u16(bytes, 4)?,
            width: read_u16(bytes, 6)?,
            height: read_u16(bytes, 8)?,
            largest_frame_size: read_u16(bytes, 10)?,
            flags: read_u16(bytes, 12)?,
        };
        let count = usize::from(header.frame_count);
        if count == 0 { return Err(RetailShpError::ZeroFrameCount); }
        if count > MAX_FRAMES { return Err(RetailShpError::TooManyFrames(count)); }
        let decoded_len = header.pixel_count();
        if decoded_len == 0 || decoded_len > MAX_PIXELS {
            return Err(RetailShpError::FrameTooLarge(decoded_len));
        }

        // The table includes one EOF record and one zeroed look-ahead record.
        let record_count = count.checked_add(TRAILER_RECORDS)
            .ok_or(RetailShpError::TooManyFrames(count))?;
        let table_end = HEADER_SIZE.checked_add(record_count.checked_mul(FRAME_RECORD_SIZE)
            .ok_or(RetailShpError::TooManyFrames(count))?)
            .ok_or(RetailShpError::TooManyFrames(count))?;
        if table_end > bytes.len() {
            return Err(RetailShpError::Truncated { needed: table_end, available: bytes.len() });
        }

        let mut entries = Vec::with_capacity(record_count);
        for index in 0..record_count {
            let at = HEADER_SIZE + index * FRAME_RECORD_SIZE;
            let packed = read_u32(bytes, at)?;
            entries.push(OffsetEntry {
                offset: packed & 0x00ff_ffff,
                format: (packed >> 24) as u8,
                reference_offset: read_u16(bytes, at + 4)?,
                reference_format: read_u16(bytes, at + 6)?,
            });
        }

        let eof = entries[count];
        if eof.format != 0 || eof.offset as usize != bytes.len() {
            return Err(RetailShpError::InvalidEofOffset {
                declared: eof.offset as usize, actual: bytes.len(),
            });
        }
        if entries[count + 1].offset != 0 || entries[count + 1].format != 0
            || entries[count + 1].reference_offset != 0
            || entries[count + 1].reference_format != 0
        {
            return Err(RetailShpError::InvalidSentinel);
        }

        let mut offsets = Vec::with_capacity(count);
        for (index, entry) in entries[..count].iter().enumerate() {
            let format = match entry.format {
                0x80 => RetailShpFormat::Lcw,
                0x40 => RetailShpFormat::XorReference,
                0x20 => RetailShpFormat::XorPrevious,
                other => return Err(RetailShpError::InvalidFormat { frame: index, format: other }),
            };
            let offset = entry.offset as usize;
            if offset < table_end || offset >= bytes.len() {
                return Err(RetailShpError::InvalidOffset { frame: index, offset, bound: bytes.len() });
            }
            if index == 0 && format != RetailShpFormat::Lcw {
                return Err(RetailShpError::FirstFrameMustBeKeyframe);
            }
            offsets.push((offset, format));
        }

        // Payload offsets are normally ordered, but use the next greater
        // offset rather than assuming frame order equals physical file order.
        let mut sorted_offsets: Vec<usize> = offsets.iter().map(|(offset, _)| *offset).collect();
        sorted_offsets.push(bytes.len());
        sorted_offsets.sort_unstable();
        sorted_offsets.dedup();

        let mut frames = Vec::with_capacity(count);
        let mut frame_by_offset = HashMap::with_capacity(count);
        for (index, entry) in entries[..count].iter().enumerate() {
            let start = offsets[index].0;
            let end = sorted_offsets.iter().copied().find(|candidate| *candidate > start)
                .unwrap_or(bytes.len());
            if end <= start {
                return Err(RetailShpError::InvalidOffset { frame: index, offset: end, bound: bytes.len() });
            }
            frame_by_offset.insert(entry.offset, index);
            frames.push(RetailShpFrame {
                data: &bytes[start..end],
                format: offsets[index].1,
                file_offset: entry.offset,
                reference_offset: entry.reference_offset,
                reference_format: entry.reference_format,
            });
        }

        Ok(Self { header, frames, frame_by_offset, bytes, decoded_len })
    }

    pub fn frame_count(&self) -> usize { self.frames.len() }

    /// Decode one indexed frame into a full-canvas row-major palette-indexed
    /// image. Dependencies are resolved recursively with cycle detection.
    pub fn decode_frame(&self, index: usize) -> Result<Vec<u8>, RetailShpError> {
        if index >= self.frames.len() {
            return Err(RetailShpError::InvalidFrameIndex { requested: index, count: self.frames.len() });
        }
        let mut cache = vec![None; self.frames.len()];
        let mut visiting = vec![false; self.frames.len()];
        self.decode_inner(index, &mut cache, &mut visiting)?;
        Ok(cache[index].take().expect("requested frame decoded"))
    }

    pub fn decode_frames(&self) -> Result<Vec<Vec<u8>>, RetailShpError> {
        let mut cache = vec![None; self.frames.len()];
        let mut visiting = vec![false; self.frames.len()];
        for index in 0..self.frames.len() {
            self.decode_inner(index, &mut cache, &mut visiting)?;
        }
        Ok(cache.into_iter().map(|frame| frame.expect("all frames decoded")).collect())
    }

    fn decode_inner(
        &self,
        index: usize,
        cache: &mut [Option<Vec<u8>>],
        visiting: &mut [bool],
    ) -> Result<(), RetailShpError> {
        if cache[index].is_some() { return Ok(()); }
        if visiting[index] { return Err(RetailShpError::ReferenceCycle { frame: index }); }
        visiting[index] = true;
        let frame = self.frames[index];
        let decoded = match frame.format {
            RetailShpFormat::Lcw => lcw::decompress_expected(frame.data, self.decoded_len)?,
            RetailShpFormat::XorPrevious => {
                let base = index.checked_sub(1)
                    .ok_or(RetailShpError::MissingPreviousFrame { frame: index })?;
                self.decode_inner(base, cache, visiting)?;
                let mut pixels = cache[base].as_ref().expect("previous frame decoded").clone();
                apply_xor_delta(&mut pixels, frame.data)?;
                pixels
            }
            RetailShpFormat::XorReference => {
                let base = *self.frame_by_offset.get(&u32::from(frame.reference_offset))
                    .ok_or(RetailShpError::MissingReference {
                        frame: index, offset: frame.reference_offset,
                    })?;
                self.decode_inner(base, cache, visiting)?;
                let mut pixels = cache[base].as_ref().expect("referenced frame decoded").clone();
                apply_xor_delta(&mut pixels, frame.data)?;
                pixels
            }
        };
        if decoded.len() != self.decoded_len {
            return Err(RetailShpError::InvalidDelta("decoded frame size mismatch"));
        }
        cache[index] = Some(decoded);
        visiting[index] = false;
        Ok(())
    }

    /// Retain access to the original source bytes for diagnostics and tools.
    pub fn source_bytes(&self) -> &'a [u8] { self.bytes }
}

/// Apply Westwood Format40 XOR commands. Input and output bounds are checked;
/// a 0x80,0x0000 long-skip terminator is required.
pub fn apply_xor_delta(dest: &mut [u8], delta: &[u8]) -> Result<usize, RetailShpError> {
    let mut src = 0usize;
    let mut dst = 0usize;
    fn byte(input: &[u8], pos: &mut usize) -> Result<u8, RetailShpError> {
        let value = *input.get(*pos).ok_or(RetailShpError::InvalidDelta("truncated command"))?;
        *pos += 1;
        Ok(value)
    }
    fn word(input: &[u8], pos: &mut usize) -> Result<usize, RetailShpError> {
        let low = usize::from(byte(input, pos)?);
        let high = usize::from(byte(input, pos)?);
        Ok(low | (high << 8))
    }

    loop {
        let command = byte(delta, &mut src)?;
        if command == 0 {
            let count = usize::from(byte(delta, &mut src)?);
            let value = byte(delta, &mut src)?;
            let end = checked_end(dst, count, dest.len())?;
            for pixel in &mut dest[dst..end] { *pixel ^= value; }
            dst = end;
        } else if command < 0x80 {
            let count = usize::from(command);
            let source_end = src.checked_add(count)
                .ok_or(RetailShpError::InvalidDelta("source overflow"))?;
            let end = checked_end(dst, count, dest.len())?;
            let literals = delta.get(src..source_end)
                .ok_or(RetailShpError::InvalidDelta("truncated literal run"))?;
            for (pixel, value) in dest[dst..end].iter_mut().zip(literals) { *pixel ^= *value; }
            src = source_end;
            dst = end;
        } else if command != 0x80 {
            dst = checked_end(dst, usize::from(command - 0x80), dest.len())?;
        } else {
            let code = word(delta, &mut src)?;
            if code == 0 { return Ok(dst); }
            if code < 0x8000 {
                dst = checked_end(dst, code, dest.len())?;
            } else if code < 0xc000 {
                let count = code - 0x8000;
                let source_end = src.checked_add(count)
                    .ok_or(RetailShpError::InvalidDelta("long source overflow"))?;
                let end = checked_end(dst, count, dest.len())?;
                let literals = delta.get(src..source_end)
                    .ok_or(RetailShpError::InvalidDelta("truncated long literal run"))?;
                for (pixel, value) in dest[dst..end].iter_mut().zip(literals) { *pixel ^= *value; }
                src = source_end;
                dst = end;
            } else {
                let count = code - 0xc000;
                let value = byte(delta, &mut src)?;
                let end = checked_end(dst, count, dest.len())?;
                for pixel in &mut dest[dst..end] { *pixel ^= value; }
                dst = end;
            }
        }
    }
}

fn checked_end(start: usize, count: usize, limit: usize) -> Result<usize, RetailShpError> {
    let end = start.checked_add(count)
        .ok_or(RetailShpError::InvalidDelta("destination overflow"))?;
    if end > limit { return Err(RetailShpError::InvalidDelta("command exceeds frame buffer")); }
    Ok(end)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(out: &mut Vec<u8>, format: u8, offset: usize, reference: u16, reference_format: u16) {
        let packed = ((u32::from(format)) << 24) | offset as u32;
        out.extend_from_slice(&packed.to_le_bytes());
        out.extend_from_slice(&reference.to_le_bytes());
        out.extend_from_slice(&reference_format.to_le_bytes());
    }

    fn fixture() -> Vec<u8> {
        let start = HEADER_SIZE + 5 * FRAME_RECORD_SIZE;
        let key = [0x83, 1, 2, 3, 0x80];
        let delta40 = [1, 0xff, 0x80, 0, 0];
        let delta20 = [0x82, 1, 0xf0, 0x80, 0, 0];
        let second = start + key.len();
        let third = second + delta40.len();
        let eof = third + delta20.len();
        let mut bytes = Vec::new();
        for value in [3u16, 0, 0, 3, 1, 5, 0] { bytes.extend_from_slice(&value.to_le_bytes()); }
        record(&mut bytes, 0x80, start, 0, 0);
        record(&mut bytes, 0x40, second, start as u16, 0x80);
        record(&mut bytes, 0x20, third, 0, 0x40);
        record(&mut bytes, 0, eof, 0, 0);
        record(&mut bytes, 0, 0, 0, 0);
        bytes.extend_from_slice(&key);
        bytes.extend_from_slice(&delta40);
        bytes.extend_from_slice(&delta20);
        bytes
    }

    #[test]
    fn decodes_lcw_keyframe_format40_reference_and_format20_previous_frame() {
        let shp = RetailShpFile::parse(&fixture()).unwrap();
        assert_eq!(shp.decode_frames().unwrap(), vec![
            vec![1, 2, 3], vec![0xfe, 2, 3], vec![0xfe, 2, 0xf3],
        ]);
    }

    #[test]
    fn format40_can_reference_a_non_adjacent_frame() {
        let start = HEADER_SIZE + 5 * FRAME_RECORD_SIZE;
        let first = [0x82, 4, 5, 0x80];
        let second = [0x82, 1, 2, 0x80];
        let delta = [1, 0x01, 0x80, 0, 0];
        let a = start;
        let b = a + first.len();
        let c = b + second.len();
        let eof = c + delta.len();
        let mut bytes = Vec::new();
        for value in [3u16, 0, 0, 2, 1, 4, 0] { bytes.extend_from_slice(&value.to_le_bytes()); }
        record(&mut bytes, 0x80, a, 0, 0);
        record(&mut bytes, 0x80, b, 0, 0);
        record(&mut bytes, 0x40, c, a as u16, 0x80);
        record(&mut bytes, 0, eof, 0, 0);
        record(&mut bytes, 0, 0, 0, 0);
        bytes.extend_from_slice(&first);
        bytes.extend_from_slice(&second);
        bytes.extend_from_slice(&delta);
        let shp = RetailShpFile::parse(&bytes).unwrap();
        assert_eq!(shp.decode_frame(2).unwrap(), [5, 5]);
    }

    #[test]
    fn supports_short_and_long_xor_command_families() {
        let mut pixels = [0u8; 12];
        let delta = [
            2, 0xaa, 0xbb, 0x82, 0, 2, 0x0f,
            0x80, 3, 0, 0x80, 2, 0x80, 0x55, 0x66,
            0x80, 1, 0xc0, 0x0f, 0x80, 0, 0,
        ];
        apply_xor_delta(&mut pixels, &delta).unwrap();
        assert_eq!(pixels, [0xaa, 0xbb, 0, 0, 0x0f, 0x0f, 0, 0, 0, 0x55, 0x66, 0x0f]);
    }

    #[test]
    fn rejects_truncated_and_out_of_bounds_delta_commands() {
        assert!(matches!(apply_xor_delta(&mut [0; 2], &[1]), Err(RetailShpError::InvalidDelta(_))));
        assert!(matches!(apply_xor_delta(&mut [0; 1], &[2, 1, 2, 0x80, 0, 0]), Err(RetailShpError::InvalidDelta(_))));
    }

    #[test]
    fn rejects_bad_eof_unknown_format_and_zero_frame_count() {
        assert!(matches!(RetailShpFile::parse(&[0; 14]), Err(RetailShpError::ZeroFrameCount)));
        let mut bytes = fixture();
        bytes[14 + 7] = 0x55;
        assert!(matches!(RetailShpFile::parse(&bytes), Err(RetailShpError::InvalidFormat { .. })));
        let mut bytes = fixture();
        let eof_record = HEADER_SIZE + 3 * FRAME_RECORD_SIZE;
        bytes[eof_record..eof_record + 4].copy_from_slice(&0u32.to_le_bytes());
        assert!(matches!(RetailShpFile::parse(&bytes), Err(RetailShpError::InvalidEofOffset { .. })));
    }
}
