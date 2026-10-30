//! Public-key session-key stream wrappers from REDALERT/PKPIPE.CPP and PKSTRAW.CPP.
//!
//! Upstream baseline: electronicarts/CnC_Remastered_Collection
//! commit f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae.
//!
//! Preserves the 56-byte Blowfish session key, 256-byte staging buffers,
//! public-key block padding, encrypt/decrypt handshake ordering, push-side
//! partial-key accumulation, pull-side all-at-once key fetch, and embedded
//! stream handoff. Cryptographic primitives and base streams are injected as
//! traits, keeping this compatibility layer portable and non-blocking.

use std::sync::Arc;

pub const BLOWFISH_KEY_SIZE: usize = 56;
pub const MAX_KEY_BLOCK_SIZE: usize = 256;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CryptControl { Encrypt, Decrypt }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamCryptoError {
    InvalidBlockSize,
    KeyBlockTooLarge,
    CipherOutputTooLarge,
    CipherOutputTooShort,
    InvalidCiphertextLength,
}

/// The source PKey interface. Implementations process complete blocks only.
pub trait PublicKeyCipher: Send + Sync {
    fn plain_block_size(&self) -> usize;
    fn crypt_block_size(&self) -> usize;
    fn encrypt(&self, source: &[u8], destination: &mut [u8]) -> Result<usize, StreamCryptoError>;
    fn decrypt(&self, source: &[u8], destination: &mut [u8]) -> Result<usize, StreamCryptoError>;
}

pub trait RandomByteSource { fn fill_bytes(&mut self, destination: &mut [u8]); }
pub trait BlowfishKeyInstaller { fn install_key(&mut self, key: &[u8]); }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyBlockLengths {
    pub plaintext: usize,
    pub ciphertext: usize,
    pub blocks: usize,
}

/// PKey::Block_Count(56) multiplied by its plain/cipher block sizes.
pub fn key_block_lengths(cipher: &dyn PublicKeyCipher) -> Result<KeyBlockLengths, StreamCryptoError> {
    let plain_block = cipher.plain_block_size();
    let crypt_block = cipher.crypt_block_size();
    if plain_block == 0 || crypt_block == 0 { return Err(StreamCryptoError::InvalidBlockSize); }
    let blocks = BLOWFISH_KEY_SIZE.checked_add(plain_block - 1)
        .ok_or(StreamCryptoError::KeyBlockTooLarge)? / plain_block;
    let plaintext = blocks.checked_mul(plain_block).ok_or(StreamCryptoError::KeyBlockTooLarge)?;
    let ciphertext = blocks.checked_mul(crypt_block).ok_or(StreamCryptoError::KeyBlockTooLarge)?;
    if plaintext > MAX_KEY_BLOCK_SIZE || ciphertext > MAX_KEY_BLOCK_SIZE {
        return Err(StreamCryptoError::KeyBlockTooLarge);
    }
    Ok(KeyBlockLengths { plaintext, ciphertext, blocks })
}

pub struct PublicKeyPipe {
    control: CryptControl,
    cipher_key: Option<Arc<dyn PublicKeyCipher>>,
    is_getting_key: bool,
    buffer: [u8; MAX_KEY_BLOCK_SIZE],
    counter: usize,
    bytes_left: usize,
}

impl PublicKeyPipe {
    pub fn new(control: CryptControl) -> Self {
        Self {
            control, cipher_key: None, is_getting_key: true,
            buffer: [0; MAX_KEY_BLOCK_SIZE], counter: 0, bytes_left: 0,
        }
    }

    pub fn control(&self) -> CryptControl { self.control }
    pub fn is_getting_key(&self) -> bool { self.is_getting_key }
    pub fn has_public_key(&self) -> bool { self.cipher_key.is_some() }
    pub fn staged_bytes_left(&self) -> usize { self.bytes_left }

    pub fn encrypted_key_length(&self) -> Result<usize, StreamCryptoError> {
        self.cipher_key.as_deref().map(key_block_lengths).transpose()
            .map(|lengths| lengths.map_or(0, |value| value.ciphertext))
    }

    pub fn plain_key_length(&self) -> Result<usize, StreamCryptoError> {
        self.cipher_key.as_deref().map(key_block_lengths).transpose()
            .map(|lengths| lengths.map_or(0, |value| value.plaintext))
    }

    /// Port of PKPipe::Key. A null key flushes Pipe before disabling the
    /// public-key stage; a decrypt key primes the fixed-size accumulator.
    pub fn set_key<F>(
        &mut self,
        key: Option<Arc<dyn PublicKeyCipher>>,
        mut flush_downstream: F,
    ) -> Result<usize, StreamCryptoError>
    where F: FnMut() -> usize {
        let flushed = if key.is_none() {
            let count = flush_downstream();
            self.is_getting_key = false;
            count
        } else { 0 };
        self.cipher_key = key;
        if let Some(cipher) = self.cipher_key.as_deref() {
            self.is_getting_key = true;
            if self.control == CryptControl::Decrypt {
                let encrypted_len = key_block_lengths(cipher)?.ciphertext;
                self.counter = encrypted_len;
                self.bytes_left = encrypted_len;
            }
        }
        Ok(flushed)
    }

    /// Port of PKPipe::Put. put_base is the inherited Pipe::Put operation;
    /// it must route data through the embedded BlowPipe. The RSA key bytes
    /// are forwarded before the Blowfish key is installed, as in C++.
    pub fn put<R, B, F>(
        &mut self,
        source: Option<&[u8]>,
        random: &mut R,
        blowfish: &mut B,
        mut put_base: F,
    ) -> Result<usize, StreamCryptoError>
    where
        R: RandomByteSource,
        B: BlowfishKeyInstaller,
        F: FnMut(Option<&[u8]>) -> usize,
    {
        let Some(mut remaining) = source else { return Ok(put_base(None)); };
        if remaining.is_empty() || self.cipher_key.is_none() {
            return Ok(put_base(Some(remaining)));
        }

        let mut total = 0;
        if self.is_getting_key {
            let cipher = self.cipher_key.as_deref().expect("key presence checked");
            let lengths = key_block_lengths(cipher)?;
            match self.control {
                CryptControl::Encrypt => {
                    let mut plain_key = [0_u8; MAX_KEY_BLOCK_SIZE];
                    random.fill_bytes(&mut plain_key[..BLOWFISH_KEY_SIZE]);
                    let written = cipher.encrypt(&plain_key[..lengths.plaintext], &mut self.buffer)?;
                    if written > self.buffer.len() { return Err(StreamCryptoError::CipherOutputTooLarge); }
                    total += put_base(Some(&self.buffer[..written]));
                    blowfish.install_key(&plain_key[..BLOWFISH_KEY_SIZE]);
                    self.is_getting_key = false;
                }
                CryptControl::Decrypt => {
                    let to_get = self.bytes_left.min(remaining.len());
                    let offset = self.counter - self.bytes_left;
                    self.buffer[offset..offset + to_get].copy_from_slice(&remaining[..to_get]);
                    remaining = &remaining[to_get..];
                    self.bytes_left -= to_get;
                    if self.bytes_left == 0 {
                        let mut plain_key = [0_u8; MAX_KEY_BLOCK_SIZE];
                        let written = cipher.decrypt(&self.buffer[..self.counter], &mut plain_key)?;
                        if written < BLOWFISH_KEY_SIZE { return Err(StreamCryptoError::CipherOutputTooShort); }
                        blowfish.install_key(&plain_key[..BLOWFISH_KEY_SIZE]);
                        self.is_getting_key = false;
                    }
                }
            }
        }
        // C++ calls Pipe::Put even when the key consumed the entire input.
        total += put_base(Some(remaining));
        Ok(total)
    }
}

pub struct PublicKeyStraw {
    control: CryptControl,
    cipher_key: Option<Arc<dyn PublicKeyCipher>>,
    is_getting_key: bool,
    buffer: [u8; MAX_KEY_BLOCK_SIZE],
    counter: usize,
    bytes_left: usize,
}

impl PublicKeyStraw {
    pub fn new(control: CryptControl) -> Self {
        Self {
            control, cipher_key: None, is_getting_key: true,
            buffer: [0; MAX_KEY_BLOCK_SIZE], counter: 0, bytes_left: 0,
        }
    }

    pub fn control(&self) -> CryptControl { self.control }
    pub fn is_getting_key(&self) -> bool { self.is_getting_key }
    pub fn has_public_key(&self) -> bool { self.cipher_key.is_some() }
    pub fn staged_bytes_left(&self) -> usize { self.bytes_left }

    pub fn encrypted_key_length(&self) -> Result<usize, StreamCryptoError> {
        self.cipher_key.as_deref().map(key_block_lengths).transpose()
            .map(|lengths| lengths.map_or(0, |value| value.ciphertext))
    }

    pub fn plain_key_length(&self) -> Result<usize, StreamCryptoError> {
        self.cipher_key.as_deref().map(key_block_lengths).transpose()
            .map(|lengths| lengths.map_or(0, |value| value.plaintext))
    }

    /// Port of PKStraw::Key. A null key does not clear IsGettingKey in the
    /// original source; staging counters are reset for both null/non-null.
    pub fn set_key(&mut self, key: Option<Arc<dyn PublicKeyCipher>>) -> Result<(), StreamCryptoError> {
        self.cipher_key = key;
        if self.cipher_key.is_some() { self.is_getting_key = true; }
        self.counter = 0;
        self.bytes_left = 0;
        if let Some(cipher) = self.cipher_key.as_deref() { let _ = key_block_lengths(cipher)?; }
        Ok(())
    }

    /// Port of PKStraw::Get. get_base represents Straw::Get through the
    /// embedded BlowStraw. Encrypt-mode key bytes may be split across caller
    /// reads; decrypt mode requires the complete RSA key block in one read.
    pub fn get<R, B, F>(
        &mut self,
        destination: &mut [u8],
        random: &mut R,
        blowfish: &mut B,
        mut get_base: F,
    ) -> Result<usize, StreamCryptoError>
    where
        R: RandomByteSource,
        B: BlowfishKeyInstaller,
        F: FnMut(&mut [u8]) -> usize,
    {
        if destination.is_empty() || self.cipher_key.is_none() {
            return Ok(get_base(destination));
        }

        if self.is_getting_key {
            let cipher = self.cipher_key.as_deref().expect("key presence checked");
            let lengths = key_block_lengths(cipher)?;
            match self.control {
                CryptControl::Decrypt => {
                    let mut encrypted_key = [0_u8; MAX_KEY_BLOCK_SIZE];
                    let got = get_base(&mut encrypted_key[..lengths.ciphertext]);
                    if got != lengths.ciphertext { return Ok(0); }
                    let written = cipher.decrypt(&encrypted_key[..got], &mut self.buffer)?;
                    if written < BLOWFISH_KEY_SIZE { return Err(StreamCryptoError::CipherOutputTooShort); }
                    blowfish.install_key(&self.buffer[..BLOWFISH_KEY_SIZE]);
                }
                CryptControl::Encrypt => {
                    let mut plain_key = [0_u8; MAX_KEY_BLOCK_SIZE];
                    random.fill_bytes(&mut plain_key[..BLOWFISH_KEY_SIZE]);
                    let written = cipher.encrypt(&plain_key[..lengths.plaintext], &mut self.buffer)?;
                    if written > self.buffer.len() { return Err(StreamCryptoError::CipherOutputTooLarge); }
                    self.counter = written;
                    self.bytes_left = written;
                    blowfish.install_key(&plain_key[..BLOWFISH_KEY_SIZE]);
                }
            }
            self.is_getting_key = false;
        }

        let mut total = 0;
        if self.bytes_left > 0 {
            let to_copy = destination.len().min(self.bytes_left);
            let offset = self.counter - self.bytes_left;
            destination[..to_copy].copy_from_slice(&self.buffer[offset..offset + to_copy]);
            self.bytes_left -= to_copy;
            total += to_copy;
        }
        if total < destination.len() { total += get_base(&mut destination[total..]); }
        Ok(total)
    }
}

// Intrusive chain rewiring for PKPipe::Put_To and PKStraw::Get_From.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StreamNodeId(usize);
impl StreamNodeId { pub const fn index(self) -> usize { self.0 } }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WrapperKind { Pipe, Straw }

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct StreamLinks {
    next: Option<StreamNodeId>,
    previous: Option<StreamNodeId>,
}

/// Index-based counterpart of the inherited ChainTo/ChainFrom pointers.
#[derive(Debug, Default)]
pub struct StreamChainGraph {
    links: Vec<StreamLinks>,
    wrapper_embedded: Vec<Option<StreamNodeId>>,
    embedded_wrapper: Vec<Option<StreamNodeId>>,
    wrapper_kind: Vec<Option<WrapperKind>>,
}

impl StreamChainGraph {
    pub fn new() -> Self { Self::default() }

    pub fn add_node(&mut self) -> StreamNodeId {
        let id = StreamNodeId(self.links.len());
        self.links.push(StreamLinks::default());
        self.wrapper_embedded.push(None);
        self.embedded_wrapper.push(None);
        self.wrapper_kind.push(None);
        id
    }

    pub fn add_pipe_wrapper(&mut self) -> (StreamNodeId, StreamNodeId) {
        self.add_wrapper(WrapperKind::Pipe)
    }
    pub fn add_straw_wrapper(&mut self) -> (StreamNodeId, StreamNodeId) {
        // PKStraw's constructor calls the base Straw::Get_From(BF) before
        // any caller-provided source is connected.
        let (wrapper, embedded) = self.add_wrapper(WrapperKind::Straw);
        self.links[wrapper.0].next = Some(embedded);
        self.links[embedded.0].previous = Some(wrapper);
        (wrapper, embedded)
    }

    fn add_wrapper(&mut self, kind: WrapperKind) -> (StreamNodeId, StreamNodeId) {
        let wrapper = self.add_node();
        let embedded = self.add_node();
        self.wrapper_embedded[wrapper.0] = Some(embedded);
        self.embedded_wrapper[embedded.0] = Some(wrapper);
        self.wrapper_kind[wrapper.0] = Some(kind);
        (wrapper, embedded)
    }

    pub fn chain_to(&self, node: StreamNodeId) -> Option<StreamNodeId> {
        self.links.get(node.0).and_then(|link| link.next)
    }
    pub fn chain_from(&self, node: StreamNodeId) -> Option<StreamNodeId> {
        self.links.get(node.0).and_then(|link| link.previous)
    }

    /// Port of PKPipe::Put_To.
    pub fn put_to(&mut self, wrapper: StreamNodeId, destination: Option<StreamNodeId>) -> Result<(), StreamCryptoError> {
        self.rewire(wrapper, destination, WrapperKind::Pipe)
    }

    /// Port of PKStraw::Get_From, including the source's wrapper ChainFrom value.
    pub fn get_from(&mut self, wrapper: StreamNodeId, source: Option<StreamNodeId>) -> Result<(), StreamCryptoError> {
        self.rewire(wrapper, source, WrapperKind::Straw)
    }

    fn rewire(
        &mut self,
        wrapper: StreamNodeId,
        destination: Option<StreamNodeId>,
        expected_kind: WrapperKind,
    ) -> Result<(), StreamCryptoError> {
        let embedded = self.wrapper_embedded.get(wrapper.0).and_then(|entry| *entry)
            .ok_or(StreamCryptoError::InvalidCiphertextLength)?;
        if self.wrapper_kind[wrapper.0] != Some(expected_kind) {
            return Err(StreamCryptoError::InvalidCiphertextLength);
        }
        if self.links[embedded.0].next == destination { return Ok(()); }

        if let Some(destination) = destination {
            let previous = self.links.get(destination.0)
                .ok_or(StreamCryptoError::InvalidCiphertextLength)?.previous;
            if let Some(previous) = previous {
                self.detach_stream(previous)?;
                self.links[destination.0].previous = None;
            }
        }
        if let Some(old_destination) = self.links[embedded.0].next {
            self.links[old_destination.0].previous = None;
        }

        self.links[embedded.0].next = destination;
        self.links[embedded.0].previous = Some(wrapper);
        self.links[wrapper.0].next = Some(embedded);
        if let Some(destination) = destination {
            let previous = if expected_kind == WrapperKind::Pipe { embedded } else { wrapper };
            self.links[destination.0].previous = Some(previous);
        }
        Ok(())
    }

    fn detach_stream(&mut self, chain_from: StreamNodeId) -> Result<(), StreamCryptoError> {
        let wrapper = self.wrapper_embedded.get(chain_from.0).and_then(|entry| *entry)
            .or_else(|| self.embedded_wrapper.get(chain_from.0).and_then(|entry| *entry));
        if let Some(wrapper) = wrapper {
            let kind = self.wrapper_kind[wrapper.0].ok_or(StreamCryptoError::InvalidCiphertextLength)?;
            self.rewire(wrapper, None, kind)
        } else {
            let next = self.links.get(chain_from.0)
                .ok_or(StreamCryptoError::InvalidCiphertextLength)?.next;
            if let Some(next) = next { self.links[next.0].previous = None; }
            self.links[chain_from.0].next = None;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeRsa;
    impl PublicKeyCipher for FakeRsa {
        fn plain_block_size(&self) -> usize { 8 }
        fn crypt_block_size(&self) -> usize { 9 }
        fn encrypt(&self, source: &[u8], destination: &mut [u8]) -> Result<usize, StreamCryptoError> {
            let blocks = source.len() / 8;
            let required = blocks * 9;
            if source.len() % 8 != 0 || destination.len() < required {
                return Err(StreamCryptoError::CipherOutputTooLarge);
            }
            for (index, chunk) in source.chunks_exact(8).enumerate() {
                destination[index * 9] = 0xA5;
                destination[index * 9 + 1..index * 9 + 9].copy_from_slice(chunk);
            }
            Ok(required)
        }
        fn decrypt(&self, source: &[u8], destination: &mut [u8]) -> Result<usize, StreamCryptoError> {
            let blocks = source.len() / 9;
            let required = blocks * 8;
            if source.len() % 9 != 0 || destination.len() < required {
                return Err(StreamCryptoError::InvalidCiphertextLength);
            }
            for (index, chunk) in source.chunks_exact(9).enumerate() {
                if chunk[0] != 0xA5 { return Err(StreamCryptoError::InvalidCiphertextLength); }
                destination[index * 8..index * 8 + 8].copy_from_slice(&chunk[1..]);
            }
            Ok(required)
        }
    }

    #[derive(Default)]
    struct FakeRandom(u8);
    impl RandomByteSource for FakeRandom {
        fn fill_bytes(&mut self, destination: &mut [u8]) {
            for byte in destination {
                self.0 = self.0.wrapping_add(1);
                *byte = self.0;
            }
        }
    }

    #[derive(Default, Debug)]
    struct FakeBlowfish { installed: Vec<Vec<u8>> }
    impl BlowfishKeyInstaller for FakeBlowfish {
        fn install_key(&mut self, key: &[u8]) { self.installed.push(key.to_vec()); }
    }

    fn fake_key() -> Arc<dyn PublicKeyCipher> { Arc::new(FakeRsa) }

    fn encode_key(key: &FakeRsa, raw: &[u8; BLOWFISH_KEY_SIZE]) -> Vec<u8> {
        let mut plain = [0_u8; MAX_KEY_BLOCK_SIZE];
        plain[..BLOWFISH_KEY_SIZE].copy_from_slice(raw);
        let lengths = key_block_lengths(key).unwrap();
        let mut encrypted = [0_u8; MAX_KEY_BLOCK_SIZE];
        let written = key.encrypt(&plain[..lengths.plaintext], &mut encrypted).unwrap();
        encrypted[..written].to_vec()
    }

    #[test]
    fn key_lengths_match_source_block_count_and_blowfish_key_size() {
        let lengths = key_block_lengths(&FakeRsa).unwrap();
        assert_eq!(BLOWFISH_KEY_SIZE, 56);
        assert_eq!(lengths, KeyBlockLengths { plaintext: 56, ciphertext: 63, blocks: 7 });
    }

    #[test]
    fn pipe_encrypts_random_session_key_before_forwarding_payload() {
        let mut pipe = PublicKeyPipe::new(CryptControl::Encrypt);
        pipe.set_key(Some(fake_key()), || 0).unwrap();
        let mut random = FakeRandom::default();
        let mut blowfish = FakeBlowfish::default();
        let mut forwarded = Vec::new();
        let count = pipe.put(Some(b"payload"), &mut random, &mut blowfish, |chunk| {
            if let Some(chunk) = chunk { forwarded.push(chunk.to_vec()); }
            chunk.map_or(0, |chunk| chunk.len())
        }).unwrap();
        assert_eq!(count, 63 + 7);
        assert_eq!(forwarded[0].len(), 63);
        assert_eq!(forwarded[1], b"payload");
        assert_eq!(blowfish.installed[0].len(), BLOWFISH_KEY_SIZE);
        assert_eq!(blowfish.installed[0][0], 1);
        assert!(!pipe.is_getting_key());
    }

    #[test]
    fn pipe_decrypt_accumulates_encrypted_key_across_put_calls() {
        let raw_key: [u8; BLOWFISH_KEY_SIZE] = std::array::from_fn(|i| (i as u8).wrapping_add(7));
        let encoded = encode_key(&FakeRsa, &raw_key);
        let mut pipe = PublicKeyPipe::new(CryptControl::Decrypt);
        pipe.set_key(Some(fake_key()), || 0).unwrap();
        let mut random = FakeRandom::default();
        let mut blowfish = FakeBlowfish::default();
        let mut output = Vec::new();
        let first = pipe.put(Some(&encoded[..17]), &mut random, &mut blowfish, |chunk| {
            if let Some(chunk) = chunk { output.extend_from_slice(chunk); }
            chunk.map_or(0, |chunk| chunk.len())
        }).unwrap();
        assert_eq!(first, 0);
        assert_eq!(pipe.staged_bytes_left(), 46);
        assert!(blowfish.installed.is_empty());

        let mut second_chunk = encoded[17..].to_vec();
        second_chunk.extend_from_slice(b"after-key");
        let second = pipe.put(Some(&second_chunk), &mut random, &mut blowfish, |chunk| {
            if let Some(chunk) = chunk { output.extend_from_slice(chunk); }
            chunk.map_or(0, |chunk| chunk.len())
        }).unwrap();
        assert_eq!(second, 9);
        assert_eq!(output, b"after-key");
        assert_eq!(blowfish.installed, vec![raw_key.to_vec()]);
        assert!(!pipe.is_getting_key());
    }

    #[test]
    fn pipe_passthrough_and_null_key_flush_follow_source_control_flow() {
        let mut pipe = PublicKeyPipe::new(CryptControl::Encrypt);
        let mut random = FakeRandom::default();
        let mut blowfish = FakeBlowfish::default();
        let mut received = Vec::new();
        assert_eq!(pipe.put(Some(b"clear"), &mut random, &mut blowfish, |chunk| {
            let data = chunk.unwrap_or_default();
            received.extend_from_slice(data);
            data.len()
        }).unwrap(), 5);
        assert_eq!(received, b"clear");

        pipe.set_key(Some(fake_key()), || 0).unwrap();
        let mut flushes = 0;
        assert_eq!(pipe.set_key(None, || { flushes += 1; 12 }).unwrap(), 12);
        assert_eq!(flushes, 1);
        assert!(!pipe.has_public_key());
        assert!(!pipe.is_getting_key());
    }

    #[test]
    fn pipe_decrypt_forwards_empty_tail_after_key_block_completion() {
        let raw_key = [0x5A; BLOWFISH_KEY_SIZE];
        let encoded = encode_key(&FakeRsa, &raw_key);
        let mut pipe = PublicKeyPipe::new(CryptControl::Decrypt);
        pipe.set_key(Some(fake_key()), || 0).unwrap();
        let mut random = FakeRandom::default();
        let mut blowfish = FakeBlowfish::default();
        let mut forwarded_calls = Vec::new();
        pipe.put(Some(&encoded), &mut random, &mut blowfish, |chunk| {
            forwarded_calls.push(chunk.map_or(usize::MAX, |chunk| chunk.len()));
            chunk.map_or(0, |chunk| chunk.len())
        }).unwrap();
        assert!(forwarded_calls.contains(&0));
        assert_eq!(blowfish.installed, vec![raw_key.to_vec()]);
    }

    #[test]
    fn invalid_block_sizes_and_oversized_key_blocks_are_rejected() {
        struct BadKey;
        impl PublicKeyCipher for BadKey {
            fn plain_block_size(&self) -> usize { 0 }
            fn crypt_block_size(&self) -> usize { 1 }
            fn encrypt(&self, _: &[u8], _: &mut [u8]) -> Result<usize, StreamCryptoError> { Ok(0) }
            fn decrypt(&self, _: &[u8], _: &mut [u8]) -> Result<usize, StreamCryptoError> { Ok(0) }
        }
        assert_eq!(key_block_lengths(&BadKey), Err(StreamCryptoError::InvalidBlockSize));

        struct HugeKey;
        impl PublicKeyCipher for HugeKey {
            fn plain_block_size(&self) -> usize { 1 }
            fn crypt_block_size(&self) -> usize { 257 }
            fn encrypt(&self, _: &[u8], _: &mut [u8]) -> Result<usize, StreamCryptoError> { Ok(0) }
            fn decrypt(&self, _: &[u8], _: &mut [u8]) -> Result<usize, StreamCryptoError> { Ok(0) }
        }
        assert_eq!(key_block_lengths(&HugeKey), Err(StreamCryptoError::KeyBlockTooLarge));
    }

    #[test]
    fn straw_encrypt_stages_rsa_block_over_multiple_reads_before_payload() {
        let mut straw = PublicKeyStraw::new(CryptControl::Encrypt);
        straw.set_key(Some(fake_key())).unwrap();
        let mut random = FakeRandom::default();
        let mut blowfish = FakeBlowfish::default();
        let mut source_reads = 0;
        let mut first = [0_u8; 13];
        assert_eq!(straw.get(&mut first, &mut random, &mut blowfish, |out| {
            source_reads += 1; out.fill(0xEE); out.len()
        }).unwrap(), 13);
        assert_eq!(source_reads, 0);
        assert_eq!(first[0], 0xA5);

        let mut rest = [0_u8; 50];
        assert_eq!(straw.get(&mut rest, &mut random, &mut blowfish, |out| {
            source_reads += 1; out.fill(0xEE); out.len()
        }).unwrap(), 50);
        assert_eq!(source_reads, 0);
        assert_eq!(straw.staged_bytes_left(), 0);

        let mut payload = [0_u8; 5];
        assert_eq!(straw.get(&mut payload, &mut random, &mut blowfish, |out| {
            source_reads += 1; out.fill(0xEE); out.len()
        }).unwrap(), 5);
        assert_eq!(payload, [0xEE; 5]);
        assert_eq!(source_reads, 1);
        assert_eq!(blowfish.installed[0].len(), BLOWFISH_KEY_SIZE);
    }

    #[test]
    fn straw_decrypt_requires_the_complete_key_block_in_one_upstream_read() {
        let raw_key = [0x31; BLOWFISH_KEY_SIZE];
        let mut partial = encode_key(&FakeRsa, &raw_key);
        let mut straw = PublicKeyStraw::new(CryptControl::Decrypt);
        straw.set_key(Some(fake_key())).unwrap();
        let mut random = FakeRandom::default();
        let mut blowfish = FakeBlowfish::default();
        let mut short = [0_u8; 5];
        let got = straw.get(&mut short, &mut random, &mut blowfish, |out| {
            let take = out.len().min(partial.len() / 2);
            out[..take].copy_from_slice(&partial[..take]);
            partial.drain(..take);
            take
        }).unwrap();
        assert_eq!(got, 0);
        assert!(blowfish.installed.is_empty());
        assert!(straw.is_getting_key());
    }

    #[test]
    fn straw_decrypt_installs_key_then_reads_payload_through_base_straw() {
        let raw_key = [0x42; BLOWFISH_KEY_SIZE];
        let mut packet = encode_key(&FakeRsa, &raw_key);
        packet.extend_from_slice(b"hello");
        let mut cursor = 0;
        let mut straw = PublicKeyStraw::new(CryptControl::Decrypt);
        straw.set_key(Some(fake_key())).unwrap();
        let mut random = FakeRandom::default();
        let mut blowfish = FakeBlowfish::default();
        let mut output = [0_u8; 5];
        assert_eq!(straw.get(&mut output, &mut random, &mut blowfish, |out| {
            let available = packet.len() - cursor;
            let take = available.min(out.len());
            out[..take].copy_from_slice(&packet[cursor..cursor + take]);
            cursor += take;
            take
        }).unwrap(), 5);
        assert_eq!(&output, b"hello");
        assert_eq!(blowfish.installed, vec![raw_key.to_vec()]);
    }

    #[test]
    fn straw_null_key_preserves_source_is_getting_key_flag_semantics() {
        let mut straw = PublicKeyStraw::new(CryptControl::Encrypt);
        assert!(straw.is_getting_key());
        straw.set_key(None).unwrap();
        assert!(straw.is_getting_key());
        assert!(!straw.has_public_key());
        straw.set_key(Some(fake_key())).unwrap();
        assert!(straw.is_getting_key());
    }

    #[test]
    fn pipe_chain_reconnects_sink_and_detaches_previous_wrapper() {
        let mut graph = StreamChainGraph::new();
        let (first, first_blow) = graph.add_pipe_wrapper();
        let (second, second_blow) = graph.add_pipe_wrapper();
        let sink = graph.add_node();
        graph.put_to(first, Some(sink)).unwrap();
        assert_eq!(graph.chain_to(first), Some(first_blow));
        assert_eq!(graph.chain_from(first_blow), Some(first));
        assert_eq!(graph.chain_to(first_blow), Some(sink));
        assert_eq!(graph.chain_from(sink), Some(first_blow));

        graph.put_to(second, Some(sink)).unwrap();
        assert_eq!(graph.chain_to(first_blow), None);
        assert_eq!(graph.chain_from(sink), Some(second_blow));
        assert_eq!(graph.chain_to(second), Some(second_blow));
        graph.put_to(second, None).unwrap();
        assert_eq!(graph.chain_to(second_blow), None);
        assert_eq!(graph.chain_from(sink), None);
    }

    #[test]
    fn straw_chain_preserves_source_wrapper_chainfrom_assignment() {
        let mut graph = StreamChainGraph::new();
        let (straw, blow) = graph.add_straw_wrapper();
        let source = graph.add_node();
        graph.get_from(straw, Some(source)).unwrap();
        assert_eq!(graph.chain_to(straw), Some(blow));
        assert_eq!(graph.chain_from(blow), Some(straw));
        assert_eq!(graph.chain_to(blow), Some(source));
        assert_eq!(graph.chain_from(source), Some(straw));
        graph.get_from(straw, None).unwrap();
        assert_eq!(graph.chain_to(blow), None);
        assert_eq!(graph.chain_from(source), None);
    }
}
