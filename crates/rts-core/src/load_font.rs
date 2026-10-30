//! Portable implementation of REDALERT/WIN32LIB/LOADFONT.CPP.
//!
//! The legacy IBM loader reads a little-endian byte count followed by the
//! font payload. A valid font has compression byte 0 and five data blocks.
//! The returned allocation begins with the original two-byte size prefix.

use std::fmt;

pub const FONT_COMPRESSION_NONE: u8 = 0;
pub const FONT_DATA_BLOCK_COUNT: u8 = 5;
pub const FONT_FILE_PREFIX_LEN: usize = 2;
pub const FONT_MIN_FILE_LEN: usize = 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LoadFontError {
    MissingLength,
    InvalidLength,
    TruncatedPayload,
    InvalidCompression(u8),
    InvalidBlockCount(u8),
}

impl fmt::Display for LoadFontError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for LoadFontError {}

/// Global defaults established by LOADFONT.CPP before a font is loaded.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FontLoaderState {
    pub x_spacing: i32,
    pub y_spacing: i32,
    pub font_width: i8,
    pub font_height: i8,
    pub font_loaded: bool,
    pub width_block_loaded: bool,
}

impl Default for FontLoaderState {
    fn default() -> Self {
        Self {
            x_spacing: 0,
            y_spacing: 0,
            font_width: 8,
            font_height: 8,
            font_loaded: false,
            width_block_loaded: false,
        }
    }
}

/// Validate and retain a complete legacy font file.
///
/// The C++ implementation stores the size prefix in the allocated buffer,
/// then validates bytes 2 and 3. This routine preserves that in-memory layout
/// while rejecting short reads that the original unchecked I/O could accept.
pub fn load_font_bytes(file: &[u8]) -> Result<Vec<u8>, LoadFontError> {
    let prefix = file.get(..FONT_FILE_PREFIX_LEN).ok_or(LoadFontError::MissingLength)?;
    let declared = u16::from_le_bytes([prefix[0], prefix[1]]) as usize;
    if declared < FONT_MIN_FILE_LEN {
        return Err(LoadFontError::InvalidLength);
    }
    if file.len() < declared {
        return Err(LoadFontError::TruncatedPayload);
    }
    let compression = file[2];
    if compression != FONT_COMPRESSION_NONE {
        return Err(LoadFontError::InvalidCompression(compression));
    }
    let blocks = file[3];
    if blocks != FONT_DATA_BLOCK_COUNT {
        return Err(LoadFontError::InvalidBlockCount(blocks));
    }
    Ok(file[..declared].to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_legacy_globals() {
        assert_eq!(FontLoaderState::default(), FontLoaderState {
            x_spacing: 0,
            y_spacing: 0,
            font_width: 8,
            font_height: 8,
            font_loaded: false,
            width_block_loaded: false,
        });
    }

    #[test]
    fn preserves_size_prefix_and_payload() {
        let bytes = [6, 0, 0, 5, 0xaa, 0xbb, 0xcc];
        assert_eq!(load_font_bytes(&bytes), Ok(vec![6, 0, 0, 5, 0xaa, 0xbb]));
    }

    #[test]
    fn rejects_short_and_invalid_files() {
        assert_eq!(load_font_bytes(&[3, 0, 0]), Err(LoadFontError::InvalidLength));
        assert_eq!(load_font_bytes(&[6, 0, 0, 5]), Err(LoadFontError::TruncatedPayload));
        assert_eq!(load_font_bytes(&[4, 0, 1, 5]), Err(LoadFontError::InvalidCompression(1)));
        assert_eq!(load_font_bytes(&[4, 0, 0, 4]), Err(LoadFontError::InvalidBlockCount(4)));
    }
}
