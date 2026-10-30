//! Streaming adapters ported from REDALERT/CRCPIPE.* and CRCSTRAW.*.
//!
//! This module composes the existing legacy checksum engine with demand-driven
//! input and push-driven output traits. The wrappers preserve the upstream
//! ordering: CRCPipe hashes the submitted source bytes before forwarding;
//! CRCStraw hashes only the number of bytes actually returned by its source.

use crate::legacy_crc::LegacyCrcEngine;
use crate::pipe::PipeSink;
use crate::straw::StrawSource;

/// Null pipe endpoint: accepts and discards all supplied bytes, as Pipe::Put does
/// when no ChainTo endpoint is attached.
#[derive(Debug, Default, Clone, Copy)]
pub struct NullPipe;

impl PipeSink for NullPipe {
    fn put(&mut self, source: &[u8]) -> usize {
        source.len()
    }
}

/// CRC-observing push adapter corresponding to CRCPipe.
#[derive(Debug, Clone)]
pub struct CrcPipe<P> {
    downstream: P,
    crc: LegacyCrcEngine,
}

impl<P> CrcPipe<P> {
    pub fn new(downstream: P) -> Self {
        Self {
            downstream,
            crc: LegacyCrcEngine::default(),
        }
    }

    pub fn result(&self) -> u32 {
        self.crc.value()
    }

    pub const fn committed_result(&self) -> u32 {
        self.crc.committed_value()
    }

    pub fn downstream(&self) -> &P {
        &self.downstream
    }

    pub fn downstream_mut(&mut self) -> &mut P {
        &mut self.downstream
    }

    pub fn into_parts(self) -> (P, LegacyCrcEngine) {
        (self.downstream, self.crc)
    }
}

impl<P: PipeSink> PipeSink for CrcPipe<P> {
    fn put(&mut self, source: &[u8]) -> usize {
        // Upstream CRCPipe::Put updates CRC before calling Pipe::Put.
        self.crc.update(source);
        self.downstream.put(source)
    }

    fn flush(&mut self) -> usize {
        self.downstream.flush()
    }

    fn end(&mut self) -> usize {
        self.downstream.end()
    }
}

/// Empty source: Straw::Get with no ChainTo returns zero bytes.
#[derive(Debug, Default, Clone, Copy)]
pub struct EmptyStraw;

impl StrawSource for EmptyStraw {
    fn get(&mut self, _destination: &mut [u8]) -> usize {
        0
    }
}

/// CRC-observing pull adapter corresponding to CRCStraw.
#[derive(Debug, Clone)]
pub struct CrcStraw<S> {
    upstream: S,
    crc: LegacyCrcEngine,
}

impl<S> CrcStraw<S> {
    pub fn new(upstream: S) -> Self {
        Self {
            upstream,
            crc: LegacyCrcEngine::default(),
        }
    }

    pub fn result(&self) -> u32 {
        self.crc.value()
    }

    pub const fn committed_result(&self) -> u32 {
        self.crc.committed_value()
    }

    pub fn upstream(&self) -> &S {
        &self.upstream
    }

    pub fn upstream_mut(&mut self) -> &mut S {
        &mut self.upstream
    }

    pub fn into_parts(self) -> (S, LegacyCrcEngine) {
        (self.upstream, self.crc)
    }
}

impl<S: StrawSource> CrcStraw<S> {
    pub fn get(&mut self, destination: &mut [u8]) -> usize {
        // Upstream CRCStraw::Get returns zero for null output or length < 1.
        if destination.is_empty() {
            return 0;
        }

        let actual = self.upstream.get(destination).min(destination.len());
        self.crc.update(&destination[..actual]);
        actual
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default)]
    struct RecordingSink {
        bytes: Vec<u8>,
        max_accept: Option<usize>,
        flush_count: usize,
    }

    impl PipeSink for RecordingSink {
        fn put(&mut self, source: &[u8]) -> usize {
            let accepted = self.max_accept.unwrap_or(source.len()).min(source.len());
            self.bytes.extend_from_slice(&source[..accepted]);
            accepted
        }

        fn flush(&mut self) -> usize {
            self.flush_count += 1;
            0
        }
    }

    #[derive(Debug)]
    struct SliceSource {
        bytes: Vec<u8>,
        offset: usize,
    }

    impl SliceSource {
        fn new(bytes: &[u8]) -> Self {
            Self { bytes: bytes.to_vec(), offset: 0 }
        }
    }

    impl StrawSource for SliceSource {
        fn get(&mut self, destination: &mut [u8]) -> usize {
            let available = self.bytes.len().saturating_sub(self.offset);
            let count = available.min(destination.len());
            destination[..count].copy_from_slice(&self.bytes[self.offset..self.offset + count]);
            self.offset += count;
            count
        }
    }

    #[test]
    fn crc_pipe_hashes_before_forwarding_to_downstream() {
        let sink = RecordingSink { max_accept: Some(2), ..RecordingSink::default() };
        let mut pipe = CrcPipe::new(sink);
        assert_eq!(pipe.put(b"abcde"), 2);
        assert_eq!(pipe.downstream().bytes, b"ab");

        let mut expected = LegacyCrcEngine::default();
        assert_eq!(pipe.result(), expected.update(b"abcde"));
    }

    #[test]
    fn crc_pipe_forwards_flush_and_end() {
        let sink = RecordingSink::default();
        let mut pipe = CrcPipe::new(sink);
        assert_eq!(pipe.flush(), 0);
        assert_eq!(pipe.downstream().flush_count, 1);
        assert_eq!(pipe.end(), 0);
        assert_eq!(pipe.downstream().flush_count, 2);
    }

    #[test]
    fn crc_straw_hashes_only_bytes_actually_read() {
        let mut straw = CrcStraw::new(SliceSource::new(b"westwood"));
        let mut first = [0; 3];
        assert_eq!(straw.get(&mut first), 3);
        assert_eq!(&first, b"wes");

        let mut second = [0; 16];
        assert_eq!(straw.get(&mut second), 5);
        assert_eq!(&second[..5], b"twood");

        let mut expected = LegacyCrcEngine::default();
        expected.update(b"westwood");
        assert_eq!(straw.result(), expected.value());
    }

    #[test]
    fn crc_straw_empty_destination_does_not_consume_input() {
        let mut straw = CrcStraw::new(SliceSource::new(b"data"));
        assert_eq!(straw.get(&mut []), 0);
        let mut output = [0; 8];
        assert_eq!(straw.get(&mut output), 4);
        assert_eq!(&output[..4], b"data");
    }
}
