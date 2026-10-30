//! Westwood fixed-dictionary digram text compression from both WIN32LIB/DIPTHONG.CPP files.

use crate::dipthong_tables::{COMMON, DIPTHONG};

/// The legacy keyboard escape marker is VK_ESCAPE (0x1b) when stored in a byte string.
pub const KA_EXTEND: u8 = 0x1b;

/// Compress a NUL-terminated legacy byte string using the original greedy pair scan.
/// Returns the encoded bytes without the terminating NUL.
pub fn dip_text(source: &[u8]) -> Vec<u8> {
    let input = nul_terminated(source);
    let mut out = Vec::with_capacity(input.len());
    let mut i = 0;
    while i < input.len() {
        let mut first = input[i];
        let next = input.get(i + 1).copied().unwrap_or(0);
        if first > 127 {
            out.push(KA_EXTEND);
            first = first.wrapping_sub(127);
        } else {
            'rows: for (row, common) in COMMON.iter().copied().enumerate() {
                if common == first {
                    for column in 0..8 {
                        if DIPTHONG[row][column] == next {
                            first = 0x80 | ((row as u8) << 3) | column as u8;
                            i += 1;
                            break 'rows;
                        }
                    }
                }
            }
        }
        out.push(first);
        i += 1;
    }
    out
}

/// Expand a NUL-terminated digram string. Output excludes the terminating NUL.
pub fn undip_text(source: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(source.len().saturating_mul(2));
    for mut byte in nul_terminated(source).iter().copied() {
        if byte & 0x80 != 0 {
            byte &= 0x7f;
            let row = ((byte & 0x78) >> 3) as usize;
            out.push(COMMON[row]);
            byte = DIPTHONG[row][(byte & 7) as usize];
        }
        out.push(byte);
    }
    out
}

/// Convert extended keyboard escape pairs to their original high-byte values.
/// A trailing escape marker is retained as-is, matching the source's byte-wise
/// behavior without reading beyond the supplied slice.
pub fn fixup_text(source: &[u8]) -> Vec<u8> {
    let input = nul_terminated(source);
    let mut out = Vec::with_capacity(input.len());
    let mut i = 0;
    while i < input.len() {
        if input[i] == KA_EXTEND && i + 1 < input.len() {
            out.push(input[i + 1].wrapping_add(127));
            i += 2;
        } else {
            out.push(input[i]);
            i += 1;
        }
    }
    out
}

fn nul_terminated(bytes: &[u8]) -> &[u8] {
    &bytes[..bytes.iter().position(|b| *b == 0).unwrap_or(bytes.len())]
}

/// The 22 hard-coded Internet UI strings returned for string IDs 4567..=4588.
pub const INTERNET_TEXT: [&str; 22] = [
    "Internet H2H",
    "Host Internet Game",
    "Join Internet Game",
    "Guest",
    "Login",
    "Login to Planet Westwood",
    "Planet Westwood Handle",
    "Planet Westwood Password",
    "Host Game",
    "Join Game",
    "Choose Type of Internet Game",
    "Join Internet Game",
    "Address of Host",
    "Connecting...",
    "Connection Error!",
    "Unable to connect to host!",
    "Connecting to host...",
    "Unable to resolve host address!",
    "Unable to accept client connection",
    "Unable to connect!",
    "Connection lost!",
    "Resolving address of host...",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtractStringError {
    NullData,
    NegativeIndex,
    IndexOutOfRange,
    OffsetOutOfRange,
}

/// Resolve an Extract_String index against a byte block of little-endian offsets.
/// Internet IDs return the source's static text instead of reading the block.
pub fn extract_string<'a>(
    data: Option<&'a [u8]>,
    string: i32,
) -> Result<ExtractedString<'a>, ExtractStringError> {
    if string < 0 {
        return Err(ExtractStringError::NegativeIndex);
    }
    let index = string as usize;
    if index >= 4567 {
        return INTERNET_TEXT
            .get(index - 4567)
            .copied()
            .map(ExtractedString::Static)
            .ok_or(ExtractStringError::IndexOutOfRange);
    }
    let bytes = data.ok_or(ExtractStringError::NullData)?;
    let pos = index.checked_mul(2).ok_or(ExtractStringError::IndexOutOfRange)?;
    let pair = bytes.get(pos..pos + 2).ok_or(ExtractStringError::IndexOutOfRange)?;
    let offset = u16::from_le_bytes([pair[0], pair[1]]) as usize;
    if offset >= bytes.len() {
        return Err(ExtractStringError::OffsetOutOfRange);
    }
    let tail = &bytes[offset..];
    let end = tail.iter().position(|b| *b == 0).unwrap_or(tail.len());
    Ok(ExtractedString::Data(&tail[..end]))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtractedString<'a> {
    Data(&'a [u8]),
    Static(&'static str),
}
