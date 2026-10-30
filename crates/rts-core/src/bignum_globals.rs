//! Static state defined by REDALERT/INT.CPP for the multiprecision Int type.
//!
//! The upstream translation unit initializes Error, Carry, Borrow, and the
//! precision-specific Remainder object to zero/default before use.

/// Initializable state shared by operations on one multiprecision precision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BigNumGlobals<const LIMBS: usize> {
    /// Last arithmetic error code (zero means no error).
    pub error: i32,
    /// Carry result from the last addition.
    pub carry: bool,
    /// Borrow result from the last subtraction.
    pub borrow: bool,
    /// Remainder accumulator, least-significant limb first.
    pub remainder: [u32; LIMBS],
}

impl<const LIMBS: usize> Default for BigNumGlobals<LIMBS> {
    fn default() -> Self {
        Self { error: 0, carry: false, borrow: false, remainder: [0; LIMBS] }
    }
}
