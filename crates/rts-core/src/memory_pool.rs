//! Paragraph-based cache allocator ported from
//! TIBERIANDAWN/WIN32LIB/MEM.CPP and WWMEM.H.
//!
//! The pinned Win32 projects use one-byte structure packing. Consequently,
//! MemPool_Type occupies 20 bytes and MemChain_Type occupies 18 bytes.
//! Pointers are represented by checked offsets into an owned arena. A block
//! handle points just after its 18-byte allocation header, matching Mem_Alloc.

use std::fmt;

pub const MEM_PARAGRAPH_SIZE: usize = 16;
pub const MEM_POOL_HEADER_SIZE: usize = 20;
pub const MEM_CHAIN_HEADER_SIZE: usize = 18;
pub const MEM_BLOCK_IN_USE: u16 = 0;
pub const MEM_BLOCK_LOCKED: u16 = 1;
const MEM_DEBUG_FILL: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MemoryBlockHandle {
    /// Offset of the first user byte, after the legacy MemChain_Type header.
    pub offset: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BlockHeader {
    offset: usize,
    paragraphs: u32,
    id: u32,
    time: u16,
}

impl BlockHeader {
    fn byte_len(self) -> usize { self.paragraphs as usize * MEM_PARAGRAPH_SIZE }
    fn data_handle(self) -> MemoryBlockHandle {
        MemoryBlockHandle { offset: self.offset + MEM_CHAIN_HEADER_SIZE }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct FreeBlock {
    offset: usize,
    paragraphs: u32,
}

impl FreeBlock {
    fn byte_len(self) -> usize { self.paragraphs as usize * MEM_PARAGRAPH_SIZE }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MemoryPoolError {
    InvalidPoolSize,
    SizeOverflow,
    InvalidBlockHandle(MemoryBlockHandle),
    BlockDataOutOfBounds,
}

impl fmt::Display for MemoryPoolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{self:?}") }
}
impl std::error::Error for MemoryPoolError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryPool {
    arena: Vec<u8>,
    total_mem: u32,
    free_mem: u32,
    free_chain: Vec<FreeBlock>,
    used_chain: Vec<BlockHeader>,
}

impl MemoryPool {
    /// Mem_Init rounds the provided size down to a paragraph boundary,
    /// places the pool control structure first, then creates one free block.
    pub fn init(requested_size: usize) -> Result<Self, MemoryPoolError> {
        let size = requested_size & !0x0f;
        if size <= MEM_POOL_HEADER_SIZE + MEM_CHAIN_HEADER_SIZE {
            return Err(MemoryPoolError::InvalidPoolSize);
        }
        let free_paragraphs = (size - MEM_POOL_HEADER_SIZE) / MEM_PARAGRAPH_SIZE;
        if free_paragraphs == 0 || free_paragraphs > u32::MAX as usize {
            return Err(MemoryPoolError::InvalidPoolSize);
        }
        let free_mem = free_paragraphs as u32;
        Ok(Self {
            arena: vec![0; size],
            total_mem: free_mem,
            free_mem,
            free_chain: vec![FreeBlock {
                offset: MEM_POOL_HEADER_SIZE,
                paragraphs: free_mem,
            }],
            used_chain: Vec::new(),
        })
    }

    pub fn arena_size(&self) -> usize { self.arena.len() }
    pub fn total_paragraphs(&self) -> u32 { self.total_mem }
    pub fn free_paragraphs(&self) -> u32 { self.free_mem }
    pub fn free_block_count(&self) -> usize { self.free_chain.len() }
    pub fn used_block_count(&self) -> usize { self.used_chain.len() }

    /// Mem_Alloc: first-fit allocation with paragraph rounding. Remainders
    /// of two paragraphs or less are absorbed into the allocated block.
    pub fn alloc(&mut self, requested_bytes: usize, id: u32, tick: u32) -> Option<MemoryBlockHandle> {
        if requested_bytes == 0 { return None; }
        let with_header = requested_bytes.checked_add(MEM_CHAIN_HEADER_SIZE)?;
        let rounded = with_header.checked_add(MEM_PARAGRAPH_SIZE - 1)? & !(MEM_PARAGRAPH_SIZE - 1);
        let mut paragraphs = u32::try_from(rounded / MEM_PARAGRAPH_SIZE).ok()?;
        if self.total_mem < paragraphs { return None; }

        let index = self.free_chain.iter().position(|node| node.paragraphs >= paragraphs)?;
        let free = self.free_chain.remove(index);
        let mut remainder = free.paragraphs - paragraphs;
        if remainder <= 2 {
            remainder = 0;
            paragraphs = free.paragraphs;
        }

        if remainder != 0 {
            let remainder_offset = free.offset.checked_add(paragraphs as usize * MEM_PARAGRAPH_SIZE)?;
            self.insert_free(FreeBlock { offset: remainder_offset, paragraphs: remainder }, false);
        }

        let block = BlockHeader { offset: free.offset, paragraphs, id, time: (tick >> 4) as u16 };
        let handle = block.data_handle();
        let insert_at = self.used_chain.partition_point(|entry| entry.offset < block.offset);
        self.used_chain.insert(insert_at, block);
        self.free_mem = self.free_mem.saturating_sub(paragraphs);

        if MEM_DEBUG_FILL {
            let start = handle.offset;
            let end = free.offset + paragraphs as usize * MEM_PARAGRAPH_SIZE;
            self.arena[start..end].fill(id as u8);
        }
        Some(handle)
    }

    /// Mem_Free: validate the returned payload handle, unlink the used block,
    /// and merge it with adjacent free regions.
    pub fn free(&mut self, handle: MemoryBlockHandle) -> bool {
        if handle.offset < MEM_CHAIN_HEADER_SIZE { return false; }
        let header_offset = handle.offset - MEM_CHAIN_HEADER_SIZE;
        let Some(index) = self.used_chain.iter().position(|b| b.offset == header_offset) else {
            return false;
        };
        let block = self.used_chain.remove(index);
        self.insert_free(FreeBlock { offset: block.offset, paragraphs: block.paragraphs }, true);
        self.free_mem = self.free_mem.saturating_add(block.paragraphs);
        true
    }

    /// Mem_Find: first matching ID on the address-sorted used chain.
    pub fn find(&self, id: u32) -> Option<MemoryBlockHandle> {
        self.used_chain.iter().find(|block| block.id == id).map(|block| block.data_handle())
    }

    /// Mem_Get_ID returns zero for a null pointer; stale/foreign handles are
    /// represented by None rather than dereferencing arbitrary memory.
    pub fn get_id(&self, handle: Option<MemoryBlockHandle>) -> Option<u32> {
        let handle = handle?;
        if handle.offset < MEM_CHAIN_HEADER_SIZE { return None; }
        let header_offset = handle.offset - MEM_CHAIN_HEADER_SIZE;
        self.used_chain.iter().find(|block| block.offset == header_offset).map(|block| block.id)
    }

    /// Mem_Reference updates the 16-bit time stamp to TickCount.Time() >> 4.
    pub fn reference(&mut self, handle: MemoryBlockHandle, tick: u32) -> bool {
        self.update_time(handle, (tick >> 4) as u16)
    }

    /// Mem_Lock_Block excludes a block from both LRU eviction and compaction.
    pub fn lock_block(&mut self, handle: MemoryBlockHandle) -> bool {
        self.update_time(handle, MEM_BLOCK_LOCKED)
    }

    /// Mem_In_Use excludes a block from LRU eviction but still allows movement.
    pub fn in_use(&mut self, handle: MemoryBlockHandle) -> bool {
        self.update_time(handle, MEM_BLOCK_IN_USE)
    }

    fn update_time(&mut self, handle: MemoryBlockHandle, time: u16) -> bool {
        if handle.offset < MEM_CHAIN_HEADER_SIZE { return false; }
        let offset = handle.offset - MEM_CHAIN_HEADER_SIZE;
        let Some(block) = self.used_chain.iter_mut().find(|block| block.offset == offset) else {
            return false;
        };
        block.time = time;
        true
    }

    /// Mem_Find_Oldest uses unsigned wrap-aware subtraction and ignores
    /// MEM_BLOCK_IN_USE/MEM_BLOCK_LOCKED (time values 0 and 1).
    pub fn find_oldest(&self, tick: u32) -> Option<MemoryBlockHandle> {
        let base_time = tick >> 4;
        self.used_chain.iter()
            .filter(|block| block.time > MEM_BLOCK_LOCKED)
            .min_by_key(|block| (block.time as u32).wrapping_sub(base_time))
            .map(|block| block.data_handle())
    }

    /// Mem_Free_Oldest returns the formerly valid handle after freeing it.
    pub fn free_oldest(&mut self, tick: u32) -> Option<MemoryBlockHandle> {
        let oldest = self.find_oldest(tick)?;
        if self.free(oldest) { Some(oldest) } else { None }
    }

    /// Mem_Pool_Size: total cache bytes less one allocation header.
    pub fn pool_size(&self) -> usize {
        (self.total_mem as usize * MEM_PARAGRAPH_SIZE).saturating_sub(MEM_CHAIN_HEADER_SIZE)
    }

    /// Mem_Avail: aggregate free bytes less a single allocation header.
    /// The returned value may be fragmented until cleanup is performed.
    pub fn available(&self) -> usize {
        (self.free_mem as usize * MEM_PARAGRAPH_SIZE).saturating_sub(MEM_CHAIN_HEADER_SIZE)
    }

    /// Mem_Largest_Avail: size of the biggest free node less its header.
    pub fn largest_available(&self) -> usize {
        self.free_chain.iter().map(|node| node.byte_len())
            .max().unwrap_or(0).saturating_sub(MEM_CHAIN_HEADER_SIZE)
    }

    /// Mem_Cleanup compacts movable used blocks toward the beginning of each
    /// region bounded by locked blocks, then reconstructs/coalesces free gaps.
    /// All old payload handles may be invalidated by this operation.
    pub fn cleanup(&mut self) {
        if self.free_mem == 0 || self.free_chain.is_empty() || self.used_chain.is_empty() {
            return;
        }

        let mut cursor = MEM_POOL_HEADER_SIZE;
        for block in &mut self.used_chain {
            let old_offset = block.offset;
            let size = block.byte_len();
            if block.time == MEM_BLOCK_LOCKED {
                cursor = old_offset.saturating_add(size);
                continue;
            }

            if old_offset != cursor {
                self.arena.copy_within(old_offset..old_offset + size, cursor);
                block.offset = cursor;
            }
            cursor = cursor.saturating_add(size);
        }

        self.rebuild_free_chain();
    }

    /// Read/write owned payload bytes while ensuring the handle still names a
    /// live block. The user-visible length excludes the 18-byte MemChain header.
    pub fn read(&self, handle: MemoryBlockHandle) -> Result<&[u8], MemoryPoolError> {
        let block = self.find_header(handle).ok_or(MemoryPoolError::InvalidBlockHandle(handle))?;
        let start = handle.offset;
        let end = block.offset + block.byte_len();
        self.arena.get(start..end).ok_or(MemoryPoolError::BlockDataOutOfBounds)
    }

    pub fn write(&mut self, handle: MemoryBlockHandle, bytes: &[u8]) -> Result<(), MemoryPoolError> {
        let block = self.find_header(handle).ok_or(MemoryPoolError::InvalidBlockHandle(handle))?;
        let start = handle.offset;
        let end = block.offset + block.byte_len();
        let target = self.arena.get_mut(start..end).ok_or(MemoryPoolError::BlockDataOutOfBounds)?;
        if bytes.len() > target.len() { return Err(MemoryPoolError::BlockDataOutOfBounds); }
        target[..bytes.len()].copy_from_slice(bytes);
        Ok(())
    }

    pub fn block_capacity(&self, handle: MemoryBlockHandle) -> Option<usize> {
        self.find_header(handle).map(|block| block.byte_len().saturating_sub(MEM_CHAIN_HEADER_SIZE))
    }

    fn find_header(&self, handle: MemoryBlockHandle) -> Option<BlockHeader> {
        if handle.offset < MEM_CHAIN_HEADER_SIZE { return None; }
        let header_offset = handle.offset - MEM_CHAIN_HEADER_SIZE;
        self.used_chain.iter().copied().find(|block| block.offset == header_offset)
    }

    fn rebuild_free_chain(&mut self) {
        let mut cursor = MEM_POOL_HEADER_SIZE;
        let mut gaps = Vec::new();
        for block in &self.used_chain {
            if block.offset > cursor {
                let gap = block.offset - cursor;
                let paragraphs = (gap / MEM_PARAGRAPH_SIZE) as u32;
                if paragraphs != 0 { gaps.push(FreeBlock { offset: cursor, paragraphs }); }
            }
            cursor = cursor.max(block.offset + block.byte_len());
        }
        if cursor < self.arena.len() {
            let paragraphs = ((self.arena.len() - cursor) / MEM_PARAGRAPH_SIZE) as u32;
            if paragraphs != 0 { gaps.push(FreeBlock { offset: cursor, paragraphs }); }
        }
        self.free_chain = gaps;
        self.free_mem = self.free_chain.iter().map(|block| block.paragraphs).sum();
    }

    fn insert_free(&mut self, new_block: FreeBlock, merge: bool) {
        if new_block.paragraphs == 0 { return; }
        let mut blocks = self.free_chain.clone();
        blocks.push(new_block);
        blocks.sort_by_key(|block| block.offset);

        if merge {
            let mut merged: Vec<FreeBlock> = Vec::with_capacity(blocks.len());
            for block in blocks {
                if let Some(last) = merged.last_mut() {
                    let last_end = last.offset + last.byte_len();
                    if last_end == block.offset {
                        last.paragraphs = last.paragraphs.saturating_add(block.paragraphs);
                        continue;
                    }
                }
                merged.push(block);
            }
            self.free_chain = merged;
        } else {
            self.free_chain = blocks;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_paragraph_pool_and_first_fit_allocates() {
        let mut pool = MemoryPool::init(4096).unwrap();
        assert_eq!(pool.arena_size(), 4096);
        assert_eq!(pool.pool_size(), 4096 - MEM_POOL_HEADER_SIZE - MEM_CHAIN_HEADER_SIZE);
        let first = pool.alloc(100, 11, 160).unwrap();
        let second = pool.alloc(200, 22, 320).unwrap();
        assert_eq!(pool.get_id(Some(first)), Some(11));
        assert_eq!(pool.get_id(Some(second)), Some(22));
        assert!(first.offset < second.offset);
    }

    #[test]
    fn free_merges_adjacent_chunks_and_updates_availability() {
        let mut pool = MemoryPool::init(2048).unwrap();
        let initial = pool.available();
        let first = pool.alloc(64, 1, 100).unwrap();
        let second = pool.alloc(64, 2, 100).unwrap();
        assert!(pool.free(first));
        assert!(pool.free(second));
        assert_eq!(pool.free_block_count(), 1);
        assert_eq!(pool.available(), initial);
        assert!(!pool.free(second));
    }

    #[test]
    fn in_use_locked_and_lru_timestamps_match_cache_semantics() {
        let mut pool = MemoryPool::init(4096).unwrap();
        let a = pool.alloc(64, 1, 0x100).unwrap();
        let b = pool.alloc(64, 2, 0x200).unwrap();
        let c = pool.alloc(64, 3, 0x300).unwrap();
        assert!(pool.in_use(a));
        assert!(pool.lock_block(b));
        assert_eq!(pool.find_oldest(0x400), Some(c));
        assert!(pool.reference(c, 0x800));
        assert_eq!(pool.find_oldest(0x900), Some(c));
        assert_eq!(pool.free_oldest(0x900), Some(c));
        assert!(pool.read(c).is_err());
    }

    #[test]
    fn id_lookup_returns_first_matching_used_block() {
        let mut pool = MemoryPool::init(4096).unwrap();
        let a = pool.alloc(20, 7, 100).unwrap();
        let b = pool.alloc(20, 7, 200).unwrap();
        assert_eq!(pool.find(7), Some(a));
        assert!(pool.free(a));
        assert_eq!(pool.find(7), Some(b));
    }

    #[test]
    fn cleanup_compacts_movable_blocks_around_locked_allocations() {
        let mut pool = MemoryPool::init(4096).unwrap();
        let a = pool.alloc(100, 1, 100).unwrap();
        let b = pool.alloc(100, 2, 200).unwrap();
        let c = pool.alloc(100, 3, 300).unwrap();
        pool.write(a, b"first").unwrap();
        pool.write(c, b"third").unwrap();
        pool.lock_block(b);
        let before_b = b.offset;
        pool.free(a);
        pool.cleanup();
        let moved_c = pool.find(3).unwrap();
        assert_eq!(b.offset, before_b);
        assert_eq!(pool.read(moved_c).unwrap()[..5], *b"third");
        assert_eq!(pool.read(b).unwrap().len(), pool.block_capacity(b).unwrap());
        assert_eq!(pool.free_block_count(), 1);
    }

    #[test]
    fn available_and_largest_available_have_distinct_fragmentation_semantics() {
        let mut pool = MemoryPool::init(4096).unwrap();
        let a = pool.alloc(512, 1, 100).unwrap();
        let b = pool.alloc(512, 2, 100).unwrap();
        let c = pool.alloc(512, 3, 100).unwrap();
        pool.free(b);
        pool.free(c);
        assert!(pool.available() >= pool.largest_available());
        assert!(pool.read(a).is_ok());
    }

    #[test]
    fn reference_handles_are_invalidated_when_block_is_freed() {
        let mut pool = MemoryPool::init(1024).unwrap();
        let block = pool.alloc(128, 0x1234, 160).unwrap();
        pool.write(block, b"hello").unwrap();
        assert_eq!(&pool.read(block).unwrap()[..5], b"hello");
        assert!(pool.reference(block, 320));
        assert_eq!(pool.get_id(Some(block)), Some(0x1234));
        assert!(pool.free(block));
        assert_eq!(pool.get_id(Some(block)), None);
    }
}
