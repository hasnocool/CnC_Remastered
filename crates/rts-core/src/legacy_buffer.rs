//! Red Alert Buffer from REDALERT/BUFF.CPP.
//!
//! Preserves the BufferPtr/Size/IsAllocated state machine. A shallow copy of
//! an owned buffer shares the allocation safely and marks the copy as a
//! non-owner; external data can be shared through Arc<RwLock<Vec<u8>>>.
//! Exclusive borrowed slices cannot be copied because doing so would create
//! aliased mutable references. Shared access is nonblocking.

use std::sync::{Arc, RwLock, TryLockError};

#[derive(Debug)]
enum BufferStorage<'a> {
    BorrowedMutable(&'a mut [u8]),
    BorrowedConst(&'a [u8]),
    SharedExternal(Arc<RwLock<Vec<u8>>>),
    Owned(Arc<RwLock<Vec<u8>>>),
    SharedOwned(Arc<RwLock<Vec<u8>>>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferError {
    NegativeAllocationSize(i32),
    AllocationFailed,
    BorrowedBufferCannotBeShallowCopied,
    BorrowedBufferCannotBeAssigned,
    ReadOnlyBuffer,
    LockContended,
    LockPoisoned,
}

impl From<TryLockError<std::sync::RwLockReadGuard<'_, Vec<u8>>>> for BufferError {
    fn from(error: TryLockError<std::sync::RwLockReadGuard<'_, Vec<u8>>>) -> Self {
        match error {
            TryLockError::WouldBlock => Self::LockContended,
            TryLockError::Poisoned(_) => Self::LockPoisoned,
        }
    }
}

impl From<TryLockError<std::sync::RwLockWriteGuard<'_, Vec<u8>>>> for BufferError {
    fn from(error: TryLockError<std::sync::RwLockWriteGuard<'_, Vec<u8>>>) -> Self {
        match error {
            TryLockError::WouldBlock => Self::LockContended,
            TryLockError::Poisoned(_) => Self::LockPoisoned,
        }
    }
}

#[derive(Debug)]
pub struct LegacyBuffer<'a> {
    storage: Option<BufferStorage<'a>>,
    size: i32,
}

impl<'a> LegacyBuffer<'a> {
    /// Null default state, equivalent to BufferPtr=NULL, Size=0.
    pub const fn null() -> Self { Self { storage: None, size: 0 } }

    /// Caller-owned mutable memory. Like the C++ pointer constructor, the
    /// supplied size is retained even when it is zero or negative.
    pub fn from_mut_slice(buffer: &'a mut [u8], size: i32) -> Self {
        Self { storage: Some(BufferStorage::BorrowedMutable(buffer)), size }
    }

    /// Const-pointer constructor. The source casts away constness internally;
    /// this Rust form keeps the view read-only.
    pub fn from_const_slice(buffer: &'a [u8], size: i32) -> Self {
        Self { storage: Some(BufferStorage::BorrowedConst(buffer)), size }
    }

    /// Shared external memory for pointer aliases that need to outlive any
    /// one Buffer handle.
    pub fn from_shared_external(buffer: Arc<RwLock<Vec<u8>>>, size: i32) -> Self {
        Self { storage: Some(BufferStorage::SharedExternal(buffer)), size }
    }

    /// Self-allocating constructor. Non-positive sizes retain the requested
    /// Size but have a null BufferPtr, matching the C++ constructor.
    pub fn allocate(size: i32) -> Self {
        if size <= 0 { return Self { storage: None, size }; }
        let mut bytes = Vec::new();
        if bytes.try_reserve_exact(size as usize).is_err() {
            return Self { storage: None, size };
        }
        bytes.resize(size as usize, 0);
        Self {
            storage: Some(BufferStorage::Owned(Arc::new(RwLock::new(bytes)))),
            size,
        }
    }

    /// Fallible counterpart that reports allocation failure instead of
    /// representing it as a null pointer with a positive Size.
    pub fn try_allocate(size: i32) -> Result<Self, BufferError> {
        if size < 0 { return Err(BufferError::NegativeAllocationSize(size)); }
        if size == 0 { return Ok(Self::null()); }
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(size as usize).map_err(|_| BufferError::AllocationFailed)?;
        bytes.resize(size as usize, 0);
        Ok(Self {
            storage: Some(BufferStorage::Owned(Arc::new(RwLock::new(bytes)))),
            size,
        })
    }

    pub const fn get_size(&self) -> i32 { self.size }
    pub fn is_valid(&self) -> bool { self.storage.is_some() }

    /// Mirrors the C++ IsAllocated bit: only the handle that performed the
    /// allocation reports true. Shallow copies remain non-owners.
    pub fn is_allocated(&self) -> bool {
        matches!(self.storage, Some(BufferStorage::Owned(_)))
    }

    pub fn capacity(&self) -> Result<usize, BufferError> {
        match &self.storage {
            None => Ok(0),
            Some(BufferStorage::BorrowedMutable(bytes)) => Ok(bytes.len()),
            Some(BufferStorage::BorrowedConst(bytes)) => Ok(bytes.len()),
            Some(BufferStorage::SharedExternal(buffer))
            | Some(BufferStorage::Owned(buffer))
            | Some(BufferStorage::SharedOwned(buffer)) => {
                let guard = buffer.try_read()?;
                Ok(guard.len())
            }
        }
    }

    fn effective_size(&self, capacity: usize) -> usize {
        if self.size <= 0 { 0 } else { (self.size as usize).min(capacity) }
    }

    pub fn with_read<T>(
        &self,
        operation: impl FnOnce(&[u8]) -> T,
    ) -> Result<Option<T>, BufferError> {
        match &self.storage {
            None => Ok(None),
            Some(BufferStorage::BorrowedMutable(bytes)) => {
                let size = self.effective_size(bytes.len());
                Ok(Some(operation(&bytes[..size])))
            }
            Some(BufferStorage::BorrowedConst(bytes)) => {
                let size = self.effective_size(bytes.len());
                Ok(Some(operation(&bytes[..size])))
            }
            Some(BufferStorage::SharedExternal(buffer))
            | Some(BufferStorage::Owned(buffer))
            | Some(BufferStorage::SharedOwned(buffer)) => {
                let guard = buffer.try_read()?;
                let size = self.effective_size(guard.len());
                Ok(Some(operation(&guard[..size])))
            }
        }
    }

    pub fn with_write<T>(
        &mut self,
        operation: impl FnOnce(&mut [u8]) -> T,
    ) -> Result<Option<T>, BufferError> {
        match &mut self.storage {
            None => Ok(None),
            Some(BufferStorage::BorrowedMutable(bytes)) => {
                let size = if self.size <= 0 { 0 } else { (self.size as usize).min(bytes.len()) };
                Ok(Some(operation(&mut bytes[..size])))
            }
            Some(BufferStorage::BorrowedConst(_)) => Err(BufferError::ReadOnlyBuffer),
            Some(BufferStorage::SharedExternal(buffer))
            | Some(BufferStorage::Owned(buffer))
            | Some(BufferStorage::SharedOwned(buffer)) => {
                let mut guard = buffer.try_write()?;
                let size = if self.size <= 0 { 0 } else { (self.size as usize).min(guard.len()) };
                Ok(Some(operation(&mut guard[..size])))
            }
        }
    }

    /// Buffer copy constructor: pointer and size are copied, but the new
    /// handle's IsAllocated bit is false. Arc-backed storage keeps the bytes
    /// alive until the last alias is dropped.
    pub fn copy_construct(&self) -> Result<Self, BufferError> {
        let storage = match &self.storage {
            None => None,
            Some(BufferStorage::SharedExternal(buffer)) => {
                Some(BufferStorage::SharedExternal(Arc::clone(buffer)))
            }
            Some(BufferStorage::Owned(buffer)) | Some(BufferStorage::SharedOwned(buffer)) => {
                Some(BufferStorage::SharedOwned(Arc::clone(buffer)))
            }
            Some(BufferStorage::BorrowedMutable(_)) | Some(BufferStorage::BorrowedConst(_)) => {
                return Err(BufferError::BorrowedBufferCannotBeShallowCopied);
            }
        };
        Ok(Self { storage, size: self.size })
    }

    /// Buffer::operator=. Existing owned storage is released first; the
    /// assigned handle points at the source storage without owning its
    /// allocation flag, exactly like the source operator.
    pub fn assign_from(&mut self, source: &Self) -> Result<(), BufferError> {
        let replacement = source.copy_construct()?;
        *self = replacement;
        Ok(())
    }

    /// Reset/destructor behavior: release owned storage, clear pointer and size.
    pub fn reset(&mut self) {
        self.storage = None;
        self.size = 0;
    }

    pub fn is_borrowed(&self) -> bool {
        matches!(
            self.storage,
            Some(BufferStorage::BorrowedMutable(_)) | Some(BufferStorage::BorrowedConst(_))
        )
    }

    pub fn is_shared(&self) -> bool {
        matches!(
            self.storage,
            Some(BufferStorage::SharedExternal(_)) | Some(BufferStorage::SharedOwned(_))
        )
    }
}

impl Default for LegacyBuffer<'_> {
    fn default() -> Self { Self::null() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_and_pointer_constructors_preserve_null_and_size_states() {
        let null = LegacyBuffer::null();
        assert!(!null.is_valid());
        assert_eq!(null.get_size(), 0);
        assert!(!null.is_allocated());

        let mut bytes = [1_u8, 2, 3, 4];
        let zero = LegacyBuffer::from_mut_slice(&mut bytes, 0);
        assert!(zero.is_valid());
        assert_eq!(zero.get_size(), 0);
        assert_eq!(zero.with_read(|slice| slice.len()).unwrap(), Some(0));
    }

    #[test]
    fn borrowed_mutable_buffer_exposes_only_the_requested_slice() {
        let mut bytes = [1_u8, 2, 3, 4, 5];
        let mut buffer = LegacyBuffer::from_mut_slice(&mut bytes, 3);
        assert!(buffer.is_borrowed());
        assert!(!buffer.is_allocated());
        buffer.with_write(|slice| slice[1] = 99).unwrap();
        assert_eq!(buffer.with_read(|slice| slice.to_vec()).unwrap(), Some(vec![1, 99, 3]));
    }

    #[test]
    fn const_pointer_constructor_is_read_only() {
        let bytes = [3_u8, 4, 5];
        let mut buffer = LegacyBuffer::from_const_slice(&bytes, 2);
        assert_eq!(buffer.with_read(|slice| slice.to_vec()).unwrap(), Some(vec![3, 4]));
        assert_eq!(buffer.with_write(|slice| slice[0] = 8), Err(BufferError::ReadOnlyBuffer));
    }

    #[test]
    fn allocated_constructor_owns_zero_initialized_bytes() {
        let mut buffer = LegacyBuffer::allocate(5);
        assert!(buffer.is_valid());
        assert!(buffer.is_allocated());
        assert_eq!(buffer.get_size(), 5);
        assert_eq!(buffer.with_read(|slice| slice.to_vec()).unwrap(), Some(vec![0; 5]));
        buffer.with_write(|slice| slice.copy_from_slice(&[1, 2, 3, 4, 5])).unwrap();
        assert_eq!(buffer.with_read(|slice| slice.to_vec()).unwrap(), Some(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn non_positive_allocation_retains_size_and_null_pointer_state() {
        let zero = LegacyBuffer::allocate(0);
        assert!(!zero.is_valid());
        assert_eq!(zero.get_size(), 0);
        let negative = LegacyBuffer::allocate(-5);
        assert!(!negative.is_valid());
        assert_eq!(negative.get_size(), -5);
        assert!(matches!(LegacyBuffer::try_allocate(-1), Err(BufferError::NegativeAllocationSize(-1))));
    }

    #[test]
    fn shallow_copy_shares_owned_allocation_without_becoming_owner() {
        let mut original = LegacyBuffer::allocate(4);
        original.with_write(|bytes| bytes.copy_from_slice(&[4, 3, 2, 1])).unwrap();
        let mut copy = original.copy_construct().unwrap();
        assert!(original.is_allocated());
        assert!(!copy.is_allocated());
        assert!(copy.is_shared());
        copy.with_write(|bytes| bytes[0] = 99).unwrap();
        assert_eq!(original.with_read(|bytes| bytes[0]).unwrap(), Some(99));
        original.reset();
        assert_eq!(copy.with_read(|bytes| bytes.to_vec()).unwrap(), Some(vec![99, 3, 2, 1]));
    }

    #[test]
    fn copy_of_borrowed_slice_is_rejected_without_aliasing_mutable_memory() {
        let mut bytes = [1_u8, 2, 3];
        let buffer = LegacyBuffer::from_mut_slice(&mut bytes, 3);
        assert_eq!(buffer.copy_construct().err(), Some(BufferError::BorrowedBufferCannotBeShallowCopied));
    }

    #[test]
    fn external_shared_pointer_can_be_shallow_copied() {
        let external = Arc::new(RwLock::new(vec![1_u8, 2, 3]));
        let original = LegacyBuffer::from_shared_external(Arc::clone(&external), 3);
        let mut copy = original.copy_construct().unwrap();
        assert!(!copy.is_allocated());
        copy.with_write(|bytes| bytes[2] = 77).unwrap();
        assert_eq!(*external.read().unwrap(), vec![1, 2, 77]);
    }

    #[test]
    fn assignment_releases_previous_state_and_shares_source_pointer() {
        let mut source = LegacyBuffer::allocate(3);
        source.with_write(|bytes| bytes.copy_from_slice(&[5, 6, 7])).unwrap();
        let mut target = LegacyBuffer::allocate(10);
        target.assign_from(&source).unwrap();
        assert_eq!(target.get_size(), 3);
        assert!(!target.is_allocated());
        assert_eq!(target.with_read(|bytes| bytes.to_vec()).unwrap(), Some(vec![5, 6, 7]));
    }

    #[test]
    fn assignment_from_unshareable_borrowed_storage_fails_without_mutation() {
        let mut borrowed_data = [8_u8, 9, 10];
        let borrowed = LegacyBuffer::from_mut_slice(&mut borrowed_data, 3);
        let mut target = LegacyBuffer::allocate(2);
        assert_eq!(target.assign_from(&borrowed), Err(BufferError::BorrowedBufferCannotBeShallowCopied));
        assert_eq!(target.get_size(), 2);
        assert!(target.is_allocated());
    }

    #[test]
    fn reset_clears_allocation_pointer_and_size_and_is_idempotent() {
        let mut buffer = LegacyBuffer::allocate(7);
        buffer.reset();
        assert!(!buffer.is_valid());
        assert_eq!(buffer.get_size(), 0);
        assert!(!buffer.is_allocated());
        buffer.reset();
        assert_eq!(buffer.get_size(), 0);
    }

    #[test]
    fn negative_external_size_is_preserved_but_safe_access_is_empty() {
        let mut bytes = [1_u8, 2, 3];
        let buffer = LegacyBuffer::from_mut_slice(&mut bytes, -1);
        assert!(buffer.is_valid());
        assert_eq!(buffer.get_size(), -1);
        assert_eq!(buffer.with_read(|slice| slice.is_empty()).unwrap(), Some(true));
    }
}
