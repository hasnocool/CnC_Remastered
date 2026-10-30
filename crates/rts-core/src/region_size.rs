//! Region-size arithmetic from TIBERIANDAWN/WIN32LIB/REGIONSZ.CPP.
//!
//! The original routine returns width multiplied by height as a signed
//! 32-bit long. This preserves that behavior with explicit wrapping arithmetic.

/// Return the legacy region area (width * height) as a signed 32-bit value.
#[inline]
pub const fn size_of_region(width: i32, height: i32) -> i32 {
    width.wrapping_mul(height)
}

/// Checked variant for callers that prefer rejecting overflow.
#[inline]
pub const fn checked_size_of_region(width: i32, height: i32) -> Option<i32> {
    width.checked_mul(height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplies_dimensions() {
        assert_eq!(size_of_region(320, 200), 64_000);
        assert_eq!(size_of_region(0, 200), 0);
        assert_eq!(size_of_region(-4, 5), -20);
    }

    #[test]
    fn exposes_overflow_explicitly() {
        assert_eq!(checked_size_of_region(i32::MAX, 2), None);
        assert_eq!(size_of_region(i32::MAX, 2), -2);
    }
}
