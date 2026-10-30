//! IFF FORM/chunk access from the shared REDALERT and TIBERIANDAWN
//! WIN32LIB/IFF.CPP implementation.

use std::fmt;

pub const FORM_ID: [u8; 4] = *b"FORM";
pub const FORM_HEADER_LEN: usize = 12;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IffError {
    NotForm,
    TruncatedHeader,
    InvalidChunkBounds,
    LengthOverflow,
    InvalidCursor,
}

impl fmt::Display for IffError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for IffError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IffFile {
    bytes: Vec<u8>,
    cursor: usize,
}

impl IffFile {
    /// Open an in-memory IFF file. The legacy routine validates only FORM.
    pub fn open(bytes: Vec<u8>) -> Result<Self, IffError> {
        if bytes.len() < 4 {
            return Err(IffError::TruncatedHeader);
        }
        if bytes[..4] != FORM_ID {
            return Err(IffError::NotForm);
        }
        if bytes.len() < FORM_HEADER_LEN {
            return Err(IffError::TruncatedHeader);
        }
        Ok(Self { bytes, cursor: 8 })
    }

    pub fn bytes(&self) -> &[u8] { &self.bytes }
    pub fn position(&self) -> usize { self.cursor }

    pub fn seek(&mut self, position: usize) -> Result<(), IffError> {
        if position > self.bytes.len() { return Err(IffError::InvalidCursor); }
        self.cursor = position;
        Ok(())
    }

    fn chunk_at(&self, at: usize) -> Result<([u8; 4], usize, usize), IffError> {
        let head = self.bytes.get(at..at.checked_add(8).ok_or(IffError::LengthOverflow)?)
            .ok_or(IffError::TruncatedHeader)?;
        let id = [head[0], head[1], head[2], head[3]];
        let size = u32::from_be_bytes([head[4], head[5], head[6], head[7]]) as usize;
        let data = at.checked_add(8).ok_or(IffError::LengthOverflow)?;
        let padded = size.checked_add(size & 1).ok_or(IffError::LengthOverflow)?;
        let end = data.checked_add(padded).ok_or(IffError::LengthOverflow)?;
        if end > self.bytes.len() { return Err(IffError::InvalidChunkBounds); }
        Ok((id, size, data))
    }

    /// Search the current position first, then continue from the first chunk
    /// at byte 12, matching the legacy first-iteration FORM-type check.
    fn find_chunk(&mut self, id: [u8; 4]) -> Result<Option<(usize, usize)>, IffError> {
        let mut at = self.cursor;
        let mut first = true;
        loop {
            if at >= self.bytes.len() { return Ok(None); }
            if at.checked_add(8).filter(|&v| v <= self.bytes.len()).is_none() {
                return if first { Err(IffError::TruncatedHeader) } else { Ok(None) };
            }
            let (found, size, data) = self.chunk_at(at)?;
            if found == id {
                return Ok(Some((at, size)));
            }
            if first {
                at = FORM_HEADER_LEN;
                first = false;
            } else {
                let padded = size.checked_add(size & 1).ok_or(IffError::LengthOverflow)?;
                at = data.checked_add(padded).ok_or(IffError::LengthOverflow)?;
            }
        }
    }

    /// Return the first matching chunk size and leave the cursor at its header.
    pub fn chunk_size(&mut self, id: [u8; 4]) -> Result<Option<usize>, IffError> {
        match self.find_chunk(id)? {
            Some((at, size)) => { self.cursor = at; Ok(Some(size)) }
            None => Ok(None),
        }
    }

    /// Read up to max_size bytes from the first matching chunk. The cursor
    /// advances to the next even-aligned chunk, as in Read_Iff_Chunk.
    pub fn read_chunk(&mut self, id: [u8; 4], max_size: usize) -> Result<Option<Vec<u8>>, IffError> {
        let Some((at, size)) = self.find_chunk(id)? else { return Ok(None) };
        let (_, _, data) = self.chunk_at(at)?;
        let count = size.min(max_size);
        let end = data.checked_add(count).ok_or(IffError::LengthOverflow)?;
        let padded_end = data.checked_add(size.checked_add(size & 1).ok_or(IffError::LengthOverflow)?)
            .ok_or(IffError::LengthOverflow)?;
        self.cursor = padded_end;
        Ok(Some(self.bytes[data..end].to_vec()))
    }

    /// Append an IFF chunk at the current cursor, update its big-endian size,
    /// and update the FORM size when the write extends the file.
    pub fn write_chunk(&mut self, id: [u8; 4], data: &[u8]) -> Result<(), IffError> {
        if data.is_empty() { return Ok(()); }
        if self.cursor > self.bytes.len() { return Err(IffError::InvalidCursor); }
        let length = u32::try_from(data.len()).map_err(|_| IffError::LengthOverflow)?;
        let start = self.cursor;
        let old_end = self.bytes.len();
        let header_end = start.checked_add(8).ok_or(IffError::LengthOverflow)?;
        let data_end = header_end.checked_add(data.len()).ok_or(IffError::LengthOverflow)?;
        let end = data_end.checked_add(data.len() & 1).ok_or(IffError::LengthOverflow)?;
        if end > self.bytes.len() { self.bytes.resize(end, 0); }
        self.bytes[start..start + 4].copy_from_slice(&id);
        self.bytes[start + 4..start + 8].copy_from_slice(&length.to_be_bytes());
        self.bytes[header_end..data_end].copy_from_slice(data);
        if data.len() & 1 != 0 { self.bytes[data_end] = 0; }
        if old_end < data_end {
            let form_size = u32::try_from(end.saturating_sub(8)).map_err(|_| IffError::LengthOverflow)?;
            self.bytes[4..8].copy_from_slice(&form_size.to_be_bytes());
        }
        self.cursor = self.bytes.len();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<u8> {
        let mut b = b"FORM\0\0\0\x16ILBM".to_vec();
        b.extend_from_slice(b"BODY\0\0\0\x03abc\0");
        b
    }

    #[test]
    fn validates_form_and_reads_even_padded_chunks() {
        let mut iff = IffFile::open(sample()).unwrap();
        assert_eq!(iff.read_chunk(*b"BODY", 8).unwrap(), Some(b"abc".to_vec()));
        assert_eq!(iff.position(), 24);
    }

    #[test]
    fn reports_chunk_size_and_restores_header_position() {
        let mut iff = IffFile::open(sample()).unwrap();
        assert_eq!(iff.chunk_size(*b"BODY").unwrap(), Some(3));
        assert_eq!(iff.position(), 12);
    }

    #[test]
    fn writes_chunk_and_updates_form_size() {
        let mut iff = IffFile::open(b"FORM\0\0\0\x04ILBM".to_vec()).unwrap();
        iff.seek(12).unwrap();
        iff.write_chunk(*b"BODY", b"abc").unwrap();
        assert_eq!(&iff.bytes()[12..16], b"BODY");
        assert_eq!(&iff.bytes()[16..20], &[0, 0, 0, 3]);
        assert_eq!(&iff.bytes()[20..23], b"abc");
        assert_eq!(&iff.bytes()[4..8], &[0, 0, 0, 16]);
    }
}
