//! Westwood WIN32LIB IRandom/Random helper routines (IRANDOM.CPP).

/// Stateful port of the 24-bit x86 assembly generator used by Random().
/// The four-byte storage mirrors the source's DWORD global; only bytes 0..=2
/// participate in the assembly routine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LegacyRandom {
    state: u32,
}

impl Default for LegacyRandom {
    fn default() -> Self { Self::new(0x1234_9876) }
}

impl LegacyRandom {
    pub const fn new(seed: u32) -> Self { Self { state: seed } }

    pub const fn state(&self) -> u32 { self.state }

    pub fn next_byte(&mut self) -> u8 {
        let mut bytes = self.state.to_le_bytes();
        let mut al = bytes[0] >> 2;
        let carry = bytes[0] & 0x02 != 0;
        let carry2 = (bytes[2] & 0x80 != 0);
        bytes[2] = bytes[2].wrapping_shl(1) | u8::from(carry);
        let carry3 = carry2;
        let carry4 = bytes[1] & 0x80 != 0;
        bytes[1] = bytes[1].wrapping_shl(1) | u8::from(carry3);
        let borrow = !carry4;
        al = al.wrapping_sub(bytes[0]).wrapping_sub(u8::from(borrow));
        let rotate_carry = al & 1;
        bytes[0] = (bytes[0] >> 1) | (rotate_carry << 7);
        self.state = u32::from_le_bytes(bytes);
        bytes[0] ^ bytes[1]
    }
}

/// Reproduce the BSR/SHL/DEC mask construction used by Get_Random_Mask.
pub const fn get_random_mask(max_value: i32) -> u32 {
    if max_value == 0 {
        return 1;
    }
    let highest = 31 - (max_value as u32).leading_zeros();
    1u32.wrapping_shl(highest + 1).wrapping_sub(1)
}

/// Inclusive rejection-sampled integer range. The C runtime's rand() source is
/// injected so callers can preserve the host CRT sequence where required.
pub fn irandom_with<F>(mut rand: F, min_value: i32, max_value: i32) -> i32
where
    F: FnMut() -> u32,
{
    let (min, max) = if min_value <= max_value {
        (min_value, max_value)
    } else {
        (max_value, min_value)
    };
    let mask = get_random_mask(max.wrapping_sub(min) );
    loop {
        let candidate = (rand() & mask).wrapping_add(min as u32) as i32;
        if candidate <= max {
            return candidate;
        }
    }
}
