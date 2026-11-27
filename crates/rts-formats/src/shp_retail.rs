// Filename: crates/rts-formats/src/shp_retail.rs
//! Retail Tiberian Dawn / Red Alert keyframe SHP parser and bounded decoder.
//! Kept separate from shp::ShpBlock, which parses the in-memory ShapeBlock_Type.

use crate::lcw;
use std::{collections::HashMap, fmt};

const HEADER: usize = 14;
const ENTRY: usize = 8;
const EXTRA: usize = 2;
const OFFSET_MASK: u32 = 0x00ff_ffff;
const MAX_FRAMES: usize = 8192;
const MAX_PIXELS: usize = 4 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RetailShpFormat { Lcw = 0x80, XorKeyframe = 0x40, XorPrevious = 0x20 }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetailShpHeader {
    pub frame_count: u16, pub x: u16, pub y: u16, pub width: u16,
    pub height: u16, pub largest_frame_size: u16, pub flags: u16,
}
impl RetailShpHeader {
    pub fn has_embedded_palette(self) -> bool { self.flags & 1 != 0 }
    pub fn pixel_count(self) -> usize { usize::from(self.width) * usize::from(self.height) }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetailShpFrame<'a> {
    pub data: &'a [u8], pub format: RetailShpFormat, pub file_offset: u32,
    pub reference_offset: u32, pub reference_format: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RetailShpError {
    Truncated { needed: usize, available: usize }, TooManyFrames(usize),
    FrameTooLarge(usize), InvalidFormat(u8), InvalidOffset { offset: usize, bound: usize },
    InvalidSentinel, FirstFrameMustBeKeyframe, MissingKeyframeReference(u32),
    InvalidDelta(&'static str), Lcw(lcw::LcwError),
}
impl fmt::Display for RetailShpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{self:?}") }
}
impl std::error::Error for RetailShpError {}
impl From<lcw::LcwError> for RetailShpError {
    fn from(value: lcw::LcwError) -> Self { Self::Lcw(value) }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetailShpFile<'a> {
    pub header: RetailShpHeader, pub embedded_palette: Option<&'a [u8]>,
    pub frames: Vec<RetailShpFrame<'a>>,
}
#[derive(Clone, Copy)]
struct OffsetEntry { offset: u32, format: u8, reference_offset: u32, reference_format: u8 }

impl<'a> RetailShpFile<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self, RetailShpError> {
        if bytes.len() < HEADER {
            return Err(RetailShpError::Truncated { needed: HEADER, available: bytes.len() });
        }
        let u16_at = |at: usize| u16::from_le_bytes([bytes[at], bytes[at + 1]]);
        let header = RetailShpHeader {
            frame_count: u16_at(0), x: u16_at(2), y: u16_at(4), width: u16_at(6),
            height: u16_at(8), largest_frame_size: u16_at(10), flags: u16_at(12),
        };
        let count = usize::from(header.frame_count);
        if count > MAX_FRAMES { return Err(RetailShpError::TooManyFrames(count)); }
        let pixels = header.pixel_count();
        if pixels > MAX_PIXELS { return Err(RetailShpError::FrameTooLarge(pixels)); }

        let entries_count = count.checked_add(EXTRA).ok_or(RetailShpError::TooManyFrames(count))?;
        let table_end = HEADER.checked_add(entries_count.checked_mul(ENTRY)
            .ok_or(RetailShpError::TooManyFrames(count))?)
            .ok_or(RetailShpError::TooManyFrames(count))?;
        if table_end > bytes.len() {
            return Err(RetailShpError::Truncated { needed: table_end, available: bytes.len() });
        }

        let mut entries = Vec::with_capacity(entries_count);
        for i in 0..entries_count {
            let at = HEADER + i * ENTRY;
            let packed = u32::from_le_bytes(bytes[at..at + 4].try_into().expect("fixed entry"));
            entries.push(OffsetEntry {
                offset: packed & OFFSET_MASK, format: (packed >> 24) as u8,
                reference_offset: { let raw = u32::from_le_bytes(bytes[at + 4..at + 8].try_into().expect("fixed reference")); raw & OFFSET_MASK },
                reference_format: (u32::from_le_bytes(bytes[at + 4..at + 8].try_into().expect("fixed reference")) >> 24) as u8,
            });
        }

        // Build_Frame adds 768 to encoded offsets when the palette flag is set.
        // The table offsets therefore address the data region before the embedded
        // palette adjustment; apply the same adjustment to frame and EOF offsets.
        let palette_bytes = if header.has_embedded_palette() { 768usize } else { 0 };
        let palette_end = table_end.checked_add(palette_bytes)
            .ok_or(RetailShpError::InvalidOffset { offset: usize::MAX, bound: bytes.len() })?;
        if palette_end > bytes.len() {
            return Err(RetailShpError::Truncated { needed: palette_end, available: bytes.len() });
        }
        let embedded_palette = header.has_embedded_palette().then(|| &bytes[table_end..palette_end]);

        let eof = entries[count];
        let eof_data_offset = (eof.offset as usize).checked_add(palette_bytes)
            .ok_or(RetailShpError::InvalidSentinel)?;
        if eof.format != 0 || (eof.offset as usize) < table_end || eof_data_offset > bytes.len() {
            return Err(RetailShpError::InvalidSentinel);
        }
        if matches!(entries[count + 1].format, 0x20 | 0x40 | 0x80) {
            return Err(RetailShpError::InvalidSentinel);
        }

        let mut frames = Vec::with_capacity(count);
        for i in 0..count {
            let entry = entries[i];
            let end = entries[i + 1].offset as usize;
            let format = match entry.format {
                0x80 => RetailShpFormat::Lcw, 0x40 => RetailShpFormat::XorKeyframe,
                0x20 => RetailShpFormat::XorPrevious,
                other => return Err(RetailShpError::InvalidFormat(other)),
            };
            let encoded_start = entry.offset as usize;
            let encoded_end = end;
            let start = encoded_start.checked_add(palette_bytes)
                .ok_or(RetailShpError::InvalidOffset { offset: usize::MAX, bound: bytes.len() })?;
            let end = encoded_end.checked_add(palette_bytes)
                .ok_or(RetailShpError::InvalidOffset { offset: usize::MAX, bound: bytes.len() })?;
            if encoded_start < table_end || encoded_end < encoded_start || start < palette_end || end > bytes.len() {
                return Err(RetailShpError::InvalidOffset { offset: start.max(end), bound: bytes.len() });
            }
            if i == 0 && format != RetailShpFormat::Lcw {
                return Err(RetailShpError::FirstFrameMustBeKeyframe);
            }
            frames.push(RetailShpFrame {
                data: &bytes[start..end], format, file_offset: entry.offset,
                reference_offset: entry.reference_offset, reference_format: entry.reference_format,
            });
        }
        Ok(Self { header, embedded_palette, frames })
    }

    pub fn decode_frames(&self) -> Result<Vec<Vec<u8>>, RetailShpError> {
        let size = self.header.pixel_count();
        if size == 0 { return Ok((0..self.frames.len()).map(|_| Vec::new()).collect()); }
        let mut decoded: Vec<Vec<u8>> = Vec::with_capacity(self.frames.len());
        let mut keyframes: HashMap<u32, usize> = HashMap::new();
        for frame in &self.frames {
            let pixels = match frame.format {
                RetailShpFormat::Lcw => {
                    let pixels = lcw::decompress_expected(frame.data, size)?;
                    keyframes.insert(frame.file_offset, decoded.len());
                    pixels
                }
                RetailShpFormat::XorKeyframe => {
                    let base = keyframes.get(&frame.reference_offset).copied()
                        .ok_or(RetailShpError::MissingKeyframeReference(frame.reference_offset))?;
                    let mut pixels = decoded.get(base).cloned()
                        .ok_or(RetailShpError::MissingKeyframeReference(frame.reference_offset))?;
                    apply_xor_delta(&mut pixels, frame.data)?;
                    pixels
                }
                RetailShpFormat::XorPrevious => {
                    // KF_DELTA stores the frame index of the XOR-keyframe at
                    // the beginning of this chain in the low 16 bits of its
                    // reference word. Rebuild from that base, as Build_Frame does.
                    if frame.reference_offset > u16::MAX as u32 {
                        return Err(RetailShpError::InvalidDelta("chain frame reference exceeds 16-bit frame index"));
                    }
                    let base_index = frame.reference_offset as usize;
                    if base_index >= decoded.len()
                        || self.frames.get(base_index).map(|base| base.format)
                            != Some(RetailShpFormat::XorKeyframe)
                    {
                        return Err(RetailShpError::InvalidDelta("chain must reference an earlier XOR-keyframe"));
                    }
                    let mut pixels = decoded.get(base_index).cloned()
                        .ok_or(RetailShpError::InvalidDelta("missing XOR chain base"))?;
                    for chain_index in (base_index + 1)..=decoded.len() {
                        let chain = &self.frames[chain_index];
                        if chain.format != RetailShpFormat::XorPrevious
                            || chain.reference_offset != frame.reference_offset
                        {
                            return Err(RetailShpError::InvalidDelta("broken XOR chain reference"));
                        }
                        apply_xor_delta(&mut pixels, chain.data)?;
                    }
                    pixels
                }
            };
            decoded.push(pixels);
        }
        Ok(decoded)
    }
}

/// Apply Westwood Format40 XOR commands. Input and output bounds are checked;
/// a 0x80,0x0000 long-skip terminator is required.
pub fn apply_xor_delta(dest: &mut [u8], delta: &[u8]) -> Result<(), RetailShpError> {
    let mut src = 0usize;
    let mut dst = 0usize;
    fn byte(input: &[u8], pos: &mut usize) -> Result<u8, RetailShpError> {
        let v = *input.get(*pos).ok_or(RetailShpError::InvalidDelta("truncated command"))?;
        *pos += 1; Ok(v)
    }
    fn word(input: &[u8], pos: &mut usize) -> Result<usize, RetailShpError> {
        let lo = usize::from(byte(input, pos)?);
        let hi = usize::from(byte(input, pos)?);
        Ok(lo | hi << 8)
    }
    loop {
        let command = byte(delta, &mut src)?;
        if command & 0x80 != 0 {
            if command != 0x80 {
                dst = dst.checked_add(usize::from(command & 0x7f))
                    .ok_or(RetailShpError::InvalidDelta("skip overflow"))?;
            } else {
                let raw = word(delta, &mut src)?;
                if raw == 0 { return Ok(()); }
                match raw >> 14 {
                    0 | 1 => dst = dst.checked_add(raw)
                        .ok_or(RetailShpError::InvalidDelta("long skip overflow"))?,
                    2 => {
                        let n = raw & 0x3fff;
                        let end_src = src.checked_add(n).ok_or(RetailShpError::InvalidDelta("source overflow"))?;
                        let end_dst = dst.checked_add(n).ok_or(RetailShpError::InvalidDelta("destination overflow"))?;
                        let values = delta.get(src..end_src).ok_or(RetailShpError::InvalidDelta("truncated long xor"))?;
                        let target = dest.get_mut(dst..end_dst).ok_or(RetailShpError::InvalidDelta("long xor exceeds frame"))?;
                        for (a, b) in target.iter_mut().zip(values) { *a ^= *b; }
                        src = end_src; dst = end_dst;
                    }
                    _ => {
                        let n = raw & 0x3fff;
                        let value = byte(delta, &mut src)?;
                        let end = dst.checked_add(n).ok_or(RetailShpError::InvalidDelta("destination overflow"))?;
                        let target = dest.get_mut(dst..end).ok_or(RetailShpError::InvalidDelta("repeated xor exceeds frame"))?;
                        for p in target { *p ^= value; }
                        dst = end;
                    }
                }
            }
        } else if command == 0 {
            let n = usize::from(byte(delta, &mut src)?);
            let value = byte(delta, &mut src)?;
            let end = dst.checked_add(n).ok_or(RetailShpError::InvalidDelta("destination overflow"))?;
            let target = dest.get_mut(dst..end).ok_or(RetailShpError::InvalidDelta("short repeated xor exceeds frame"))?;
            for p in target { *p ^= value; }
            dst = end;
        } else {
            let n = usize::from(command);
            let end_src = src.checked_add(n).ok_or(RetailShpError::InvalidDelta("source overflow"))?;
            let end_dst = dst.checked_add(n).ok_or(RetailShpError::InvalidDelta("destination overflow"))?;
            let values = delta.get(src..end_src).ok_or(RetailShpError::InvalidDelta("truncated short xor"))?;
            let target = dest.get_mut(dst..end_dst).ok_or(RetailShpError::InvalidDelta("short xor exceeds frame"))?;
            for (a, b) in target.iter_mut().zip(values) { *a ^= *b; }
            src = end_src; dst = end_dst;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn entry(out: &mut Vec<u8>, format: u8, offset: usize, reference: u32, reference_format: u8) {
        out.extend_from_slice(&((((format as u32) << 24) | offset as u32).to_le_bytes()));
        out.extend_from_slice(&((((reference_format as u32) << 24) | reference).to_le_bytes()));
    }
    fn fixture() -> Vec<u8> {
        let start = HEADER + 5 * ENTRY;
        let key = [0x83, 1, 2, 3, 0x80];
        let d1 = [1, 0xff, 0x80, 0, 0];
        let d2 = [0x82, 1, 0xf0, 0x80, 0, 0];
        let second = start + key.len();
        let third = second + d1.len();
        let eof = third + d2.len();
        let mut b = Vec::new();
        for v in [3u16, 0, 0, 3, 1, 5, 0] { b.extend_from_slice(&v.to_le_bytes()); }
        entry(&mut b, 0x80, start, 0, 0);
        entry(&mut b, 0x40, second, start as u32, 0x80);
        entry(&mut b, 0x20, third, 1, 0x48);
        entry(&mut b, 0, eof, 0, 0);
        entry(&mut b, 0, 0, 0, 0);
        b.extend_from_slice(&key); b.extend_from_slice(&d1); b.extend_from_slice(&d2);
        b
    }
    #[test]
    fn decodes_keyframe_xor_keyframe_and_chained_previous_frame() {
        let shp = RetailShpFile::parse(&fixture()).unwrap();
        assert_eq!(shp.decode_frames().unwrap(), vec![
            vec![1, 2, 3], vec![0xfe, 2, 3], vec![0xfe, 2, 0xf3],
        ]);
    }
    #[test]
    fn supports_format40_short_and_long_command_families() {
        let mut pixels = [0u8; 12];
        let delta = [2, 0xaa, 0xbb, 0x82, 0, 2, 0x0f, 0x80, 3, 0,
            0x80, 2, 0x80, 0x55, 0x66, 0x80, 1, 0xc0, 0x0f, 0x80, 0, 0];
        apply_xor_delta(&mut pixels, &delta).unwrap();
        assert_eq!(pixels, [0xaa, 0xbb, 0, 0, 0x0f, 0x0f, 0, 0, 0, 0x55, 0x66, 0x0f]);
    }
    #[test]
    fn rejects_truncated_and_out_of_bounds_delta_commands() {
        assert!(matches!(apply_xor_delta(&mut [0; 2], &[1]), Err(RetailShpError::InvalidDelta(_))));
        assert!(matches!(apply_xor_delta(&mut [0; 1], &[2, 1, 2, 0x80, 0, 0]), Err(RetailShpError::InvalidDelta(_))));
    }
    #[test]
    fn parses_embedded_palette_before_frame_data() {
        let table_end = HEADER + 3 * ENTRY;
        let start = table_end;
        let key = [0x81, 7, 0x80];
        let eof = start + key.len();
        let mut b = Vec::new();
        for v in [1u16, 0, 0, 1, 1, 1, 1] { b.extend_from_slice(&v.to_le_bytes()); }
        entry(&mut b, 0x80, start, 0, 0); entry(&mut b, 0, eof, 0, 0); entry(&mut b, 0, 0, 0, 0);
        b.extend_from_slice(&[0; 768]); b.extend_from_slice(&key);
        let shp = RetailShpFile::parse(&b).unwrap();
        assert_eq!(shp.embedded_palette.unwrap().len(), 768);
        assert_eq!(shp.decode_frames().unwrap(), vec![vec![7]]);
    }
}
