//! Buffered pull adapter ported from REDALERT/CSTRAW.H and CSTRAW.CPP.
//!
//! CacheStraw batches small downstream reads into fixed-size source requests.
//! It does not translate bytes and preserves the source's read-ahead behavior.

use crate::straw::StrawSource;

#[derive(Debug, Clone)]
pub struct CacheStraw<S> {
    upstream: S,
    buffer: Vec<u8>,
    index: usize,
    length: usize,
}

impl<S: StrawSource> CacheStraw<S> {
    pub fn new(upstream: S, capacity: usize) -> Self {
        Self { upstream, buffer: vec![0; capacity], index: 0, length: 0 }
    }

    pub fn with_buffer(upstream: S, buffer: Vec<u8>) -> Self {
        Self { upstream, buffer, index: 0, length: 0 }
    }

    pub fn capacity(&self) -> usize { self.buffer.len() }
    pub const fn cached_bytes(&self) -> usize { self.length }
    pub const fn cache_offset(&self) -> usize { self.index }

    pub fn upstream(&self) -> &S { &self.upstream }
    pub fn upstream_mut(&mut self) -> &mut S { &mut self.upstream }

    /// Port of CacheStraw::Get: drain existing bytes, then refill the entire
    /// buffer from the upstream straw until the caller is satisfied or EOF.
    pub fn get(&mut self, destination: &mut [u8]) -> usize {
        if destination.is_empty() || self.buffer.is_empty() {
            return 0;
        }

        let mut total = 0;
        while total < destination.len() {
            if self.length > 0 {
                let to_copy = self.length.min(destination.len() - total);
                destination[total..total + to_copy]
                    .copy_from_slice(&self.buffer[self.index..self.index + to_copy]);
                self.index += to_copy;
                self.length -= to_copy;
                total += to_copy;
                if total == destination.len() {
                    break;
                }
            }

            self.length = self.upstream.get(&mut self.buffer).min(self.buffer.len());
            self.index = 0;
            if self.length == 0 {
                break;
            }
        }
        total
    }

    pub fn into_upstream(self) -> S { self.upstream }
}

impl<S: StrawSource> StrawSource for CacheStraw<S> {
    fn get(&mut self, destination: &mut [u8]) -> usize {
        CacheStraw::get(self, destination)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::straw::SliceStraw;

    #[test]
    fn cache_batches_reads_and_drains_partial_blocks() {
        let upstream = SliceStraw::new(b"abcdefghij".to_vec());
        let mut cache = CacheStraw::new(upstream, 4);
        let mut first = [0; 2];
        assert_eq!(cache.get(&mut first), 2);
        assert_eq!(&first, b"ab");
        assert_eq!(cache.cached_bytes(), 2);

        let mut second = [0; 5];
        assert_eq!(cache.get(&mut second), 5);
        assert_eq!(&second, b"cdefg");
        assert_eq!(cache.cached_bytes(), 1);

        let mut last = [0; 8];
        assert_eq!(cache.get(&mut last), 3);
        assert_eq!(&last[..3], b"hij");
        assert_eq!(cache.get(&mut last), 0);
    }

    #[test]
    fn zero_capacity_cache_returns_eof() {
        let upstream = SliceStraw::new(b"data".to_vec());
        let mut cache = CacheStraw::new(upstream, 0);
        assert_eq!(cache.get(&mut [0; 4]), 0);
    }
}
