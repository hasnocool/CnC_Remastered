//! Red Alert WarheadTypeClass from REDALERT/WARHEAD.CPP and WARHEAD.H.

use std::collections::HashMap;

use crate::fixed16::Fixed16;
use crate::heap::{FixedIndexHeap, HeapError, HeapHandle};

pub const ARMOR_NONE: usize = 0;
pub const ARMOR_WOOD: usize = 1;
pub const ARMOR_ALUMINUM: usize = 2;
pub const ARMOR_STEEL: usize = 3;
pub const ARMOR_CONCRETE: usize = 4;
pub const ARMOR_COUNT: usize = 5;

pub const WARHEAD_NONE: i32 = -1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WarheadTypeClass {
    pub id: i32,
    pub ini_name: String,
    pub spread_factor: i32,
    pub is_wall_destroyer: bool,
    pub is_wood_destroyer: bool,
    pub is_tiberium_destroyer: bool,
    pub is_organic: bool,
    pub modifiers: [Fixed16; ARMOR_COUNT],
    pub explosion_set: i32,
    pub infantry_death: i32,
}

impl WarheadTypeClass {
    /// Constructor defaults from WARHEAD.CPP; the heap supplies the stable
    /// logical WarheadType ID in place of Warheads.ID(this).
    pub fn new(id: i32, name: impl Into<String>) -> Self {
        Self {
            id,
            ini_name: name.into(),
            spread_factor: 1,
            is_wall_destroyer: false,
            is_wood_destroyer: false,
            is_tiberium_destroyer: false,
            is_organic: false,
            modifiers: [Fixed16::ONE; ARMOR_COUNT],
            explosion_set: 0,
            infantry_death: 0,
        }
    }

    pub fn name(&self) -> &str { &self.ini_name }

    /// Read_INI reads only when the section exists. Verses is parsed only
    /// when that key is present; omitted fields preserve constructor values.
    pub fn read_ini(&mut self, ini: &impl WarheadIni) -> Result<bool, WarheadReadError> {
        if !ini.is_present(self.name()) { return Ok(false); }

        self.spread_factor = ini.get_int(self.name(), "Spread", self.spread_factor);
        self.is_wall_destroyer = ini.get_bool(self.name(), "Wall", self.is_wall_destroyer);
        self.is_wood_destroyer = ini.get_bool(self.name(), "Wood", self.is_wood_destroyer);
        self.is_tiberium_destroyer = ini.get_bool(self.name(), "Ore", self.is_tiberium_destroyer);
        self.explosion_set = ini.get_int(self.name(), "Explosion", self.explosion_set);
        self.infantry_death = ini.get_int(self.name(), "InfDeath", self.infantry_death);

        if let Some(verses) = ini.get_string(self.name(), "Verses") {
            let values: Vec<&str> = verses.split(',').take(ARMOR_COUNT).collect();
            if values.len() != ARMOR_COUNT {
                return Err(WarheadReadError::InvalidVersesCount {
                    expected: ARMOR_COUNT,
                    supplied: values.len(),
                });
            }
            for (index, value) in values.into_iter().enumerate() {
                self.modifiers[index] = Fixed16::from_ascii(value.trim());
            }
            self.is_organic = self.modifiers[ARMOR_STEEL] == Fixed16::ZERO;
        }
        Ok(true)
    }

    pub const fn modifier_against(&self, armor: usize) -> Option<Fixed16> {
        if armor < ARMOR_COUNT { Some(self.modifiers[armor]) } else { None }
    }

    pub const fn is_destroyer(&self) -> bool {
        self.is_wall_destroyer || self.is_wood_destroyer || self.is_tiberium_destroyer
    }
}

pub trait WarheadIni {
    fn is_present(&self, section: &str) -> bool;
    fn get_int(&self, section: &str, key: &str, default: i32) -> i32;
    fn get_bool(&self, section: &str, key: &str, default: bool) -> bool;
    /// Some(value) means Get_String found the key and copied it to the buffer.
    fn get_string(&self, section: &str, key: &str) -> Option<String>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WarheadReadError {
    InvalidVersesCount { expected: usize, supplied: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct WarheadHandle(pub HeapHandle);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WarheadRegistryError {
    Heap(HeapError),
    InvalidHandle(WarheadHandle),
    InvalidWarheadId(i32),
}

impl From<HeapError> for WarheadRegistryError {
    fn from(value: HeapError) -> Self { Self::Heap(value) }
}

/// TFixedIHeapClass-backed warhead storage. Values live in a typed map while
/// the indexed heap preserves the original fixed-capacity allocation order.
#[derive(Clone, Debug)]
pub struct WarheadRegistry {
    heap: FixedIndexHeap,
    objects: HashMap<HeapHandle, WarheadTypeClass>,
}

impl WarheadRegistry {
    pub fn new(capacity: usize) -> Result<Self, WarheadRegistryError> {
        let mut heap = FixedIndexHeap::new(1);
        heap.set_heap(capacity, None)?;
        Ok(Self { heap, objects: HashMap::new() })
    }

    pub fn count(&self) -> usize { self.heap.count() }
    pub fn capacity(&self) -> usize { self.heap.length() }
    pub fn available(&self) -> usize { self.heap.avail() }

    /// WarheadTypeClass::operator new followed by constructor ID assignment.
    pub fn allocate(&mut self, name: impl Into<String>) -> Option<WarheadHandle> {
        let handle = self.heap.allocate()?;
        let id = self.heap.logical_id(handle)? as i32;
        self.objects.insert(handle, WarheadTypeClass::new(id, name));
        Some(WarheadHandle(handle))
    }

    /// operator delete returns the slot to the indexed heap. The legacy
    /// FixedIHeapClass Free routine removes ActivePointers and reports false;
    /// the slot is nevertheless reclaimed. Remaining logical IDs are renumbered.
    pub fn free(&mut self, handle: WarheadHandle) -> Result<(), WarheadRegistryError> {
        if self.objects.remove(&handle.0).is_none() {
            return Err(WarheadRegistryError::InvalidHandle(handle));
        }
        let _legacy_free_result = self.heap.free(Some(handle.0));
        self.renumber_active_ids();
        Ok(())
    }

    fn renumber_active_ids(&mut self) {
        let active = self.heap.active_pointers().to_vec();
        for (id, handle) in active.into_iter().enumerate() {
            if let Some(object) = self.objects.get_mut(&handle) {
                object.id = id as i32;
            }
        }
    }

    pub fn as_pointer(&self, id: i32) -> Option<&WarheadTypeClass> {
        if id == WARHEAD_NONE || id < 0 { return None; }
        let handle = self.heap.active_ptr(id as usize)?;
        self.objects.get(&handle)
    }

    pub fn as_pointer_mut(&mut self, id: i32) -> Option<&mut WarheadTypeClass> {
        if id == WARHEAD_NONE || id < 0 { return None; }
        let handle = self.heap.active_ptr(id as usize)?;
        self.objects.get_mut(&handle)
    }

    pub fn by_handle(&self, handle: WarheadHandle) -> Option<&WarheadTypeClass> {
        self.objects.get(&handle.0)
    }

    pub fn by_handle_mut(&mut self, handle: WarheadHandle) -> Option<&mut WarheadTypeClass> {
        self.objects.get_mut(&handle.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct IniData {
        sections: HashMap<String, HashMap<String, String>>,
    }

    impl IniData {
        fn set(&mut self, section: &str, key: &str, value: &str) {
            self.sections.entry(section.to_owned()).or_default()
                .insert(key.to_owned(), value.to_owned());
        }
    }

    impl WarheadIni for IniData {
        fn is_present(&self, section: &str) -> bool { self.sections.contains_key(section) }
        fn get_int(&self, section: &str, key: &str, default: i32) -> i32 {
            self.sections.get(section).and_then(|s| s.get(key))
                .and_then(|v| v.parse().ok()).unwrap_or(default)
        }
        fn get_bool(&self, section: &str, key: &str, default: bool) -> bool {
            self.sections.get(section).and_then(|s| s.get(key)).map(|v| {
                matches!(v.to_ascii_lowercase().as_str(), "yes" | "true" | "1" | "on")
            }).unwrap_or(default)
        }
        fn get_string(&self, section: &str, key: &str) -> Option<String> {
            self.sections.get(section)?.get(key).cloned()
        }
    }

    #[test]
    fn constructor_initializes_modifiers_and_boolean_flags() {
        let warhead = WarheadTypeClass::new(4, "HE");
        assert_eq!(warhead.id, 4);
        assert_eq!(warhead.name(), "HE");
        assert_eq!(warhead.spread_factor, 1);
        assert_eq!(warhead.modifiers, [Fixed16::ONE; ARMOR_COUNT]);
        assert!(!warhead.is_wall_destroyer);
        assert!(!warhead.is_wood_destroyer);
        assert!(!warhead.is_tiberium_destroyer);
        assert!(!warhead.is_organic);
        assert_eq!(warhead.explosion_set, 0);
        assert_eq!(warhead.infantry_death, 0);
    }

    #[test]
    fn read_ini_updates_all_fields_and_parses_five_fixed_verses() {
        let mut ini = IniData::default();
        ini.set("HE", "Spread", "2");
        ini.set("HE", "Wall", "yes");
        ini.set("HE", "Wood", "true");
        ini.set("HE", "Ore", "1");
        ini.set("HE", "Explosion", "3");
        ini.set("HE", "InfDeath", "4");
        ini.set("HE", "Verses", "100%,50%,75%,0%,25%");
        let mut warhead = WarheadTypeClass::new(1, "HE");

        assert!(warhead.read_ini(&ini).unwrap());
        assert_eq!(warhead.spread_factor, 2);
        assert!(warhead.is_wall_destroyer);
        assert!(warhead.is_wood_destroyer);
        assert!(warhead.is_tiberium_destroyer);
        assert!(warhead.is_organic);
        assert_eq!(warhead.explosion_set, 3);
        assert_eq!(warhead.infantry_death, 4);
        assert_eq!(warhead.modifiers[ARMOR_NONE], Fixed16::ONE);
        assert_eq!(warhead.modifiers[ARMOR_WOOD], Fixed16::ONE_HALF);
        assert_eq!(warhead.modifiers[ARMOR_ALUMINUM], Fixed16::from_ratio(3, 4));
        assert_eq!(warhead.modifiers[ARMOR_STEEL], Fixed16::ZERO);
        assert_eq!(warhead.modifiers[ARMOR_CONCRETE], Fixed16::ONE_QUARTER);
    }

    #[test]
    fn absent_section_and_absent_verses_preserve_defaults() {
        let mut warhead = WarheadTypeClass::new(0, "SA");
        let empty = IniData::default();
        assert!(!warhead.read_ini(&empty).unwrap());
        let mut ini = IniData::default();
        ini.set("SA", "Spread", "3");
        assert!(warhead.read_ini(&ini).unwrap());
        assert_eq!(warhead.spread_factor, 3);
        assert_eq!(warhead.modifiers, [Fixed16::ONE; ARMOR_COUNT]);
        assert!(!warhead.is_organic);
    }

    #[test]
    fn registry_allocates_ids_resolves_and_releases_heap_slots() {
        let mut registry = WarheadRegistry::new(2).unwrap();
        let small = registry.allocate("SA").unwrap();
        let high = registry.allocate("HE").unwrap();
        assert_eq!(registry.by_handle(small).unwrap().id, 0);
        assert_eq!(registry.by_handle(high).unwrap().id, 1);
        assert_eq!(registry.as_pointer(1).unwrap().name(), "HE");
        assert!(registry.allocate("AP").is_none());
        registry.free(small).unwrap();
        assert_eq!(registry.count(), 1);
        assert_eq!(registry.as_pointer(0).unwrap().name(), "HE");
        assert_eq!(registry.as_pointer(-1), None);
        assert_eq!(registry.as_pointer(WARHEAD_NONE), None);
    }

    #[test]
    fn invalid_verses_count_is_reported_without_indexing_past_modifiers() {
        let mut ini = IniData::default();
        ini.set("HE", "Verses", "100%,50%");
        let mut warhead = WarheadTypeClass::new(0, "HE");
        assert_eq!(
            warhead.read_ini(&ini),
            Err(WarheadReadError::InvalidVersesCount { expected: 5, supplied: 2 })
        );
    }
}
