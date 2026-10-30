//! Independent AircraftClass routines ported from REDALERT/AIRCRAFT.CPP.
//!
//! Upstream baseline: electronicarts/CnC_Remastered_Collection
//! commit f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae.
//!
//! This is intentionally a partial translation unit. It ports routines whose
//! behavior can be isolated without the full simulation object graph; aircraft
//! missions, radio, combat, movement state machines, rendering, INI and save /
//! load integration remain outstanding and are not claimed as complete.

use crate::facing::DirType;
use crate::fixed16::Fixed16;
use crate::map::Coordinate;
use crate::red_alert_aircraft_types::{AircraftType, AircraftTypeRecord, AIRCRAFT_TYPES, MAP_CELL_W, REFRESH_EOL};

pub const AIRCRAFT_SORT_Y_OFFSET: u32 = 0x0080_0000;

pub const AIRCRAFT_OVERLAP_STANDARD: [i16; 16] = [
    -(MAP_CELL_W - 1), -MAP_CELL_W, -(MAP_CELL_W + 1), -1, 0, 1,
    MAP_CELL_W - 1, MAP_CELL_W, MAP_CELL_W + 1,
    -((MAP_CELL_W * 2) - 1), -(MAP_CELL_W * 2), -((MAP_CELL_W * 2) + 1),
    -((MAP_CELL_W * 3) - 1), -(MAP_CELL_W * 3), -((MAP_CELL_W * 3) + 1),
    REFRESH_EOL,
];

pub const AIRCRAFT_OVERLAP_BADGER: [i16; 26] = [
    -(MAP_CELL_W - 2), -(MAP_CELL_W - 1), -MAP_CELL_W, -(MAP_CELL_W + 1), -(MAP_CELL_W + 2),
    -2, -1, 0, 1, 2,
    MAP_CELL_W - 2, MAP_CELL_W - 1, MAP_CELL_W, MAP_CELL_W + 1, MAP_CELL_W + 2,
    -((MAP_CELL_W * 2) - 2), -((MAP_CELL_W * 2) - 1), -(MAP_CELL_W * 2),
    -((MAP_CELL_W * 2) + 1), -((MAP_CELL_W * 2) + 2),
    -((MAP_CELL_W * 3) - 2), -((MAP_CELL_W * 3) - 1), -(MAP_CELL_W * 3),
    -((MAP_CELL_W * 3) + 1), -((MAP_CELL_W * 3) + 2),
    REFRESH_EOL,
];

/// The source returns the aircraft-specific list while redrawing or airborne.
/// None delegates to AircraftTypeClass::Overlap_List, matching the C++ fallback.
pub fn overlap_offsets(kind: AircraftType, height: i32, redraw: bool) -> Option<&'static [i16]> {
    if !redraw && height == 0 {
        return None;
    }
    Some(if kind == AircraftType::Badger { &AIRCRAFT_OVERLAP_BADGER } else { &AIRCRAFT_OVERLAP_STANDARD })
}

/// Port of AircraftClass::Sort_Y: wrapping 32-bit packed coordinate addition.
pub const fn sort_y(coordinate: Coordinate) -> Coordinate {
    Coordinate(coordinate.0.wrapping_add(AIRCRAFT_SORT_Y_OFFSET))
}

/// Port of AircraftClass::Pose_Dir.
pub fn pose_direction(kind: AircraftType) -> DirType {
    if kind == AircraftType::Transport {
        DirType::NORTH
    } else if AIRCRAFT_TYPES[kind as usize].fixed_wing {
        DirType::EAST
    } else {
        DirType::NORTH_EAST
    }
}

/// Port of AircraftClass::In_Which_Layer's override. False delegates to FootClass.
pub fn uses_top_layer(class: &AircraftTypeRecord, height: i32) -> bool {
    class.fixed_wing && height > 0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AircraftPipState {
    pub kind: AircraftType,
    pub mission_is_hunt: bool,
    pub passenger_count: i32,
    pub max_passengers: i32,
    pub ammo: i32,
    pub max_ammo: i32,
    pub max_pips: i32,
}

/// Port of AircraftClass::Pip_Count, including the Badger/HUNT exception and
/// the legacy fixed-point multiply followed by rounded unsigned conversion.
pub fn pip_count(state: AircraftPipState) -> i32 {
    let carries_passengers = state.max_passengers > 0
        && (state.kind != AircraftType::Badger || !state.mission_is_hunt);
    if carries_passengers {
        return state.passenger_count;
    }
    if state.ammo == 0 {
        return 0;
    }
    let pips = Fixed16::from_ratio(state.ammo, state.max_ammo)
        .mul_integer(state.max_pips)
        .to_unsigned() as i32;
    if pips == 0 { 1 } else { pips }
}

/// The calculated MPH value used by AircraftClass::Set_Speed. The original
/// method also updates FootClass movement speed before calling Fly_Speed.
pub fn effective_flight_speed(
    max_speed: u32,
    speed_bias: Fixed16,
    house_airspeed_bias: Fixed16,
    light_speed_cap: u32,
) -> u32 {
    let class_adjusted = Fixed16::from_unsigned(max_speed)
        .mul_fixed(speed_bias)
        .to_unsigned();
    Fixed16::from_unsigned(class_adjusted)
        .mul_fixed(house_airspeed_bias)
        .to_unsigned()
        .min(light_speed_cap)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvacueeKind { Einstein, General, Delphi, Chan, Tanya, Other }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CivEvacCandidate {
    pub is_infantry: bool,
    pub kind: EvacueeKind,
    pub is_civilian: bool,
    pub is_technician: bool,
}

/// Port of _Counts_As_Civ_Evac. Named mission characters are accepted before
/// civilian/technician checks; Tanya requires the scenario evacuation flag.
pub fn counts_as_civ_evac(candidate: Option<CivEvacCandidate>, scenario_tanya_evac: bool) -> bool {
    let Some(candidate) = candidate else { return false; };
    if !candidate.is_infantry { return false; }
    match candidate.kind {
        EvacueeKind::Einstein | EvacueeKind::General | EvacueeKind::Delphi | EvacueeKind::Chan => return true,
        EvacueeKind::Tanya if scenario_tanya_evac => return true,
        _ => {}
    }
    candidate.is_civilian && !candidate.is_technician
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixed16::Fixed16;

    fn candidate(kind: EvacueeKind, civilian: bool, technician: bool) -> CivEvacCandidate {
        CivEvacCandidate { is_infantry: true, kind, is_civilian: civilian, is_technician: technician }
    }

    #[test]
    fn overlap_lists_match_source_footprints_and_fallback() {
        assert_eq!(AIRCRAFT_OVERLAP_STANDARD.len(), 16);
        assert_eq!(AIRCRAFT_OVERLAP_BADGER.len(), 26);
        assert_eq!(AIRCRAFT_OVERLAP_STANDARD.last(), Some(&REFRESH_EOL));
        assert_eq!(AIRCRAFT_OVERLAP_BADGER.last(), Some(&REFRESH_EOL));
        assert_eq!(overlap_offsets(AircraftType::Mig, 1, false), Some(&AIRCRAFT_OVERLAP_STANDARD[..]));
        assert_eq!(overlap_offsets(AircraftType::Badger, 1, false), Some(&AIRCRAFT_OVERLAP_BADGER[..]));
        assert_eq!(overlap_offsets(AircraftType::Badger, 0, false), None);
        assert_eq!(overlap_offsets(AircraftType::Badger, 0, true), Some(&AIRCRAFT_OVERLAP_BADGER[..]));
    }

    #[test]
    fn sort_y_uses_wrapping_packed_coordinate_addition() {
        assert_eq!(sort_y(Coordinate(0x1234_5678)).0, 0x12B4_5678);
        assert_eq!(sort_y(Coordinate(0xFFFF_FFFF)).0, 0x007F_FFFF);
    }

    #[test]
    fn pose_direction_matches_transport_fixed_wing_and_rotor_rules() {
        assert_eq!(pose_direction(AircraftType::Transport), DirType::NORTH);
        assert_eq!(pose_direction(AircraftType::Mig), DirType::EAST);
        assert_eq!(pose_direction(AircraftType::Longbow), DirType::NORTH_EAST);
    }

    #[test]
    fn fixed_wing_aircraft_use_top_layer_only_while_airborne() {
        assert!(uses_top_layer(&AIRCRAFT_TYPES[AircraftType::Mig as usize], 1));
        assert!(!uses_top_layer(&AIRCRAFT_TYPES[AircraftType::Mig as usize], 0));
        assert!(!uses_top_layer(&AIRCRAFT_TYPES[AircraftType::Longbow as usize], 1));
    }

    #[test]
    fn pip_count_selects_passengers_or_rounded_ammo_pips() {
        let cargo = AircraftPipState {
            kind: AircraftType::Transport, mission_is_hunt: false, passenger_count: 4,
            max_passengers: 5, ammo: 0, max_ammo: 0, max_pips: 5,
        };
        assert_eq!(pip_count(cargo), 4);
        let attack = AircraftPipState {
            kind: AircraftType::Mig, mission_is_hunt: false, passenger_count: 0,
            max_passengers: 0, ammo: 3, max_ammo: 10, max_pips: 5,
        };
        assert_eq!(pip_count(attack), 2);
        assert_eq!(pip_count(AircraftPipState { ammo: 1, ..attack }), 1);
        assert_eq!(pip_count(AircraftPipState { ammo: 0, ..attack }), 0);
    }

    #[test]
    fn badger_hunt_mission_displays_ammo_instead_of_passenger_pips() {
        let state = AircraftPipState {
            kind: AircraftType::Badger, mission_is_hunt: true, passenger_count: 6,
            max_passengers: 8, ammo: 2, max_ammo: 4, max_pips: 5,
        };
        assert_eq!(pip_count(state), 3);
    }

    #[test]
    fn flight_speed_matches_left_associative_fixed_multiply_and_cap() {
        assert_eq!(effective_flight_speed(100, Fixed16::from_ratio(3, 4), Fixed16::from_ratio(2, 1), 255), 150);
        assert_eq!(effective_flight_speed(100, Fixed16::ONE, Fixed16::from_ratio(2, 1), 150), 150);
        assert_eq!(effective_flight_speed(100, Fixed16::ZERO, Fixed16::ONE, 255), 0);
    }

    #[test]
    fn civilian_evacuation_rules_preserve_specials_tanya_and_technician_exclusion() {
        assert!(!counts_as_civ_evac(None, true));
        assert!(!counts_as_civ_evac(Some(CivEvacCandidate {
            is_infantry: false, ..candidate(EvacueeKind::Other, true, false)
        }), true));
        assert!(counts_as_civ_evac(Some(candidate(EvacueeKind::Einstein, false, true)), false));
        assert!(counts_as_civ_evac(Some(candidate(EvacueeKind::General, false, false)), false));
        assert!(counts_as_civ_evac(Some(candidate(EvacueeKind::Tanya, false, false)), true));
        assert!(!counts_as_civ_evac(Some(candidate(EvacueeKind::Tanya, false, false)), false));
        assert!(counts_as_civ_evac(Some(candidate(EvacueeKind::Other, true, false)), false));
        assert!(!counts_as_civ_evac(Some(candidate(EvacueeKind::Other, true, true)), false));
    }
}
