// Filename: crates/rts-formats/src/tmp.rs
//! Bounded readers for Tiberian Dawn and Red Alert TMP terrain tile archives.
//!
//! The two retail variants share indexed fixed-size tile payloads and a byte
//! lookup table, but their headers and table offsets differ. Blank lookup
//! entries (255) are represented as None and never read as image data.

use crate::palette_rgba::{Palette256, PaletteError, RgbaImage};
use std::fmt;

const TD_HEADER_SIZE: usize = 32;
const RA_HEADER_SIZE: usize = 40;
const MAX_TILE_PIXELS: usize = 4 * 1024 * 1024;
const MAX_TILES: usize = 1_000_000;
const BLANK_TILE: u8 = 255;
const TD_MAGIC: u32 = 0x0d1a_ffff;
const RA_MAGIC: u16 = 0x2c73;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TmpVariant { TiberianDawn, RedAlert }

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TmpError {
    Truncated { needed: usize, available: usize },
    UnknownVariant,
    InvalidDimensions { width: u16, height: u16 },
    TileTooLarge(usize),
    TooManyTiles(usize),
    InvalidImageOffset { offset: usize, minimum: usize },
    InvalidIndexRange { start: usize, end: usize, file_len: usize },
    TileDataOutOfBounds { tile: u8, start: usize, end: usize, index_start: usize },
    InvalidTileIndex(usize),
    Palette(PaletteError),
}
impl fmt::Display for TmpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{self:?}") }
}
impl std::error::Error for TmpError {}
impl From<PaletteError> for TmpError {
    fn from(value: PaletteError) -> Self { Self::Palette(value) }
}

#[derive(Clone, Debug)]
pub struct TmpArchive<'a> {
    pub variant: TmpVariant,
    pub width: u16,
    pub height: u16,
    tiles: Vec<Option<&'a [u8]>>,
}

fn read_u16(bytes: &[u8], offset: usize) -> Result<u16, TmpError> {
    let end = offset.checked_add(2).ok_or(TmpError::Truncated {
        needed: usize::MAX, available: bytes.len(),
    })?;
    let value = bytes.get(offset..end).ok_or(TmpError::Truncated {
        needed: end, available: bytes.len(),
    })?;
    Ok(u16::from_le_bytes([value[0], value[1]]))
}
fn read_u32(bytes: &[u8], offset: usize) -> Result<u32, TmpError> {
    let end = offset.checked_add(4).ok_or(TmpError::Truncated {
        needed: usize::MAX, available: bytes.len(),
    })?;
    let value = bytes.get(offset..end).ok_or(TmpError::Truncated {
        needed: end, available: bytes.len(),
    })?;
    Ok(u32::from_le_bytes([value[0], value[1], value[2], value[3]]))
}
fn read_i32(bytes: &[u8], offset: usize) -> Result<i32, TmpError> {
    Ok(i32::from_le_bytes(read_u32(bytes, offset)?.to_le_bytes()))
}

impl<'a> TmpArchive<'a> {
    /// Detect the variant from its fixed header signature and parse the file.
    pub fn parse(bytes: &'a [u8]) -> Result<Self, TmpError> {
        let variant = if bytes.len() >= 24
            && read_u32(bytes, 16)? == 0
            && read_u32(bytes, 20)? == TD_MAGIC
        {
            TmpVariant::TiberianDawn
        } else if bytes.len() >= 28
            && read_u32(bytes, 20)? == 0
            && read_u16(bytes, 26)? == RA_MAGIC
        {
            TmpVariant::RedAlert
        } else {
            return Err(TmpError::UnknownVariant);
        };
        Self::parse_as(bytes, variant)
    }

    /// Parse a known variant explicitly. This is useful for tools that already
    /// know which game supplied the file and want signature validation too.
    pub fn parse_as(bytes: &'a [u8], variant: TmpVariant) -> Result<Self, TmpError> {
        let header_size = match variant {
            TmpVariant::TiberianDawn => TD_HEADER_SIZE,
            TmpVariant::RedAlert => RA_HEADER_SIZE,
        };
        if bytes.len() < header_size {
            return Err(TmpError::Truncated { needed: header_size, available: bytes.len() });
        }

        match variant {
            TmpVariant::TiberianDawn => {
                if read_u32(bytes, 16)? != 0 || read_u32(bytes, 20)? != TD_MAGIC {
                    return Err(TmpError::UnknownVariant);
                }
            }
            TmpVariant::RedAlert => {
                if read_u32(bytes, 20)? != 0 || read_u16(bytes, 26)? != RA_MAGIC {
                    return Err(TmpError::UnknownVariant);
                }
            }
        }

        let width = read_u16(bytes, 0)?;
        let height = read_u16(bytes, 2)?;
        if width == 0 || height == 0 {
            return Err(TmpError::InvalidDimensions { width, height });
        }
        let tile_pixels = usize::from(width).checked_mul(usize::from(height))
            .ok_or(TmpError::TileTooLarge(usize::MAX))?;
        if tile_pixels > MAX_TILE_PIXELS { return Err(TmpError::TileTooLarge(tile_pixels)); }

        let (image_offset_at, index_end_at, index_start_at) = match variant {
            TmpVariant::TiberianDawn => (12, 24, 28),
            TmpVariant::RedAlert => (16, 28, 36),
        };
        let image_start = usize::try_from(read_u32(bytes, image_offset_at)?)
            .map_err(|_| TmpError::InvalidImageOffset { offset: usize::MAX, minimum: header_size })?;
        let index_end = usize::try_from(read_i32(bytes, index_end_at)?)
            .map_err(|_| TmpError::InvalidIndexRange { start: 0, end: 0, file_len: bytes.len() })?;
        let index_start = usize::try_from(read_i32(bytes, index_start_at)?)
            .map_err(|_| TmpError::InvalidIndexRange { start: 0, end: 0, file_len: bytes.len() })?;

        if image_start < header_size || image_start > index_start {
            return Err(TmpError::InvalidImageOffset { offset: image_start, minimum: header_size });
        }
        if index_start > index_end || index_end > bytes.len() {
            return Err(TmpError::InvalidIndexRange { start: index_start, end: index_end, file_len: bytes.len() });
        }
        let tile_count = index_end - index_start;
        if tile_count > MAX_TILES { return Err(TmpError::TooManyTiles(tile_count)); }

        let tile_index = bytes.get(index_start..index_end).ok_or(TmpError::InvalidIndexRange {
            start: index_start, end: index_end, file_len: bytes.len(),
        })?;
        let mut tiles = Vec::with_capacity(tile_count);
        for &tile in tile_index {
            if tile == BLANK_TILE {
                tiles.push(None);
                continue;
            }
            let start = image_start.checked_add(usize::from(tile).checked_mul(tile_pixels)
                .ok_or(TmpError::TileDataOutOfBounds {
                    tile, start: usize::MAX, end: usize::MAX, index_start,
                })?)
                .ok_or(TmpError::TileDataOutOfBounds {
                    tile, start: usize::MAX, end: usize::MAX, index_start,
                })?;
            let end = start.checked_add(tile_pixels).ok_or(TmpError::TileDataOutOfBounds {
                tile, start, end: usize::MAX, index_start,
            })?;
            if end > index_start || end > bytes.len() {
                return Err(TmpError::TileDataOutOfBounds { tile, start, end, index_start });
            }
            tiles.push(Some(&bytes[start..end]));
        }

        Ok(Self { variant, width, height, tiles })
    }

    pub fn tile_count(&self) -> usize { self.tiles.len() }

    /// Return None for the archive's blank-tile marker (255).
    pub fn tile(&self, index: usize) -> Result<Option<&'a [u8]>, TmpError> {
        self.tiles.get(index).copied().ok_or(TmpError::InvalidTileIndex(index))
    }

    /// Expand a non-blank indexed tile into RGBA8 for renderer texture upload.
    /// Blank tiles remain None so terrain composition can skip their draw call.
    pub fn tile_rgba(
        &self,
        index: usize,
        palette: &Palette256,
        transparent_index: Option<u8>,
    ) -> Result<Option<RgbaImage>, TmpError> {
        self.tile(index)?.map(|pixels| {
            Ok(palette.expand_rgba(
                u32::from(self.width),
                u32::from(self.height),
                pixels,
                transparent_index,
            )?)
        }).transpose()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::palette_rgba::{PaletteEncoding, PALETTE_BYTES};

    fn td_fixture() -> Vec<u8> {
        let mut bytes = vec![0u8; 43];
        bytes[0..2].copy_from_slice(&2u16.to_le_bytes());
        bytes[2..4].copy_from_slice(&2u16.to_le_bytes());
        bytes[12..16].copy_from_slice(&32u32.to_le_bytes());
        bytes[20..24].copy_from_slice(&TD_MAGIC.to_le_bytes());
        bytes[24..28].copy_from_slice(&43i32.to_le_bytes());
        bytes[28..32].copy_from_slice(&40i32.to_le_bytes());
        bytes[32..36].copy_from_slice(&[1, 2, 3, 4]);
        bytes[36..40].copy_from_slice(&[5, 6, 7, 8]);
        bytes[40..43].copy_from_slice(&[0, BLANK_TILE, 1]);
        bytes
    }

    fn ra_fixture() -> Vec<u8> {
        let mut bytes = vec![0u8; 51];
        bytes[0..2].copy_from_slice(&2u16.to_le_bytes());
        bytes[2..4].copy_from_slice(&2u16.to_le_bytes());
        bytes[16..20].copy_from_slice(&40u32.to_le_bytes());
        bytes[26..28].copy_from_slice(&RA_MAGIC.to_le_bytes());
        bytes[28..32].copy_from_slice(&51i32.to_le_bytes());
        bytes[36..40].copy_from_slice(&48i32.to_le_bytes());
        bytes[40..44].copy_from_slice(&[9, 10, 11, 12]);
        bytes[44..48].copy_from_slice(&[13, 14, 15, 16]);
        bytes[48..51].copy_from_slice(&[1, BLANK_TILE, 0]);
        bytes
    }

    #[test]
    fn parses_td_tile_table_and_blank_entries() {
        let archive = TmpArchive::parse(&td_fixture()).unwrap();
        assert_eq!(archive.variant, TmpVariant::TiberianDawn);
        assert_eq!((archive.width, archive.height, archive.tile_count()), (2, 2, 3));
        assert_eq!(archive.tile(0).unwrap(), Some(&[1, 2, 3, 4][..]));
        assert_eq!(archive.tile(1).unwrap(), None);
        assert_eq!(archive.tile(2).unwrap(), Some(&[5, 6, 7, 8][..]));
    }

    #[test]
    fn parses_ra_header_with_variant_specific_offsets() {
        let archive = TmpArchive::parse(&ra_fixture()).unwrap();
        assert_eq!(archive.variant, TmpVariant::RedAlert);
        assert_eq!(archive.tile(0).unwrap(), Some(&[13, 14, 15, 16][..]));
        assert_eq!(archive.tile(1).unwrap(), None);
        assert_eq!(archive.tile(2).unwrap(), Some(&[9, 10, 11, 12][..]));
    }

    #[test]
    fn expands_tmp_pixels_through_palette() {
        let archive = TmpArchive::parse(&td_fixture()).unwrap();
        let mut raw = vec![0u8; PALETTE_BYTES];
        raw[3..6].copy_from_slice(&[21, 22, 23]);
        raw[6..9].copy_from_slice(&[31, 32, 33]);
        raw[9..12].copy_from_slice(&[41, 42, 43]);
        raw[12..15].copy_from_slice(&[51, 52, 53]);
        let palette = Palette256::parse(&raw, PaletteEncoding::Rgb8).unwrap();
        let image = archive.tile_rgba(0, &palette, None).unwrap().unwrap();
        assert_eq!((image.width, image.height), (2, 2));
        assert_eq!(image.pixels, [
            21, 22, 23, 255, 31, 32, 33, 255,
            41, 42, 43, 255, 51, 52, 53, 255,
        ]);
        assert!(archive.tile_rgba(1, &palette, None).unwrap().is_none());
    }

    #[test]
    fn rejects_invalid_index_ranges_and_tile_data_offsets() {
        let mut bytes = td_fixture();
        bytes[28..32].copy_from_slice(&44i32.to_le_bytes());
        assert!(matches!(TmpArchive::parse(&bytes), Err(TmpError::InvalidIndexRange { .. })));

        let mut bytes = td_fixture();
        bytes[40] = 200;
        assert!(matches!(TmpArchive::parse(&bytes), Err(TmpError::TileDataOutOfBounds { .. })));
    }
}
