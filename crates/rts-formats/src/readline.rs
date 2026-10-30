//! Line-oriented input helpers ported from REDALERT/READLINE.H and READLINE.CPP.

use std::io::{self, Read};

/// Byte-at-a-time source compatible with the legacy Straw interface.
pub trait ByteSource {
    fn read_byte(&mut self) -> io::Result<Option<u8>>;
}

impl<R: Read> ByteSource for R {
    fn read_byte(&mut self) -> io::Result<Option<u8>> {
        let mut byte = [0_u8; 1];
        loop {
            match self.read(&mut byte) {
                Ok(0) => return Ok(None),
                Ok(_) => return Ok(Some(byte[0])),
                Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
                Err(error) => return Err(error),
            }
        }
    }
}

impl ByteSource for crate::straw::StrawHandle {
    fn read_byte(&mut self) -> io::Result<Option<u8>> {
        let mut byte = [0_u8; 1];
        match self.get(&mut byte) {
            0 => Ok(None),
            _ => Ok(Some(byte[0])),
        }
    }
}

/// Port of strtrimcpp: strip leading and trailing ASCII whitespace from a
/// NUL-terminated byte buffer. Returns the resulting byte length.
pub fn trim_c_buffer(buffer: &mut [u8]) -> usize {
    if buffer.is_empty() {
        return 0;
    }

    let nul = buffer.iter().position(|byte| *byte == 0).unwrap_or(buffer.len());
    let mut start = 0;
    while start < nul && buffer[start].is_ascii_whitespace() {
        start += 1;
    }

    let mut end = nul;
    while end > start && buffer[end - 1].is_ascii_whitespace() {
        end -= 1;
    }

    let length = end - start;
    if start != 0 && length != 0 {
        buffer.copy_within(start..end, 0);
    }
    buffer[length] = 0;
    length
}

/// Port of Read_Line(Straw&, char*, int, bool&).
///
/// Reads through LF, drops CR bytes, and leaves one NUL byte for termination.
/// A long line is fully consumed while excess bytes are discarded. Reaching
/// EOF before LF sets `eof` and discards any partial line, matching the source.
pub fn read_line(
    source: &mut impl ByteSource,
    destination: &mut [u8],
    eof: &mut bool,
) -> io::Result<usize> {
    if destination.is_empty() {
        return Ok(0);
    }

    let mut count = 0;
    loop {
        match source.read_byte()? {
            None => {
                *eof = true;
                destination[0] = 0;
                count = 0;
                break;
            }
            Some(b'\n') => break,
            Some(b'\r') => {}
            Some(byte) if count + 1 < destination.len() => {
                destination[count] = byte;
                count += 1;
            }
            Some(_) => {}
        }
    }

    destination[count] = 0;
    Ok(trim_c_buffer(destination))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn trims_leading_and_trailing_ascii_whitespace() {
        let mut buffer = *b"  Westwood \t \0";
        let len = trim_c_buffer(&mut buffer);
        assert_eq!(len, 8);
        assert_eq!(&buffer[..len], b"Westwood");
        assert_eq!(buffer[len], 0);
    }

    #[test]
    fn reads_crlf_and_trims_result() {
        let mut source = Cursor::new(b"  hello world  \r\nnext\n".to_vec());
        let mut output = [0_u8; 32];
        let mut eof = false;

        assert_eq!(read_line(&mut source, &mut output, &mut eof).unwrap(), 11);
        assert_eq!(&output[..11], b"hello world");
        assert!(!eof);

        assert_eq!(read_line(&mut source, &mut output, &mut eof).unwrap(), 4);
        assert_eq!(&output[..4], b"next");
        assert!(!eof);
    }

    #[test]
    fn overlong_line_is_consumed_and_truncated_without_overflow() {
        let mut source = Cursor::new(b"abcdefghij\nZ\n".to_vec());
        let mut output = [0_u8; 5];
        let mut eof = false;

        assert_eq!(read_line(&mut source, &mut output, &mut eof).unwrap(), 4);
        assert_eq!(&output, b"abcd");
        assert_eq!(read_line(&mut source, &mut output, &mut eof).unwrap(), 1);
        assert_eq!(&output[..2], b"Z\0");
    }

    #[test]
    fn eof_discards_partial_line_and_sets_flag() {
        let mut source = Cursor::new(b"unterminated".to_vec());
        let mut output = [0_u8; 32];
        let mut eof = false;

        assert_eq!(read_line(&mut source, &mut output, &mut eof).unwrap(), 0);
        assert!(eof);
        assert_eq!(output[0], 0);
    }

    #[test]
    fn empty_destination_does_not_consume_input() {
        let mut source = Cursor::new(b"one\n".to_vec());
        let mut eof = false;
        assert_eq!(read_line(&mut source, &mut [], &mut eof).unwrap(), 0);
        let mut output = [0_u8; 8];
        assert_eq!(read_line(&mut source, &mut output, &mut eof).unwrap(), 3);
        assert_eq!(&output[..3], b"one");
    }

    #[test]
    fn reads_from_linked_straw() {
        let bytes = crate::straw::StrawHandle::new(crate::straw::SliceStraw::new(b"stream\n".to_vec()));
        let mut input = crate::straw::StrawHandle::new(crate::straw::EmptyStraw);
        input.get_from(Some(bytes)).unwrap();

        let mut output = [0_u8; 16];
        let mut eof = false;
        assert_eq!(read_line(&mut input, &mut output, &mut eof).unwrap(), 6);
        assert_eq!(&output[..6], b"stream");
    }
}
