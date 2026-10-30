//! Westwood Base64 encoder/decoder ported from REDALERT/BASE64.CPP.
//!
//! This preserves the original 4-character packet conversion, ignored invalid
//! characters, '=' end marker, fixed output capacities, and partial-packet
//! decoding behavior. The codec intentionally exposes packet-level functions;
//! Pipe/Straw buffering lives in base64_streams.rs.

pub const ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncodeResult {
    pub written: usize,
    pub consumed: usize,
}

fn encode_packet(source: &[u8]) -> [u8; 4] {
    let first = source.first().copied().unwrap_or(0);
    let second = source.get(1).copied().unwrap_or(0);
    let third = source.get(2).copied().unwrap_or(0);

    let o1 = first >> 2;
    let o2 = ((first & 0x03) << 4) | (second >> 4);
    let o3 = ((second & 0x0F) << 2) | (third >> 6);
    let o4 = third & 0x3F;

    let padding = 3 - source.len();
    [
        ALPHABET[usize::from(o1)],
        ALPHABET[usize::from(o2)],
        if padding >= 2 { b'=' } else { ALPHABET[usize::from(o3)] },
        if padding >= 1 { b'=' } else { ALPHABET[usize::from(o4)] },
    ]
}

/// Port of Base64_Encode(source, slen, dest, dlen).
///
/// It encodes as many complete 3-byte input groups as fit into the supplied
/// output capacity. A final 1- or 2-byte group is padded and emitted when four
/// output bytes are available. When one destination byte remains, the original
/// function writes a trailing NUL courtesy byte but does not count it.
pub fn encode_into(source: &[u8], destination: &mut [u8]) -> EncodeResult {
    if source.is_empty() || destination.is_empty() {
        return EncodeResult { written: 0, consumed: 0 };
    }

    let mut consumed = 0_usize;
    let mut written = 0_usize;

    while consumed < source.len() && destination.len() - written >= 4 {
        let count = (source.len() - consumed).min(3);
        let packet = encode_packet(&source[consumed..consumed + count]);
        destination[written..written + 4].copy_from_slice(&packet);
        consumed += count;
        written += 4;
    }

    if written < destination.len() {
        destination[written] = 0;
    }

    EncodeResult { written, consumed }
}

fn decode_value(byte: u8) -> Option<u8> {
    match byte {
        b'A'..=b'Z' => Some(byte - b'A'),
        b'a'..=b'z' => Some(byte - b'a' + 26),
        b'0'..=b'9' => Some(byte - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

/// Port of Base64_Decode(source, slen, dest, dlen).
///
/// Characters outside the Base64 alphabet are ignored. '=' ends the decode
/// call immediately, regardless of remaining source bytes. As in the original
/// implementation, one first output byte is written for each non-empty input
/// packet even when fewer than two valid alphabet characters were collected.
pub fn decode_into(source: &[u8], destination: &mut [u8]) -> usize {
    if source.is_empty() || destination.is_empty() {
        return 0;
    }

    let mut input_cursor = 0_usize;
    let mut output_cursor = 0_usize;

    while input_cursor < source.len() && output_cursor < destination.len() {
        let mut values = [0_u8; 4];
        let mut count = 0_usize;

        while count < 4 && input_cursor < source.len() {
            let byte = source[input_cursor];
            input_cursor += 1;

            if byte == b'=' {
                input_cursor = source.len();
                break;
            }

            if let Some(value) = decode_value(byte) {
                values[count] = value;
                count += 1;
            }
        }

        // These byte expressions are the little-endian PacketType union
        // layout used by the original Win32 build.
        let first = (values[0] << 2) | (values[1] >> 4);
        destination[output_cursor] = first;
        output_cursor += 1;

        if output_cursor < destination.len() && count > 2 {
            let second = ((values[1] & 0x0F) << 4) | (values[2] >> 2);
            destination[output_cursor] = second;
            output_cursor += 1;
        }

        if output_cursor < destination.len() && count > 3 {
            let third = ((values[2] & 0x03) << 6) | values[3];
            destination[output_cursor] = third;
            output_cursor += 1;
        }
    }

    output_cursor
}

/// Convenience function for complete input buffers.
pub fn encode_all(source: &[u8]) -> Vec<u8> {
    let capacity = source.len().saturating_add(2) / 3 * 4;
    let mut output = vec![0_u8; capacity];
    let result = encode_into(source, &mut output);
    output.truncate(result.written);
    output
}

/// Convenience function for complete input buffers.
///
/// The caller still controls the output limit because the legacy decoder does
/// not carry a destination length in its encoded input format.
pub fn decode_all(source: &[u8], output_limit: usize) -> Vec<u8> {
    let mut output = vec![0_u8; output_limit];
    let written = decode_into(source, &mut output);
    output.truncate(written);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_alphabet_vectors_match_expected_output() {
        assert_eq!(encode_all(b""), b"");
        assert_eq!(encode_all(b"f"), b"Zg==");
        assert_eq!(encode_all(b"fo"), b"Zm8=");
        assert_eq!(encode_all(b"foo"), b"Zm9v");
        assert_eq!(encode_all(b"foobar"), b"Zm9vYmFy");
    }

    #[test]
    fn round_trips_all_final_packet_lengths() {
        for bytes in [
            b"abc".as_slice(),
            b"abcd".as_slice(),
            b"abcde".as_slice(),
            b"abcdef".as_slice(),
            b"arbitrary binary payload\0\xFF".as_slice(),
        ] {
            assert_eq!(decode_all(&encode_all(bytes), bytes.len() + 1)[..bytes.len()], *bytes);
        }
    }

    #[test]
    fn capacity_consumption_matches_fixed_four_character_packets() {
        let mut output = [0xAA; 5];
        let result = encode_into(b"abcdef", &mut output);
        assert_eq!(result.written, 4);
        assert_eq!(result.consumed, 3);
        assert_eq!(&output[..4], b"YWJj");
        assert_eq!(output[4], 0);
    }

    #[test]
    fn decoder_ignores_invalid_bytes_and_stops_at_padding() {
        assert_eq!(decode_all(b"Z m9v!YmFy", 32), b"foo");
        assert_eq!(decode_all(b"Zm9v=ignored", 32), b"foo");
    }

    #[test]
    fn decoder_preserves_legacy_partial_packet_behavior() {
        assert_eq!(decode_all(b"Zg", 8), b"f");
        assert_eq!(decode_all(b"Z", 8), [b'\0']);
        assert_eq!(decode_all(b"\n\r", 8), [b'\0']);
    }

    #[test]
    fn decoder_obeys_destination_capacity() {
        let mut output = [0_u8; 2];
        assert_eq!(decode_into(b"Zm9v", &mut output), 2);
        assert_eq!(&output, b"fo");
    }
}
