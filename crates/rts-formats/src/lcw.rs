//! Westwood LCW (Format80) block decompressor ported from REDALERT/LCW.CPP
//! and REDALERT/LCWUNCMP.CPP.
//!
//! The decoder accepts a caller-supplied output limit because the original
//! functions ignore their length argument and can write beyond a destination
//! buffer for corrupt input. This implementation rejects malformed offsets,
//! truncated commands, and output-limit overruns.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LcwError {
    UnexpectedEndOfInput,
    InvalidBackReference { offset: usize, output_len: usize },
    OutputLimitExceeded { requested: usize, limit: usize },
    MissingTerminator,
    LengthOutOfRange { size: usize },
    InvalidBlockSize(usize),
    InvalidBlockHeader,
    OutputSizeMismatch { actual: usize, expected: usize },
}

fn read_u8(input: &[u8], cursor: &mut usize) -> Result<u8, LcwError> {
    let value = *input.get(*cursor).ok_or(LcwError::UnexpectedEndOfInput)?;
    *cursor += 1;
    Ok(value)
}

fn read_u16_le(input: &[u8], cursor: &mut usize) -> Result<usize, LcwError> {
    let low = usize::from(read_u8(input, cursor)?);
    let high = usize::from(read_u8(input, cursor)?);
    Ok(low | (high << 8))
}

fn reserve_output(output: &[u8], count: usize, limit: usize) -> Result<(), LcwError> {
    let requested = output.len().checked_add(count).unwrap_or(usize::MAX);
    if requested > limit {
        return Err(LcwError::OutputLimitExceeded { requested, limit });
    }
    Ok(())
}

fn copy_back_distance(
    output: &mut Vec<u8>,
    distance: usize,
    count: usize,
    limit: usize,
) -> Result<(), LcwError> {
    reserve_output(output, count, limit)?;
    if distance == 0 || distance > output.len() {
        return Err(LcwError::InvalidBackReference {
            offset: distance,
            output_len: output.len(),
        });
    }
    for _ in 0..count {
        let value = output[output.len() - distance];
        output.push(value);
    }
    Ok(())
}

fn copy_from_absolute(
    output: &mut Vec<u8>,
    offset: usize,
    count: usize,
    limit: usize,
) -> Result<(), LcwError> {
    reserve_output(output, count, limit)?;
    if count != 0 && offset >= output.len() {
        return Err(LcwError::InvalidBackReference {
            offset,
            output_len: output.len(),
        });
    }
    for index in 0..count {
        let source = offset.checked_add(index).ok_or(LcwError::InvalidBackReference {
            offset,
            output_len: output.len(),
        })?;
        let value = *output.get(source).ok_or(LcwError::InvalidBackReference {
            offset: source,
            output_len: output.len(),
        })?;
        output.push(value);
    }
    Ok(())
}

/// Decompress one LCW block. Returns when the 0x80 end marker is encountered.
pub fn decompress(input: &[u8], output_limit: usize) -> Result<Vec<u8>, LcwError> {
    let mut output = Vec::new();
    let mut cursor = 0;

    loop {
        let opcode = read_u8(input, &mut cursor)?;

        if opcode & 0x80 == 0 {
            // 0xxx yyyy, yyyyyyyy: short relative back-reference.
            let count = usize::from(opcode >> 4) + 3;
            let low = usize::from(read_u8(input, &mut cursor)?);
            let distance = low | (usize::from(opcode & 0x0F) << 8);
            copy_back_distance(&mut output, distance, count, output_limit)?;
            continue;
        }

        if opcode & 0x40 == 0 {
            if opcode == 0x80 {
                return Ok(output);
            }

            // 10xxxxxx: literal run, count 1..63.
            let count = usize::from(opcode & 0x3F);
            reserve_output(&output, count, output_limit)?;
            let end = cursor.checked_add(count).ok_or(LcwError::UnexpectedEndOfInput)?;
            let literal = input.get(cursor..end).ok_or(LcwError::UnexpectedEndOfInput)?;
            output.extend_from_slice(literal);
            cursor = end;
            continue;
        }

        match opcode {
            0xFE => {
                // Long repeated-byte run: u16 count followed by the byte value.
                let count = read_u16_le(input, &mut cursor)?;
                let value = read_u8(input, &mut cursor)?;
                reserve_output(&output, count, output_limit)?;
                output.resize(output.len() + count, value);
            }
            0xFF => {
                // Long absolute destination copy: u16 count, u16 output offset.
                let count = read_u16_le(input, &mut cursor)?;
                let offset = read_u16_le(input, &mut cursor)?;
                copy_from_absolute(&mut output, offset, count, output_limit)?;
            }
            _ => {
                // 11xxxxxx: medium absolute destination copy, count=(low6)+3.
                let count = usize::from(opcode & 0x3F) + 3;
                let offset = read_u16_le(input, &mut cursor)?;
                copy_from_absolute(&mut output, offset, count, output_limit)?;
            }
        }
    }
}

/// Safe version of LCW_Uncomp / LCW_Uncompress which additionally checks that
/// the decoded block does not exceed the requested output size.
pub fn decompress_expected(input: &[u8], expected_size: usize) -> Result<Vec<u8>, LcwError> {
    let output = decompress(input, expected_size)?;
    if output.len() != expected_size {
        return Err(LcwError::OutputSizeMismatch {
            actual: output.len(),
            expected: expected_size,
        });
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_run_decodes() {
        let compressed = [0x83, b'a', b'b', b'c', 0x80];
        assert_eq!(decompress(&compressed, 32).unwrap(), b"abc");
    }

    #[test]
    fn short_relative_copy_supports_overlap() {
        let compressed = [0x81, b'A', 0x00, 0x01, 0x80];
        assert_eq!(decompress(&compressed, 32).unwrap(), b"AAAA");
    }

    #[test]
    fn medium_absolute_copy_decodes() {
        let compressed = [0x83, b'a', b'b', b'c', 0xC0, 0x00, 0x00, 0x80];
        assert_eq!(decompress(&compressed, 32).unwrap(), b"abcabc");
    }

    #[test]
    fn long_absolute_copy_decodes() {
        let compressed = [0x81, b'Z', 0xFF, 0x03, 0x00, 0x00, 0x00, 0x80];
        assert_eq!(decompress(&compressed, 32).unwrap(), b"ZZZZ");
    }

    #[test]
    fn long_repeat_run_decodes() {
        let compressed = [0xFE, 0x05, 0x00, b'x', 0x80];
        assert_eq!(decompress(&compressed, 32).unwrap(), b"xxxxx");
    }

    #[test]
    fn rejects_truncated_commands() {
        assert_eq!(
            decompress(&[0x83, b'a'], 32),
            Err(LcwError::UnexpectedEndOfInput)
        );
        assert_eq!(
            decompress(&[0x00], 32),
            Err(LcwError::UnexpectedEndOfInput)
        );
    }

    #[test]
    fn rejects_back_references_before_output_start() {
        assert_eq!(
            decompress(&[0x00, 0x01, 0x80], 32),
            Err(LcwError::InvalidBackReference { offset: 1, output_len: 0 })
        );
    }

    #[test]
    fn enforces_output_limit() {
        assert_eq!(
            decompress(&[0xFE, 0x05, 0x00, b'x', 0x80], 4),
            Err(LcwError::OutputLimitExceeded { requested: 5, limit: 4 })
        );
    }

    #[test]
    fn expected_size_checks_final_block_length() {
        let compressed = [0x83, b'a', b'b', b'c', 0x80];
        assert_eq!(decompress_expected(&compressed, 3).unwrap(), b"abc");
        assert_eq!(
            decompress_expected(&compressed, 4),
            Err(LcwError::OutputSizeMismatch { actual: 3, expected: 4 })
        );
    }
}
