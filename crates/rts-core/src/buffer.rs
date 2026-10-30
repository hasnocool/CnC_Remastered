//! Westwood BufferClass ownership and size semantics from WIN32LIB/BUFFER.CPP.

/// A buffer either borrows caller memory or owns an allocation, matching the
/// legacy Allocated flag and destructor behavior.
#[derive(Debug)]
pub enum BufferStorage<'a> {
    Borrowed(&'a mut [u8]),
    Owned(Vec<u8>),
}

/// Rust representation of BufferClass.
#[derive(Debug)]
pub struct BufferClass<'a> {
    storage: Option<BufferStorage<'a>>,
    size: usize,
}

impl<'a> BufferClass<'a> {
    /// Default constructor: no storage, zero size, not allocated.
    pub const fn empty() -> Self {
        Self { storage: None, size: 0 }
    }

    /// Constructor with caller-supplied memory.
    pub fn from_slice(buffer: &'a mut [u8], size: usize) -> Self {
        let size = size.min(buffer.len());
        Self { storage: Some(BufferStorage::Borrowed(buffer)), size }
    }

    /// Size-only constructor: allocate and own a zero-initialized byte array.
    pub fn allocate(size: usize) -> Self {
        Self { storage: Some(BufferStorage::Owned(vec![0; size])), size }
    }

    pub fn size(&self) -> usize { self.size }

    pub fn is_allocated(&self) -> bool {
        matches!(self.storage, Some(BufferStorage::Owned(_)))
    }

    pub fn as_slice(&self) -> &[u8] {
        match &self.storage {
            Some(BufferStorage::Borrowed(bytes)) => &bytes[..self.size],
            Some(BufferStorage::Owned(bytes)) => &bytes[..self.size],
            None => &[],
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        match &mut self.storage {
            Some(BufferStorage::Borrowed(bytes)) => &mut bytes[..self.size],
            Some(BufferStorage::Owned(bytes)) => &mut bytes[..self.size],
            None => &mut [],
        }
    }

    /// Rebind an empty/default-constructed buffer to caller memory.
    pub fn attach(&mut self, buffer: &'a mut [u8], size: usize) {
        self.size = size.min(buffer.len());
        self.storage = Some(BufferStorage::Borrowed(buffer));
    }

    pub fn clear(&mut self) {
        self.storage = None;
        self.size = 0;
    }
}

impl Default for BufferClass<'_> {
    fn default() -> Self { Self::empty() }
}
