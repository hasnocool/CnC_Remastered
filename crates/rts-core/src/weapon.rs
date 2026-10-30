//! Red Alert WeaponTypeClass from REDALERT/WEAPON.CPP and WEAPON.H.

use std::collections::HashMap;

use crate::heap::{FixedIndexHeap, HeapError, HeapHandle};
use crate::warhead::WarheadRegistry;

pub const WEAPON_NONE: i32 = -1;
pub const BULLET_NONE: i32 = -1;
pub const WARHEAD_NONE: i32 = -1;
pub const ARMOR_NONE: usize = 0;
pub const ARMOR_COUNT: usize = 5;

pub const MPH_IMMOBILE: u8 = 0;
pub const VOC_NONE: i16 = -1;
pub const ANIM_NONE: i8 = -1;

pub const THREAT_NORMAL: u16 = 0x0000;
pub const THREAT_AIR: u16 = 0x0004;
pub const THREAT_INFANTRY: u16 = 0x0008;
pub const THREAT_VEHICLES: u16 = 0x0010;
pub const THREAT_BUILDINGS: u16 = 0x0020;
pub const THREAT_BOATS: u16 = 0x0080;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BulletTypeProfile {
    pub is_anti_aircraft: bool,
    pub is_anti_ground: bool,
}

pub trait BulletTypeLookup {
    fn bullet_type(&self, id: i32) -> Option<BulletTypeProfile>;
}

pub trait WeaponIni {
    fn is_present(&self, section: &str) -> bool;
    fn get_bool(&self, section: &str, key: &str, default: bool) -> bool;
    fn get_int(&self, section: &str, key: &str, default: i32) -> i32;
    fn get_mph_type(&self, section: &str, key: &str, default: u8) -> u8;
    fn get_lepton(&self, section: &str, key: &str, default: i32) -> i32;
    fn get_voc_type(&self, section: &str, key: &str, default: i16) -> i16;
    fn get_anim_type(&self, section: &str, key: &str, default: i8) -> i8;
    fn get_warhead_type(&self, section: &str, key: &str, default: i32) -> i32;
    fn get_bullet_type(&self, section: &str, key: &str, default: i32) -> i32;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WeaponTypeClass {
    pub id: i32,
    pub ini_name: String,
    pub is_turbo_boosted: bool,
    pub is_supressed: bool,
    pub is_camera: bool,
    pub is_electric: bool,
    pub burst: i32,
    pub bullet_id: Option<i32>,
    pub attack: i32,
    pub max_speed: u8,
    pub warhead_id: Option<i32>,
    pub rate_of_fire: i32,
    pub range: i32,
    pub sound: i16,
    pub animation: i8,
}

impl WeaponTypeClass {
    /// Constructor values from WEAPON.CPP. IsTurboBoosted is zero because
    /// the fixed heap returns cleared storage before construction.
    pub fn new(id: i32, name: impl Into<String>) -> Self {
        Self {
            id,
            ini_name: name.into(),
            is_turbo_boosted: false,
            is_supressed: false,
            is_camera: false,
            is_electric: false,
            burst: 1,
            bullet_id: None,
            attack: 0,
            max_speed: MPH_IMMOBILE,
            warhead_id: None,
            rate_of_fire: 0,
            range: 0,
            sound: VOC_NONE,
            animation: ANIM_NONE,
        }
    }

    pub fn name(&self) -> &str { &self.ini_name }

    /// The C++ destructor nulls the three pointer members.
    pub fn destroy(&mut self) {
        self.ini_name.clear();
        self.bullet_id = None;
        self.warhead_id = None;
    }

    /// Read_INI preserves existing values when keys are absent, and resolves
    /// WarheadPtr through WarheadTypeClass::As_Pointer. Bullet is held as its
    /// type ID; a caller-provided BulletTypeLookup resolves its static data.
    pub fn read_ini(
        &mut self,
        ini: &impl WeaponIni,
        warheads: &WarheadRegistry,
    ) -> bool {
        if !ini.is_present(self.name()) { return false; }

        self.is_supressed = ini.get_bool(self.name(), "Supress", self.is_supressed);
        self.burst = ini.get_int(self.name(), "Burst", self.burst);
        self.attack = ini.get_int(self.name(), "Damage", self.attack);
        self.max_speed = ini.get_mph_type(self.name(), "Speed", self.max_speed);
        self.rate_of_fire = ini.get_int(self.name(), "ROF", self.rate_of_fire);
        self.range = ini.get_lepton(self.name(), "Range", self.range);
        self.sound = ini.get_voc_type(self.name(), "Report", self.sound);
        self.animation = ini.get_anim_type(self.name(), "Anim", self.animation);
        self.is_camera = ini.get_bool(self.name(), "Camera", self.is_camera);
        self.is_electric = ini.get_bool(self.name(), "Charges", self.is_electric);
        self.is_turbo_boosted = ini.get_bool(self.name(), "TurboBoost", self.is_turbo_boosted);

        let current_warhead = self.warhead_id.unwrap_or(WARHEAD_NONE);
        let warhead_type = ini.get_warhead_type(self.name(), "Warhead", current_warhead);
        if warhead_type != WARHEAD_NONE {
            self.warhead_id = warheads.as_pointer(warhead_type)
                .map(|warhead| warhead.id);
        } else {
            self.warhead_id = None;
        }

        let current_bullet = self.bullet_id.unwrap_or(BULLET_NONE);
        let bullet_type = ini.get_bullet_type(self.name(), "Projectile", current_bullet);
        if bullet_type != BULLET_NONE {
            self.bullet_id = Some(bullet_type);
        } else {
            self.bullet_id = None;
        }
        true
    }

    /// Allowed_Threats derives the result solely from the configured bullet
    /// type's anti-air/anti-ground flags. Normal threat is always included.
    pub fn allowed_threats(&self, bullets: &impl BulletTypeLookup) -> Option<u16> {
        let bullet_id = self.bullet_id?;
        let bullet = bullets.bullet_type(bullet_id)?;
        let mut threat = THREAT_NORMAL;
        if bullet.is_anti_aircraft { threat |= THREAT_AIR; }
        if bullet.is_anti_ground {
            threat |= THREAT_INFANTRY | THREAT_VEHICLES | THREAT_BOATS | THREAT_BUILDINGS;
        }
        Some(threat)
    }

    /// Is_Wall_Destroyer is true only when the resolved warhead exists and
    /// its IsWallDestroyer flag is set.
    pub fn is_wall_destroyer(&self, warheads: &WarheadRegistry) -> bool {
        self.warhead_id.and_then(|id| warheads.as_pointer(id))
            .is_some_and(|warhead| warhead.is_wall_destroyer)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct WeaponHandle(pub HeapHandle);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WeaponRegistryError {
    Heap(HeapError),
    InvalidHandle(WeaponHandle),
}

impl From<HeapError> for WeaponRegistryError {
    fn from(value: HeapError) -> Self { Self::Heap(value) }
}

#[derive(Clone, Debug)]
pub struct WeaponRegistry {
    heap: FixedIndexHeap,
    objects: HashMap<HeapHandle, WeaponTypeClass>,
}

impl WeaponRegistry {
    pub fn new(capacity: usize) -> Result<Self, WeaponRegistryError> {
        let mut heap = FixedIndexHeap::new(1);
        heap.set_heap(capacity, None)?;
        Ok(Self { heap, objects: HashMap::new() })
    }

    pub fn count(&self) -> usize { self.heap.count() }
    pub fn capacity(&self) -> usize { self.heap.length() }
    pub fn available(&self) -> usize { self.heap.avail() }

    /// operator new uses Weapons.Alloc; the constructor assigns Weapons.ID(this).
    pub fn allocate(&mut self, name: impl Into<String>) -> Option<WeaponHandle> {
        let handle = self.heap.allocate()?;
        let id = self.heap.logical_id(handle)? as i32;
        self.objects.insert(handle, WeaponTypeClass::new(id, name));
        Some(WeaponHandle(handle))
    }

    /// operator delete returns the slot to Weapons. The legacy FixedIHeap
    /// Free return value is false even though it frees the object slot.
    pub fn free(&mut self, handle: WeaponHandle) -> Result<(), WeaponRegistryError> {
        if self.objects.remove(&handle.0).is_none() {
            return Err(WeaponRegistryError::InvalidHandle(handle));
        }
        let _legacy_free_result = self.heap.free(Some(handle.0));
        self.renumber_active_ids();
        Ok(())
    }

    fn renumber_active_ids(&mut self) {
        for (id, handle) in self.heap.active_pointers().iter().copied().enumerate() {
            if let Some(object) = self.objects.get_mut(&handle) {
                object.id = id as i32;
            }
        }
    }

    pub fn as_pointer(&self, id: i32) -> Option<&WeaponTypeClass> {
        if id == WEAPON_NONE || id < 0 { return None; }
        let handle = self.heap.active_ptr(id as usize)?;
        self.objects.get(&handle)
    }

    pub fn as_pointer_mut(&mut self, id: i32) -> Option<&mut WeaponTypeClass> {
        if id == WEAPON_NONE || id < 0 { return None; }
        let handle = self.heap.active_ptr(id as usize)?;
        self.objects.get_mut(&handle)
    }

    pub fn by_handle(&self, handle: WeaponHandle) -> Option<&WeaponTypeClass> {
        self.objects.get(&handle.0)
    }

    pub fn by_handle_mut(&mut self, handle: WeaponHandle) -> Option<&mut WeaponTypeClass> {
        self.objects.get_mut(&handle.0)
    }

    pub fn weapon_from_name(&self, name: Option<&str>) -> i32 {
        let Some(name) = name else { return WEAPON_NONE; };
        self.heap.active_pointers().iter().find_map(|handle| {
            let weapon = self.objects.get(handle)?;
            weapon.name().eq_ignore_ascii_case(name).then_some(weapon.id)
        }).unwrap_or(WEAPON_NONE)
    }
}

/// Armor_From_Name scans the five armor names in enum order, returning
/// ARMOR_NONE for null or an unmatched string. The caller supplies the
/// canonical ArmorName table from the game data.
pub fn armor_from_name(name: Option<&str>, armor_names: &[&str; ARMOR_COUNT]) -> usize {
    let Some(name) = name else { return ARMOR_NONE; };
    armor_names.iter().position(|candidate| candidate.eq_ignore_ascii_case(name))
        .unwrap_or(ARMOR_NONE)
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

    impl WeaponIni for IniData {
        fn is_present(&self, section: &str) -> bool { self.sections.contains_key(section) }
        fn get_bool(&self, section: &str, key: &str, default: bool) -> bool {
            self.sections.get(section).and_then(|s| s.get(key)).map(|value|
                matches!(value.to_ascii_lowercase().as_str(), "yes" | "true" | "1" | "on")
            ).unwrap_or(default)
        }
        fn get_int(&self, section: &str, key: &str, default: i32) -> i32 {
            self.sections.get(section).and_then(|s| s.get(key)).and_then(|v| v.parse().ok()).unwrap_or(default)
        }
        fn get_mph_type(&self, section: &str, key: &str, default: u8) -> u8 {
            self.get_int(section, key, default as i32).clamp(0, 255) as u8
        }
        fn get_lepton(&self, section: &str, key: &str, default: i32) -> i32 {
            self.get_int(section, key, default)
        }
        fn get_voc_type(&self, section: &str, key: &str, default: i16) -> i16 {
            self.get_int(section, key, default as i32).clamp(i16::MIN as i32, i16::MAX as i32) as i16
        }
        fn get_anim_type(&self, section: &str, key: &str, default: i8) -> i8 {
            self.get_int(section, key, default as i32).clamp(i8::MIN as i32, i8::MAX as i32) as i8
        }
        fn get_warhead_type(&self, section: &str, key: &str, default: i32) -> i32 {
            self.get_int(section, key, default)
        }
        fn get_bullet_type(&self, section: &str, key: &str, default: i32) -> i32 {
            self.get_int(section, key, default)
        }
    }

    struct BulletCatalog(HashMap<i32, BulletTypeProfile>);
    impl BulletTypeLookup for BulletCatalog {
        fn bullet_type(&self, id: i32) -> Option<BulletTypeProfile> { self.0.get(&id).copied() }
    }

    fn default_warheads() -> WarheadRegistry {
        let mut registry = WarheadRegistry::new(2).unwrap();
        let handle = registry.allocate("AP").unwrap();
        registry.by_handle_mut(handle).unwrap().is_wall_destroyer = true;
        registry
    }

    #[test]
    fn constructor_defaults_match_weapon_type_cpp() {
        let weapon = WeaponTypeClass::new(7, "120mm");
        assert_eq!(weapon.id, 7);
        assert_eq!(weapon.name(), "120mm");
        assert_eq!(weapon.burst, 1);
        assert_eq!(weapon.max_speed, MPH_IMMOBILE);
        assert_eq!(weapon.sound, VOC_NONE);
        assert_eq!(weapon.animation, ANIM_NONE);
        assert_eq!(weapon.attack, 0);
        assert_eq!(weapon.range, 0);
        assert_eq!(weapon.rate_of_fire, 0);
        assert!(!weapon.is_turbo_boosted);
        assert!(!weapon.is_supressed);
        assert!(!weapon.is_camera);
        assert!(!weapon.is_electric);
        assert_eq!(weapon.bullet_id, None);
        assert_eq!(weapon.warhead_id, None);
    }

    #[test]
    fn read_ini_sets_weapon_fields_and_resolves_bullet_and_warhead() {
        let mut warheads = default_warheads();
        let mut ini = IniData::default();
        ini.set("120mm", "Supress", "yes");
        ini.set("120mm", "Burst", "2");
        ini.set("120mm", "Damage", "75");
        ini.set("120mm", "Speed", "60");
        ini.set("120mm", "ROF", "50");
        ini.set("120mm", "Range", "500");
        ini.set("120mm", "Report", "9");
        ini.set("120mm", "Anim", "3");
        ini.set("120mm", "Camera", "true");
        ini.set("120mm", "Charges", "false");
        ini.set("120mm", "TurboBoost", "true");
        ini.set("120mm", "Warhead", "0");
        ini.set("120mm", "Projectile", "4");

        let mut registry = WeaponRegistry::new(2).unwrap();
        let handle = registry.allocate("120mm").unwrap();
        assert!(registry.by_handle_mut(handle).unwrap().read_ini(&ini, &warheads));

        let weapon = registry.by_handle(handle).unwrap();
        assert!(weapon.is_supressed);
        assert_eq!(weapon.burst, 2);
        assert_eq!(weapon.attack, 75);
        assert_eq!(weapon.max_speed, 60);
        assert_eq!(weapon.rate_of_fire, 50);
        assert_eq!(weapon.range, 500);
        assert_eq!(weapon.sound, 9);
        assert_eq!(weapon.animation, 3);
        assert!(weapon.is_camera);
        assert!(!weapon.is_electric);
        assert!(weapon.is_turbo_boosted);
        assert_eq!(weapon.warhead_id, Some(0));
        assert_eq!(weapon.bullet_id, Some(4));

        let _ = &mut warheads;
    }

    #[test]
    fn absent_section_does_not_mutate_weapon_defaults() {
        let warheads = default_warheads();
        let ini = IniData::default();
        let mut weapon = WeaponTypeClass::new(0, "none");
        assert!(!weapon.read_ini(&ini, &warheads));
        assert_eq!(weapon.burst, 1);
        assert_eq!(weapon.bullet_id, None);
    }

    #[test]
    fn allowed_threats_combine_air_and_ground_flags() {
        let catalog = BulletCatalog(HashMap::from([
            (1, BulletTypeProfile { is_anti_aircraft: true, is_anti_ground: false }),
            (2, BulletTypeProfile { is_anti_aircraft: false, is_anti_ground: true }),
            (3, BulletTypeProfile { is_anti_aircraft: true, is_anti_ground: true }),
        ]));
        let mut weapon = WeaponTypeClass::new(0, "aa");
        weapon.bullet_id = Some(1);
        assert_eq!(weapon.allowed_threats(&catalog), Some(THREAT_AIR));
        weapon.bullet_id = Some(2);
        assert_eq!(weapon.allowed_threats(&catalog),
            Some(THREAT_INFANTRY | THREAT_VEHICLES | THREAT_BOATS | THREAT_BUILDINGS));
        weapon.bullet_id = Some(3);
        assert_eq!(weapon.allowed_threats(&catalog),
            Some(THREAT_AIR | THREAT_INFANTRY | THREAT_VEHICLES | THREAT_BOATS | THREAT_BUILDINGS));
    }

    #[test]
    fn wall_destroyer_follows_resolved_warhead_class() {
        let mut warheads = WarheadRegistry::new(1).unwrap();
        let handle = warheads.allocate("Wall").unwrap();
        warheads.by_handle_mut(handle).unwrap().is_wall_destroyer = true;
        let mut weapon = WeaponTypeClass::new(0, "WallGun");
        weapon.warhead_id = Some(0);
        assert!(weapon.is_wall_destroyer(&warheads));
        weapon.warhead_id = None;
        assert!(!weapon.is_wall_destroyer(&warheads));
    }

    #[test]
    fn weapon_and_armor_name_lookup_are_case_insensitive() {
        let mut registry = WeaponRegistry::new(3).unwrap();
        let colt = registry.allocate("Colt45").unwrap();
        registry.allocate("MammothTusk").unwrap();
        assert_eq!(registry.as_pointer(0).unwrap().name(), "Colt45");
        assert_eq!(registry.weapon_from_name(Some("cOlT45")), 0);
        assert_eq!(registry.weapon_from_name(Some("missing")), WEAPON_NONE);
        assert_eq!(registry.weapon_from_name(None), WEAPON_NONE);
        assert!(registry.by_handle(colt).is_some());

        let names = ["none", "wood", "aluminum", "steel", "concrete"];
        assert_eq!(armor_from_name(Some("STEEL"), &names), 3);
        assert_eq!(armor_from_name(Some("missing"), &names), ARMOR_NONE);
        assert_eq!(armor_from_name(None, &names), ARMOR_NONE);
    }

    #[test]
    fn delete_clears_pointer_fields_and_registry_slot() {
        let mut registry = WeaponRegistry::new(1).unwrap();
        let handle = registry.allocate("gun").unwrap();
        registry.by_handle_mut(handle).unwrap().bullet_id = Some(3);
        registry.by_handle_mut(handle).unwrap().warhead_id = Some(2);
        registry.by_handle_mut(handle).unwrap().destroy();
        assert_eq!(registry.by_handle(handle).unwrap().name(), "");
        assert_eq!(registry.by_handle(handle).unwrap().bullet_id, None);
        assert_eq!(registry.by_handle(handle).unwrap().warhead_id, None);
        registry.free(handle).unwrap();
        assert_eq!(registry.count(), 0);
    }
}
