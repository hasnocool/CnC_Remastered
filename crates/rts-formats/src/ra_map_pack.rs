//! Red Alert MapPack section decoder.
//!
//! Upstream references:
//! - electronicarts/CnC_Red_Alert: CODE/MAP.CPP, MapClass::Read_Binary and
//!   MapClass::Write_Binary (new-format maps store all u16 template IDs, then
//!   all u8 logical icon IDs).
//! - electronicarts/CnC_Remastered_Collection:
//!   CnCTDRAMapEditor/RedAlert/GamePlugin.cs, DecompressLCWSection (MapPack
//!   chunks carry little-endian u16 compressed and decompressed lengths).
//!
//! `RaTemplateMap::decode_ini_section` accepts the concatenated INI Base64
//! value directly; `decode_lcw_chunks` is also exposed for already-decoded
//! framed bytes. Both paths validate the RA LCW framing and two-plane payload.

use crate::lcw::{self, LcwError};
use core::fmt;

/// Maximum decoded size of one LCW MapPack chunk, matching the upstream editor.
pub const RA_MAPPACK_CHUNK_LIMIT: usize = 8192;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RaMapPackError {
    InvalidDimensions { width: u32, height: u32 },
    SizeOverflow,
    AllocationFailed { bytes: usize },
    TruncatedChunkHeader { offset: usize },
    InvalidChunkLengths { offset: usize, compressed: usize, decompressed: usize },
    TruncatedChunkData { offset: usize, expected: usize, remaining: usize },
    ChunkOutputTooLarge { offset: usize, requested: usize, remaining: usize },
    Lcw { offset: usize, source: LcwError },
    OutputSizeMismatch { actual: usize, expected: usize },
    TrailingIncompleteData { offset: usize, remaining: usize },
    InvalidBase64 { offset: usize, byte: u8 },
    InvalidBase64Length { remaining: usize },
    InvalidBase64Padding { offset: usize },
    Base64OutputTooLarge { requested: usize, limit: usize },
}

impl fmt::Display for RaMapPackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDimensions { width, height } => write!(f, "invalid RA MapPack dimensions {width}x{height}"),
            Self::SizeOverflow => f.write_str("RA MapPack size arithmetic overflow"),
            Self::AllocationFailed { bytes } => write!(f, "could not reserve {bytes} bytes for RA MapPack"),
            Self::TruncatedChunkHeader { offset } => write!(f, "RA MapPack chunk header is truncated at byte {offset}"),
            Self::InvalidChunkLengths { offset, compressed, decompressed } => write!(f, "invalid RA MapPack chunk lengths at byte {offset}: compressed={compressed}, decompressed={decompressed}"),
            Self::TruncatedChunkData { offset, expected, remaining } => write!(f, "RA MapPack chunk at byte {offset} needs {expected} compressed bytes, only {remaining} remain"),
            Self::ChunkOutputTooLarge { offset, requested, remaining } => write!(f, "RA MapPack chunk at byte {offset} expands to {requested} bytes with only {remaining} output bytes remaining"),
            Self::Lcw { offset, source } => write!(f, "RA MapPack LCW chunk at byte {offset} failed: {source:?}"),
            Self::OutputSizeMismatch { actual, expected } => write!(f, "RA MapPack decoded to {actual} bytes, expected exactly {expected}"),
            Self::TrailingIncompleteData { offset, remaining } => write!(f, "RA MapPack has {remaining} trailing bytes at byte {offset}"),
            Self::InvalidBase64 { offset, byte } => write!(f, "invalid Base64 byte {byte:#04x} at offset {offset}"),
            Self::InvalidBase64Length { remaining } => write!(f, "RA MapPack Base64 ends with an incomplete {remaining}-byte quartet"),
            Self::InvalidBase64Padding { offset } => write!(f, "invalid RA MapPack Base64 padding at offset {offset}"),
            Self::Base64OutputTooLarge { requested, limit } => write!(f, "RA MapPack Base64 decoded to at least {requested} bytes, limit is {limit}"),
        }
    }
}

impl std::error::Error for RaMapPackError {}

/// Raw map template reference. Catalog/theater validation is deliberately a
/// later stage so decoding never silently rewrites unknown upstream IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RaTemplateCell {
    pub template_type: u16,
    pub icon: u8,
}

/// Decoded row-major Red Alert template layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RaTemplateMap {
    width: u32,
    height: u32,
    cells: Vec<RaTemplateCell>,
}

impl RaTemplateMap {
    /// Decode concatenated INI [MapPack] Base64 values and their LCW chunks.
    /// ASCII whitespace is ignored, matching Convert.FromBase64String.
    pub fn decode_ini_section(encoded: &str, width: u32, height: u32) -> Result<Self, RaMapPackError> {
        if width == 0 || height == 0 {
            return Err(RaMapPackError::InvalidDimensions { width, height });
        }
        let cells = (width as usize).checked_mul(height as usize).ok_or(RaMapPackError::SizeOverflow)?;
        let expected = cells.checked_mul(3).ok_or(RaMapPackError::SizeOverflow)?;
        let chunk_count = expected.div_ceil(RA_MAPPACK_CHUNK_LIMIT);
        let compressed_limit = expected.checked_mul(2)
            .and_then(|n| n.checked_add(chunk_count.checked_mul(4)?))
            .ok_or(RaMapPackError::SizeOverflow)?;
        let framed = decode_base64(encoded, compressed_limit)?;
        Self::decode_lcw_chunks(&framed, width, height)
    }

    pub fn decode_lcw_chunks(bytes: &[u8], width: u32, height: u32) -> Result<Self, RaMapPackError> {
        if width == 0 || height == 0 {
            return Err(RaMapPackError::InvalidDimensions { width, height });
        }
        let cell_count = (width as usize).checked_mul(height as usize).ok_or(RaMapPackError::SizeOverflow)?;
        let expected = cell_count.checked_mul(3).ok_or(RaMapPackError::SizeOverflow)?;
        let mut decoded = Vec::new();
        decoded.try_reserve_exact(expected).map_err(|_| RaMapPackError::AllocationFailed { bytes: expected })?;

        let mut cursor = 0usize;
        while cursor < bytes.len() {
            let header_offset = cursor;
            let header_end = cursor.checked_add(4).ok_or(RaMapPackError::SizeOverflow)?;
            let header = bytes.get(cursor..header_end).ok_or(RaMapPackError::TruncatedChunkHeader { offset: header_offset })?;
            let compressed_len = usize::from(u16::from_le_bytes([header[0], header[1]]));
            let decompressed_len = usize::from(u16::from_le_bytes([header[2], header[3]]));
            cursor = header_end;

            if compressed_len == 0 || decompressed_len == 0 || decompressed_len > RA_MAPPACK_CHUNK_LIMIT {
                return Err(RaMapPackError::InvalidChunkLengths {
                    offset: header_offset,
                    compressed: compressed_len,
                    decompressed: decompressed_len,
                });
            }
            let remaining_output = expected.saturating_sub(decoded.len());
            if decompressed_len > remaining_output {
                return Err(RaMapPackError::ChunkOutputTooLarge {
                    offset: header_offset,
                    requested: decompressed_len,
                    remaining: remaining_output,
                });
            }
            let data_end = cursor.checked_add(compressed_len).ok_or(RaMapPackError::SizeOverflow)?;
            let compressed = bytes.get(cursor..data_end).ok_or(RaMapPackError::TruncatedChunkData {
                offset: header_offset,
                expected: compressed_len,
                remaining: bytes.len().saturating_sub(cursor),
            })?;
            let block = lcw::decompress_expected(compressed, decompressed_len)
                .map_err(|source| RaMapPackError::Lcw { offset: header_offset, source })?;
            decoded.extend_from_slice(&block);
            cursor = data_end;
        }

        if decoded.len() != expected {
            return Err(RaMapPackError::OutputSizeMismatch { actual: decoded.len(), expected });
        }

        // MAP.CPP's new-format Write_Binary writes the entire TType plane first,
        // followed by the entire TIcon plane. Each TType is a little-endian u16.
        let mut cells = Vec::new();
        cells.try_reserve_exact(cell_count).map_err(|_| RaMapPackError::AllocationFailed { bytes: cell_count })?;
        let type_bytes = cell_count.checked_mul(2).ok_or(RaMapPackError::SizeOverflow)?;
        let (types, icons) = decoded.split_at(type_bytes);
        for index in 0..cell_count {
            let at = index.checked_mul(2).ok_or(RaMapPackError::SizeOverflow)?;
            let template_type = u16::from_le_bytes([types[at], types[at + 1]]);
            cells.push(RaTemplateCell { template_type, icon: icons[index] });
        }
        Ok(Self { width, height, cells })
    }

    pub const fn width(&self) -> u32 { self.width }
    pub const fn height(&self) -> u32 { self.height }
    pub fn cells(&self) -> &[RaTemplateCell] { &self.cells }
    pub fn cell_at(&self, x: u32, y: u32) -> Option<RaTemplateCell> {
        if x >= self.width || y >= self.height { return None; }
        self.cells.get((y as usize).checked_mul(self.width as usize)?.checked_add(x as usize)?).copied()
    }
}

fn decode_base64(encoded: &str, output_limit: usize) -> Result<Vec<u8>, RaMapPackError> {
    fn value(byte: u8) -> Option<u8> {
        match byte {
            b'A'..=b'Z' => Some(byte - b'A'),
            b'a'..=b'z' => Some(byte - b'a' + 26),
            b'0'..=b'9' => Some(byte - b'0' + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    }

    let mut output = Vec::new();
    let mut quartet = [0u8; 4];
    let mut count = 0usize;
    let mut padded = false;
    for (offset, byte) in encoded.bytes().enumerate() {
        if byte.is_ascii_whitespace() { continue; }
        if padded { return Err(RaMapPackError::InvalidBase64Padding { offset }); }
        quartet[count] = byte;
        count += 1;
        if count != 4 { continue; }

        let a = value(quartet[0]).ok_or(RaMapPackError::InvalidBase64 { offset: offset - 3, byte: quartet[0] })?;
        let b = value(quartet[1]).ok_or(RaMapPackError::InvalidBase64 { offset: offset - 2, byte: quartet[1] })?;
        let (c, d, produced, has_padding) = match (quartet[2], quartet[3]) {
            (b'=', b'=') => (0, 0, 1usize, true),
            (b'=', _) => return Err(RaMapPackError::InvalidBase64Padding { offset: offset - 1 }),
            (third, b'=') => (value(third).ok_or(RaMapPackError::InvalidBase64 { offset: offset - 1, byte: third })?, 0, 2usize, true),
            (third, fourth) => (
                value(third).ok_or(RaMapPackError::InvalidBase64 { offset: offset - 1, byte: third })?,
                value(fourth).ok_or(RaMapPackError::InvalidBase64 { offset, byte: fourth })?,
                3usize,
                false,
            ),
        };
        if (quartet[2] == b'=' && quartet[3] != b'=') ||
            (quartet[2] == b'=' && (b & 0x0F) != 0) ||
            (quartet[3] == b'=' && quartet[2] != b'=' && (c & 0x03) != 0)
        {
            return Err(RaMapPackError::InvalidBase64Padding { offset: offset - 1 });
        }

        let requested = output.len().checked_add(produced).ok_or(RaMapPackError::SizeOverflow)?;
        if requested > output_limit {
            return Err(RaMapPackError::Base64OutputTooLarge { requested, limit: output_limit });
        }
        output.try_reserve(produced).map_err(|_| RaMapPackError::AllocationFailed { bytes: requested })?;
        output.push((a << 2) | (b >> 4));
        if produced > 1 { output.push((b << 4) | (c >> 2)); }
        if produced > 2 { output.push((c << 6) | d); }
        padded = has_padding;
        count = 0;
    }
    if count != 0 { return Err(RaMapPackError::InvalidBase64Length { remaining: count }); }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lcw_literal_block(input: &[u8]) -> Vec<u8> {
        let mut output = Vec::new();
        for chunk in input.chunks(63) {
            output.push(0x80 | u8::try_from(chunk.len()).unwrap());
            output.extend_from_slice(chunk);
        }
        output.push(0x80);
        output
    }

    fn framed_block(input: &[u8], output: &mut Vec<u8>) {
        let compressed = lcw_literal_block(input);
        output.extend_from_slice(&u16::try_from(compressed.len()).unwrap().to_le_bytes());
        output.extend_from_slice(&u16::try_from(input.len()).unwrap().to_le_bytes());
        output.extend_from_slice(&compressed);
    }

    #[test]
    fn decodes_base64_ini_section_and_whitespace() {
        let raw = [5, 0, 3, 0, 0x83, 1, 2, 3, 0x80];
        let encoded = crate::base64::encode_all(&raw);
        let wrapped = format!("{}\r\n{}", &encoded[..4], &encoded[4..]);
        let map = RaTemplateMap::decode_ini_section(&wrapped, 1, 1).unwrap();
        assert_eq!(map.cell_at(0, 0), Some(RaTemplateCell { template_type: 0x0201, icon: 3 }));
    }

    #[test]
    fn rejects_invalid_base64_padding_and_incomplete_quartets() {
        assert!(matches!(decode_base64("AA=A", 16), Err(RaMapPackError::InvalidBase64Padding { .. })));
        assert!(matches!(decode_base64("AAA", 16), Err(RaMapPackError::InvalidBase64Length { .. })));
    }

    #[test]
    fn decodes_type_plane_then_icon_plane_in_row_major_order() {
        let mut raw = Vec::new();
        for value in [3u16, 7, 0x1234, 9] { raw.extend_from_slice(&value.to_le_bytes()); }
        raw.extend_from_slice(&[1, 2, 3, 4]);
        let mut framed = Vec::new();
        framed_block(&raw, &mut framed);
        let map = RaTemplateMap::decode_lcw_chunks(&framed, 2, 2).unwrap();
        assert_eq!(map.cell_at(0, 0), Some(RaTemplateCell { template_type: 3, icon: 1 }));
        assert_eq!(map.cell_at(1, 0), Some(RaTemplateCell { template_type: 7, icon: 2 }));
        assert_eq!(map.cell_at(0, 1), Some(RaTemplateCell { template_type: 0x1234, icon: 3 }));
        assert_eq!(map.cell_at(1, 1), Some(RaTemplateCell { template_type: 9, icon: 4 }));
        assert_eq!(map.cell_at(2, 1), None);
    }

    #[test]
    fn accepts_multiple_framed_lcw_chunks() {
        let mut raw = Vec::new();
        for value in [11u16, 12] { raw.extend_from_slice(&value.to_le_bytes()); }
        raw.extend_from_slice(&[5, 6]);
        let mut framed = Vec::new();
        framed_block(&raw[..3], &mut framed);
        framed_block(&raw[3..], &mut framed);
        let map = RaTemplateMap::decode_lcw_chunks(&framed, 2, 1).unwrap();
        assert_eq!(map.cells(), &[
            RaTemplateCell { template_type: 11, icon: 5 },
            RaTemplateCell { template_type: 12, icon: 6 },
        ]);
    }

    #[test]
    fn rejects_zero_dimensions_and_incomplete_headers() {
        assert!(matches!(RaTemplateMap::decode_lcw_chunks(&[], 0, 1), Err(RaMapPackError::InvalidDimensions { .. })));
        assert!(matches!(RaTemplateMap::decode_lcw_chunks(&[1, 0, 1], 1, 1), Err(RaMapPackError::TruncatedChunkHeader { .. })));
    }

    #[test]
    fn rejects_truncated_compressed_chunk() {
        let framed = [5, 0, 3, 0, 0x82, b'a'];
        assert!(matches!(RaTemplateMap::decode_lcw_chunks(&framed, 1, 1), Err(RaMapPackError::TruncatedChunkData { .. })));
    }

    #[test]
    fn rejects_chunk_output_beyond_map_buffer() {
        let mut framed = Vec::new();
        framed_block(&[0, 0, 0, 0], &mut framed);
        assert!(matches!(RaTemplateMap::decode_lcw_chunks(&framed, 1, 1), Err(RaMapPackError::ChunkOutputTooLarge { .. })));
    }

    #[test]
    fn rejects_wrong_total_decoded_size() {
        let mut framed = Vec::new();
        framed_block(&[0, 0], &mut framed);
        assert!(matches!(RaTemplateMap::decode_lcw_chunks(&framed, 1, 1), Err(RaMapPackError::OutputSizeMismatch { .. })));
    }

    #[test]
    fn rejects_chunk_lengths_above_upstream_buffer_limit() {
        let framed = [1, 0, 1, 32]; // decompressed length 8193
        assert!(matches!(RaTemplateMap::decode_lcw_chunks(&framed, 1, 1), Err(RaMapPackError::InvalidChunkLengths { .. })));
    }
}
