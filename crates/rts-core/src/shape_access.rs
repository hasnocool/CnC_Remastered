//! Checked accessors ported from both variants' WIN32LIB/GETSHAPE.CPP.
//!
//! The packed legacy Shape_Type fields are decoded explicitly rather than
//! relying on host alignment: Height@2, Width@3, OriginalHeight@5,
//! ShapeSize@6, DataLength@8. Shape block offsets are signed 32-bit values
//! beginning immediately after the 16-bit shape count.

use std::fmt;

pub const SHAPE_HEADER_SIZE: usize = 12;
pub const SHAPE_BLOCK_COUNT_SIZE: usize = 2;
pub const SHAPE_BLOCK_OFFSET_SIZE: usize = 4;
pub const SHAPE_HEIGHT_OFFSET: usize = 2;
pub const SHAPE_WIDTH_OFFSET: usize = 3;
pub const SHAPE_ORIGINAL_HEIGHT_OFFSET: usize = 5;
pub const SHAPE_SIZE_OFFSET: usize = 6;
pub const SHAPE_DATA_LENGTH_OFFSET: usize = 8;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ShapeAccessError {
    TruncatedShapeHeader,
    TruncatedShapeBlock,
    ShapeIndexOutOfRange { index: usize, count: usize },
    NegativeShapeOffset(i32),
    ShapeOffsetOutOfBounds(usize),
    PrefixDataOutOfBounds { index: usize },
    OffsetOverflow,
}
impl fmt::Display for ShapeAccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{self:?}") }
}
impl std::error::Error for ShapeAccessError {}

fn read_u16(bytes: &[u8], offset: usize) -> Result<u16, ShapeAccessError> {
    let pair = bytes.get(offset..offset.checked_add(2).ok_or(ShapeAccessError::OffsetOverflow)?)
        .ok_or(ShapeAccessError::TruncatedShapeHeader)?;
    Ok(u16::from_le_bytes([pair[0], pair[1]]))
}

pub fn extract_shape_count(block: &[u8]) -> Result<usize, ShapeAccessError> {
    let count = block.get(..2).ok_or(ShapeAccessError::TruncatedShapeBlock)?;
    Ok(u16::from_le_bytes([count[0], count[1]]) as usize)
}

/// Resolve a shape-table index to its legacy pointer offset (block + 2 + offset).
pub fn extract_shape(block: &[u8], index: usize) -> Result<usize, ShapeAccessError> {
    let count = extract_shape_count(block)?;
    if index >= count {
        return Err(ShapeAccessError::ShapeIndexOutOfRange { index, count });
    }
    let entry = SHAPE_BLOCK_COUNT_SIZE
        .checked_add(index.checked_mul(SHAPE_BLOCK_OFFSET_SIZE).ok_or(ShapeAccessError::OffsetOverflow)?)
        .ok_or(ShapeAccessError::OffsetOverflow)?;
    let raw = block.get(entry..entry + 4).ok_or(ShapeAccessError::TruncatedShapeBlock)?;
    let offset = i32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]);
    if offset < 0 { return Err(ShapeAccessError::NegativeShapeOffset(offset)); }
    let shape_at = SHAPE_BLOCK_COUNT_SIZE.checked_add(offset as usize).ok_or(ShapeAccessError::OffsetOverflow)?;
    block.get(shape_at..shape_at + SHAPE_HEADER_SIZE).ok_or(ShapeAccessError::ShapeOffsetOutOfBounds(shape_at))?;
    Ok(shape_at)
}

pub fn shape_size(shape: Option<&[u8]>) -> Result<usize, ShapeAccessError> {
    let Some(shape) = shape else { return Ok(0) };
    Ok(read_u16(shape, SHAPE_SIZE_OFFSET)? as usize)
}
pub fn shape_uncompressed_size(shape: &[u8]) -> Result<usize, ShapeAccessError> {
    Ok(read_u16(shape, SHAPE_DATA_LENGTH_OFFSET)? as usize)
}
pub fn shape_width(shape: &[u8]) -> Result<usize, ShapeAccessError> {
    Ok(read_u16(shape, SHAPE_WIDTH_OFFSET)? as usize)
}
pub fn shape_height(shape: &[u8]) -> Result<u8, ShapeAccessError> {
    Ok(*shape.get(SHAPE_HEIGHT_OFFSET).ok_or(ShapeAccessError::TruncatedShapeHeader)?)
}
pub fn shape_original_height(shape: &[u8]) -> Result<u8, ShapeAccessError> {
    Ok(*shape.get(SHAPE_ORIGINAL_HEIGHT_OFFSET).ok_or(ShapeAccessError::TruncatedShapeHeader)?)
}

/// Mutates the byte-sized Height field; the C++ WORD argument truncates to u8.
pub fn set_shape_height(shape: &mut [u8], new_height: u16) -> Result<u8, ShapeAccessError> {
    let height = shape.get_mut(SHAPE_HEIGHT_OFFSET).ok_or(ShapeAccessError::TruncatedShapeHeader)?;
    let old = *height;
    *height = new_height as u8;
    Ok(old)
}
pub fn restore_shape_height(shape: &mut [u8]) -> Result<u8, ShapeAccessError> {
    let original = shape_original_height(shape)?;
    let height = shape.get_mut(SHAPE_HEIGHT_OFFSET).ok_or(ShapeAccessError::TruncatedShapeHeader)?;
    let old = *height;
    *height = original;
    Ok(old)
}

/// Reads a MAKESHPS prefix WORD stored immediately before a shape header.
/// data_index=0 addresses the closest preceding WORD.
pub fn get_shape_data(
    block: &[u8], shape_offset: usize, data_index: usize,
) -> Result<u16, ShapeAccessError> {
    let words_back = data_index.checked_add(1).ok_or(ShapeAccessError::OffsetOverflow)?;
    let byte_back = words_back.checked_mul(2).ok_or(ShapeAccessError::OffsetOverflow)?;
    let at = shape_offset.checked_sub(byte_back).ok_or(ShapeAccessError::PrefixDataOutOfBounds { index: data_index })?;
    let pair = block.get(at..at + 2).ok_or(ShapeAccessError::PrefixDataOutOfBounds { index: data_index })?;
    Ok(u16::from_le_bytes([pair[0], pair[1]]))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_shape() -> Vec<u8> {
        let mut shape = vec![0u8; SHAPE_HEADER_SIZE];
        shape[2] = 20;
        shape[3..5].copy_from_slice(&48u16.to_le_bytes());
        shape[5] = 24;
        shape[6..8].copy_from_slice(&64u16.to_le_bytes());
        shape[8..10].copy_from_slice(&96u16.to_le_bytes());
        shape
    }

    #[test]
    fn extracts_shape_block_entry_and_metadata() {
        let mut block = vec![0u8; 2 + 4 + SHAPE_HEADER_SIZE];
        block[..2].copy_from_slice(&1u16.to_le_bytes());
        block[2..6].copy_from_slice(&4i32.to_le_bytes());
        block[6..].copy_from_slice(&sample_shape());
        let at = extract_shape(&block, 0).unwrap();
        assert_eq!(at, 6);
        let shape = &block[at..];
        assert_eq!(shape_size(Some(shape)).unwrap(), 64);
        assert_eq!(shape_uncompressed_size(shape).unwrap(), 96);
        assert_eq!(shape_width(shape).unwrap(), 48);
        assert_eq!(shape_height(shape).unwrap(), 20);
        assert_eq!(shape_original_height(shape).unwrap(), 24);
        assert_eq!(shape_size(None).unwrap(), 0);
    }

    #[test]
    fn shape_height_set_and_restore_return_previous_values() {
        let mut shape = sample_shape();
        assert_eq!(set_shape_height(&mut shape, 300).unwrap(), 20);
        assert_eq!(shape_height(&shape).unwrap(), 44);
        assert_eq!(restore_shape_height(&mut shape).unwrap(), 44);
        assert_eq!(shape_height(&shape).unwrap(), 24);
    }

    #[test]
    fn reads_special_prefix_words_before_shape() {
        let mut block = vec![0u8; 6 + SHAPE_HEADER_SIZE];
        block[2..4].copy_from_slice(&0x1234u16.to_le_bytes());
        block[4..6].copy_from_slice(&0x5678u16.to_le_bytes());
        assert_eq!(get_shape_data(&block, 6, 0).unwrap(), 0x5678);
        assert_eq!(get_shape_data(&block, 6, 1).unwrap(), 0x1234);
    }

    #[test]
    fn rejects_bad_shape_indices_and_offsets() {
        let mut block = vec![0u8; 6];
        block[..2].copy_from_slice(&1u16.to_le_bytes());
        block[2..6].copy_from_slice(&(-1i32).to_le_bytes());
        assert_eq!(extract_shape(&block, 0), Err(ShapeAccessError::NegativeShapeOffset(-1)));
        assert!(matches!(extract_shape(&block, 1), Err(ShapeAccessError::ShapeIndexOutOfRange { .. })));
        assert_eq!(extract_shape_count(&[1]), Err(ShapeAccessError::TruncatedShapeBlock));
    }
}
