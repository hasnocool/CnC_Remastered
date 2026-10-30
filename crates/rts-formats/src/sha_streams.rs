//! SHA-aware Pipe and Straw adapters ported from SHAPIPE.CPP and SHASTRAW.CPP.
//!
//! SHAPipe hashes submitted bytes before forwarding them. SHAStraw hashes only
//! bytes returned by the upstream source. Result snapshots are non-destructive.

use crate::pipe::PipeSink;
use crate::sha1::Sha1;
use crate::straw::StrawSource;

#[derive(Debug, Clone)]
pub struct ShaPipe<P> {
    downstream: P,
    sha: Sha1,
}

impl<P> ShaPipe<P> {
    pub fn new(downstream: P) -> Self {
        Self { downstream, sha: Sha1::new() }
    }

    pub fn result(&self) -> [u8; Sha1::DIGEST_SIZE] { self.sha.result() }
    pub fn downstream(&self) -> &P { &self.downstream }
    pub fn downstream_mut(&mut self) -> &mut P { &mut self.downstream }
    pub fn into_parts(self) -> (P, Sha1) { (self.downstream, self.sha) }
}

impl<P: PipeSink> PipeSink for ShaPipe<P> {
    fn put(&mut self, source: &[u8]) -> usize {
        self.sha.update(source);
        self.downstream.put(source)
    }
    fn flush(&mut self) -> usize { self.downstream.flush() }
    fn end(&mut self) -> usize { self.downstream.end() }
}

#[derive(Debug, Clone)]
pub struct ShaStraw<S> {
    upstream: S,
    sha: Sha1,
}

impl<S> ShaStraw<S> {
    pub fn new(upstream: S) -> Self {
        Self { upstream, sha: Sha1::new() }
    }
    pub fn result(&self) -> [u8; Sha1::DIGEST_SIZE] { self.sha.result() }
    pub fn upstream(&self) -> &S { &self.upstream }
    pub fn upstream_mut(&mut self) -> &mut S { &mut self.upstream }
    pub fn into_parts(self) -> (S, Sha1) { (self.upstream, self.sha) }
}

impl<S: StrawSource> StrawSource for ShaStraw<S> {
    fn get(&mut self, destination: &mut [u8]) -> usize {
        if destination.is_empty() { return 0; }
        let actual = self.upstream.get(destination).min(destination.len());
        self.sha.update(&destination[..actual]);
        actual
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipe::{PipeHandle, PipeSink, PassthroughPipe};
    use crate::straw::{EmptyStraw, SliceStraw, StrawHandle};

    #[derive(Debug, Default)]
    struct Sink(Vec<u8>);

    impl PipeSink for Sink {
        fn put(&mut self, source: &[u8]) -> usize {
            self.0.extend_from_slice(source);
            source.len()
        }
    }

    #[test]
    fn sha_pipe_hashes_before_forwarding() {
        let mut pipe = ShaPipe::new(Sink::default());
        assert_eq!(pipe.put(b"abc"), 3);
        assert_eq!(pipe.downstream().0, b"abc");
        assert_eq!(pipe.result(), Sha1::digest(b"abc"));
    }

    #[test]
    fn sha_pipe_composes_with_the_base_pipe_chain() {
        let mut pipe = ShaPipe::new(PipeHandle::new(PassthroughPipe));
        assert_eq!(pipe.put(b"chain"), 5);
        assert_eq!(pipe.result(), Sha1::digest(b"chain"));
    }

    #[test]
    fn sha_straw_hashes_only_bytes_read_and_result_is_non_destructive() {
        let source = SliceStraw::new(b"westwood".to_vec());
        let mut straw = ShaStraw::new(source);
        let mut first = [0; 3];
        assert_eq!(straw.get(&mut first), 3);
        let snapshot = straw.result();
        assert_eq!(snapshot, Sha1::digest(b"wes"));

        let mut rest = [0; 8];
        assert_eq!(straw.get(&mut rest), 5);
        assert_eq!(straw.result(), Sha1::digest(b"westwood"));
        assert_eq!(snapshot, Sha1::digest(b"wes"));
    }

    #[test]
    fn sha_straw_composes_with_linked_straw_handles() {
        let upstream = StrawHandle::new(SliceStraw::new(b"input".to_vec()));
        let mut consumer = StrawHandle::new(EmptyStraw);
        consumer.get_from(Some(upstream)).unwrap();
        let mut sha_straw = ShaStraw::new(consumer);
        let mut destination = [0; 5];
        assert_eq!(sha_straw.get(&mut destination), 5);
        assert_eq!(sha_straw.result(), Sha1::digest(b"input"));
    }
}
