//! Stable-ID pointer wrapper from REDALERT/CCPTR.CPP and CCPTR.H.
//!
//! CCPtr stores a heap index rather than a raw pointer. Its relational
//! operators intentionally preserve the original header's comparison
//! expressions, including the nonstandard <= and >= definitions.

use std::marker::PhantomData;

pub trait StableNamedObject {
    fn stable_id(&self) -> i32;
    fn name(&self) -> &str;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CCPtr<T> {
    id: i32,
    marker: PhantomData<fn() -> T>,
}

impl<T> Default for CCPtr<T> {
    fn default() -> Self { Self::new() }
}

impl<T> CCPtr<T> {
    pub const fn new() -> Self { Self { id: -1, marker: PhantomData } }
    pub const fn from_raw(id: i32) -> Self { Self { id, marker: PhantomData } }
    pub fn from_object(value: Option<&impl StableNamedObject>) -> Self {
        Self::from_raw(value.map_or(-1, |object| object.stable_id()))
    }
    pub const fn is_valid(self) -> bool { self.id != -1 }
    pub const fn raw(self) -> i32 { self.id }
    pub fn set_raw(&mut self, value: i32) { self.id = value; }

    pub fn get<'a, O: StableNamedObject>(&self, heap: &'a CcpHeap<O>) -> Option<&'a O> {
        if self.id < 0 { return None; }
        heap.entries.get(self.id as usize).and_then(Option::as_ref)
    }

    pub fn greater_than<O: StableNamedObject>(&self, rhs: &Self, heap: &CcpHeap<O>) -> Result<bool, CcpPtrError> {
        let left = self.get(heap).ok_or(CcpPtrError::InvalidHeapId(self.id))?;
        let right = rhs.get(heap).ok_or(CcpPtrError::InvalidHeapId(rhs.id))?;
        Ok(ascii_case_insensitive_cmp(left.name(), right.name()).is_gt())
    }

    pub fn less_than<O: StableNamedObject>(&self, rhs: &Self, heap: &CcpHeap<O>) -> Result<bool, CcpPtrError> {
        Ok(self.id != rhs.id && rhs.greater_than(self, heap)?)
    }

    /// Exact CCPTR.H expression: rvalue > *this (does not include equality).
    pub fn less_equal<O: StableNamedObject>(&self, rhs: &Self, heap: &CcpHeap<O>) -> Result<bool, CcpPtrError> {
        rhs.greater_than(self, heap)
    }

    /// Exact CCPTR.H expression: equality OR rvalue > *this.
    pub fn greater_equal<O: StableNamedObject>(&self, rhs: &Self, heap: &CcpHeap<O>) -> Result<bool, CcpPtrError> {
        Ok(self.id == rhs.id || rhs.greater_than(self, heap)?)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CcpPtrError { InvalidHeapId(i32) }

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct CcpHeap<T> {
    entries: Vec<Option<T>>,
}

impl<T> CcpHeap<T> {
    pub fn new() -> Self { Self { entries: Vec::new() } }
    pub fn insert(&mut self, value: T) -> i32 {
        let id = self.entries.len() as i32;
        self.entries.push(Some(value));
        id
    }
    pub fn remove(&mut self, id: i32) -> Option<T> {
        if id < 0 { return None; }
        self.entries.get_mut(id as usize).and_then(Option::take)
    }
    pub fn len(&self) -> usize { self.entries.len() }
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }
}

fn ascii_case_insensitive_cmp(left: &str, right: &str) -> std::cmp::Ordering {
    left.bytes().map(|b| b.to_ascii_lowercase())
        .cmp(right.bytes().map(|b| b.to_ascii_lowercase()))
}

/// Explicit template instantiations in the upstream translation unit.
pub const CCPTR_INSTANTIATED_TYPES: &[&str] = &[
    "AircraftClass", "AircraftTypeClass", "AnimClass", "AnimTypeClass",
    "BuildingClass", "BuildingTypeClass", "BulletClass", "BulletTypeClass",
    "FactoryClass", "HouseClass", "HouseTypeClass", "InfantryClass",
    "InfantryTypeClass", "OverlayClass", "OverlayTypeClass", "SmudgeClass",
    "SmudgeTypeClass", "TeamClass", "TeamTypeClass", "TemplateClass",
    "TemplateTypeClass", "TerrainClass", "TerrainTypeClass", "TriggerClass",
    "TriggerTypeClass", "UnitClass", "UnitTypeClass", "VesselClass",
    "VesselTypeClass", "WarheadTypeClass", "WeaponTypeClass",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug)]
    struct Object { id: i32, name: &'static str }
    impl StableNamedObject for Object {
        fn stable_id(&self) -> i32 { self.id }
        fn name(&self) -> &str { self.name }
    }

    #[test]
    fn stores_stable_heap_ids_and_resolves_objects() {
        let mut heap = CcpHeap::new();
        let id = heap.insert(Object { id: 0, name: "Tank" });
        let ptr = CCPtr::<Object>::from_raw(id);
        assert!(ptr.is_valid());
        assert_eq!(ptr.get(&heap).unwrap().name(), "Tank");
        assert!(CCPtr::<Object>::new().get(&heap).is_none());
    }

    #[test]
    fn greater_than_uses_case_insensitive_name_comparison() {
        let mut heap = CcpHeap::new();
        let a = heap.insert(Object { id: 0, name: "Alpha" });
        let b = heap.insert(Object { id: 1, name: "beta" });
        let left = CCPtr::<Object>::from_raw(a);
        let right = CCPtr::<Object>::from_raw(b);
        assert!(!left.greater_than(&right, &heap).unwrap());
        assert!(right.greater_than(&left, &heap).unwrap());
        assert!(left.less_than(&right, &heap).unwrap());
    }

    #[test]
    fn preserves_original_nonstandard_relational_operator_expressions() {
        let mut heap = CcpHeap::new();
        let a = CCPtr::<Object>::from_raw(heap.insert(Object { id: 0, name: "Alpha" }));
        let b = CCPtr::<Object>::from_raw(heap.insert(Object { id: 1, name: "Beta" }));
        assert!(a.less_equal(&b, &heap).unwrap());
        assert!(a.greater_equal(&b, &heap).unwrap());
        assert!(!a.less_equal(&a, &heap).unwrap());
        assert!(a.greater_equal(&a, &heap).unwrap());
    }

    #[test]
    fn null_and_raw_id_semantics_match_header() {
        let mut ptr = CCPtr::<Object>::new();
        assert!(!ptr.is_valid());
        ptr.set_raw(7);
        assert_eq!(ptr.raw(), 7);
        assert_eq!(CCPTR_INSTANTIATED_TYPES.len(), 31);
    }
}
