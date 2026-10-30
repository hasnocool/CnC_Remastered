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
        if decode_match(input, &mut cursor, &mut output, token, output_limit)? {
            return Ok(output);
        }

        // The first match after this run is the special three-byte M1 form.
        let trailing = usize::from(input[cursor - 2] & 3);
        if trailing > 0 {
            copy_literals(input, &mut cursor, &mut output, trailing, output_limit)?;
        }
    }

    loop {
        let token = read_byte(input, &mut cursor)?;

        if token < 16 {
            // Literal run. The first three bytes are encoded as a fixed prefix,
            // followed by an optional extension count in the token.
            let extra = if token == 0 {
                read_extended(input, &mut cursor, 15)?
            } else {
                usize::from(token)
            };

            copy_literals(input, &mut cursor, &mut output, 3, output_limit)?;
            copy_literals(input, &mut cursor, &mut output, extra, output_limit)?;

            let first_match = read_byte(input, &mut cursor)?;
            if first_match < 16 {
                // Special first-literal-run M1 has a 0x800 base distance and
                // always copies three bytes.
                let low = usize::from(read_byte(input, &mut cursor)?);
                let distance = 0x801 + usize::from(first_match >> 2) + (low << 2);
                copy_match(&mut output, distance, 3, output_limit)?;
            } else if decode_match(
                input,
                &mut cursor,
                &mut output,
                first_match,
                output_limit,
            )? {
                return Ok(output);
            }
        } else if decode_match(input, &mut cursor, &mut output, token, output_limit)? {
            return Ok(output);
        }

        // The low two bits in the preceding match token/offset encode up to
        // three trailing literal bytes.
        let trailing = usize::from(input[cursor - 2] & 3);
        if trailing > 0 {
            copy_literals(input, &mut cursor, &mut output, trailing, output_limit)?;
        }
    }
}

fn hash_at(input: &[u8], index: usize) -> usize {
    let value = ((u32::from(input[index + 2]) << 10)
        ^ (u32::from(input[index + 1]) << 5))
        ^ u32::from(input[index]);
    ((40_799_u32.wrapping_mul(value) >> 5) as usize) & (LZO_DICTIONARY_SIZE - 1)
}

fn emit_literal_run(
    output: &mut Vec<u8>,
    literal_start: usize,
    literal_end: usize,
    input: &[u8],
) {
    let count = literal_end - literal_start;
    if count == 0 {
        return;
    }

    if count <= 3 {
        // A preceding M2/M3/M4 command's final offset byte stores the count.
        let previous_match_header = output.len() - 2;
        output[previous_match_header] |= count as u8;
        output.extend_from_slice(&input[literal_start..literal_end]);
        return;
    }

    if output.is_empty() && count <= 238 {
        output.push((17 + count) as u8);
    } else if count <= 18 {
        output.push((count - 3) as u8);
    } else {
        let mut extension = count - 18;
        output.push(0);
        while extension > 255 {
            extension -= 255;
            output.push(0);
        }
        output.push(extension as u8);
    }

    output.extend_from_slice(&input[literal_start..literal_end]);
}

fn emit_match(output: &mut Vec<u8>, distance: usize, count: usize) {
    if count <= 8 && distance <= LZO_M2_MAX_OFFSET {
        let adjusted = distance - 1;
        output.push((((count - 1) << 5) | ((adjusted & 7) << 2)) as u8);
        output.push((adjusted >> 3) as u8);
        return;
    }

    if distance <= LZO_M3_MAX_OFFSET {
        let adjusted = distance - 1;
        if count <= 33 {
            output.push((32 | (count - 2)) as u8);
        } else {
            output.push(32);
            emit_length_extension(output, count - 33);
        }
        output.push(((adjusted & 63) << 2) as u8);
        output.push((adjusted >> 6) as u8);
        return;
    }

    let adjusted = distance - 0x4000;
    if count <= 9 {
        output.push((16 | ((adjusted & 0x4000) >> 11) | (count - 2)) as u8);
    } else {
        output.push((16 | ((adjusted & 0x4000) >> 11)) as u8);
        emit_length_extension(output, count - 9);
    }
    output.push(((adjusted & 63) << 2) as u8);
    output.push((adjusted >> 6) as u8);
}

fn emit_length_extension(output: &mut Vec<u8>, mut count: usize) {
    while count > 255 {
        output.push(0);
        count -= 255;
    }
    output.push(count as u8);
}

/// Compress one LZO1X block.
///
/// This follows the source's 14-bit three-byte hash, 0xBFFF look-behind
/// ceiling, M2/M3/M4 command families, literal-run extensions, and EOF marker.
/// Candidate selection is deterministic (the latest candidate in each hash
/// bucket replaces the earlier one).
pub fn compress(input: &[u8]) -> Result<Vec<u8>, LzoError> {
    if input.is_empty() {
        return Ok(LZO_EOF_MARKER.to_vec());
    }

    if input.len() <= 13 {
        let mut output = Vec::with_capacity(input.len() + 4);
        output.push((17 + input.len()) as u8);
        output.extend_from_slice(input);
        output.extend_from_slice(&LZO_EOF_MARKER);
        return Ok(output);
    }

    let mut dictionary = vec![None::<usize>; LZO_DICTIONARY_SIZE];
    let mut output = Vec::with_capacity(input.len() + 32);
    let mut input_position = 4_usize;
    let mut literal_start = 0_usize;
    let input_end = input.len();
    let scan_end = input_end - 13;

    // Source preloads the first four hash positions.
    for index in 0..4 {
        dictionary[hash_at(input, index)] = Some(index);
    }

    while input_position < scan_end {
        let slot = hash_at(input, input_position);
        let candidate = dictionary[slot];
        dictionary[slot] = Some(input_position);

        let Some(candidate) = candidate else {
            input_position += 1;
            continue;
        };

        let distance = input_position - candidate;
        if distance == 0 || distance > LZO_M4_MAX_OFFSET {
            input_position += 1;
            continue;
        }

        if input[candidate..candidate + 3] != input[input_position..input_position + 3] {
            input_position += 1;
            continue;
        }

        let mut match_length = 3_usize;
        while input_position + match_length < input_end
            && input[candidate + match_length] == input[input_position + match_length]
        {
            match_length += 1;
        }

        emit_literal_run(&mut output, literal_start, input_position, input);
        emit_match(&mut output, distance, match_length);
        input_position += match_length;
        literal_start = input_position;
    }

    emit_literal_run(&mut output, literal_start, input_end, input);
    output.extend_from_slice(&LZO_EOF_MARKER);
    Ok(output)
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
