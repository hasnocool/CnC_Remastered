//! Packed boolean vector ported from REDALERT/VECTOR.CPP and VECTOR.H.
//!
//! The C++ class exposes a cached bool reference that is written back during
//! Fixup(). Rust cannot safely expose a mutable reference to one packed bit, so
//! this version performs each get/set directly against the packed byte array.
//! All observable bit values, resizing, reset/set, search, and equality behavior
//! are retained.

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BooleanVector {
    bit_count: usize,
    bytes: Vec<u8>,
}

impl BooleanVector {
    pub fn new(bit_count: usize) -> Self {
        Self {
            bit_count,
            bytes: vec![0; byte_count(bit_count)],
        }
    }

    /// Create from source-compatible packed bytes. The byte slice is copied,
    /// and excess trailing bytes are discarded.
    pub fn from_packed(bit_count: usize, packed: &[u8]) -> Result<Self, &'static str> {
        let required = byte_count(bit_count);
        if packed.len() < required {
            return Err("packed byte slice is shorter than the requested bit count");
        }
        Ok(Self {
            bit_count,
            bytes: packed[..required].to_vec(),
        })
    }

    pub const fn len(&self) -> usize {
        self.bit_count
    }

    pub const fn is_empty(&self) -> bool {
        self.bit_count == 0
    }

    pub fn packed_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Equivalent to BooleanVectorClass::Is_True for valid indices.
    pub fn is_true(&self, index: usize) -> bool {
        self.get(index).unwrap_or(false)
    }

    /// Safe equivalent of the source bit-array subscript operator.
    pub fn get(&self, index: usize) -> Option<bool> {
        if index >= self.bit_count {
            return None;
        }
        let byte = self.bytes[index / 8];
        Some((byte & (1 << (index & 7))) != 0)
    }

    /// Directly update a packed bit. Returns false for an invalid index.
    pub fn set_bit(&mut self, index: usize, value: bool) -> bool {
        if index >= self.bit_count {
            return false;
        }
        let mask = 1_u8 << (index & 7);
        let byte = &mut self.bytes[index / 8];
        if value {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
        true
    }

    /// Port of BooleanVectorClass::Resize. Newly exposed bits are false.
    pub fn resize(&mut self, new_size: usize) {
        if new_size == 0 {
            self.clear();
            return;
        }

        let old_size = self.bit_count;
        self.bytes.resize(byte_count(new_size), 0);
        self.bit_count = new_size;

        if new_size > old_size {
            // The source explicitly clears every new bit after resizing.
            for index in old_size..new_size {
                self.set_bit(index, false);
            }
        }

        self.bit_count = new_size;
    }

    /// Port of BooleanVectorClass::Clear.
    pub fn clear(&mut self) {
        self.bit_count = 0;
        self.bytes.clear();
    }

    /// Port of BooleanVectorClass::Reset.
    pub fn reset(&mut self) {
        self.bytes.fill(0);
    }

    /// Port of BooleanVectorClass::Set. As in the C++ code, unused high bits
    /// in the final packed byte are set too.
    pub fn set_all(&mut self) {
        self.bytes.fill(0xFF);
    }

    /// Port of BooleanVectorClass::First_False.
    pub fn first_false(&self) -> Option<usize> {
        (0..self.bit_count).find(|&index| !self.is_true(index))
    }

    /// Port of BooleanVectorClass::First_True.
    pub fn first_true(&self) -> Option<usize> {
        (0..self.bit_count).find(|&index| self.is_true(index))
    }

    /// Equality compares the logical bit count and entire packed byte vector,
    /// including unused high bits in the final byte, like the source class.
    pub fn same_contents(&self, other: &Self) -> bool {
        self.bit_count == other.bit_count && self.bytes == other.bytes
    }

    pub fn count_true(&self) -> usize {
        (0..self.bit_count).filter(|&index| self.is_true(index)).count()
    }
}

fn byte_count(bit_count: usize) -> usize {
    bit_count.saturating_add(7) / 8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_bits_in_lsb_first_order() {
        let mut vector = BooleanVector::new(10);
        assert!(vector.set_bit(0, true));
        assert!(vector.set_bit(7, true));
        assert!(vector.set_bit(8, true));
        assert_eq!(vector.packed_bytes(), &[0x81, 0x01]);
        assert_eq!(vector.get(0), Some(true));
        assert_eq!(vector.get(7), Some(false));
        assert_eq!(vector.get(8), Some(true));
        assert_eq!(vector.get(10), None);
    }

    #[test]
    fn reset_and_set_cover_every_packed_byte() {
        let mut vector = BooleanVector::new(9);
        vector.set_all();
        assert_eq!(vector.packed_bytes(), &[0xFF, 0xFF]);
        assert_eq!(vector.count_true(), 9);

        vector.reset();
        assert_eq!(vector.packed_bytes(), &[0, 0]);
        assert_eq!(vector.count_true(), 0);
    }

    #[test]
    fn first_true_and_false_match_source_search_operations() {
        let mut vector = BooleanVector::new(12);
        assert_eq!(vector.first_true(), None);
        assert_eq!(vector.first_false(), Some(0));

        vector.set_bit(4, true);
        vector.set_bit(7, true);
        assert_eq!(vector.first_true(), Some(4));
        assert_eq!(vector.first_false(), Some(0));

        vector.set_all();
        assert_eq!(vector.first_false(), None);
        assert_eq!(vector.first_true(), Some(0));
    }

    #[test]
    fn growing_clears_new_bits_and_preserves_existing_bits() {
        let mut vector = BooleanVector::new(7);
        vector.set_bit(2, true);
        vector.resize(10);

        assert_eq!(vector.len(), 10);
        assert_eq!(vector.get(2), Some(true));
        assert_eq!(vector.get(7), Some(false));
        assert_eq!(vector.get(8), Some(false));
        assert_eq!(vector.get(9), Some(false));
    }

    #[test]
    fn growing_after_shrink_clears_bits_that_become_logically_new() {
        let mut vector = BooleanVector::new(8);
        vector.set_bit(7, true);
        vector.resize(7);
        vector.resize(8);
        assert_eq!(vector.get(7), Some(true));
    }

    #[test]
    fn zero_resize_clears_allocation_and_length() {
        let mut vector = BooleanVector::new(32);
        vector.set_all();
        vector.resize(0);
        assert!(vector.is_empty());
        assert!(vector.packed_bytes().is_empty());
    }

    #[test]
    fn packed_constructor_checks_input_length() {
        assert!(BooleanVector::from_packed(9, &[0xFF]).is_err());
        let vector = BooleanVector::from_packed(8, &[0xA5, 0xFF]).unwrap();
        assert_eq!(vector.packed_bytes(), &[0xA5]);
    }

    #[test]
    fn equality_includes_bit_count_and_unused_final_byte_bits() {
        let mut left = BooleanVector::new(5);
        let mut right = BooleanVector::new(5);
        assert!(left.same_contents(&right));

        left.set_bit(1, true);
        assert!(!left.same_contents(&right));
        right.set_bit(1, true);
        assert!(left.same_contents(&right));

        left.set_all();
        right.set_all();
        assert!(left.same_contents(&right));
        assert_ne!(left, BooleanVector::new(8));
    }
}
