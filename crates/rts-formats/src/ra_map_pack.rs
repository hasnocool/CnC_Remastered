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
//! The caller is responsible for decoding the INI section's concatenated
//! Base64 value before passing its bytes to this module. This decoder handles
//! only the RA LCW chunk framing and the two-plane MapPack payload.

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
