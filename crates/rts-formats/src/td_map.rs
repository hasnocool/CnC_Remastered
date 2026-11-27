//! Tiberian Dawn map-editor binary terrain layer decoder.
//!
//! Based on EA Remastered Collection TiberianDawn/GamePlugin.cs LoadBinary:
//! one template-type byte and one logical-icon byte per cell, iterating rows
//! then columns. This editor BIN payload is distinct from the engine's LCW
//! stream and from Red Alert's MapPack serialization. Dimensions are supplied
//! by the surrounding map document because this byte stream has no header.

use core::fmt;

pub const TD_MAP_MAX_CELLS: usize = 1_048_576;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TdMapBinaryError {
    InvalidDimensions { width: u32, height: u32 },
    TooManyCells { cells: usize, limit: usize },
    SizeOverflow,
    AllocationFailed { cells: usize },
    DataLengthMismatch { actual: usize, expected: usize },
}

impl fmt::Display for TdMapBinaryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDimensions { width, height } => write!(f, "invalid Tiberian Dawn map dimensions {width}x{height}"),
            Self::TooManyCells { cells, limit } => write!(f, "Tiberian Dawn map has {cells} cells; limit is {limit}"),
            Self::SizeOverflow => f.write_str("Tiberian Dawn map size arithmetic overflow"),
            Self::AllocationFailed { cells } => write!(f, "could not reserve Tiberian Dawn map storage for {cells} cells"),
            Self::DataLengthMismatch { actual, expected } => write!(f, "Tiberian Dawn binary map has {actual} bytes; expected exactly {expected}"),
        }
    }
}
impl std::error::Error for TdMapBinaryError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TdTemplateCell {
    pub template_type: u8,
    pub icon: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TdTemplateMap {
    width: u32,
    height: u32,
    cells: Vec<TdTemplateCell>,
}

impl TdTemplateMap {
    /// Decode the raw editor BIN terrain layer using externally supplied map metrics.
    /// Every cell is [template_type: u8, logical_icon: u8], row-major.
    pub fn decode_binary(bytes: &[u8], width: u32, height: u32) -> Result<Self, TdMapBinaryError> {
        if width == 0 || height == 0 {
            return Err(TdMapBinaryError::InvalidDimensions { width, height });
        }
        let cell_count = (width as usize).checked_mul(height as usize)
            .ok_or(TdMapBinaryError::SizeOverflow)?;
        if cell_count > TD_MAP_MAX_CELLS {
            return Err(TdMapBinaryError::TooManyCells { cells: cell_count, limit: TD_MAP_MAX_CELLS });
        }
        let expected_bytes = cell_count.checked_mul(2).ok_or(TdMapBinaryError::SizeOverflow)?;
        if bytes.len() != expected_bytes {
            return Err(TdMapBinaryError::DataLengthMismatch { actual: bytes.len(), expected: expected_bytes });
        }
        let mut cells = Vec::new();
        cells.try_reserve_exact(cell_count)
            .map_err(|_| TdMapBinaryError::AllocationFailed { cells: cell_count })?;
        for pair in bytes.chunks_exact(2) {
            cells.push(TdTemplateCell { template_type: pair[0], icon: pair[1] });
        }
        Ok(Self { width, height, cells })
    }

    pub const fn width(&self) -> u32 { self.width }
    pub const fn height(&self) -> u32 { self.height }
    pub fn cells(&self) -> &[TdTemplateCell] { &self.cells }
    pub fn cell_at(&self, x: u32, y: u32) -> Option<TdTemplateCell> {
        if x >= self.width || y >= self.height { return None; }
        let index = (y as usize).checked_mul(self.width as usize)?.checked_add(x as usize)?;
        self.cells.get(index).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_type_and_icon_pairs_in_row_major_order() {
        let bytes = [0, 0, 3, 7, 12, 4, 255, 22];
        let map = TdTemplateMap::decode_binary(&bytes, 2, 2).unwrap();
        assert_eq!(map.cell_at(0, 0), Some(TdTemplateCell { template_type: 0, icon: 0 }));
        assert_eq!(map.cell_at(1, 0), Some(TdTemplateCell { template_type: 3, icon: 7 }));
        assert_eq!(map.cell_at(0, 1), Some(TdTemplateCell { template_type: 12, icon: 4 }));
        assert_eq!(map.cell_at(1, 1), Some(TdTemplateCell { template_type: 255, icon: 22 }));
        assert_eq!(map.cell_at(2, 1), None);
    }

    #[test]
    fn rejects_truncated_and_trailing_data() {
        assert_eq!(TdTemplateMap::decode_binary(&[1, 2, 3], 2, 1),
            Err(TdMapBinaryError::DataLengthMismatch { actual: 3, expected: 4 }));
        assert_eq!(TdTemplateMap::decode_binary(&[1, 2, 3, 4, 5], 2, 1),
            Err(TdMapBinaryError::DataLengthMismatch { actual: 5, expected: 4 }));
    }

    #[test]
    fn rejects_invalid_dimensions_and_excessive_cell_counts() {
        assert!(matches!(TdTemplateMap::decode_binary(&[], 0, 1), Err(TdMapBinaryError::InvalidDimensions { .. })));
        assert!(matches!(TdTemplateMap::decode_binary(&[], 1025, 1024), Err(TdMapBinaryError::TooManyCells { .. })));
    }
}
