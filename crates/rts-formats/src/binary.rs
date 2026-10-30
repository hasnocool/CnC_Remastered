use core::fmt;

/// A checked little-endian reader for untrusted binary data.
#[derive(Debug, Clone)]
pub struct BinaryReader<'a> {
    bytes: &'a [u8],
    offset: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BinaryReadError {
    pub offset: usize,
    pub requested: usize,
    pub remaining: usize,
}

impl fmt::Display for BinaryReadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "unexpected end of input at byte {}: requested {}, remaining {}",
            self.offset, self.requested, self.remaining
        )
    }
}

impl std::error::Error for BinaryReadError {}

impl<'a> BinaryReader<'a> {
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    pub const fn position(&self) -> usize {
        self.offset
    }

    pub const fn remaining(&self) -> usize {
        self.bytes.len() - self.offset
    }

    pub fn read_exact(&mut self, length: usize) -> Result<&'a [u8], BinaryReadError> {
        let end = self.offset.checked_add(length).ok_or(BinaryReadError {
            offset: self.offset,
            requested: length,
            remaining: self.remaining(),
        })?;

        if end > self.bytes.len() {
            return Err(BinaryReadError {
                offset: self.offset,
                requested: length,
                remaining: self.remaining(),
            });
        }

        let start = self.offset;
        self.offset = end;
        Ok(&self.bytes[start..end])
    }

    pub fn read_u8(&mut self) -> Result<u8, BinaryReadError> {
        Ok(self.read_exact(1)?[0])
    }

    pub fn read_i16_le(&mut self) -> Result<i16, BinaryReadError> {
        let bytes = self.read_exact(2)?;
        Ok(i16::from_le_bytes([bytes[0], bytes[1]]))
    }

    pub fn read_u16_le(&mut self) -> Result<u16, BinaryReadError> {
        let bytes = self.read_exact(2)?;
        Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    pub fn read_i32_le(&mut self) -> Result<i32, BinaryReadError> {
        let bytes = self.read_exact(4)?;
        Ok(i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    pub fn read_u32_le(&mut self) -> Result<u32, BinaryReadError> {
        let bytes = self.read_exact(4)?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_little_endian_values() {
        let bytes = [0x34, 0x12, 0x78, 0x56, 0x34, 0x12];
        let mut reader = BinaryReader::new(&bytes);
        assert_eq!(reader.read_u16_le(), Ok(0x1234));
        assert_eq!(reader.read_u32_le(), Ok(0x1234_5678));
        assert_eq!(reader.remaining(), 0);
    }

    #[test]
    fn failed_read_does_not_advance_cursor() {
        let bytes = [1, 2, 3];
        let mut reader = BinaryReader::new(&bytes);
        let error = reader.read_u32_le().expect_err("input is truncated");
        assert_eq!(error.offset, 0);
        assert_eq!(reader.position(), 0);
    }
}
