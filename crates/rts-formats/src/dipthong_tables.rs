//! Westwood's static text-compression frequency tables from _DIPTABL.CPP.
//!
//! The Red Alert and Tiberian Dawn copies are byte-for-byte identical.

/// Common first-character table used by the legacy diphthong encoder.
pub const COMMON: [u8; 16] = *b" eta in osrlhc dupm";

/// Character-choice table indexed by the common-character slot and a 3-bit
/// secondary selector. Spaces and punctuation are significant data.
pub const DIPTHONG: [[u8; 8]; 16] = [
    *b"tasio wb", *b" rnsdalm", *b"h ieoras", *b"nrtlc sy",
    *b"nst cloer", *b" dtgesio", *b"nr ufmsw", *b" tep.ica",
    *b"e oiadur", *b" laeiyod", *b"eia otr u", *b"etoakhlr",
    *b" eiu,.oa", *b"nsrctlai", *b"leoiratp", *b"eaoip bm",
];

/// Return the common table character for a 4-bit selector.
#[inline]
pub const fn common(index: usize) -> Option<u8> {
    if index < COMMON.len() { Some(COMMON[index]) } else { None }
}

/// Return the table character for a common slot and 3-bit selector.
#[inline]
pub const fn dipthong(row: usize, column: usize) -> Option<u8> {
    if row < 16 && column < 8 { Some(DIPTHONG[row][column]) } else { None }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preserves_table_dimensions_and_endpoints() {
        assert_eq!(COMMON.len(), 16);
        assert_eq!(DIPTHONG.len(), 16);
        assert_eq!(DIPTHONG[0], *b"tasio wb");
        assert_eq!(DIPTHONG[15], *b"eaoip bm");
        assert_eq!(dipthong(16, 0), None);
        assert_eq!(dipthong(0, 8), None);
    }
}
