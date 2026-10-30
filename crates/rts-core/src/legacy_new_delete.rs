//! Tiberian Dawn global C++ allocation-operator compatibility from
//! `TIBERIANDAWN/WIN32LIB/NEWDEL.CPP`.
//!
//! The original translation unit overrides scalar/array new and delete and
//! delegates every operation to the Westwood `Alloc(size, MEM_NEW)` and
//! `Free(pointer)` routines. Rust cannot safely replace its process-wide
//! allocator from an ordinary crate, so callers inject that allocator here.

/// Allocation category passed to the original `Alloc` function.
pub const MEM_NEW: u32 = 0x0000_0001;

/// Host implementation of the legacy `Alloc` / `Free` pair.
pub trait LegacyAllocator {
    type Block;

    /// Allocate exactly `size` bytes using the legacy MEM_NEW category.
    fn alloc(&mut self, size: u32, flags: u32) -> Option<Self::Block>;

    /// Release a block previously returned by this allocator.
    fn free(&mut self, block: Self::Block);
}

/// Equivalent of scalar `operator new(size_t)`.
#[inline]
pub fn operator_new<A: LegacyAllocator>(allocator: &mut A, size: usize) -> Option<A::Block> {
    allocator.alloc(size as u32, MEM_NEW)
}

/// Equivalent of array `operator new[](size_t)`.
///
/// The source uses the same allocator and flag as scalar allocation.
#[inline]
pub fn operator_new_array<A: LegacyAllocator>(
    allocator: &mut A,
    size: usize,
) -> Option<A::Block> {
    allocator.alloc(size as u32, MEM_NEW)
}

/// Equivalent of scalar `operator delete(void*)`.
#[inline]
pub fn operator_delete<A: LegacyAllocator>(allocator: &mut A, block: Option<A::Block>) {
    if let Some(block) = block {
        allocator.free(block);
    }
}

/// Equivalent of array `operator delete[](void*)`.
#[inline]
pub fn operator_delete_array<A: LegacyAllocator>(
    allocator: &mut A,
    block: Option<A::Block>,
) {
    if let Some(block) = block {
        allocator.free(block);
    }
}
