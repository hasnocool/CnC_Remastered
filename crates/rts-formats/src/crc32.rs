//! Standard reflected CRC-32 used by the map-editor utility code.
///
/// This is intentionally distinct from the engine's legacy rotate-and-add checksum
/// used for MIX filename indexing.
pub fn crc32(bytes: &[u8]) -> u32 {
    let mut remainder = 0xFFFF_FFFF_u32;

    for &byte in bytes {
        let mut value = (remainder ^ u32::from(byte)) & 0xFF;
        for _ in 0..8 {
            value = if value & 1 != 0 {
                (value >> 1) ^ 0xEDB8_8320
            } else {
                value >> 1
            };
        }
        remainder = (remainder >> 8) ^ value;
    }

    !remainder
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_standard_crc32_check_value() {
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
    }

    #[test]
    fn empty_input_has_zero_crc() {
        assert_eq!(crc32(b""), 0);
    }
}
