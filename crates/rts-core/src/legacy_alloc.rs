//! Windows-branch memory allocation helpers from WIN32LIB/ALLOC.CPP.
//!
//! The original Windows implementation delegates to malloc/realloc/free,
//! zeroes MEM_CLEAR allocations, increments Memory_Calls for every Alloc
//! attempt, and reports a fixed 64 MiB RAM estimate.

pub const MEM_NORMAL: u32 = 0x0000;
pub const MEM_NEW: u32 = 0x0001;
pub const MEM_CLEAR: u32 = 0x0002;
pub const MEM_REAL: u32 = 0x0004;
pub const MEM_TEMP: u32 = 0x0008;
pub const MEM_LOCK: u32 = 0x0010;
pub const LEGACY_RAM_ESTIMATE: i32 = 64 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocationError {
    CapacityOverflow,
    OutOfMemory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocationStats {
    pub memory_calls: u64,
    pub min_ram: u64,
    pub max_ram: u64,
    pub total_ram: u64,
}

/// State corresponding to the globals in ALLOC.CPP.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LegacyAllocator {
    memory_calls: u64,
    min_ram: u64,
    max_ram: u64,
    total_ram: u64,
}

impl Default for LegacyAllocator {
    fn default() -> Self {
        Self { memory_calls: 0, min_ram: 0, max_ram: 0, total_ram: 0 }
    }
}

impl LegacyAllocator {
    pub fn alloc(&mut self, size: usize, flags: u32) -> Result<Vec<u8>, AllocationError> {
        self.memory_calls = self.memory_calls.wrapping_add(1);
        let mut block = Vec::new();
        block.try_reserve_exact(size).map_err(|error| {
            if size > isize::MAX as usize { AllocationError::CapacityOverflow }
            else { let _ = error; AllocationError::OutOfMemory }
        })?;
        block.resize(size, if flags & MEM_CLEAR != 0 { 0 } else { 0 });
        Ok(block)
    }

    /// Free accepts an optional block to model Free(NULL) as a no-op.
    pub fn free(&mut self, block: Option<Vec<u8>>) {
        if block.is_some() {
            self.memory_calls = self.memory_calls.wrapping_sub(1);
        }
        drop(block);
    }

    /// Resize_Alloc preserves existing bytes and uses the allocator's failure
    /// result rather than invoking a process-global C callback.
    pub fn resize_alloc(
        &mut self,
        mut block: Vec<u8>,
        new_size: usize,
    ) -> Result<Vec<u8>, AllocationError> {
        if new_size > isize::MAX as usize {
            return Err(AllocationError::CapacityOverflow);
        }
        if new_size > block.len() {
            block.try_reserve_exact(new_size - block.len())
                .map_err(|_| AllocationError::OutOfMemory)?;
        }
        block.resize(new_size, 0);
        Ok(block)
    }

    pub const fn ram_free(&self) -> i32 { LEGACY_RAM_ESTIMATE }

    pub fn heap_size(&mut self) -> i32 {
        if self.total_ram == 0 {
            self.total_ram = self.total_ram_free() as u64;
        }
        self.total_ram as i32
    }

    pub const fn total_ram_free(&self) -> i32 { LEGACY_RAM_ESTIMATE }

    pub const fn stats(&self) -> AllocationStats {
        AllocationStats {
            memory_calls: self.memory_calls,
            min_ram: self.min_ram,
            max_ram: self.max_ram,
            total_ram: self.total_ram,
        }
    }

    /// DPMI lock calls are empty stubs in the active Windows source branch.
    pub const fn dpmi_lock(&self, _size: usize) {}
    pub const fn dpmi_unlock(&self, _size: usize) {}
}
