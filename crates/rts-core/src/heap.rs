//! Fixed-size and indexed object heaps from both variants' HEAP.CPP/HEAP.H.
//!
//! FixedHeapClass uses a BooleanVector free map and first-false allocation.
//! FixedIHeapClass additionally maintains an active-pointer order and clears
//! each allocated slot to zero. Red Alert persists slot bytes via Pipe/Straw;
//! Tiberian Dawn delegates object serialization to per-slot callbacks.

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct HeapHandle {
    pub byte_offset: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeapError {
    InvalidBlockSize,
    SizeOverflow,
    BufferTooSmall { required: usize, supplied: usize },
    HeapNotInitialized,
    InvalidHandle(HeapHandle),
    InvalidSlot(usize),
    TooManyObjects { requested: usize, capacity: usize },
    CorruptArchive,
    IoFailure,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixedHeap {
    block_size: usize,
    buffer: Option<Vec<u8>>,
    owns_buffer: bool,
    total_count: usize,
    active_count: usize,
    free_flags: Vec<bool>,
}

impl FixedHeap {
    pub const fn new(block_size: usize) -> Self {
        Self {
            block_size,
            buffer: None,
            owns_buffer: false,
            total_count: 0,
            active_count: 0,
            free_flags: Vec::new(),
        }
    }

    pub fn count(&self) -> usize { self.active_count }
    pub fn length(&self) -> usize { self.total_count }
    pub fn avail(&self) -> usize { self.total_count.saturating_sub(self.active_count) }
    pub fn block_size(&self) -> usize { self.block_size }
    pub fn is_allocated(&self) -> bool { self.owns_buffer }
    pub fn free_flags(&self) -> &[bool] { &self.free_flags }

    /// Set_Heap first clears prior state, rejects a zero block size, and treats
    /// count=0 as a successful disabled heap.
    pub fn set_heap(&mut self, count: usize, supplied_buffer: Option<Vec<u8>>) -> Result<bool, HeapError> {
        self.clear();
        if self.block_size == 0 { return Err(HeapError::InvalidBlockSize); }
        if count == 0 { return Ok(true); }
        let required = count.checked_mul(self.block_size).ok_or(HeapError::SizeOverflow)?;
        let (buffer, owns) = match supplied_buffer {
            Some(buffer) => {
                if buffer.len() < required {
                    return Err(HeapError::BufferTooSmall { required, supplied: buffer.len() });
                }
                (buffer, false)
            }
            None => (vec![0; required], true),
        };
        self.buffer = Some(buffer);
        self.owns_buffer = owns;
        self.total_count = count;
        self.active_count = 0;
        self.free_flags = vec![false; count];
        Ok(true)
    }

    /// Allocate marks and returns the first false FreeFlag entry.
    pub fn allocate(&mut self) -> Option<HeapHandle> {
        if self.active_count >= self.total_count { return None; }
        let index = self.free_flags.iter().position(|allocated| !*allocated)?;
        self.free_flags[index] = true;
        self.active_count += 1;
        Some(HeapHandle { byte_offset: index * self.block_size })
    }

    /// ID is pointer-relative division by block size; alignment is not checked.
    pub fn id(&self, handle: Option<HeapHandle>) -> Option<usize> {
        let handle = handle?;
        if self.block_size == 0 { return None; }
        Some(handle.byte_offset / self.block_size)
    }

    /// Free returns false for null, foreign, inactive, or already-free blocks.
    pub fn free(&mut self, handle: Option<HeapHandle>) -> bool {
        if self.active_count == 0 { return false; }
        let Some(index) = self.id(handle) else { return false; };
        if index >= self.total_count || !self.free_flags[index] { return false; }
        self.active_count -= 1;
        self.free_flags[index] = false;
        true
    }

    pub fn free_all(&mut self) -> bool {
        self.active_count = 0;
        self.free_flags.fill(false);
        true
    }

    pub fn clear(&mut self) {
        self.buffer = None;
        self.owns_buffer = false;
        self.active_count = 0;
        self.total_count = 0;
        self.free_flags.clear();
    }

    pub fn slot(&self, index: usize) -> Option<&[u8]> {
        if index >= self.total_count { return None; }
        let start = index.checked_mul(self.block_size)?;
        self.buffer.as_ref()?.get(start..start + self.block_size)
    }

    pub fn slot_mut(&mut self, index: usize) -> Option<&mut [u8]> {
        if index >= self.total_count { return None; }
        let start = index.checked_mul(self.block_size)?;
        self.buffer.as_mut()?.get_mut(start..start + self.block_size)
    }

    pub fn bytes(&self) -> Option<&[u8]> { self.buffer.as_deref() }

    fn mark_loaded(&mut self, index: usize) -> Result<HeapHandle, HeapError> {
        if index >= self.total_count { return Err(HeapError::InvalidSlot(index)); }
        if self.free_flags[index] { return Err(HeapError::CorruptArchive); }
        self.free_flags[index] = true;
        self.active_count += 1;
        Ok(HeapHandle { byte_offset: index * self.block_size })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixedIndexHeap {
    heap: FixedHeap,
    active_pointers: Vec<HeapHandle>,
}

impl FixedIndexHeap {
    pub const fn new(block_size: usize) -> Self {
        Self { heap: FixedHeap::new(block_size), active_pointers: Vec::new() }
    }

    pub fn count(&self) -> usize { self.heap.count() }
    pub fn length(&self) -> usize { self.heap.length() }
    pub fn avail(&self) -> usize { self.heap.avail() }
    pub fn active_pointers(&self) -> &[HeapHandle] { &self.active_pointers }
    pub fn raw_ptr(&self, index: usize) -> Option<HeapHandle> {
        (index < self.heap.length()).then_some(HeapHandle { byte_offset: index * self.heap.block_size() })
    }
    pub fn active_ptr(&self, index: usize) -> Option<HeapHandle> { self.active_pointers.get(index).copied() }

    pub fn set_heap(&mut self, count: usize, supplied_buffer: Option<Vec<u8>>) -> Result<bool, HeapError> {
        self.clear();
        self.heap.set_heap(count, supplied_buffer)?;
        self.active_pointers.reserve(count);
        Ok(true)
    }

    /// FixedIHeapClass::Allocate zeroes every byte in the newly allocated slot.
    pub fn allocate(&mut self) -> Option<HeapHandle> {
        let handle = self.heap.allocate()?;
        let index = handle.byte_offset / self.heap.block_size();
        self.heap.slot_mut(index)?.fill(0);
        self.active_pointers.push(handle);
        Some(handle)
    }

    /// Source quirk retained: FixedIHeapClass::Free removes the pointer and
    /// frees the slot but returns FALSE even when the base Free succeeded.
    pub fn free(&mut self, handle: Option<HeapHandle>) -> bool {
        if self.heap.free(handle) {
            if let Some(handle) = handle {
                if let Some(index) = self.active_pointers.iter().position(|item| *item == handle) {
                    self.active_pointers.remove(index);
                }
            }
        }
        false
    }

    pub fn free_all(&mut self) -> bool {
        self.active_pointers.clear();
        self.heap.free_all()
    }

    pub fn clear(&mut self) {
        self.heap.clear();
        self.active_pointers.clear();
    }

    pub fn logical_id(&self, handle: Option<HeapHandle>) -> Option<usize> {
        let handle = handle?;
        self.active_pointers.iter().position(|item| *item == handle)
    }

    pub fn slot(&self, index: usize) -> Option<&[u8]> { self.heap.slot(index) }
    pub fn slot_mut(&mut self, index: usize) -> Option<&mut [u8]> { self.heap.slot_mut(index) }
    pub fn heap(&self) -> &FixedHeap { &self.heap }
    pub fn heap_mut(&mut self) -> &mut FixedHeap { &mut self.heap }

    /// Recreate the indexed heap from Red Alert's raw Pipe byte stream:
    /// i32 active count, then for each active slot i32 physical ID + slot bytes.
    pub fn save_red_alert_raw(&self) -> Result<Vec<u8>, HeapError> {
        let mut output = Vec::new();
        output.extend_from_slice(&(self.count() as i32).to_le_bytes());
        for handle in &self.active_pointers {
            let id = self.heap.id(Some(*handle)).ok_or(HeapError::InvalidHandle(*handle))?;
            output.extend_from_slice(&(id as i32).to_le_bytes());
            output.extend_from_slice(self.heap.slot(id).ok_or(HeapError::InvalidSlot(id))?);
        }
        Ok(output)
    }

    pub fn load_red_alert_raw(&mut self, input: &[u8]) -> Result<usize, HeapError> {
        let mut cursor = 0usize;
        let count = read_i32(input, &mut cursor)?;
        if count < 0 || count as usize > self.length() {
            return Err(HeapError::TooManyObjects { requested: count.max(0) as usize, capacity: self.length() });
        }
        self.free_all();
        for _ in 0..count {
            let id = read_i32(input, &mut cursor)?;
            if id < 0 { return Err(HeapError::InvalidSlot(id as usize)); }
            let index = id as usize;
            let end = cursor.checked_add(self.heap.block_size()).ok_or(HeapError::SizeOverflow)?;
            let bytes = input.get(cursor..end).ok_or(HeapError::CorruptArchive)?;
            let handle = self.heap.mark_loaded(index)?;
            self.heap.slot_mut(index).ok_or(HeapError::InvalidSlot(index))?.copy_from_slice(bytes);
            self.active_pointers.push(handle);
            cursor = end;
        }
        Ok(cursor)
    }

    /// Tiberian Dawn writes count and physical ID, then delegates each object
    /// body to the caller's legacy Save routine.
    pub fn save_tiberian_dawn(
        &self,
        mut save_object: impl FnMut(HeapHandle, &[u8], &mut Vec<u8>) -> Result<(), HeapError>,
    ) -> Result<Vec<u8>, HeapError> {
        let mut output = Vec::new();
        output.extend_from_slice(&(self.count() as i32).to_le_bytes());
        for handle in &self.active_pointers {
            let id = self.heap.id(Some(*handle)).ok_or(HeapError::InvalidHandle(*handle))?;
            output.extend_from_slice(&(id as i32).to_le_bytes());
            save_object(*handle, self.heap.slot(id).ok_or(HeapError::InvalidSlot(id))?, &mut output)?;
        }
        Ok(output)
    }

    pub fn load_tiberian_dawn(
        &mut self,
        input: &[u8],
        mut load_object: impl FnMut(HeapHandle, &mut [u8], &[u8], &mut usize) -> Result<(), HeapError>,
    ) -> Result<usize, HeapError> {
        let mut cursor = 0usize;
        let count = read_i32(input, &mut cursor)?;
        if count < 0 || count as usize > self.length() {
            return Err(HeapError::TooManyObjects { requested: count.max(0) as usize, capacity: self.length() });
        }
        self.free_all();
        for _ in 0..count {
            let id = read_i32(input, &mut cursor)?;
            if id < 0 { return Err(HeapError::InvalidSlot(id as usize)); }
            let index = id as usize;
            let handle = self.heap.mark_loaded(index)?;
            self.active_pointers.push(handle);
            let mut slot = self.heap.slot_mut(index).ok_or(HeapError::InvalidSlot(index))?;
            load_object(handle, &mut slot, input, &mut cursor)?;
        }
        Ok(cursor)
    }

    /// TFixedIHeapClass::Code_Pointers / Decode_Pointers delegate in active order.
    pub fn visit_active_mut(&mut self, mut visit: impl FnMut(HeapHandle, &mut [u8])) {
        let active = self.active_pointers.clone();
        for handle in active {
            if let Some(slot) = self.heap.slot_mut(handle.byte_offset / self.heap.block_size()) {
                visit(handle, slot);
            }
        }
    }
}

fn read_i32(input: &[u8], cursor: &mut usize) -> Result<i32, HeapError> {
    let end = cursor.checked_add(4).ok_or(HeapError::SizeOverflow)?;
    let bytes = input.get(*cursor..end).ok_or(HeapError::CorruptArchive)?;
    *cursor = end;
    Ok(i32::from_le_bytes(bytes.try_into().map_err(|_| HeapError::CorruptArchive)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_heap_allocates_first_free_slot_and_checks_double_free() {
        let mut heap = FixedHeap::new(4);
        heap.set_heap(3, None).unwrap();
        let a = heap.allocate().unwrap();
        let b = heap.allocate().unwrap();
        assert_eq!(heap.id(Some(a)), Some(0));
        assert_eq!(heap.id(Some(b)), Some(1));
        assert!(heap.free(Some(a)));
        assert!(!heap.free(Some(a)));
        assert_eq!(heap.allocate(), Some(a));
        assert_eq!(heap.count(), 2);
    }

    #[test]
    fn fixed_heap_can_use_supplied_storage_and_clear_it() {
        let mut heap = FixedHeap::new(2);
        heap.set_heap(2, Some(vec![7; 4])).unwrap();
        assert!(!heap.is_allocated());
        assert_eq!(heap.slot(1), Some(&[7, 7][..]));
        heap.clear();
        assert_eq!(heap.length(), 0);
        assert_eq!(heap.count(), 0);
    }

    #[test]
    fn indexed_heap_zeroes_new_slot_tracks_active_order_and_preserves_false_return_bug() {
        let mut heap = FixedIndexHeap::new(3);
        heap.set_heap(2, None).unwrap();
        let first = heap.allocate().unwrap();
        heap.slot_mut(0).unwrap().copy_from_slice(&[1, 2, 3]);
        let second = heap.allocate().unwrap();
        assert_eq!(heap.slot(1), Some(&[0, 0, 0][..]));
        assert_eq!(heap.logical_id(Some(second)), Some(1));
        assert!(!heap.free(Some(first)));
        assert_eq!(heap.active_ptr(0), Some(second));
        assert_eq!(heap.count(), 1);
    }

    #[test]
    fn red_alert_raw_save_load_preserves_physical_slot_ids_and_bytes() {
        let mut source = FixedIndexHeap::new(2);
        source.set_heap(4, None).unwrap();
        let a = source.allocate().unwrap();
        source.slot_mut(a.byte_offset / 2).unwrap().copy_from_slice(&[9, 8]);
        source.allocate();
        source.free_all();
        let restored = source.allocate().unwrap();
        source.slot_mut(restored.byte_offset / 2).unwrap().copy_from_slice(&[3, 4]);
        let bytes = source.save_red_alert_raw().unwrap();

        let mut destination = FixedIndexHeap::new(2);
        destination.set_heap(4, None).unwrap();
        destination.load_red_alert_raw(&bytes).unwrap();
        assert_eq!(destination.active_ptr(0), Some(restored));
        assert_eq!(destination.slot(0), Some(&[3, 4][..]));
    }

    #[test]
    fn tiberian_dawn_codec_hooks_roundtrip_active_slots() {
        let mut source = FixedIndexHeap::new(2);
        source.set_heap(3, None).unwrap();
        let handle = source.allocate().unwrap();
        source.slot_mut(0).unwrap().copy_from_slice(&[5, 6]);
        let bytes = source.save_tiberian_dawn(|_, slot, out| {
            out.extend_from_slice(slot);
            Ok(())
        }).unwrap();

        let mut destination = FixedIndexHeap::new(2);
        destination.set_heap(3, None).unwrap();
        destination.load_tiberian_dawn(&bytes, |_, slot, input, cursor| {
            let end = *cursor + 2;
            slot.copy_from_slice(input.get(*cursor..end).ok_or(HeapError::CorruptArchive)?);
            *cursor = end;
            Ok(())
        }).unwrap();
        assert_eq!(destination.active_ptr(0), Some(handle));
        assert_eq!(destination.slot(0), Some(&[5, 6][..]));
    }

    #[test]
    fn failed_set_heap_leaves_heap_disabled() {
        let mut heap = FixedHeap::new(0);
        assert_eq!(heap.set_heap(4, None), Err(HeapError::InvalidBlockSize));
        assert_eq!(heap.length(), 0);
        assert_eq!(heap.count(), 0);
    }
}
