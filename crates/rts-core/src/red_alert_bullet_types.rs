//! Red Alert bullet-type registry ported from REDALERT/BBDATA.CPP.
//! Upstream baseline: electronicarts/CnC_Remastered_Collection at
//! f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae.
//!
//! Preserves the 18 records in special-heap/enum order, constructor defaults,
//! Read_INI semantics (including the legacy "Inaccuate" spelling and inverted
//! "Rotates" field), safe indexed lookup, and debug/release shape-load planning.
//! Host code performs the actual non-blocking loose-file/archive retrieval.

pub const BULLET_COUNT: usize = 18;
pub const BULLET_FIRST: u8 = 0;
pub const BULLET_RTTI: &str = "RTTI_BULLETTYPE";
pub const BULLET_TEXT_ID: &str = "TXT_NONE";

pub const BULLET_NAMES: [&str; BULLET_COUNT] = [
    "Invisible", "Cannon", "Ack", "Torpedo", "FROG", "HeatSeeker",
    "LaserGuided", "Lobbed", "Bomblet", "Ballistic", "Parachute",
    "Fireball", "LeapDog", "Catapult", "AAMissile", "GPSSatellite",
    "NukeUp", "NukeDown",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BulletTypeId(u8);

impl BulletTypeId {
    pub const fn new(index: u8) -> Option<Self> {
        if (index as usize) < BULLET_COUNT { Some(Self(index)) } else { None }
    }
    pub const fn index(self) -> usize { self.0 as usize }
    pub const fn value(self) -> u8 { self.0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectTypeDefaults {
    pub is_sentient: bool,
    pub is_stealthy: bool,
    pub is_selectable: bool,
    pub is_legal_target: bool,
    pub is_insignificant: bool,
    pub is_immune: bool,
    pub is_footprint: bool,
}

pub const OBJECT_TYPE_DEFAULTS: ObjectTypeDefaults = ObjectTypeDefaults {
    is_sentient: true,
    is_stealthy: true,
    is_selectable: false,
    is_legal_target: false,
    is_insignificant: true,
    is_immune: true,
    is_footprint: false,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BulletTypeRecord {
    pub id: BulletTypeId,
    pub name: &'static str,
    /// Empty means ObjectTypeClass::Graphic_Name falls back to Name().
    pub graphic_name: String,
    pub is_high: bool,
    pub is_shadow: bool,
    pub is_arcing: bool,
    pub is_dropping: bool,
    pub is_invisible: bool,
    pub is_proximity_armed: bool,
    pub is_flame_equipped: bool,
    pub is_fueled: bool,
    pub is_faceless: bool,
    pub is_inaccurate: bool,
    pub is_translucent: bool,
    pub is_anti_aircraft: bool,
    pub is_anti_ground: bool,
    pub is_anti_sub: bool,
    pub is_degenerate: bool,
    pub is_sub_surface: bool,
    pub is_parachuted: bool,
    pub is_gigundo: bool,
    pub rotation: u8,
    pub arming: i32,
    pub tumble: i32,
    pub object_type_defaults: ObjectTypeDefaults,
    /// ImageData starts null in the C++ ObjectTypeClass constructor.
    pub image_loaded: bool,
}

impl BulletTypeRecord {
    fn new(index: usize, name: &'static str) -> Self {
        Self {
            id: BulletTypeId(index as u8),
            name,
            graphic_name: String::new(),
            is_high: false,
            is_shadow: true,
            is_arcing: false,
            is_dropping: false,
            is_invisible: false,
            is_proximity_armed: false,
            is_flame_equipped: false,
            is_fueled: false,
            is_faceless: true,
            is_inaccurate: false,
            is_translucent: false,
            is_anti_aircraft: false,
            is_anti_ground: true,
            is_anti_sub: false,
            is_degenerate: false,
            is_sub_surface: false,
            is_parachuted: false,
            is_gigundo: false,
            rotation: 0,
            arming: 0,
            tumble: 0,
            object_type_defaults: OBJECT_TYPE_DEFAULTS,
            image_loaded: false,
        }
    }

    pub fn graphic_asset_name(&self) -> &str {
        if self.graphic_name.is_empty() { self.name } else { &self.graphic_name }
    }

    pub fn image_filename(&self) -> String {
        format!("{}.SHP", self.graphic_asset_name())
    }

    pub const fn can_create_and_place(&self) -> bool { false }
    pub const fn can_create_one_of(&self) -> bool { false }
}

/// Minimal INI contract used by BulletTypeClass::Read_INI.
pub trait BulletIniReader {
    fn is_present(&self, section: &str) -> bool;
    fn get_int(&self, section: &str, key: &str, default: i32) -> i32;
    fn get_bool(&self, section: &str, key: &str, default: bool) -> bool;
    fn get_string(&self, section: &str, key: &str, default: &str) -> String;
}

/// Fixed-capacity counterpart of BulletTypes special heap. Heap index and
/// BulletType enum number are identical in the upstream engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BulletTypeTable {
    records: [BulletTypeRecord; BULLET_COUNT],
}

impl Default for BulletTypeTable {
    fn default() -> Self { Self::init_heap() }
}

impl BulletTypeTable {
    /// Port of BulletTypeClass::Init_Heap in exact C++ allocation order.
    pub fn init_heap() -> Self {
        Self {
            records: std::array::from_fn(|index| BulletTypeRecord::new(index, BULLET_NAMES[index])),
        }
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub const fn is_empty(&self) -> bool { false }
    pub fn records(&self) -> &[BulletTypeRecord; BULLET_COUNT] { &self.records }

    pub fn as_reference(&self, id: BulletTypeId) -> &BulletTypeRecord {
        &self.records[id.index()]
    }

    pub fn as_reference_mut(&mut self, id: BulletTypeId) -> &mut BulletTypeRecord {
        &mut self.records[id.index()]
    }

    /// Port of BulletTypeClass::Read_INI. Missing sections leave the record
    /// untouched. The legacy spelling "Inaccuate" is intentionally retained.
    pub fn read_ini<I: BulletIniReader>(&mut self, id: BulletTypeId, ini: &I) -> bool {
        let record = self.as_reference_mut(id);
        if !ini.is_present(record.name) { return false; }

        let section = record.name;
        record.arming = ini.get_int(section, "Arm", record.arming);
        // C++ assigns the integer result to an unsigned char, truncating to 8 bits.
        record.rotation = ini.get_int(section, "ROT", i32::from(record.rotation)) as u8;
        record.tumble = ini.get_int(section, "Frames", record.tumble);
        record.is_high = ini.get_bool(section, "High", record.is_high);
        record.is_shadow = ini.get_bool(section, "Shadow", record.is_shadow);
        record.is_arcing = ini.get_bool(section, "Arcing", record.is_arcing);
        record.is_dropping = ini.get_bool(section, "Dropping", record.is_dropping);
        record.is_invisible = ini.get_bool(section, "Inviso", record.is_invisible);
        record.is_proximity_armed = ini.get_bool(section, "Proximity", record.is_proximity_armed);
        record.is_flame_equipped = ini.get_bool(section, "Animates", record.is_flame_equipped);
        record.is_fueled = ini.get_bool(section, "Ranged", record.is_fueled);
        record.is_inaccurate = ini.get_bool(section, "Inaccuate", record.is_inaccurate);
        record.is_anti_aircraft = ini.get_bool(section, "AA", record.is_anti_aircraft);
        record.is_anti_ground = ini.get_bool(section, "AG", record.is_anti_ground);
        record.is_anti_sub = ini.get_bool(section, "ASW", record.is_anti_sub);
        record.is_degenerate = ini.get_bool(section, "Degenerates", record.is_degenerate);
        record.is_sub_surface = ini.get_bool(section, "UnderWater", record.is_sub_surface);
        record.is_parachuted = ini.get_bool(section, "Parachuted", record.is_parachuted);
        record.is_faceless = !ini.get_bool(section, "Rotates", !record.is_faceless);
        record.is_translucent = ini.get_bool(section, "Translucent", record.is_translucent);
        record.is_gigundo = ini.get_bool(section, "Gigundo", record.is_gigundo);
        record.graphic_name = ini.get_string(section, "Image", &record.graphic_name);
        true
    }

    /// Port of BulletTypeClass::One_Time. Invisible records are skipped;
    /// debug uses loose-file-then-archive lookup, release uses archive lookup.
    pub fn plan_one_time_assets(&self, debug_build: bool) -> Vec<BulletAssetRequest> {
        let lookup = if debug_build {
            BulletAssetLookup::LooseFileThenArchive
        } else {
            BulletAssetLookup::ArchiveOnly
        };
        self.records.iter()
            .filter(|record| !record.is_invisible)
            .map(|record| BulletAssetRequest {
                bullet: record.id,
                filename: record.image_filename(),
                lookup,
            })
            .collect()
    }

    /// BulletTypeClass::Init(TheaterType) is an empty inline method in TYPE.H.
    pub fn init_for_theater(&mut self, _theater: u8) {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BulletAssetLookup { LooseFileThenArchive, ArchiveOnly }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BulletAssetRequest {
    pub bullet: BulletTypeId,
    pub filename: String,
    pub lookup: BulletAssetLookup,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[derive(Default)]
    struct Ini {
        sections: HashMap<String, HashMap<String, String>>,
    }

    impl Ini {
        fn with_section(mut self, section: &str, entries: &[(&str, &str)]) -> Self {
            let values = entries.iter()
                .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
                .collect();
            self.sections.insert(section.to_owned(), values);
            self
        }
    }

    impl BulletIniReader for Ini {
        fn is_present(&self, section: &str) -> bool {
            self.sections.contains_key(section)
        }
        fn get_int(&self, section: &str, key: &str, default: i32) -> i32 {
            self.sections.get(section).and_then(|values| values.get(key))
                .and_then(|value| value.parse::<i32>().ok()).unwrap_or(default)
        }
        fn get_bool(&self, section: &str, key: &str, default: bool) -> bool {
            self.sections.get(section).and_then(|values| values.get(key))
                .map(|value| matches!(value.as_str(), "true" | "yes" | "1" | "on"))
                .unwrap_or(default)
        }
        fn get_string(&self, section: &str, key: &str, default: &str) -> String {
            self.sections.get(section).and_then(|values| values.get(key))
                .cloned().unwrap_or_else(|| default.to_owned())
        }
    }

    #[test]
    fn heap_table_preserves_all_18_source_enum_slots() {
        let table = BulletTypeTable::init_heap();
        assert_eq!(table.len(), BULLET_COUNT);
        assert_eq!(table.records().iter().map(|r| r.name).collect::<Vec<_>>(), BULLET_NAMES);
        for (index, record) in table.records().iter().enumerate() {
            assert_eq!(record.id.index(), index);
            assert_eq!(BulletTypeId::new(index as u8), Some(record.id));
        }
        assert_eq!(BulletTypeId::new(BULLET_COUNT as u8), None);
    }

    #[test]
    fn constructor_defaults_match_object_and_bullet_type_initializers() {
        let table = BulletTypeTable::default();
        let bullet = table.as_reference(BulletTypeId::new(0).unwrap());
        assert_eq!(bullet.object_type_defaults, OBJECT_TYPE_DEFAULTS);
        assert!(!bullet.is_high);
        assert!(bullet.is_shadow);
        assert!(!bullet.is_arcing);
        assert!(!bullet.is_invisible);
        assert!(bullet.is_faceless);
        assert!(bullet.is_anti_ground);
        assert!(!bullet.is_anti_aircraft);
        assert_eq!((bullet.rotation, bullet.arming, bullet.tumble), (0, 0, 0));
        assert!(!bullet.image_loaded);
    }

    #[test]
    fn missing_ini_section_returns_false_without_mutating_record() {
        let mut table = BulletTypeTable::default();
        let id = BulletTypeId::new(1).unwrap();
        let before = table.as_reference(id).clone();
        assert!(!table.read_ini(id, &Ini::default()));
        assert_eq!(table.as_reference(id), &before);
    }

    #[test]
    fn read_ini_applies_all_fields_and_preserves_rotates_inversion() {
        let mut table = BulletTypeTable::default();
        let id = BulletTypeId::new(1).unwrap();
        let ini = Ini::default().with_section("Cannon", &[
            ("Arm", "12"), ("ROT", "7"), ("Frames", "4"),
            ("High", "true"), ("Shadow", "false"), ("Arcing", "true"),
            ("Dropping", "true"), ("Inviso", "true"), ("Proximity", "true"),
            ("Animates", "true"), ("Ranged", "true"), ("Inaccuate", "true"),
            ("AA", "true"), ("AG", "false"), ("ASW", "true"),
            ("Degenerates", "true"), ("UnderWater", "true"), ("Parachuted", "true"),
            ("Rotates", "true"), ("Translucent", "true"), ("Gigundo", "true"),
            ("Image", "CUSTOM"),
        ]);
        assert!(table.read_ini(id, &ini));
        let bullet = table.as_reference(id);
        assert_eq!((bullet.arming, bullet.rotation, bullet.tumble), (12, 7, 4));
        assert!(bullet.is_high);
        assert!(!bullet.is_shadow);
        assert!(bullet.is_arcing);
        assert!(bullet.is_dropping);
        assert!(bullet.is_invisible);
        assert!(bullet.is_proximity_armed);
        assert!(bullet.is_flame_equipped);
        assert!(bullet.is_fueled);
        assert!(bullet.is_inaccurate);
        assert!(bullet.is_anti_aircraft);
        assert!(!bullet.is_anti_ground);
        assert!(bullet.is_anti_sub);
        assert!(bullet.is_degenerate);
        assert!(bullet.is_sub_surface);
        assert!(bullet.is_parachuted);
        assert!(!bullet.is_faceless);
        assert!(bullet.is_translucent);
        assert!(bullet.is_gigundo);
        assert_eq!(bullet.graphic_name, "CUSTOM");
    }

    #[test]
    fn rotates_default_and_false_override_preserve_faceless_default() {
        let mut table = BulletTypeTable::default();
        let id = BulletTypeId::new(2).unwrap();
        let ini = Ini::default().with_section("Ack", &[("Rotates", "false")]);
        assert!(table.read_ini(id, &ini));
        assert!(table.as_reference(id).is_faceless);
    }

    #[test]
    fn graphic_name_falls_back_to_object_name_until_overridden() {
        let mut table = BulletTypeTable::default();
        let id = BulletTypeId::new(3).unwrap();
        assert_eq!(table.as_reference(id).image_filename(), "Torpedo.SHP");
        let ini = Ini::default().with_section("Torpedo", &[("Image", "TORPEDO2")]);
        assert!(table.read_ini(id, &ini));
        assert_eq!(table.as_reference(id).image_filename(), "TORPEDO2.SHP");
    }

    #[test]
    fn one_time_assets_skip_invisible_and_use_build_specific_lookup() {
        let mut table = BulletTypeTable::default();
        let id = BulletTypeId::new(0).unwrap();
        let ini = Ini::default().with_section("Invisible", &[("Inviso", "true")]);
        assert!(table.read_ini(id, &ini));
        let debug = table.plan_one_time_assets(true);
        assert_eq!(debug.len(), BULLET_COUNT - 1);
        assert!(debug.iter().all(|r| r.lookup == BulletAssetLookup::LooseFileThenArchive));
        assert!(debug.iter().all(|r| r.filename.ends_with(".SHP")));
        assert!(debug.iter().all(|r| r.bullet != id));
        let release = table.plan_one_time_assets(false);
        assert!(release.iter().all(|r| r.lookup == BulletAssetLookup::ArchiveOnly));
    }

    #[test]
    fn bullet_types_refuse_generic_object_creation() {
        let table = BulletTypeTable::default();
        assert!(table.records().iter().all(|r| !r.can_create_and_place()));
        assert!(table.records().iter().all(|r| !r.can_create_one_of()));
    }

    #[test]
    fn theater_init_is_a_no_op() {
        let mut table = BulletTypeTable::default();
        let before = table.clone();
        table.init_for_theater(3);
        assert_eq!(table, before);
    }
}
