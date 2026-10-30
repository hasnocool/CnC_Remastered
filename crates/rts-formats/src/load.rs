//! Port of the shared REDALERT/WIN32LIB/LOAD.CPP and
//! TIBERIANDAWN/WIN32LIB/LOAD.CPP data-loading and decompression routines.
//!
//! The source-compatible compression header is eight bytes: method, pad,
//! little-endian u32 output size, and little-endian u16 skipped-data size.

use std::fmt;
use std::io::{Read, Write};
use crate::lcw::{decompress_expected, LcwError};

pub const COMP_HEADER_SIZE: usize = 8;
pub const METHOD_NONE: u8 = 0;
pub const METHOD_LZW12: u8 = 1;
pub const METHOD_LZW14: u8 = 2;
pub const METHOD_HORIZONTAL: u8 = 3;
pub const METHOD_LCW: u8 = 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompressionMethod { None, Lzw12, Lzw14, Horizontal, Lcw, Unknown(u8) }

impl From<u8> for CompressionMethod {
    fn from(value: u8) -> Self {
        match value {
            METHOD_NONE => Self::None,
            METHOD_LZW12 => Self::Lzw12,
            METHOD_LZW14 => Self::Lzw14,
            METHOD_HORIZONTAL => Self::Horizontal,
            METHOD_LCW => Self::Lcw,
            other => Self::Unknown(other),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompHeader {
    pub method: CompressionMethod,
    pub pad: u8,
    pub output_size: usize,
    pub skip_size: usize,
}

impl CompHeader {
    pub fn parse(bytes: &[u8]) -> Result<Self, LoadError> {
        let h = bytes.get(..COMP_HEADER_SIZE).ok_or(LoadError::TruncatedHeader)?;
        Ok(Self {
            method: h[0].into(),
            pad: h[1],
            output_size: u32::from_le_bytes([h[2], h[3], h[4], h[5]]) as usize,
            skip_size: u16::from_le_bytes([h[6], h[7]]) as usize,
        })
    }

    pub fn encode(&self) -> Result<[u8; COMP_HEADER_SIZE], LoadError> {
        let output_size = u32::try_from(self.output_size).map_err(|_| LoadError::SizeOverflow)?;
        let skip_size = u16::try_from(self.skip_size).map_err(|_| LoadError::SizeOverflow)?;
        let method = match self.method {
            CompressionMethod::None => METHOD_NONE,
            CompressionMethod::Lzw12 => METHOD_LZW12,
            CompressionMethod::Lzw14 => METHOD_LZW14,
            CompressionMethod::Horizontal => METHOD_HORIZONTAL,
            CompressionMethod::Lcw => METHOD_LCW,
            CompressionMethod::Unknown(value) => value,
        };
        let mut bytes = [0; COMP_HEADER_SIZE];
        bytes[0] = method;
        bytes[1] = self.pad;
        bytes[2..6].copy_from_slice(&output_size.to_le_bytes());
        bytes[6..8].copy_from_slice(&skip_size.to_le_bytes());
        Ok(bytes)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LoadError {
    Io(String), TruncatedFileSize, TruncatedHeader, TruncatedSkippedData,
    TruncatedCompressedData, ScratchBufferTooSmall { required: usize, capacity: usize },
    OutputLimitExceeded { requested: usize, limit: usize }, SizeOverflow, Lcw(LcwError),
}
impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{self:?}") }
}
impl std::error::Error for LoadError {}
impl From<LcwError> for LoadError {
    fn from(value: LcwError) -> Self { Self::Lcw(value) }
}

/// Load_Data: read at most max_bytes and return the number of bytes read.
pub fn load_data<R: Read>(reader: &mut R, max_bytes: usize) -> Result<Vec<u8>, LoadError> {
    let mut data = vec![0; max_bytes];
    let mut used = 0;
    while used < max_bytes {
        match reader.read(&mut data[used..]) {
            Ok(0) => break,
            Ok(n) => used += n,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(LoadError::Io(e.to_string())),
        }
    }
    data.truncate(used);
    Ok(data)
}

/// Write_Data: write the supplied bytes and report the actual byte count.
pub fn write_data<W: Write>(writer: &mut W, data: &[u8]) -> Result<usize, LoadError> {
    let mut used = 0;
    while used < data.len() {
        match writer.write(&data[used..]) {
            Ok(0) => break,
            Ok(n) => used += n,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(LoadError::Io(e.to_string())),
        }
    }
    Ok(used)
}

/// Load_Alloc_Data: bounded whole-stream allocation.
pub fn load_alloc_data<R: Read>(reader: &mut R, max_bytes: usize) -> Result<Vec<u8>, LoadError> {
    load_data(reader, max_bytes)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UncompressedBlock {
    pub method: CompressionMethod,
    pub declared_size: usize,
    /// HORIZONTAL is a no-op in the pinned build because
    /// LIB_EXTERNS_RESOLVED is FALSE in wwstd.h.
    pub output: Vec<u8>,
    pub writes_output: bool,
}

/// Uncompress_Data dispatch. Unknown, LZW12 and LZW14 use the source's
/// default NOCOMPRESS branch; HORIZONTAL has no active decoder in this build.
pub fn uncompress_data(src: &[u8], output_limit: usize) -> Result<UncompressedBlock, LoadError> {
    let header = CompHeader::parse(src)?;
    if header.output_size > output_limit {
        return Err(LoadError::OutputLimitExceeded { requested: header.output_size, limit: output_limit });
    }
    let start = COMP_HEADER_SIZE.checked_add(header.skip_size).ok_or(LoadError::SizeOverflow)?;
    let data = src.get(start..).ok_or(LoadError::TruncatedSkippedData)?;
    let (output, writes_output) = match header.method {
        CompressionMethod::Lcw => (decompress_expected(data, header.output_size)?, true),
        CompressionMethod::Horizontal => (Vec::new(), false),
        CompressionMethod::None | CompressionMethod::Lzw12 | CompressionMethod::Lzw14
        | CompressionMethod::Unknown(_) => {
            (data.get(..header.output_size).ok_or(LoadError::TruncatedCompressedData)?.to_vec(), true)
        }
    };
    Ok(UncompressedBlock { method: header.method, declared_size: header.output_size, output, writes_output })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoadedUncompressed {
    pub block: UncompressedBlock,
    pub staged_offset: usize,
    pub staged_source: Vec<u8>,
    pub reserved_data: Vec<u8>,
}

/// Load_Uncompress over a complete file image. The leading u16 is the byte
/// count following that prefix. Compressed input is staged at the end of the
/// scratch allocation, preserving the original alias-safe loading strategy.
pub fn load_uncompress(
    file: &[u8], scratch_capacity: usize, capture_reserved_data: bool, output_limit: usize,
) -> Result<LoadedUncompressed, LoadError> {
    let prefix = file.get(..2).ok_or(LoadError::TruncatedFileSize)?;
    let payload_size = u16::from_le_bytes([prefix[0], prefix[1]]) as usize;
    let payload_end = 2usize.checked_add(payload_size).ok_or(LoadError::SizeOverflow)?;
    let payload = file.get(2..payload_end).ok_or(LoadError::TruncatedCompressedData)?;
    let mut header = CompHeader::parse(payload)?;
    let after_header = payload.get(COMP_HEADER_SIZE..).ok_or(LoadError::TruncatedHeader)?;
    if header.skip_size > after_header.len() { return Err(LoadError::TruncatedSkippedData); }

    let reserved_data = if capture_reserved_data { after_header[..header.skip_size].to_vec() } else { Vec::new() };
    let compressed = &after_header[header.skip_size..];
    let staged_len = COMP_HEADER_SIZE.checked_add(compressed.len()).ok_or(LoadError::SizeOverflow)?;
    if staged_len > scratch_capacity {
        return Err(LoadError::ScratchBufferTooSmall { required: staged_len, capacity: scratch_capacity });
    }

    header.skip_size = 0;
    let mut staged_source = Vec::with_capacity(staged_len);
    staged_source.extend_from_slice(&header.encode()?);
    staged_source.extend_from_slice(compressed);
    let staged_offset = scratch_capacity - staged_len;
    let block = uncompress_data(&staged_source, output_limit)?;
    Ok(LoadedUncompressed { block, staged_offset, staged_source, reserved_data })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn header(method: u8, size: u32, skip: u16) -> Vec<u8> {
        let mut v = vec![method, 0];
        v.extend_from_slice(&size.to_le_bytes());
        v.extend_from_slice(&skip.to_le_bytes());
        v
    }

    #[test]
    fn header_uses_legacy_little_endian_layout() {
        let bytes = header(METHOD_LCW, 0x12345678, 0x2345);
        let h = CompHeader::parse(&bytes).unwrap();
        assert_eq!(h.method, CompressionMethod::Lcw);
        assert_eq!(h.output_size, 0x12345678);
        assert_eq!(h.skip_size, 0x2345);
        assert_eq!(h.encode().unwrap().to_vec(), bytes);
    }

    #[test]
    fn raw_and_horizontal_modes_match_active_compile_configuration() {
        let mut raw = header(METHOD_NONE, 3, 0);
        raw.extend_from_slice(b"abc");
        assert_eq!(uncompress_data(&raw, 8).unwrap().output, b"abc");
        let mut horizontal = header(METHOD_HORIZONTAL, 3, 0);
        horizontal.extend_from_slice(b"abc");
        let result = uncompress_data(&horizontal, 8).unwrap();
        assert!(!result.writes_output);
        assert!(result.output.is_empty());
        assert_eq!(result.declared_size, 3);
    }

    #[test]
    fn unsupported_methods_follow_default_raw_copy_branch() {
        for method in [METHOD_LZW12, METHOD_LZW14, 0x7f] {
            let mut bytes = header(method, 2, 0);
            bytes.extend_from_slice(b"ok");
            assert_eq!(uncompress_data(&bytes, 4).unwrap().output, b"ok");
        }
    }

    #[test]
    fn load_uncompress_captures_skip_data_and_stages_at_buffer_end() {
        let mut payload = header(METHOD_NONE, 3, 2);
        payload.extend_from_slice(b"rsabc");
        let mut file = (payload.len() as u16).to_le_bytes().to_vec();
        file.extend_from_slice(&payload);
        let loaded = load_uncompress(&file, 32, true, 16).unwrap();
        assert_eq!(loaded.reserved_data, b"rs");
        assert_eq!(loaded.block.output, b"abc");
        assert_eq!(loaded.staged_offset, 19);
        assert_eq!(&loaded.staged_source[6..8], &[0, 0]);
    }

    #[test]
    fn load_write_counts_and_malformed_input() {
        let mut source = &b"abcdef"[..];
        assert_eq!(load_data(&mut source, 3).unwrap(), b"abc");
        let mut output = Vec::new();
        assert_eq!(write_data(&mut output, b"abc").unwrap(), 3);
        assert_eq!(output, b"abc");
        assert_eq!(load_uncompress(&[4, 0, 0], 64, false, 64), Err(LoadError::TruncatedCompressedData));
        let mut raw = header(METHOD_NONE, 5, 0);
        raw.extend_from_slice(b"short");
        assert!(matches!(uncompress_data(&raw, 4), Err(LoadError::OutputLimitExceeded { .. })));
    }
}
