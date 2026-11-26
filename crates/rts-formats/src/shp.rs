// Filename: crates/rts-formats/src/shp.rs
//! Bounds-checked SHP shape-block and frame-metadata reader.
//! Layout follows ShapeBlock_Type, Shape_Type and Extract_Shape in upstream
//! TIBERIANDAWN/WIN32LIB/SHAPE.H and GETSHAPE.CPP. Pixel/RLE decoding is a
//! separate pending milestone: this API exposes encoded bytes only.

use std::fmt;
pub const SHAPE_HEADER_BYTES: usize = 26;
const COUNT_BYTES: usize = 2;
const OFFSET_BYTES: usize = 4;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ShpError {
    TruncatedCount,
    OffsetTableOverflow,
    TruncatedOffsetTable { required: usize, available: usize },
    FrameIndexOutOfRange { index: usize, count: usize },
    InvalidFrameOffset { index: usize, offset: usize },
    TruncatedShapeHeader { index: usize },
    InvalidShapeSize { index: usize, size: usize, available: usize },
    InvalidDataLength { index: usize, length: usize, capacity: usize },
}
impl fmt::Display for ShpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{self:?}") }
}
impl std::error::Error for ShpError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ShapeHeader {
    pub shape_type: u16,
    pub height: u8,
    pub width: u16,
    pub original_height: u8,
    pub shape_size: u16,
    pub data_length: u16,
    pub color_table: [u8; 16],
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ShapeFrame<'a> {
    pub header: ShapeHeader,
    /// Encoded bytes after the 26-byte packed header; not decoded pixels.
    pub encoded_data: &'a [u8],
}
#[derive(Clone, Copy, Debug)]
pub struct ShpBlock<'a> { bytes: &'a [u8], frame_count: usize, data_start: usize }

impl<'a> ShpBlock<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self, ShpError> {
        let count = bytes.get(..COUNT_BYTES).ok_or(ShpError::TruncatedCount)?;
        let frame_count = u16::from_le_bytes([count[0], count[1]]) as usize;
        let table_bytes = frame_count.checked_mul(OFFSET_BYTES).ok_or(ShpError::OffsetTableOverflow)?;
        let table_end = COUNT_BYTES.checked_add(table_bytes).ok_or(ShpError::OffsetTableOverflow)?;
        if table_end > bytes.len() {
            return Err(ShpError::TruncatedOffsetTable { required: table_end, available: bytes.len() });
        }
        Ok(Self { bytes, frame_count, data_start: table_end })
    }
    pub fn frame_count(&self) -> usize { self.frame_count }

    /// Offsets are relative to byte 2, matching upstream Extract_Shape.
    pub fn frame(&self, index: usize) -> Result<ShapeFrame<'a>, ShpError> {
        if index >= self.frame_count {
            return Err(ShpError::FrameIndexOutOfRange { index, count: self.frame_count });
        }
        let entry = COUNT_BYTES + index * OFFSET_BYTES;
        let offset = u32::from_le_bytes(self.bytes[entry..entry + OFFSET_BYTES]
            .try_into().expect("validated four-byte offset entry")) as usize;
        let start = COUNT_BYTES.checked_add(offset)
            .ok_or(ShpError::InvalidFrameOffset { index, offset })?;
        if start < self.data_start {
            return Err(ShpError::InvalidFrameOffset { index, offset });
        }
        let header_end = start.checked_add(SHAPE_HEADER_BYTES)
            .ok_or(ShpError::TruncatedShapeHeader { index })?;
        let h = self.bytes.get(start..header_end).ok_or(ShpError::TruncatedShapeHeader { index })?;
        let mut color_table = [0; 16];
        color_table.copy_from_slice(&h[10..26]);
        let header = ShapeHeader {
            shape_type: u16::from_le_bytes([h[0], h[1]]),
            height: h[2],
            width: u16::from_le_bytes([h[3], h[4]]),
            original_height: h[5],
            shape_size: u16::from_le_bytes([h[6], h[7]]),
            data_length: u16::from_le_bytes([h[8], h[9]]),
            color_table,
        };
        let available = self.bytes.len() - start;
        let size = header.shape_size as usize;
        if size < SHAPE_HEADER_BYTES || size > available {
            return Err(ShpError::InvalidShapeSize { index, size, available });
        }
        // DataLength is the decoded/uncompressed payload size in Shape_Type;
        // it may legitimately exceed the compressed bytes stored in ShapeSize.
        Ok(ShapeFrame { header, encoded_data: &self.bytes[header_end..start + size] })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> Vec<u8> {
        let mut b = vec![];
        b.extend_from_slice(&1u16.to_le_bytes());
        b.extend_from_slice(&4u32.to_le_bytes());
        b.extend_from_slice(&0u16.to_le_bytes());
        b.push(2);
        b.extend_from_slice(&3u16.to_le_bytes());
        b.push(2);
        b.extend_from_slice(&29u16.to_le_bytes());
        b.extend_from_slice(&3u16.to_le_bytes());
        b.extend_from_slice(&[0; 16]);
        b.extend_from_slice(&[7, 8, 9]);
        b
    }
    #[test]
    fn parses_frame_header_and_bounded_encoded_payload() {
        let b = fixture();
        let block = ShpBlock::parse(&b).unwrap();
        let frame = block.frame(0).unwrap();
        assert_eq!((frame.header.width, frame.header.height), (3, 2));
        assert_eq!(frame.encoded_data, &[7, 8, 9]);
    }
    #[test]
    fn rejects_truncated_table_and_bad_indices() {
        assert!(matches!(ShpBlock::parse(&[1, 0]), Err(ShpError::TruncatedOffsetTable { .. })));
        assert!(matches!(ShpBlock::parse(&fixture()).unwrap().frame(1),
            Err(ShpError::FrameIndexOutOfRange { .. })));
    }
    #[test]
    fn accepts_compressed_payload_smaller_than_decoded_data_length() {
        let mut b = fixture();
        b[14..16].copy_from_slice(&900u16.to_le_bytes());
        assert_eq!(ShpBlock::parse(&b).unwrap().frame(0).unwrap().header.data_length, 900);
    }
    #[test]
    fn rejects_offsets_that_point_into_the_offset_table() {
        let mut b = fixture();
        b[2..6].copy_from_slice(&0u32.to_le_bytes());
        assert!(matches!(ShpBlock::parse(&b).unwrap().frame(0), Err(ShpError::InvalidFrameOffset { .. })));
    }
    #[test]
    fn rejects_shape_extent_outside_archive() {
        let mut b = fixture();
        b[12..14].copy_from_slice(&500u16.to_le_bytes());
        assert!(matches!(ShpBlock::parse(&b).unwrap().frame(0),
            Err(ShpError::InvalidShapeSize { .. })));
    }
}
