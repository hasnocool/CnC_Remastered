//! Westwood's deterministic LZO1X-1 codec ported from REDALERT/LZO1X_C.CPP
//! and LZO1X_D.CPP.
//!
//! This is the 1996 LZO1X format used by the game's block streams. It is not
//! interchangeable with LZ4, raw LZO variants, or the LZO1Y dialect.

use std::collections::HashMap;

pub const LZO_EOF_MARKER: [u8; 3] = [0x11, 0x00, 0x00];
pub const LZO_M2_MAX_OFFSET: usize = 0x0800;
pub const LZO_M3_MAX_OFFSET: usize = 0x4000;
pub const LZO_M4_MAX_OFFSET: usize = 0xBFFF;
const LZO_DICTIONARY_SIZE: usize = 0x4000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LzoError {
    UnexpectedEndOfInput,
    MissingEndMarker,
    InvalidBackReference { distance: usize, output_len: usize },
    OutputLimitExceeded { requested: usize, limit: usize },
    LengthOverflow,
    TrailingInput(usize),
}

fn reserve_output(output: &[u8], count: usize, limit: usize) -> Result<(), LzoError> {
    let requested = output
        .len()
        .checked_add(count)
        .ok_or(LzoError::LengthOverflow)?;
    if requested > limit {
        return Err(LzoError::OutputLimitExceeded { requested, limit });
    }
    Ok(())
}

fn read_byte(input: &[u8], cursor: &mut usize) -> Result<u8, LzoError> {
    let value = *input.get(*cursor).ok_or(LzoError::UnexpectedEndOfInput)?;
    *cursor += 1;
    Ok(value)
}

fn read_extended(input: &[u8], cursor: &mut usize, base: usize) -> Result<usize, LzoError> {
    let mut count = base;
    loop {
        let value = usize::from(read_byte(input, cursor)?);
        count = count.checked_add(value).ok_or(LzoError::LengthOverflow)?;
        if value != 0 {
            return Ok(count);
        }
        count = count.checked_add(255).ok_or(LzoError::LengthOverflow)?;
    }
}

fn copy_literals(
    input: &[u8],
    cursor: &mut usize,
    output: &mut Vec<u8>,
    count: usize,
    limit: usize,
) -> Result<(), LzoError> {
    reserve_output(output, count, limit)?;
    let end = cursor.checked_add(count).ok_or(LzoError::LengthOverflow)?;
    let literal_bytes = input
        .get(*cursor..end)
        .ok_or(LzoError::UnexpectedEndOfInput)?;
    output.extend_from_slice(literal_bytes);
    *cursor = end;
    Ok(())
}

fn copy_match(
    output: &mut Vec<u8>,
    distance: usize,
    count: usize,
    limit: usize,
) -> Result<(), LzoError> {
    reserve_output(output, count, limit)?;
    if distance == 0 || distance > output.len() {
        return Err(LzoError::InvalidBackReference {
            distance,
            output_len: output.len(),
        });
    }

    for _ in 0..count {
        let byte = output[output.len() - distance];
        output.push(byte);
    }
    Ok(())
}

fn decode_match(
    input: &[u8],
    cursor: &mut usize,
    output: &mut Vec<u8>,
    token: u8,
    output_limit: usize,
) -> Result<bool, LzoError> {
    let mut length: usize;
    let distance: usize;

    if token < 16 {
        // M1 match: 2 bytes copied from a short look-behind.
        let low = usize::from(read_byte(input, cursor)?);
        distance = 1 + usize::from(token >> 2) + (low << 2);
        length = 2;
    } else if token >= 64 {
        // M2 match: compact length and offset.
        let low = usize::from(read_byte(input, cursor)?);
        distance = 1 + usize::from((token >> 2) & 7) + (low << 3);
        length = usize::from(token >> 5) + 1;
    } else if token >= 32 {
        // M3 match: optional extended length, 14-bit distance.
        let short_length = usize::from(token & 0x1F);
        let tail = if short_length == 0 {
            read_extended(input, cursor, 31)?
        } else {
            short_length
        };
        let offset_low = usize::from(read_byte(input, cursor)?);
        let offset_high = usize::from(read_byte(input, cursor)?);
        distance = 1 + (offset_low >> 2) + (offset_high << 6);
        length = 2 + tail;
    } else {
        // M4 match or the unique end-of-stream marker.
        let offset_flag = usize::from(token & 0x08) << 11;
        let short_length = usize::from(token & 0x07);
        let tail = if short_length == 0 {
            read_extended(input, cursor, 7)?
        } else {
            short_length
        };
        let offset_low = usize::from(read_byte(input, cursor)?);
        let offset_high = usize::from(read_byte(input, cursor)?);
        let offset_extra = offset_flag + (offset_low >> 2) + (offset_high << 6);

        if offset_extra == 0 && tail == 1 {
            if *cursor != input.len() {
                return Err(LzoError::TrailingInput(input.len() - *cursor));
            }
            return Ok(true);
        }

        distance = 0x4000 + offset_extra;
        length = 2 + tail;
    }

    copy_match(output, distance, length, output_limit)?;
    Ok(false)
}

/// Decompress one source-compatible LZO1X block with strict bounds checks.
///
/// The C++ implementation is an unsafe speed-oriented decoder; this port
/// rejects missing operands, invalid look-behind distances, output overruns,
/// and trailing bytes after the EOF marker.
fn decode_initial_m1(
    input: &[u8],
    cursor: &mut usize,
    output: &mut Vec<u8>,
    token: u8,
    output_limit: usize,
) -> Result<(), LzoError> {
    let low = usize::from(read_byte(input, cursor)?);
    let distance = 0x801 + usize::from(token >> 2) + (low << 2);
    copy_match(output, distance, 3, output_limit)
}

fn trailing_literals(
    input: &[u8],
    cursor: &mut usize,
    output: &mut Vec<u8>,
    output_limit: usize,
) -> Result<(), LzoError> {
    if *cursor < 2 {
        return Err(LzoError::UnexpectedEndOfInput);
    }
    let count = usize::from(input[*cursor - 2] & 3);
    copy_literals(input, cursor, output, count, output_limit)
}

/// Decompress one source-compatible LZO1X block with strict bounds checks.
///
/// The C++ implementation is an unsafe speed-oriented decoder; this port
/// rejects missing operands, invalid look-behind distances, output overruns,
/// and trailing bytes after the EOF marker.
pub fn decompress(input: &[u8], output_limit: usize) -> Result<Vec<u8>, LzoError> {
    let Some(&first) = input.first() else {
        return Err(LzoError::MissingEndMarker);
    };

    let mut output = Vec::new();
    let mut cursor = 0_usize;

    // Fast initial literal run: token 18..255 means (token - 17) literals.
    if first > 17 {
        cursor = 1;
        copy_literals(
            input,
            &mut cursor,
            &mut output,
            usize::from(first - 17),
            output_limit,
        )?;

        let token = read_byte(input, &mut cursor)?;
        if token < 16 {
            decode_initial_m1(input, &mut cursor, &mut output, token, output_limit)?;
        } else if decode_match(input, &mut cursor, &mut output, token, output_limit)? {
            return Ok(output);
        }

        trailing_literals(input, &mut cursor, &mut output, output_limit)?;

        // After the first literal run the decoder is in its match loop;
        // tokens below 16 are regular two-byte M1 matches, not new literal runs.
        loop {
            let token = read_byte(input, &mut cursor)?;
            if decode_match(input, &mut cursor, &mut output, token, output_limit)? {
                return Ok(output);
            }
            trailing_literals(input, &mut cursor, &mut output, output_limit)?;
        }
    }

    // General LZO stream. A token below 16 at the top-level is a literal run;
    // its following token may use the special initial three-byte M1 form.
    loop {
        let token = read_byte(input, &mut cursor)?;

        if token < 16 {
            let extra = if token == 0 {
                read_extended(input, &mut cursor, 15)?
            } else {
                usize::from(token)
            };

            copy_literals(input, &mut cursor, &mut output, 3, output_limit)?;
            copy_literals(input, &mut cursor, &mut output, extra, output_limit)?;

            let first_match = read_byte(input, &mut cursor)?;
            if first_match < 16 {
                decode_initial_m1(
                    input,
                    &mut cursor,
                    &mut output,
                    first_match,
                    output_limit,
                )?;
            } else if decode_match(
                input,
                &mut cursor,
                &mut output,
                first_match,
                output_limit,
            )? {
                return Ok(output);
            }

            trailing_literals(input, &mut cursor, &mut output, output_limit)?;

            // Subsequent tokens are all matches (M1/M2/M3/M4).
            loop {
                let next_match = read_byte(input, &mut cursor)?;
                if decode_match(
                    input,
                    &mut cursor,
                    &mut output,
                    next_match,
                    output_limit,
                )? {
                    return Ok(output);
                }
                trailing_literals(input, &mut cursor, &mut output, output_limit)?;
            }
        } else {
            if decode_match(input, &mut cursor, &mut output, token, output_limit)? {
                return Ok(output);
            }
            trailing_literals(input, &mut cursor, &mut output, output_limit)?;

            loop {
                let next_match = read_byte(input, &mut cursor)?;
                if decode_match(
                    input,
                    &mut cursor,
                    &mut output,
                    next_match,
                    output_limit,
                )? {
                    return Ok(output);
                }
                trailing_literals(input, &mut cursor, &mut output, output_limit)?;
            }
        }
    }
}

pub fn decompress_expected(input: &[u8], expected_size: usize) -> Result<Vec<u8>, LzoError> {
    let output = decompress(input, expected_size)?;
    if output.len() != expected_size {
        return Err(LzoError::OutputLimitExceeded {
            requested: output.len(),
            limit: expected_size,
        });
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_emits_source_eof_marker() {
        assert_eq!(compress(&[]).unwrap(), LZO_EOF_MARKER);
        assert!(decompress_expected(&LZO_EOF_MARKER, 0).unwrap().is_empty());
    }

    #[test]
    fn short_literal_only_blocks_match_source_format() {
        for size in 1..=13 {
            let input: Vec<u8> = (0..size).map(|value| (value + 65) as u8).collect();
            let compressed = compress(&input).unwrap();
            assert_eq!(compressed[0], (17 + size) as u8);
            assert_eq!(decompress_expected(&compressed, size).unwrap(), input);
        }
    }

    #[test]
    fn medium_and_large_literal_runs_round_trip() {
        for size in [14_usize, 19, 238, 239, 700] {
            let input: Vec<u8> = (0..size)
                .map(|value| ((value * 73 + 17) & 0xFF) as u8)
                .collect();
            let compressed = compress(&input).unwrap();
            assert_eq!(decompress_expected(&compressed, size).unwrap(), input);
        }
    }

    #[test]
    fn repeated_input_uses_back_references() {
        for size in [16_usize, 64, 256, 4096, 20_000] {
            let input: Vec<u8> = (0..size)
                .map(|value| b"Westwood RTS LZO format "[value % 23])
                .collect();
            let compressed = compress(&input).unwrap();
            assert_eq!(decompress_expected(&compressed, size).unwrap(), input);
            assert!(compressed.len() < input.len());
        }
    }

    #[test]
    fn decoder_rejects_missing_eof_and_bad_lookbehind() {
        assert_eq!(decompress(&[0x12, b'A'], 10), Err(LzoError::UnexpectedEndOfInput));
        assert!(matches!(
            decompress(&[0x40, 0xFF, 0x11, 0, 0], 100),
            Err(LzoError::InvalidBackReference { .. }) | Err(LzoError::UnexpectedEndOfInput)
        ));
    }

    #[test]
    fn decoder_enforces_output_limit() {
        let compressed = compress(b"limited output check").unwrap();
        assert!(matches!(
            decompress(&compressed, 2),
            Err(LzoError::OutputLimitExceeded { .. })
        ));
    }
}
