//! Data and source-derived helpers ported from REDALERT/AADATA.CPP.
//!
//! The original file defines seven static AircraftTypeClass records and the
//! type-class helpers. Game-object creation, heap allocation, and SHP loading
//! are represented as deterministic data/requests so engine integration can
//! supply the platform-specific services.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AircraftType {
    Transport = 0,
    Badger = 1,
    U2 = 2,
    Mig = 3,
    Yak = 4,
    Longbow = 5,
    Hind = 6,
}

pub const AIRCRAFT_COUNT: usize = 7;
pub const AIRCRAFT_FIRST: AircraftType = AircraftType::Transport;
pub const REFRESH_EOL: i16 = i16::MAX;
pub const MAP_CELL_W: i16 = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreferredBuilding { None, Airstrip, Helipad }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AircraftTypeRecord {
    pub kind: AircraftType,
    pub text_id: &'static str,
    pub ini_name: &'static str,
    pub vertical_offset: i16,
    pub primary_weapon_offset: i16,
    pub primary_weapon_lateral: i16,
    pub fixed_wing: bool,
    pub rotor_equipped: bool,
    pub rotor_custom: bool,
    pub landable_on_clear: bool,
    pub stealthy: bool,
    pub selectable: bool,
    pub legal_target: bool,
    pub insignificant: bool,
    pub immune: bool,
    pub preferred_building: PreferredBuilding,
    pub landing_speed: u8,
    pub rotation_stages: u8,
    pub default_mission: &'static str,
}

const fn record(
    kind: AircraftType, text_id: &'static str, ini_name: &'static str,
    primary_weapon_offset: i16, primary_weapon_lateral: i16,
    fixed_wing: bool, rotor_equipped: bool, rotor_custom: bool,
    landable_on_clear: bool, selectable: bool,
    preferred_building: PreferredBuilding, landing_speed: u8, rotation_stages: u8,
) -> AircraftTypeRecord {
    AircraftTypeRecord {
        kind, text_id, ini_name, vertical_offset: 0,
        primary_weapon_offset, primary_weapon_lateral,
        fixed_wing, rotor_equipped, rotor_custom, landable_on_clear,
        stealthy: true, selectable, legal_target: true, insignificant: false,
        immune: false, preferred_building, landing_speed, rotation_stages,
        default_mission: "HUNT",
    }
}

/// In the precise order used by AircraftTypeClass::Init_Heap. Heap index is
/// intentionally coupled to the enum/type number in the original engine.
pub const AIRCRAFT_TYPES: [AircraftTypeRecord; AIRCRAFT_COUNT] = [
    record(AircraftType::Transport, "TXT_TRANS", "TRAN", 0, 0, false, true, true, true, true, PreferredBuilding::None, 0xFF, 32),
    record(AircraftType::Badger, "TXT_BADGER", "BADR", 0, 0, true, false, false, false, false, PreferredBuilding::None, 0xFF, 16),
    record(AircraftType::U2, "TXT_U2", "U2", 0, 0, true, false, false, false, false, PreferredBuilding::None, 0xFF, 16),
    record(AircraftType::Mig, "TXT_MIG", "MIG", 0x20, 0x20, true, false, false, false, true, PreferredBuilding::Airstrip, 0xC0, 16),
    record(AircraftType::Yak, "TXT_YAK", "YAK", 0x20, 0x20, true, false, false, false, true, PreferredBuilding::Airstrip, 0xFF, 16),
    record(AircraftType::Longbow, "TXT_HELI", "HELI", 0x40, 0, false, true, false, false, true, PreferredBuilding::Helipad, 0xFF, 32),
    record(AircraftType::Hind, "TXT_ORCA", "HIND", 0x40, 0, false, true, false, false, true, PreferredBuilding::Helipad, 0xFF, 32),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AircraftDimensions { pub width: u16, pub height: u16 }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AircraftAssetPaths {
    pub cameo: String,
    pub image: String,
}

pub fn as_reference(kind: AircraftType) -> &'static AircraftTypeRecord {
    &AIRCRAFT_TYPES[kind as usize]
}

/// Mirrors stricmp: ASCII case-insensitive lookup, null/empty/missing names
/// return None (the Rust equivalent of AIRCRAFT_NONE).
pub fn from_name(name: Option<&str>) -> Option<AircraftType> {
    let name = name?;
    AIRCRAFT_TYPES.iter()
        .find(|record| record.ini_name.eq_ignore_ascii_case(name))
        .map(|record| record.kind)
}

/// Preserves the source's asset naming rule: GRAPHIC_NAME + ICON.SHP and
/// GRAPHIC_NAME.SHP, with extension/path separators normalized by the host.
pub fn asset_paths(graphic_name: &str) -> AircraftAssetPaths {
    AircraftAssetPaths {
        cameo: format!("{graphic_name}ICON.SHP"),
        image: format!("{graphic_name}.SHP"),
    }
}

pub const fn rotor_asset_names() -> (&'static str, &'static str) {
    ("LROTOR.SHP", "RROTOR.SHP")
}

pub const fn dimensions(kind: AircraftType) -> AircraftDimensions {
    match kind {
        AircraftType::Badger => AircraftDimensions { width: 56, height: 56 },
        _ => AircraftDimensions { width: 21, height: 20 },
    }
}

pub const fn occupy_list() -> [i16; 2] { [0, REFRESH_EOL] }

pub const fn overlap_list() -> [i16; 9] {
    [
        -(MAP_CELL_W - 1), -MAP_CELL_W, -(MAP_CELL_W + 1), -1,
        1, MAP_CELL_W - 1, MAP_CELL_W, MAP_CELL_W + 1, REFRESH_EOL,
    ]
}

/// The original refuses to create aircraft through the generic placement API.
pub const fn create_and_place(_cell: u32, _house: u8) -> bool { false }

/// Camera weapons do not display weapon pips; otherwise a weapon-equipped
/// aircraft uses five pips, falling back to its passenger capacity.
pub const fn max_pips(has_primary_weapon: bool, primary_weapon_is_camera: bool, max_passengers: u8) -> u8 {
    if has_primary_weapon && !primary_weapon_is_camera { 5 } else { max_passengers }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_contains_all_seven_types_in_heap_order() {
        assert_eq!(AIRCRAFT_TYPES.len(), 7);
        for (index, record) in AIRCRAFT_TYPES.iter().enumerate() {
            assert_eq!(record.kind as usize, index);
        }
        assert_eq!(AIRCRAFT_TYPES[0].kind, AircraftType::Transport);
        assert_eq!(AIRCRAFT_TYPES[1].kind, AircraftType::Badger);
        assert_eq!(AIRCRAFT_TYPES[6].kind, AircraftType::Hind);
    }

    #[test]
    fn ini_names_are_case_insensitive_and_unknown_names_are_none() {
        assert_eq!(from_name(Some("mIg")), Some(AircraftType::Mig));
        assert_eq!(from_name(Some("TRAN")), Some(AircraftType::Transport));
        assert_eq!(from_name(Some("badger")), None);
        assert_eq!(from_name(None), None);
    }

    #[test]
    fn source_flags_and_aircraft_weapon_offsets_are_preserved() {
        let mig = as_reference(AircraftType::Mig);
        assert_eq!((mig.primary_weapon_offset, mig.primary_weapon_lateral), (0x20, 0x20));
        assert!(mig.selectable);
        assert_eq!(mig.preferred_building, PreferredBuilding::Airstrip);
        let transport = as_reference(AircraftType::Transport);
        assert!(transport.rotor_equipped && transport.rotor_custom && transport.landable_on_clear);
        assert!(!as_reference(AircraftType::Badger).selectable);
    }

    #[test]
    fn dimensions_match_source_special_case_for_badger() {
        assert_eq!(dimensions(AircraftType::Badger), AircraftDimensions { width: 56, height: 56 });
        assert_eq!(dimensions(AircraftType::Hind), AircraftDimensions { width: 21, height: 20 });
    }

    #[test]
    fn source_asset_names_are_deterministic() {
        assert_eq!(asset_paths("MIG"), AircraftAssetPaths { cameo: "MIGICON.SHP".into(), image: "MIG.SHP".into() });
        assert_eq!(rotor_asset_names(), ("LROTOR.SHP", "RROTOR.SHP"));
    }

    #[test]
    fn occupation_overlap_and_create_place_rules_match_source() {
        assert_eq!(occupy_list(), [0, REFRESH_EOL]);
        assert_eq!(overlap_list(), [-127, -128, -129, -1, 1, 127, 128, 129, REFRESH_EOL]);
        assert!(!create_and_place(123, 2));
    }

    #[test]
    fn pip_count_handles_camera_weapon_and_passenger_fallback() {
        assert_eq!(max_pips(true, false, 3), 5);
        assert_eq!(max_pips(true, true, 3), 3);
        assert_eq!(max_pips(false, false, 4), 4);
    }
}
