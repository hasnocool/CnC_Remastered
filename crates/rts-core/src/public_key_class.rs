//! Westwood PKey public/private key implementation from REDALERT/PK.CPP and PK.H.
//!
//! Upstream baseline: electronicarts/CnC_Remastered_Collection at
//! f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae.
//!
//! Ports DER key import/export, modulus precision, block sizing, full-block
//! modular-exponentiation encryption/decryption, the 65537 fast exponent, and
//! the exact active Generate() behavior (a no-op because its key-generation
//! implementation is enclosed in #if (0) in this revision). BigInt's fixed
//! precision storage and modular exponent engine are supplied by PKeyInteger;
//! this avoids the source's raw memmove into a C++ object representation.

pub const FAST_EXPONENT: u32 = 65_537;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PKeyError {
    InvalidBlockSize,
    InvalidPrecision,
    SourceTooLarge,
    DestinationTooSmall { required: usize, available: usize },
    IntegerBackend,
    DerEncoding,
    DerDecoding,
    SizeOverflow,
}

pub trait PKeyInteger: Clone + Default {
    /// DERDecode equivalent. Implementations consume one DER integer from the
    /// supplied buffer and report the consumed byte count when available.
    fn der_decode(&mut self, source: &[u8]) -> Result<usize, PKeyError>;
    fn der_encode(&self, destination: &mut [u8]) -> Result<usize, PKeyError>;
    fn bit_count(&self) -> usize;

    /// Imports/exports the same little-endian fixed-precision representation
    /// that C++ memmove(&BigInt, bytes, block_size) uses.
    fn import_raw_le(&mut self, source: &[u8]) -> Result<(), PKeyError>;
    fn export_raw_le(&self, destination: &mut [u8]) -> Result<(), PKeyError>;

    /// Equivalent to value.exp_b_mod_c(exponent, modulus).
    fn exp_b_mod_c(
        &self,
        exponent: &Self,
        modulus: &Self,
    ) -> Result<Self, PKeyError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PKey<I: PKeyInteger> {
    modulus: I,
    exponent: I,
    bit_precision: i32,
}

impl<I: PKeyInteger> Default for PKey<I> {
    fn default() -> Self {
        Self { modulus: I::default(), exponent: I::default(), bit_precision: 0 }
    }
}

impl<I: PKeyInteger> PKey<I> {
    /// PKey() : Modulus(0), Exponent(0), BitPrecision(0).
    pub fn new() -> Self { Self::default() }

    /// Port of PKey(exponent, modulus). DER decodes exponent first, then
    /// modulus, and BitPrecision is Modulus.BitCount()-1.
    pub fn from_der(exponent_der: &[u8], modulus_der: &[u8]) -> Result<Self, PKeyError> {
        let mut key = Self::new();
        key.decode_exponent(exponent_der)?;
        key.decode_modulus(modulus_der)?;
        Ok(key)
    }

    pub fn bit_precision(&self) -> i32 { self.bit_precision }
    pub fn modulus(&self) -> &I { &self.modulus }
    pub fn exponent(&self) -> &I { &self.exponent }

    /// C++ Plain_Block_Size: (BitPrecision - 1) / 8, using truncation toward
    /// zero as in the original signed integer expression.
    pub fn plain_block_size(&self) -> usize {
        ((self.bit_precision - 1) / 8).max(0) as usize
    }

    pub fn crypt_block_size(&self) -> usize {
        self.plain_block_size().saturating_add(1)
    }

    /// C++ Block_Count formula. In particular, zero input returns one block
    /// whenever the key has a non-zero plain block size.
    pub fn block_count(&self, plaintext_length: usize) -> Result<usize, PKeyError> {
        let block_size = self.plain_block_size();
        if block_size == 0 { return Err(PKeyError::InvalidBlockSize); }
        Ok(plaintext_length.saturating_sub(1) / block_size + 1)
    }

    /// Encode_Modulus(NULL) returns 0; a non-null destination DER-encodes it.
    pub fn encode_modulus(
        &self,
        destination: Option<&mut [u8]>,
    ) -> Result<usize, PKeyError> {
        match destination {
            None => Ok(0),
            Some(output) => self.modulus.der_encode(output),
        }
    }

    /// Encode_Exponent(NULL) returns 0; otherwise DER-encodes the exponent.
    pub fn encode_exponent(
        &self,
        destination: Option<&mut [u8]>,
    ) -> Result<usize, PKeyError> {
        match destination {
            None => Ok(0),
            Some(output) => self.exponent.der_encode(output),
        }
    }

    pub fn decode_modulus(&mut self, source: &[u8]) -> Result<usize, PKeyError> {
        let consumed = self.modulus.der_decode(source)?;
        self.bit_precision = self.modulus.bit_count() as i32 - 1;
        Ok(consumed)
    }

    pub fn decode_exponent(&mut self, source: &[u8]) -> Result<usize, PKeyError> {
        self.exponent.der_decode(source)
    }

    /// C++ Fast_Exponent() returns 65537 (long).
    pub const fn fast_exponent() -> u32 { FAST_EXPONENT }

    /// PKey::Generate's key-generation loop is completely disabled by #if (0)
    /// in the pinned upstream source. The active function only evaluates its
    /// two unused references and leaves both keys unchanged.
    pub fn generate(
        _random_source: &mut impl RandomByteSource,
        _bits: i32,
        _fast_key: &mut Self,
        _slow_key: &mut Self,
    ) {}

    /// Encrypt complete plaintext blocks only. Any partial final block is
    /// ignored, matching PK.CPP. Destination sizing is checked before writes.
    pub fn encrypt(
        &self,
        source: &[u8],
        destination: &mut [u8],
    ) -> Result<usize, PKeyError> {
        self.transform_blocks(source, destination, true)
    }

    /// Decrypt complete ciphertext blocks only; partial final blocks are
    /// ignored, matching PK.CPP.
    pub fn decrypt(
        &self,
        source: &[u8],
        destination: &mut [u8],
    ) -> Result<usize, PKeyError> {
        self.transform_blocks(source, destination, false)
    }

    fn transform_blocks(
        &self,
        source: &[u8],
        destination: &mut [u8],
        encrypting: bool,
    ) -> Result<usize, PKeyError> {
        let plain_size = self.plain_block_size();
        let crypt_size = self.crypt_block_size();
        if plain_size == 0 || crypt_size == 0 {
            return Err(PKeyError::InvalidBlockSize);
        }

        let input_block = if encrypting { plain_size } else { crypt_size };
        let output_block = if encrypting { crypt_size } else { plain_size };
        let block_count = source.len() / input_block;
        let required = block_count.checked_mul(output_block).ok_or(PKeyError::SizeOverflow)?;
        if destination.len() < required {
            return Err(PKeyError::DestinationTooSmall {
                required,
                available: destination.len(),
            });
        }

        for index in 0..block_count {
            let input_start = index * input_block;
            let output_start = index * output_block;
            let mut value = I::default();
            value.import_raw_le(&source[input_start..input_start + input_block])?;
            let transformed = value.exp_b_mod_c(&self.exponent, &self.modulus)?;
            transformed.export_raw_le(&mut destination[output_start..output_start + output_block])?;
        }
        Ok(required)
    }
}

pub trait RandomByteSource {
    fn fill_bytes(&mut self, destination: &mut [u8]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Default, PartialEq, Eq)]
    struct TinyInt(u128);

    impl PKeyInteger for TinyInt {
        fn der_decode(&mut self, source: &[u8]) -> Result<usize, PKeyError> {
            if source.len() < 16 { return Err(PKeyError::DerDecoding); }
            self.0 = u128::from_le_bytes(source[..16].try_into().map_err(|_| PKeyError::DerDecoding)?);
            Ok(16)
        }

        fn der_encode(&self, destination: &mut [u8]) -> Result<usize, PKeyError> {
            if destination.len() < 16 { return Err(PKeyError::DestinationTooSmall { required: 16, available: destination.len() }); }
            destination[..16].copy_from_slice(&self.0.to_le_bytes());
            Ok(16)
        }

        fn bit_count(&self) -> usize {
            if self.0 == 0 { 0 } else { 128 - self.0.leading_zeros() as usize }
        }

        fn import_raw_le(&mut self, source: &[u8]) -> Result<(), PKeyError> {
            if source.len() > 16 { return Err(PKeyError::SourceTooLarge); }
            let mut raw = [0_u8; 16];
            raw[..source.len()].copy_from_slice(source);
            self.0 = u128::from_le_bytes(raw);
            Ok(())
        }

        fn export_raw_le(&self, destination: &mut [u8]) -> Result<(), PKeyError> {
            if destination.len() > 16 { return Err(PKeyError::DestinationTooSmall { required: destination.len(), available: 16 }); }
            let raw = self.0.to_le_bytes();
            destination.copy_from_slice(&raw[..destination.len()]);
            Ok(())
        }

        fn exp_b_mod_c(&self, exponent: &Self, modulus: &Self) -> Result<Self, PKeyError> {
            if modulus.0 == 0 { return Err(PKeyError::IntegerBackend); }
            fn add_mod(a: u128, b: u128, m: u128) -> u128 {
                if a >= m - b { a - (m - b) } else { a + b }
            }
            fn mul_mod(mut a: u128, mut b: u128, m: u128) -> u128 {
                a %= m;
                let mut result = 0;
                while b > 0 {
                    if b & 1 == 1 { result = add_mod(result, a, m); }
                    b >>= 1;
                    if b > 0 { a = add_mod(a, a, m); }
                }
                result
            }
            let mut base = self.0 % modulus.0;
            let mut power = exponent.0;
            let mut result = 1 % modulus.0;
            while power > 0 {
                if power & 1 == 1 { result = mul_mod(result, base, modulus.0); }
                power >>= 1;
                if power > 0 { base = mul_mod(base, base, modulus.0); }
            }
            Ok(Self(result))
        }
    }

    struct DeterministicRandom;
    impl RandomByteSource for DeterministicRandom {
        fn fill_bytes(&mut self, destination: &mut [u8]) {
            destination.fill(0xA5);
        }
    }

    fn encode(value: u128) -> [u8; 16] { value.to_le_bytes() }

    fn key_with_modulus(modulus: u128, exponent: u128) -> PKey<TinyInt> {
        PKey::from_der(&encode(exponent), &encode(modulus)).unwrap()
    }

    #[test]
    fn default_key_has_zero_precision_and_invalid_block_sizes() {
        let key = PKey::<TinyInt>::new();
        assert_eq!(key.bit_precision(), 0);
        assert_eq!(key.plain_block_size(), 0);
        assert_eq!(key.crypt_block_size(), 1);
        assert_eq!(key.encrypt(b"", &mut []).unwrap_err(), PKeyError::InvalidBlockSize);
    }

    #[test]
    fn der_constructor_decodes_exponent_then_modulus_and_sets_precision() {
        let key = key_with_modulus(0xFFFF, 17);
        assert_eq!(key.modulus().0, 0xFFFF);
        assert_eq!(key.exponent().0, 17);
        assert_eq!(key.bit_precision(), 15);
        assert_eq!(key.plain_block_size(), 1);
        assert_eq!(key.crypt_block_size(), 2);
    }

    #[test]
    fn modulus_decode_updates_precision_but_exponent_decode_does_not() {
        let mut key = PKey::<TinyInt>::new();
        key.decode_exponent(&encode(17)).unwrap();
        assert_eq!(key.bit_precision(), 0);
        key.decode_modulus(&encode(0xFFFF)).unwrap();
        assert_eq!(key.bit_precision(), 15);
        key.decode_exponent(&encode(257)).unwrap();
        assert_eq!(key.bit_precision(), 15);
    }

    #[test]
    fn der_encode_null_returns_zero_and_non_null_encodes_integer() {
        let key = key_with_modulus(0xFFFF, 17);
        assert_eq!(key.encode_modulus(None).unwrap(), 0);
        assert_eq!(key.encode_exponent(None).unwrap(), 0);
        let mut output = [0; 16];
        assert_eq!(key.encode_exponent(Some(&mut output)).unwrap(), 16);
        assert_eq!(u128::from_le_bytes(output), 17);
    }

    #[test]
    fn block_count_matches_source_formula_including_zero_length_edge() {
        let key = key_with_modulus(0xFFFF, 17);
        assert_eq!(key.block_count(0), Ok(1));
        assert_eq!(key.block_count(1), Ok(1));
        assert_eq!(key.block_count(2), Ok(2));
        assert_eq!(key.block_count(9), Ok(9));
    }

    #[test]
    fn encrypt_and_decrypt_process_whole_blocks_and_ignore_partial_tail() {
        // A 16-bit modulus gives 1-byte plaintext and 2-byte ciphertext blocks.
        let key = key_with_modulus(0xFFFF, 17);
        let mut encrypted = [0_u8; 6];
        let written = key.encrypt(&[2, 3, 4, 99], &mut encrypted).unwrap();
        assert_eq!(written, 6);
        let mut decrypted = [0_u8; 4];
        let restored = key.decrypt(&encrypted, &mut decrypted).unwrap();
        assert_eq!(restored, 3);
        assert_eq!(&decrypted[..3], &[2, 3, 4]);
    }

    #[test]
    fn encryption_and_decryption_check_destination_capacity_before_writing() {
        let key = key_with_modulus(0xFFFF, 17);
        let mut short = [0xCC; 1];
        assert_eq!(
            key.encrypt(&[7], &mut short),
            Err(PKeyError::DestinationTooSmall { required: 2, available: 1 }),
        );
        assert_eq!(short, [0xCC]);
    }

    #[test]
    fn fast_exponent_matches_upstream_and_generate_is_active_noop() {
        assert_eq!(PKey::<TinyInt>::fast_exponent(), 65_537);
        let mut fast = key_with_modulus(0xFFFF, 17);
        let mut slow = key_with_modulus(0xFFEF, 257);
        let old_fast = fast.clone();
        let old_slow = slow.clone();
        PKey::generate(&mut DeterministicRandom, 1024, &mut fast, &mut slow);
        assert_eq!(fast, old_fast);
        assert_eq!(slow, old_slow);
    }

    #[test]
    fn encrypted_blocks_round_trip_under_small_test_key() {
        let key = key_with_modulus(0xFFFF, 17);
        let input = [10, 22, 33, 44, 55];
        let mut encrypted = [0; 10];
        let written = key.encrypt(&input, &mut encrypted).unwrap();
        let mut output = [0; 5];
        let decoded = key.decrypt(&encrypted[..written], &mut output).unwrap();
        assert_eq!(decoded, input.len());
        assert_eq!(output, input);
    }
}
