//! Red Alert's unsigned Q16.16 fixed-point type, ported from FIXED.H/FIXED.CPP.

const PRECISION: u32 = 1 << 16;
const FRACTION_MASK: u32 = PRECISION - 1;
const WHOLE_MASK: u32 = !FRACTION_MASK;

/// Non-negative unsigned Q16.16 fixed point value.
///
/// The original class stores the 16-bit fractional component in the low word
/// and the 16-bit whole component in the high word on the supported x86 build.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fixed16 {
    raw: u32,
}

impl Fixed16 {
    pub const ZERO: Self = Self { raw: 0 };
    pub const ONE: Self = Self { raw: PRECISION };
    pub const ONE_HALF: Self = Self { raw: PRECISION / 2 };
    pub const ONE_THIRD: Self = Self { raw: PRECISION / 3 };
    pub const ONE_QUARTER: Self = Self { raw: PRECISION / 4 };
    pub const THREE_QUARTERS: Self = Self { raw: (PRECISION * 3) / 4 };
    pub const TWO_THIRDS: Self = Self { raw: (PRECISION * 2) / 3 };

    pub const fn from_raw(raw: u32) -> Self {
        Self { raw }
    }

    /// The original integer constructors truncate the whole value to 16 bits.
    pub const fn from_integer(value: i32) -> Self {
        Self { raw: (value as u16 as u32) << 16 }
    }

    pub const fn from_unsigned(value: u32) -> Self {
        Self { raw: (value as u16) << 16 }
    }

    /// The original ratio constructor returns zero for a zero denominator.
    pub fn from_ratio(numerator: i32, denominator: i32) -> Self {
        if denominator == 0 {
            return Self::ZERO;
        }
        let numerator = numerator as i64 as u64;
        let denominator = denominator as i64 as u64;
        let raw = numerator.wrapping_mul(u64::from(PRECISION)) / denominator;
        Self { raw: raw as u32 }
    }

    /// Port of the C++ float constructor, including its half-unit bias.
    pub fn from_f32(mut value: f32) -> Self {
        value += 1.0_f32 / ((PRECISION as f32) * 2.0_f32);
        let whole = value as u16;
        let fraction = ((value - f32::from(whole)) * PRECISION as f32) as u16;
        Self { raw: (u32::from(whole) << 16) | u32::from(fraction) }
    }

    /// Port of the FIXED.CPP ASCII constructor. Invalid or absent numeric
    /// text follows atoi-like behavior and produces zero components.
    pub fn from_ascii(input: &str) -> Self {
        let trimmed = input.trim_start();
        if trimmed.is_empty() {
            return Self::ZERO;
        }

        let bytes = trimmed.as_bytes();
        let mut digits_end = 0;
        while digits_end < bytes.len() && bytes[digits_end].is_ascii_digit() {
            digits_end += 1;
        }

        if digits_end > 0 && bytes.get(digits_end) == Some(&b'%') {
            let percent = atoi_prefix(trimmed.as_bytes()) as u64;
            let raw = percent.wrapping_mul(u64::from(PRECISION)) / 100;
            return Self { raw: raw as u32 };
        }

        let original_starts_with_dot = input.as_bytes().first() == Some(&b'.');
        let whole = if original_starts_with_dot {
            0
        } else {
            atoi_prefix(input.as_bytes()) as u16
        };

        let fraction = if let Some(dot) = bytes.iter().position(|byte| *byte == b'.') {
            let fractional_bytes = &bytes[dot + 1..];
            let mut digits = 0_u64;
            let mut digit_count = 0_u32;
            let mut base = 1_u32;
            for &byte in fractional_bytes {
                if !byte.is_ascii_digit() {
                    break;
                }
                digits = digits.wrapping_mul(10).wrapping_add(u64::from(byte - b'0'));
                digit_count += 1;
                base = base.wrapping_mul(10);
            }

            if digit_count == 0 || base == 0 {
                0
            } else {
                ((digits.wrapping_mul(u64::from(PRECISION)) / u64::from(base)) as u16)
            }
        } else {
            0
        };

        Self { raw: (u32::from(whole) << 16) | u32::from(fraction) }
    }

    pub const fn raw(self) -> u32 {
        self.raw
    }

    pub const fn whole(self) -> u16 {
        (self.raw >> 16) as u16
    }

    pub const fn fraction(self) -> u16 {
        (self.raw & FRACTION_MASK) as u16
    }

    /// Port of the implicit conversion to unsigned, rounded to nearest.
    pub const fn to_unsigned(self) -> u32 {
        ((self.raw as u64 + u64::from(PRECISION / 2)) / u64::from(PRECISION)) as u32
    }

    pub fn to_f32(self) -> f32 {
        self.whole() as f32 + self.fraction() as f32 / PRECISION as f32
    }

    /// Q16.16 multiplication with the same 64-bit intermediate and truncation.
    pub const fn mul_fixed(self, rhs: Self) -> Self {
        Self {
            raw: ((self.raw as u64 * rhs.raw as u64) / u64::from(PRECISION)) as u32,
        }
    }

    /// Integer multiplication wraps in the original unsigned 32-bit storage.
    pub const fn mul_integer(self, rhs: i32) -> Self {
        Self { raw: self.raw.wrapping_mul(rhs as u32) }
    }

    /// The source leaves the value unchanged for division by zero or by one.
    pub const fn div_fixed(self, rhs: Self) -> Self {
        if rhs.raw == 0 || rhs.raw == PRECISION {
            return self;
        }
        let numerator = (self.raw as u64)
            .wrapping_mul(u64::from(PRECISION))
            .wrapping_add(u64::from(PRECISION / 2));
        Self { raw: (numerator / rhs.raw as u64) as u32 }
    }

    pub const fn div_integer(self, rhs: i32) -> Self {
        if rhs == 0 {
            self
        } else {
            Self { raw: self.raw / rhs as u32 }
        }
    }

    pub const fn add(self, rhs: Self) -> Self {
        Self { raw: self.raw.wrapping_add(rhs.raw) }
    }

    pub const fn sub(self, rhs: Self) -> Self {
        Self { raw: self.raw.wrapping_sub(rhs.raw) }
    }

    pub const fn shift_right(self, amount: u32) -> Self {
        Self { raw: self.raw.wrapping_shr(amount) }
    }

    pub const fn shift_left(self, amount: u32) -> Self {
        Self { raw: self.raw.wrapping_shl(amount) }
    }

    pub fn round_up(&mut self) {
        self.raw = self.raw.wrapping_add(PRECISION - 1) & WHOLE_MASK;
    }

    pub fn round_down(&mut self) {
        self.raw &= WHOLE_MASK;
    }

    pub fn round(&mut self) {
        if self.fraction() >= (PRECISION / 2) as u16 {
            self.round_up();
        }
        self.round_down();
    }

    pub fn saturate_unsigned(&mut self, cap: u32) {
        let cap_raw = cap.wrapping_mul(PRECISION);
        if self.raw > cap_raw {
            self.raw = cap_raw;
        }
    }

    pub fn saturate_fixed(&mut self, cap: Self) {
        if *self > cap {
            *self = cap;
        }
    }

    pub fn sub_saturate_unsigned(&mut self, cap: u32) {
        let cap_raw = cap.wrapping_mul(PRECISION);
        if self.raw >= cap_raw {
            self.raw = cap_raw.wrapping_sub(1);
        }
    }

    pub fn sub_saturate_fixed(&mut self, cap: Self) {
        if *self >= cap {
            self.raw = cap.raw.wrapping_sub(1);
        }
    }

    pub fn inverse(&mut self) {
        *self = Self::ONE.div_fixed(*self);
    }

    /// Port of FIXED.CPP To_ASCII, using an owned UTF-8 string.
    pub fn to_ascii_string(self) -> String {
        let whole = self.whole();
        let fraction = (u32::from(self.fraction()) * 1000) / PRECISION;
        if fraction == 0 {
            return whole.to_string();
        }

        let mut result = format!("{whole}.{fraction:02}");
        while result.ends_with('0') {
            result.pop();
        }
        result
    }

    /// Write a safely NUL-terminated representation into a caller-owned buffer.
    /// Returns the number of UTF-8 bytes written, excluding the terminator.
    pub fn write_ascii(self, buffer: &mut [u8]) -> usize {
        if buffer.is_empty() {
            return 0;
        }
        let text = self.to_ascii_string();
        let count = text.len().min(buffer.len() - 1);
        buffer[..count].copy_from_slice(&text.as_bytes()[..count]);
        buffer[count] = 0;
        count
    }
}

fn atoi_prefix(bytes: &[u8]) -> i64 {
    let mut index = 0;
    while index < bytes.len() && bytes[index].is_ascii_whitespace() {
        index += 1;
    }

    let sign = match bytes.get(index) {
        Some(b'-') => {
            index += 1;
            -1_i64
        }
        Some(b'+') => {
            index += 1;
            1_i64
        }
        _ => 1_i64,
    };

    let mut value = 0_i64;
    let mut found = false;
    while index < bytes.len() && bytes[index].is_ascii_digit() {
        found = true;
        value = value
            .wrapping_mul(10)
            .wrapping_add(i64::from(bytes[index] - b'0'));
        index += 1;
    }

    if found { value.wrapping_mul(sign) } else { 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constants_match_q16_16_values() {
        assert_eq!(Fixed16::ONE_HALF.raw(), 0x0000_8000);
        assert_eq!(Fixed16::ONE_THIRD.raw(), 0x0000_5555);
        assert_eq!(Fixed16::ONE_QUARTER.raw(), 0x0000_4000);
        assert_eq!(Fixed16::THREE_QUARTERS.raw(), 0x0000_C000);
        assert_eq!(Fixed16::TWO_THIRDS.raw(), 0x0000_AAAA);
    }

    #[test]
    fn integer_and_ratio_constructors_match_source_layout() {
        assert_eq!(Fixed16::from_integer(3).raw(), 0x0003_0000);
        assert_eq!(Fixed16::from_ratio(1, 2), Fixed16::ONE_HALF);
        assert_eq!(Fixed16::from_ratio(1, 3), Fixed16::ONE_THIRD);
        assert_eq!(Fixed16::from_ratio(1, 0), Fixed16::ZERO);
    }

    #[test]
    fn ascii_constructor_handles_decimal_and_percent_forms() {
        assert_eq!(Fixed16::from_ascii("1.5").to_ascii_string(), "1.5");
        assert_eq!(Fixed16::from_ascii("  .25").to_ascii_string(), "0.25");
        assert_eq!(Fixed16::from_ascii("25%").to_ascii_string(), "0.25");
        assert_eq!(Fixed16::from_ascii("100%"), Fixed16::ONE);
        assert_eq!(Fixed16::from_ascii("invalid"), Fixed16::ZERO);
    }

    #[test]
    fn arithmetic_preserves_unsigned_q16_16_rules() {
        assert_eq!(Fixed16::ONE_HALF.mul_fixed(Fixed16::ONE_HALF), Fixed16::ONE_QUARTER);
        assert_eq!(Fixed16::ONE_HALF.add(Fixed16::ONE_HALF), Fixed16::ONE);
        assert_eq!(Fixed16::ONE.sub(Fixed16::ONE_HALF), Fixed16::ONE_HALF);
        assert_eq!(Fixed16::ONE_HALF.to_unsigned(), 1);
        assert_eq!(Fixed16::from_ascii("1.49").to_unsigned(), 1);
        assert_eq!(Fixed16::from_ascii("1.5").to_unsigned(), 2);
    }

    #[test]
    fn ascii_buffer_is_always_terminated_when_nonempty() {
        let mut buffer = [0xFF; 4];
        let written = Fixed16::from_ascii("12.5").write_ascii(&mut buffer);
        assert_eq!(written, 3);
        assert_eq!(&buffer, b"12.\0");
    }
}
