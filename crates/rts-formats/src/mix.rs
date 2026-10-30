use core::fmt;

use crate::binary::{BinaryReadError, BinaryReader};

const LEGACY_MIX_HEADER_SIZE: usize = 6;
const LEGACY_MIX_ENTRY_SIZE: usize = 12;

/// One index record in the legacy MIX layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MixEntry {
    /// Signed 32-bit CRC field, matching the upstream Windows `long` comparison.
    pub crc: i32,
    /// Offset relative to the start of the MIX data section.
    pub offset: u32,
    pub size: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MixParseError {
    Binary(BinaryReadError),
    NegativeFileCount(i16),
    NegativeDataSize(i32),
    UnsortedIndex { previous_crc: i32, current_crc: i32 },
    EntryOutOfBounds { crc: i32, offset: u32, size: u32, data_size: u32 },
    TruncatedDataSection { required_end: usize, file_size: usize },
    SizeOverflow,
}

impl fmt::Display for MixParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Binary(error) => write!(formatter, "{error}"),
            Self::NegativeFileCount(count) => write!(formatter, "negative MIX entry count: {count}"),
            Self::NegativeDataSize(size) => write!(formatter, "negative MIX data size: {size}"),
            Self::UnsortedIndex { previous_crc, current_crc } => {
                write!(formatter, "MIX index is not CRC-sorted: {previous_crc} precedes {current_crc}")
            }
            Self::EntryOutOfBounds { crc, offset, size, data_size } => write!(
                formatter,
                "MIX entry {crc} range ({offset} + {size}) exceeds data section size {data_size}"
            ),
            Self::TruncatedDataSection { required_end, file_size } => write!(
                formatter,
                "MIX data section ends at {required_end}, beyond file size {file_size}"
            ),
            Self::SizeOverflow => write!(formatter, "MIX archive size arithmetic overflow"),
        }
    }
}

impl std::error::Error for MixParseError {}

impl From<BinaryReadError> for MixParseError {
    fn from(error: BinaryReadError) -> Self {
        Self::Binary(error)
    }
}

/// A view over the legacy MIX archive layout consumed by upstream MixFileClass.
///
/// The upstream project packs structures to one-byte alignment: a 16-bit signed count,
/// a 32-bit signed data size, then 12-byte records containing CRC, relative offset, size.
#[derive(Debug, Clone)]
pub struct MixArchive<'a> {
    bytes: &'a [u8],
    data_offset: usize,
    data_size: usize,
    entries: Vec<MixEntry>,
}

impl<'a> MixArchive<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self, MixParseError> {
        let mut reader = BinaryReader::new(bytes);
        let count = reader.read_i16_le()?;
        let signed_data_size = reader.read_i32_le()?;

        if count < 0 {
            return Err(MixParseError::NegativeFileCount(count));
        }
        if signed_data_size < 0 {
            return Err(MixParseError::NegativeDataSize(signed_data_size));
        }

        let count = usize::try_from(count).map_err(|_| MixParseError::SizeOverflow)?;
        let data_size = usize::try_from(signed_data_size).map_err(|_| MixParseError::SizeOverflow)?;
        let index_size = count
            .checked_mul(LEGACY_MIX_ENTRY_SIZE)
            .ok_or(MixParseError::SizeOverflow)?;
        let data_offset = LEGACY_MIX_HEADER_SIZE
            .checked_add(index_size)
            .ok_or(MixParseError::SizeOverflow)?;
        let required_end = data_offset
            .checked_add(data_size)
            .ok_or(MixParseError::SizeOverflow)?;

        let mut entries: Vec<MixEntry> = Vec::with_capacity(count);
        for _ in 0..count {
            let crc = reader.read_i32_le()?;
            let signed_offset = reader.read_i32_le()?;
            let signed_size = reader.read_i32_le()?;

            if signed_offset < 0 || signed_size < 0 {
                return Err(MixParseError::EntryOutOfBounds {
                    crc,
                    offset: signed_offset.max(0) as u32,
                    size: signed_size.max(0) as u32,
                    data_size: data_size.min(u32::MAX as usize) as u32,
                });
            }

            let offset = signed_offset as u32;
            let size = signed_size as u32;
            let entry_end = u64::from(offset) + u64::from(size);
            if entry_end > data_size as u64 {
                return Err(MixParseError::EntryOutOfBounds {
                    crc,
                    offset,
                    size,
                    data_size: data_size.min(u32::MAX as usize) as u32,
                });
            }

            if let Some(previous) = entries.last() {
                if previous.crc > crc {
                    return Err(MixParseError::UnsortedIndex {
                        previous_crc: previous.crc,
                        current_crc: crc,
                    });
                }
            }

            entries.push(MixEntry { crc, offset, size });
        }

        if required_end > bytes.len() {
            return Err(MixParseError::TruncatedDataSection {
                required_end,
                file_size: bytes.len(),
            });
        }

        Ok(Self { bytes, data_offset, data_size, entries })
    }

    pub fn entries(&self) -> &[MixEntry] {
        &self.entries
    }

    pub fn data_section(&self) -> &'a [u8] {
        &self.bytes[self.data_offset..self.data_offset + self.data_size]
    }

    pub fn find_by_crc(&self, crc: i32) -> Option<&MixEntry> {
        self.entries
            .binary_search_by_key(&crc, |entry| entry.crc)
            .ok()
            .map(|index| &self.entries[index])
    }

    pub fn file_data(&self, entry: &MixEntry) -> Option<&'a [u8]> {
        let offset = usize::try_from(entry.offset).ok()?;
        let size = usize::try_from(entry.size).ok()?;
        let start = self.data_offset.checked_add(offset)?;
        let end = start.checked_add(size)?;
        if end > self.data_offset + self.data_size || end > self.bytes.len() {
            return None;
        }
        Some(&self.bytes[start..end])
    }

    pub fn get_by_crc(&self, crc: i32) -> Option<&'a [u8]> {
        self.find_by_crc(crc).and_then(|entry| self.file_data(entry))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn one_entry_archive(offset: i32, size: i32, data_size: i32) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&1_i16.to_le_bytes());
        bytes.extend_from_slice(&data_size.to_le_bytes());
        bytes.extend_from_slice(&0x1234_i32.to_le_bytes());
        bytes.extend_from_slice(&offset.to_le_bytes());
        bytes.extend_from_slice(&size.to_le_bytes());
        bytes.extend_from_slice(b"abc");
        bytes
    }

    #[test]
    fn parses_legacy_header_index_and_relative_payload() {
        let bytes = one_entry_archive(0, 3, 3);
        let archive = MixArchive::parse(&bytes).expect("valid MIX");
        assert_eq!(archive.entries().len(), 1);
        assert_eq!(archive.get_by_crc(0x1234), Some(&b"abc"[..]));
        assert_eq!(archive.get_by_crc(0x4321), None);
    }

    #[test]
    fn rejects_entry_outside_data_section() {
        let bytes = one_entry_archive(2, 3, 3);
        assert!(matches!(
            MixArchive::parse(&bytes),
            Err(MixParseError::EntryOutOfBounds { .. })
        ));
    }

    #[test]
    fn rejects_truncated_data_section() {
        let mut bytes = one_entry_archive(0, 3, 4);
        bytes.truncate(20);
        assert!(matches!(
            MixArchive::parse(&bytes),
            Err(MixParseError::TruncatedDataSection { .. })
        ));
    }

    #[test]
    fn rejects_crc_index_out_of_order() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&2_i16.to_le_bytes());
        bytes.extend_from_slice(&0_i32.to_le_bytes());
        for crc in [10_i32, 5_i32] {
            bytes.extend_from_slice(&crc.to_le_bytes());
            bytes.extend_from_slice(&0_i32.to_le_bytes());
            bytes.extend_from_slice(&0_i32.to_le_bytes());
        }
        assert!(matches!(
            MixArchive::parse(&bytes),
            Err(MixParseError::UnsortedIndex { .. })
        ));
    }
}
