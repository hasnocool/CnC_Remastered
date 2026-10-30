//! Westwood 12-bit LZW engine ported from REDALERT/LZW.H and LZW.CPP.
//!
//! Unlike GIF-style or TIFF-style LZW, this implementation writes each
//! dictionary code as a little-endian 16-bit word. The stream uses EOS=256,
//! starts dictionary allocation at 257, and caps codes at 4095.

pub const LZW_BITS: usize = 12;
pub const LZW_MAX_CODE: u16 = (1 << LZW_BITS) - 1;
pub const LZW_FIRST_CODE: u16 = 257;
pub const LZW_END_OF_STREAM: u16 = 256;
pub const LZW_TABLE_SIZE: usize = 5021;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LzwError {
    OddCodeStreamLength(usize),
    InvalidCode(u16),
    MissingDictionaryEntry(u16),
    DictionaryChainTooDeep,
    OutputLimitExceeded { requested: usize, limit: usize },
    LengthOutOfRange(usize),
    InvalidBlockSize(usize),
    InvalidBlockHeader,
    UnexpectedEndOfInput,
    OutputSizeMismatch { actual: usize, expected: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct HashEntry {
    code_value: Option<u16>,
    parent_code: u16,
    character: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct DecodeEntry {
    parent_code: u16,
    character: u8,
    present: bool,
}

fn make_lzw_hash(parent_code: u16, character: u8) -> usize {
    (usize::from(character) << (LZW_BITS - 8)) ^ usize::from(parent_code)
}

fn find_child_node(
    dictionary: &[HashEntry],
    parent_code: u16,
    character: u8,
) -> Option<usize> {
    let initial = make_lzw_hash(parent_code, character);
    if initial >= LZW_TABLE_SIZE {
        return None;
    }

    let mut index = initial;
    let offset = if index != 0 {
        LZW_TABLE_SIZE - index
    } else {
        1
    };

    loop {
        let entry = dictionary[index];
        if entry.code_value.is_some()
            && entry.parent_code == parent_code
            && entry.character == character
        {
            return Some(index);
        }

        if entry.code_value.is_none() {
            return Some(index);
        }

        index = if index >= offset {
            index - offset
        } else {
            index + LZW_TABLE_SIZE - offset
        };

        if index == initial {
            return None;
        }
    }
}

/// Compress bytes into the source-compatible 16-bit codeword stream.
pub fn compress(input: &[u8]) -> Result<Vec<u8>, LzwError> {
    let mut dictionary = vec![HashEntry::default(); LZW_TABLE_SIZE];
    let mut output = Vec::with_capacity(input.len().saturating_mul(2).saturating_add(2));
    let mut next_code = LZW_FIRST_CODE;

    if input.is_empty() {
        output.extend_from_slice(&LZW_END_OF_STREAM.to_le_bytes());
        return Ok(output);
    }

    let mut string_code = u16::from(input[0]);
    for &character in &input[1..] {
        let slot = find_child_node(&dictionary, string_code, character);

        if let Some(index) = slot {
            if let Some(code_value) = dictionary[index].code_value {
                string_code = code_value;
                continue;
            }

            if next_code <= LZW_MAX_CODE {
                dictionary[index] = HashEntry {
                    code_value: Some(next_code),
                    parent_code: string_code,
                    character,
                };
                next_code += 1;
            }
        }

        // A missing dictionary match emits the current phrase and starts a
        // new phrase at the current byte.
        output.extend_from_slice(&string_code.to_le_bytes());
        string_code = u16::from(character);
    }

    output.extend_from_slice(&string_code.to_le_bytes());
    if string_code != LZW_END_OF_STREAM {
        output.extend_from_slice(&LZW_END_OF_STREAM.to_le_bytes());
    }

    Ok(output)
}

fn decode_string(
    dictionary: &[DecodeEntry],
    code: u16,
) -> Result<Vec<u8>, LzwError> {
    let mut reversed = Vec::new();
    let mut current = code;

    loop {
        if current < 256 {
            reversed.push(current as u8);
            break;
        }

        let index = usize::from(current);
        let entry = dictionary
            .get(index)
            .filter(|entry| entry.present)
            .ok_or(LzwError::MissingDictionaryEntry(current))?;

        reversed.push(entry.character);
        current = entry.parent_code;

        if reversed.len() > LZW_TABLE_SIZE {
            return Err(LzwError::DictionaryChainTooDeep);
        }
    }

    reversed.reverse();
    Ok(reversed)
}

/// Decompress the little-endian 16-bit codeword stream.
pub fn decompress(input: &[u8], output_limit: usize) -> Result<Vec<u8>, LzwError> {
    if input.len() % 2 != 0 {
        return Err(LzwError::OddCodeStreamLength(input.len()));
    }

    let codes: Vec<u16> = input
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect();

    let Some(&first_code) = codes.first() else {
        return Ok(Vec::new());
    };

    // LZWEngine::Compress emits EOS directly for an empty input block.
    if first_code == LZW_END_OF_STREAM {
        return Ok(Vec::new());
    }
    if first_code > 255 {
        return Err(LzwError::InvalidCode(first_code));
    }

    let mut dictionary = vec![DecodeEntry::default(); LZW_TABLE_SIZE];
    let mut output = vec![first_code as u8];
    if output.len() > output_limit {
        return Err(LzwError::OutputLimitExceeded {
            requested: output.len(),
            limit: output_limit,
        });
    }

    let mut old_code = first_code;
    let mut first_character = first_code as u8;
    let mut next_code = LZW_FIRST_CODE;

    for &new_code in codes.iter().skip(1) {
        if new_code == LZW_END_OF_STREAM {
            break;
        }
        if new_code > LZW_MAX_CODE {
            return Err(LzwError::InvalidCode(new_code));
        }

        let decoded = if new_code < next_code {
            decode_string(&dictionary, new_code)?
        } else if new_code == next_code {
            // KwKwK case: decode the previous phrase and append its first byte.
            let mut phrase = decode_string(&dictionary, old_code)?;
            phrase.push(first_character);
            phrase
        } else {
            return Err(LzwError::InvalidCode(new_code));
        };

        let new_first_character = *decoded
            .first()
            .ok_or(LzwError::InvalidCode(new_code))?;

        let requested = output
            .len()
            .checked_add(decoded.len())
            .unwrap_or(usize::MAX);
        if requested > output_limit {
            return Err(LzwError::OutputLimitExceeded {
                requested,
                limit: output_limit,
            });
        }
        output.extend_from_slice(&decoded);

        if next_code <= LZW_MAX_CODE {
            let slot = usize::from(next_code);
            dictionary[slot] = DecodeEntry {
                parent_code: old_code,
                character: new_first_character,
                present: true,
            };
            next_code += 1;
        }

        old_code = new_code;
        first_character = new_first_character;
    }

    Ok(output)
}

pub fn decompress_expected(input: &[u8], expected_size: usize) -> Result<Vec<u8>, LzwError> {
    let output = decompress(input, expected_size)?;
    if output.len() != expected_size {
        return Err(LzwError::OutputSizeMismatch {
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
    fn empty_block_is_a_single_eos_codeword() {
        let encoded = compress(&[]).unwrap();
        assert_eq!(encoded, [0x00, 0x01]);
        assert!(decompress_expected(&encoded, 0).unwrap().is_empty());
    }

    #[test]
    fn one_byte_block_has_literal_and_eos_codewords() {
        let encoded = compress(b"A").unwrap();
        assert_eq!(encoded, [b'A', 0x00, 0x00, 0x01]);
        assert_eq!(decompress_expected(&encoded, 1).unwrap(), b"A");
    }

    #[test]
    fn compressor_and_decoder_round_trip_all_byte_values() {
        let input: Vec<u8> = (0..=255).collect();
        let encoded = compress(&input).unwrap();
        assert_eq!(encoded.len() % 2, 0);
        assert_eq!(decompress_expected(&encoded, input.len()).unwrap(), input);
    }

    #[test]
    fn repetitive_input_allocates_dictionary_codes() {
        let input = b"ABABABAABABABAABABABAABABABA";
        let encoded = compress(input).unwrap();
        assert_eq!(decompress_expected(&encoded, input.len()).unwrap(), input);
        assert!(encoded.len() < input.len() * 2);
    }

    #[test]
    fn large_block_exercises_dictionary_saturation() {
        let input: Vec<u8> = (0..12_000)
            .map(|index| ((index * 73 + index / 11) & 0xFF) as u8)
            .collect();
        let encoded = compress(&input).unwrap();
        assert_eq!(decompress_expected(&encoded, input.len()).unwrap(), input);
    }

    #[test]
    fn rejects_odd_code_streams() {
        assert_eq!(
            decompress(&[0x41], 100),
            Err(LzwError::OddCodeStreamLength(1))
        );
    }

    #[test]
    fn rejects_unknown_dictionary_codes() {
        // The first code is 'A'; 300 is beyond next_code=257.
        assert_eq!(decompress(&[b'A', 0, 0x2C, 0x01], 32), Err(LzwError::InvalidCode(300)));
    }

    #[test]
    fn enforces_the_decoded_output_limit() {
        let encoded = compress(b"hello").unwrap();
        assert!(matches!(
            decompress(&encoded, 2),
            Err(LzwError::OutputLimitExceeded { .. })
        ));
    }
}
