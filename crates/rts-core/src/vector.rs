//! Generic and dynamic vector containers ported from REDALERT/VECTOR.H and VECTOR.CPP.
//!
//! The original C++ vector may either own an allocated array or overlay a caller
//! supplied memory block. Rust cannot safely preserve raw placement-new aliasing,
//! so `new_fixed`/ `from_fixed_slice` preserve the fixed-capacity growth rule
//! while storing a safe owned copy. Values, active counts, growth increments,
//! ordering, lookup, and deletion semantics are retained.

/// Fixed-capacity vector equivalent to the upstream `VectorClass<T>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VectorClass<T> {
    elements: Vec<T>,
    is_allocated: bool,
}

impl<T: Clone + Default> VectorClass<T> {
    /// Allocate an owned vector. It may be automatically grown by a dynamic
    /// vector that uses this as its base storage.
    pub fn new(size: usize) -> Self {
        Self {
            elements: vec![T::default(); size],
            is_allocated: size != 0,
        }
    }

    /// Model a vector constructed over caller-provided storage. The provided
    /// values are copied into safe Rust-owned backing storage, but the vector
    /// retains the source's non-owning/fixed-capacity growth policy.
    pub fn from_fixed_slice(elements: &[T]) -> Self {
        Self {
            elements: elements.to_vec(),
            is_allocated: false,
        }
    }

    pub fn length(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn is_allocated(&self) -> bool {
        self.is_allocated
    }

    pub fn as_slice(&self) -> &[T] {
        &self.elements
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.elements
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.elements.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.elements.get_mut(index)
    }

    /// Port of VectorClass<T>::Resize. Existing values are assigned into the
    /// new array up to min(old length, new length); additional values are
    /// default-constructed.
    pub fn resize(&mut self, new_size: usize) {
        if new_size == 0 {
            self.clear();
            return;
        }
        self.elements.resize(new_size, T::default());
        self.is_allocated = true;
    }

    /// Resize into a source-provided fixed buffer, represented as a copied
    /// slice so no aliasing or unsafe placement-new is required.
    pub fn resize_fixed(&mut self, new_size: usize, buffer: &[T]) -> bool {
        if buffer.len() < new_size {
            return false;
        }
        if new_size == 0 {
            self.clear();
            return true;
        }

        let mut replacement = buffer[..new_size].to_vec();
        let copy_count = new_size.min(self.elements.len());
        replacement[..copy_count].clone_from_slice(&self.elements[..copy_count]);
        self.elements = replacement;
        self.is_allocated = false;
        true
    }

    /// Port of VectorClass<T>::Clear. The original only frees memory when it
    /// owns it; its pointer and logical size are cleared either way.
    pub fn clear(&mut self) {
        self.elements.clear();
        self.elements.shrink_to_fit();
        self.is_allocated = false;
    }

    /// Port of value-based VectorClass<T>::ID.
    pub fn id(&self, value: &T) -> Option<usize>
    where
        T: PartialEq,
    {
        self.elements.iter().position(|candidate| candidate == value)
    }

    /// Safe pointer-identity equivalent of VectorClass<T>::ID(T const *).
    /// Returns an index only when the supplied pointer is exactly an element
    /// address in this vector.
    pub fn id_by_pointer(&self, pointer: *const T) -> Option<usize> {
        self.elements
            .iter()
            .position(|element| core::ptr::eq(element as *const T, pointer))
    }
}

/// Packed-at-the-front container ported from `DynamicVectorClass<T>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicVectorClass<T> {
    vector: VectorClass<T>,
    active_count: usize,
    growth_step: usize,
}

impl<T: Clone + Default + PartialEq> DynamicVectorClass<T> {
    pub fn new(size: usize) -> Self {
        Self {
            vector: VectorClass::new(size),
            active_count: 0,
            growth_step: 10,
        }
    }

    pub fn from_fixed_slice(elements: &[T]) -> Self {
        Self {
            vector: VectorClass::from_fixed_slice(elements),
            active_count: 0,
            growth_step: 10,
        }
    }

    pub fn count(&self) -> usize {
        self.active_count
    }

    pub fn length(&self) -> usize {
        self.vector.length()
    }

    pub fn is_empty(&self) -> bool {
        self.active_count == 0
    }

    pub fn growth_step(&self) -> usize {
        self.growth_step
    }

    /// Returns the previous step, matching Set_Growth_Step's return behavior.
    pub fn set_growth_step(&mut self, step: usize) -> usize {
        core::mem::replace(&mut self.growth_step, step)
    }

    pub fn as_slice(&self) -> &[T] {
        &self.vector.as_slice()[..self.active_count]
    }

    pub fn as_capacity_slice(&self) -> &[T] {
        self.vector.as_slice()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.active_count {
            self.vector.get(index)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.active_count {
            self.vector.get_mut(index)
        } else {
            None
        }
    }

    pub fn resize(&mut self, new_size: usize) {
        self.vector.resize(new_size);
        self.active_count = self.active_count.min(new_size);
    }

    pub fn clear(&mut self) {
        self.active_count = 0;
        self.vector.clear();
    }

    /// Port of Delete_All: logical count becomes zero; capacity and values are
    /// retained for subsequent reuse.
    pub fn delete_all(&mut self) {
        self.active_count = 0;
    }

    fn ensure_room(&mut self) -> bool {
        if self.active_count < self.length() {
            return true;
        }

        // The C++ Add/Add_Head path only grows an owned vector, or one whose
        // initial maximum length is zero. A fixed preallocated vector cannot
        // grow implicitly.
        if (self.vector.is_allocated() || self.length() == 0) && self.growth_step > 0 {
            let Some(new_size) = self.length().checked_add(self.growth_step) else {
                return false;
            };
            self.vector.resize(new_size);
            return true;
        }

        false
    }

    pub fn add(&mut self, value: T) -> bool {
        if !self.ensure_room() {
            return false;
        }

        self.vector.as_mut_slice()[self.active_count] = value;
        self.active_count += 1;
        true
    }

    pub fn add_head(&mut self, value: T) -> bool {
        if !self.ensure_room() {
            return false;
        }

        for index in (0..self.active_count).rev() {
            let previous = self.vector.as_slice()[index].clone();
            self.vector.as_mut_slice()[index + 1] = previous;
        }
        self.vector.as_mut_slice()[0] = value;
        self.active_count += 1;
        true
    }

    /// Delete by active index. As in C++, the final inactive slot is not
    /// cleared; it retains the last assigned value after the left shift.
    pub fn delete(&mut self, index: usize) -> bool {
        if index >= self.active_count {
            return false;
        }

        self.active_count -= 1;
        for current in index..self.active_count {
            let next = self.vector.as_slice()[current + 1].clone();
            self.vector.as_mut_slice()[current] = next;
        }
        true
    }

    pub fn delete_value(&mut self, value: &T) -> bool {
        match self.id(value) {
            Some(index) => self.delete(index),
            None => false,
        }
    }

    pub fn id(&self, value: &T) -> Option<usize> {
        self.as_slice().iter().position(|candidate| candidate == value)
    }

    pub fn vector(&self) -> &VectorClass<T> {
        &self.vector
    }

    pub fn vector_mut(&mut self) -> &mut VectorClass<T> {
        &mut self.vector
    }
}

/// Multi-context dynamic vector equivalent to `DynamicVectorArrayClass`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicVectorArray<T, const COUNT: usize, const DEFAULT: usize = 0> {
    collections: [DynamicVectorClass<T>; COUNT],
    active: usize,
}

impl<T: Clone + Default + PartialEq, const COUNT: usize, const DEFAULT: usize>
    DynamicVectorArray<T, COUNT, DEFAULT>
{
    pub fn new() -> Self {
        assert!(COUNT > 0, "a dynamic vector array needs at least one context");
        assert!(DEFAULT < COUNT, "default context is outside the collection");
        Self {
            collections: core::array::from_fn(|_| DynamicVectorClass::new(0)),
            active: DEFAULT,
        }
    }

    pub fn active_context(&self) -> usize {
        self.active
    }

    pub fn set_active_context(&mut self, context: usize) -> bool {
        if context >= COUNT {
            return false;
        }
        self.active = context;
        true
    }

    pub fn count(&self) -> usize {
        self.collections[self.active].count()
    }

    pub fn count_at(&self, context: usize) -> Option<usize> {
        self.collections.get(context).map(DynamicVectorClass::count)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.collections[self.active].get(index)
    }

    pub fn add(&mut self, value: T) -> bool {
        self.collections[self.active].add(value)
    }

    pub fn add_head(&mut self, value: T) -> bool {
        self.collections[self.active].add_head(value)
    }

    pub fn delete(&mut self, index: usize) -> bool {
        self.collections[self.active].delete(index)
    }

    pub fn delete_value(&mut self, value: &T) -> bool {
        self.collections[self.active].delete_value(value)
    }

    pub fn delete_all_at(&mut self, context: usize, value: &T) -> Option<usize> {
        let collection = self.collections.get_mut(context)?;
        let mut removed = 0;
        while collection.delete_value(value) {
            removed += 1;
        }
        Some(removed)
    }

    pub fn delete_all_except(&mut self, value: &T, except: usize) -> usize {
        let mut removed = 0;
        for (context, collection) in self.collections.iter_mut().enumerate() {
            if context != except {
                while collection.delete_value(value) {
                    removed += 1;
                }
            }
        }
        removed
    }

    pub fn clear(&mut self) {
        self.collections[self.active].clear();
    }

    pub fn clear_all(&mut self) {
        for collection in &mut self.collections {
            collection.clear();
        }
    }

    pub fn raw(&self) -> &DynamicVectorClass<T> {
        &self.collections[self.active]
    }

    /// Tiberian Dawn's DynamicVectorArrayClass::Raw_Count returns the
    /// compile-time context count, independent of the active collection.
    pub const fn raw_count(&self) -> usize {
        COUNT
    }

    /// Mutable counterpart of the source's Raw() reference.
    pub fn raw_mut(&mut self) -> &mut DynamicVectorClass<T> {
        &mut self.collections[self.active]
    }

    pub fn raw_at(&self, context: usize) -> Option<&DynamicVectorClass<T>> {
        self.collections.get(context)
    }

    /// Mutable counterpart of the source's Raw(context) reference.
    pub fn raw_at_mut(&mut self, context: usize) -> Option<&mut DynamicVectorClass<T>> {
        self.collections.get_mut(context)
    }

    /// Source's context-taking Add_Head overload mistakenly calls Add rather
    /// than Add_Head. This method preserves that observed implementation.
    pub fn add_head_at(&mut self, context: usize, value: T) -> Option<bool> {
        Some(self.collections.get_mut(context)?.add(value))
    }
}

impl<T: Clone + Default + PartialEq, const COUNT: usize, const DEFAULT: usize> Default
    for DynamicVectorArray<T, COUNT, DEFAULT>
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_vector_resize_copy_and_lookup() {
        let mut vector = VectorClass::<i32>::new(3);
        vector.as_mut_slice().copy_from_slice(&[10, 20, 30]);
        assert_eq!(vector.length(), 3);
        assert_eq!(vector.id(&20), Some(1));
        assert_eq!(vector.id(&99), None);

        let pointer = &vector.as_slice()[2] as *const i32;
        assert_eq!(vector.id_by_pointer(pointer), Some(2));
        assert_eq!(vector.id_by_pointer(&99 as *const i32), None);

        vector.resize(5);
        assert_eq!(vector.as_slice(), &[10, 20, 30, 0, 0]);
        assert!(vector.is_allocated());

        vector.resize(2);
        assert_eq!(vector.as_slice(), &[10, 20]);
        vector.clear();
        assert!(vector.is_empty());
        assert!(!vector.is_allocated());
    }

    #[test]
    fn fixed_storage_prevents_implicit_dynamic_growth() {
        let mut dynamic = DynamicVectorClass::from_fixed_slice(&[0_i32, 0]);
        assert!(dynamic.add(1));
        assert!(dynamic.add(2));
        assert!(!dynamic.add(3));
        assert_eq!(dynamic.as_slice(), &[1, 2]);

        dynamic.resize(3);
        assert!(dynamic.add(3));
        assert_eq!(dynamic.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn dynamic_vector_grows_by_configured_step() {
        let mut dynamic = DynamicVectorClass::<i32>::new(0);
        assert_eq!(dynamic.growth_step(), 10);
        assert!(dynamic.add(4));
        assert_eq!(dynamic.length(), 10);
        assert!(dynamic.add(5));
        assert_eq!(dynamic.as_slice(), &[4, 5]);

        dynamic.set_growth_step(2);
        dynamic.resize(2);
        assert_eq!(dynamic.count(), 2);
        assert!(dynamic.add(6));
        assert_eq!(dynamic.length(), 4);
        assert_eq!(dynamic.as_slice(), &[4, 5, 6]);
    }

    #[test]
    fn add_head_and_delete_preserve_packed_order() {
        let mut dynamic = DynamicVectorClass::<i32>::new(3);
        assert!(dynamic.add(2));
        assert!(dynamic.add(3));
        assert!(dynamic.add_head(1));
        assert_eq!(dynamic.as_slice(), &[1, 2, 3]);

        assert!(dynamic.delete(1));
        assert_eq!(dynamic.as_slice(), &[1, 3]);
        assert!(!dynamic.delete(7));
        assert!(dynamic.delete_value(&1));
        assert_eq!(dynamic.as_slice(), &[3]);
        assert_eq!(dynamic.id(&3), Some(0));
    }

    #[test]
    fn delete_all_retains_capacity_and_growth_policy() {
        let mut dynamic = DynamicVectorClass::<i32>::new(2);
        dynamic.set_growth_step(3);
        dynamic.add(1);
        dynamic.add(2);
        dynamic.delete_all();
        assert_eq!(dynamic.length(), 2);
        assert_eq!(dynamic.count(), 0);
        assert!(dynamic.add(3));
    }

    #[test]
    fn context_array_isolates_active_vectors() {
        let mut vectors = DynamicVectorArray::<i32, 3, 1>::new();
        assert_eq!(vectors.active_context(), 1);
        assert_eq!(vectors.raw_count(), 3);
        assert!(vectors.add(11));
        assert!(vectors.set_active_context(2));
        assert!(vectors.add(22));
        assert_eq!(vectors.count(), 1);
        assert_eq!(vectors.get(0), Some(&22));
        assert_eq!(vectors.count_at(1), Some(1));
        assert_eq!(vectors.count_at(2), Some(1));

        assert!(vectors.set_active_context(1));
        assert_eq!(vectors.get(0), Some(&11));
        assert!(vectors.raw_at_mut(2).unwrap().add(23));
        assert_eq!(vectors.raw_at(2).unwrap().as_slice(), &[22, 23]);
        assert_eq!(vectors.delete_all_at(2, &22), Some(1));
        assert_eq!(vectors.count_at(2), Some(0));
    }

    #[test]
    fn invalid_context_is_rejected() {
        let mut vectors = DynamicVectorArray::<u8, 2>::new();
        assert!(!vectors.set_active_context(2));
    }
}
