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
use crate::coordinate::Coordinate;
use crate::red_alert_aircraft_types::{AircraftType, AircraftTypeRecord, AIRCRAFT_TYPES, MAP_CELL_W, REFRESH_EOL};

pub const AIRCRAFT_SORT_Y_OFFSET: u32 = 0x0080_0000;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeNumberError {
    BodyShapeIndexOutOfRange { index: usize },
}

/// Port of AircraftClass::Shape_Number. Facing16 and Facing32 are the exact
/// upstream lookup tables; they are injected rather than regenerated, because
/// the 3D-rendered unit silhouettes have a source-defined angular bias.
pub fn shape_number(
    rotation: u8,
    facing: DirType,
    body_shape: &[i32],
    facing16: &[u8; 256],
    facing32: &[u8; 256],
    door_closed: bool,
    door_stage: i32,
) -> Result<i32, ShapeNumberError> {
    let direction = usize::from(facing.0);
    let shape = match rotation {
        32 => body_shape.get(usize::from(facing32[direction]) )
            .copied()
            .ok_or(ShapeNumberError::BodyShapeIndexOutOfRange {
                index: usize::from(facing32[direction]),
            })?,
        16 => {
            let index = usize::from(facing16[direction]) * 2;
            body_shape.get(index).copied()
                .ok_or(ShapeNumberError::BodyShapeIndexOutOfRange { index })? / 2
        }
        8 => {
            let index = usize::from((facing.0 | 0x10) >> 5) * 4;
            body_shape.get(index).copied()
                .ok_or(ShapeNumberError::BodyShapeIndexOutOfRange { index })? / 4
        }
        _ => 0,
    };
    Ok(if door_closed { shape } else { i32::from(rotation) + door_stage })
}


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

/// Candidate sweep order from AircraftClass::Desired_Load_Dir. FacingType is
/// N=0, NE=1, E=2, SE=3, S=4, SW=5, W=6, NW=7.
pub const LOAD_APPROACH_FACING_ORDER: [u8; 8] = [4, 4, 5, 3, 6, 2, 7, 1];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoadApproachResult {
    /// Last candidate assigned to the C++ moveto reference, whether or not a
    /// usable destination was found.
    pub move_to_cell: u32,
    /// The source always returns DIR_N, including when no candidate is usable.
    pub direction: DirType,
    pub found: bool,
}

/// Port of AircraftClass::Desired_Load_Dir with map access injected by the
/// engine. The callbacks retain the source's map-check short-circuit order.
pub fn desired_load_approach<A, R, C>(
    center_cell: u32,
    passenger_cell: u32,
    mut adjacent_cell: A,
    mut in_radar: R,
    mut is_clear_to_move: C,
) -> LoadApproachResult
where
    A: FnMut(u32, u8) -> u32,
    R: FnMut(u32) -> bool,
    C: FnMut(u32) -> bool,
{
    let mut move_to_cell = center_cell;
    for facing in LOAD_APPROACH_FACING_ORDER {
        move_to_cell = adjacent_cell(center_cell, facing);
        if in_radar(move_to_cell)
            && (passenger_cell == move_to_cell || is_clear_to_move(move_to_cell))
        {
            return LoadApproachResult {
                move_to_cell,
                direction: DirType::NORTH,
                found: true,
            };
        }
    }
    LoadApproachResult {
        move_to_cell,
        direction: DirType::NORTH,
        found: false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DestinationAssignmentPlan {
    /// The early dest == NavCom return suppresses even the base-class call.
    pub delegate_to_foot_class: bool,
    /// Whether the source calls Process_Take_Off before delegating.
    pub process_takeoff: bool,
    /// Whether the source clears Status to zero after requesting takeoff.
    pub clear_status: bool,
}

/// Pure decision portion of AircraftClass::Assign_Destination. Actual takeoff
/// and FootClass destination updates remain engine services.
pub const fn destination_assignment_plan(
    destination_is_legal: bool,
    destination_matches_navcom: bool,
    navcom_is_legal: bool,
    fixed_wing: bool,
    is_landing: bool,
) -> DestinationAssignmentPlan {
    if destination_matches_navcom {
        return DestinationAssignmentPlan {
            delegate_to_foot_class: false,
            process_takeoff: false,
            clear_status: false,
        };
    }
    let process_takeoff = destination_is_legal
        && fixed_wing
        && (is_landing || navcom_is_legal);
    DestinationAssignmentPlan {
        delegate_to_foot_class: true,
        process_takeoff,
        clear_status: process_takeoff,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TakeOffPlan {
    pub is_landing: bool,
    pub is_taking_off: bool,
    pub complete: bool,
    pub speed: Option<u8>,
    pub close_door_with_frames: Option<(u8, u8)>,
    pub copy_secondary_to_primary: bool,
    pub primary_desired: Option<DirType>,
    pub secondary_desired: Option<DirType>,
}

/// Pure state-transition plan for AircraftClass::Process_Take_Off.
/// FLIGHT_LEVEL is supplied by the variant's flight constants. Facing and speed
/// updates are returned as actions for the simulation thread to apply.
pub const fn process_take_off(
    fixed_wing: bool,
    height: i32,
    flight_level: i32,
    navcom_direction: DirType,
    primary_desired: DirType,
) -> TakeOffPlan {
    if fixed_wing {
        return TakeOffPlan {
            is_landing: false,
            is_taking_off: true,
            complete: height == flight_level,
            speed: Some(u8::MAX),
            close_door_with_frames: None,
            copy_secondary_to_primary: false,
            primary_desired: None,
            secondary_desired: None,
        };
    }

    if height == 0 {
        TakeOffPlan {
            is_landing: false,
            is_taking_off: true,
            complete: false,
            speed: None,
            close_door_with_frames: Some((5, 4)),
            copy_secondary_to_primary: true,
            primary_desired: None,
            secondary_desired: None,
        }
    } else if height == flight_level / 2 {
        TakeOffPlan {
            is_landing: false,
            is_taking_off: true,
            complete: false,
            speed: None,
            close_door_with_frames: None,
            copy_secondary_to_primary: false,
            primary_desired: Some(navcom_direction),
            secondary_desired: None,
        }
    } else if height == flight_level - flight_level / 3 {
        TakeOffPlan {
            is_landing: false,
            is_taking_off: true,
            complete: false,
            speed: Some(0x20),
            close_door_with_frames: None,
            copy_secondary_to_primary: false,
            primary_desired: None,
            secondary_desired: Some(primary_desired),
        }
    } else if height == flight_level - flight_level / 5 {
        TakeOffPlan {
            is_landing: false,
            is_taking_off: true,
            complete: false,
            speed: Some(0x40),
            close_door_with_frames: None,
            copy_secondary_to_primary: false,
            primary_desired: None,
            secondary_desired: None,
        }
    } else if height == flight_level {
        TakeOffPlan {
            is_landing: false,
            is_taking_off: false,
            complete: true,
            speed: Some(u8::MAX),
            close_door_with_frames: None,
            copy_secondary_to_primary: false,
            primary_desired: None,
            secondary_desired: None,
        }
    } else {
        TakeOffPlan {
            is_landing: false,
            is_taking_off: true,
            complete: false,
            speed: None,
            close_door_with_frames: None,
            copy_secondary_to_primary: false,
            primary_desired: None,
            secondary_desired: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LandingPlan {
    pub is_landing: bool,
    pub is_taking_off: bool,
    pub complete: bool,
    pub speed: Option<u32>,
    pub secondary_desired: Option<DirType>,
}

/// Pure state-transition plan for AircraftClass::Process_Landing. The caller
/// supplies distance and the already-computed direction toward NavCom.
pub fn process_landing(
    fixed_wing: bool,
    height: i32,
    flight_level: i32,
    distance_to_navcom: i32,
    direction_to_navcom: DirType,
    landing_speed: u32,
    house_airspeed_bias: Fixed16,
) -> LandingPlan {
    if fixed_wing {
        let secondary_desired = (distance_to_navcom > 0x0100).then_some(direction_to_navcom);
        if height == 0 {
            return LandingPlan {
                is_landing: false,
                is_taking_off: false,
                complete: true,
                speed: Some(0),
                secondary_desired,
            };
        }
        let speed = Fixed16::from_unsigned(landing_speed)
            .div_fixed(house_airspeed_bias)
            .to_unsigned();
        LandingPlan {
            is_landing: true,
            is_taking_off: false,
            complete: false,
            speed: Some(speed),
            secondary_desired,
        }
    } else if height == 0 {
        LandingPlan {
            is_landing: false,
            is_taking_off: false,
            complete: true,
            speed: None,
            secondary_desired: None,
        }
    } else if height == flight_level / 2 {
        LandingPlan {
            is_landing: true,
            is_taking_off: false,
            complete: false,
            speed: Some(0),
            secondary_desired: None,
        }
    } else {
        LandingPlan {
            is_landing: true,
            is_taking_off: false,
            complete: false,
            speed: None,
            secondary_desired: None,
        }
    }
}

/// Port of AircraftClass::Fire_Direction: fire from the current body facing.
pub const fn fire_direction(secondary_facing: DirType) -> DirType {
    secondary_facing
}

/// Port of AircraftClass::Look's sight-range selection. Grounded aircraft
/// always request a one-cell reveal, regardless of their configured sight.
pub fn look_sight_range(configured_sight: i32, height: i32) -> Option<i32> {
    let range = if height == 0 { 1 } else { configured_sight };
    (range != 0).then_some(range)
}

/// Port of AircraftClass::Spied_By. A landed aircraft inherits its contacted
/// helipad's spy flags only while physically in radio contact with that exact
/// building; airborne and non-contacting aircraft keep FootClass flags.
pub const fn spied_by_flags(
    foot_flags: u32,
    height: i32,
    in_radio_contact: bool,
    current_building_is_contact: bool,
    building_flags: u32,
) -> u32 {
    if height == 0 && in_radio_contact && current_building_is_contact {
        foot_flags | building_flags
    } else {
        foot_flags
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn candidate(kind: EvacueeKind, civilian: bool, technician: bool) -> CivEvacCandidate {
        CivEvacCandidate { is_infantry: true, kind, is_civilian: civilian, is_technician: technician }
    }

    #[test]
    fn fire_direction_is_the_secondary_body_facing() {
        let facing = DirType(173);
        assert_eq!(fire_direction(facing), facing);
    }

    #[test]
    fn grounded_aircraft_always_request_one_cell_of_sight() {
        assert_eq!(look_sight_range(7, 0), Some(1));
        assert_eq!(look_sight_range(0, 0), Some(1));
        assert_eq!(look_sight_range(7, 1), Some(7));
        assert_eq!(look_sight_range(0, 1), None);
    }

    #[test]
    fn spy_flags_are_inherited_only_from_the_contacted_landed_helipad() {
        assert_eq!(spied_by_flags(0x03, 0, true, true, 0x08), 0x0B);
        assert_eq!(spied_by_flags(0x03, 1, true, true, 0x08), 0x03);
        assert_eq!(spied_by_flags(0x03, 0, false, true, 0x08), 0x03);
        assert_eq!(spied_by_flags(0x03, 0, true, false, 0x08), 0x03);
    }

    #[test]
    fn desired_load_approach_preserves_exact_sweep_order_and_short_circuiting() {
        assert_eq!(LOAD_APPROACH_FACING_ORDER, [4, 4, 5, 3, 6, 2, 7, 1]);
        let mut observed = Vec::new();
        let result = desired_load_approach(
            100,
            77,
            |center, facing| {
                observed.push(facing);
                center + u32::from(facing)
            },
            |cell| cell < 200,
            |_| true,
        );
        assert!(result.found);
        assert_eq!(result.move_to_cell, 104);
        assert_eq!(result.direction, DirType::NORTH);
        assert_eq!(observed, [4]);
    }

    #[test]
    fn desired_load_approach_tests_passenger_cell_only_after_radar_check() {
        let mut clear_checks = 0;
        let result = desired_load_approach(
            100,
            104,
            |center, facing| center + u32::from(facing),
            |cell| cell == 104,
            |_| { clear_checks += 1; true },
        );
        assert!(result.found);
        assert_eq!(result.move_to_cell, 104);
        assert_eq!(clear_checks, 0);
    }

    #[test]
    fn failed_load_approach_leaves_last_candidate_and_returns_north() {
        let mut observed = Vec::new();
        let result = desired_load_approach(
            100,
            999,
            |center, facing| { observed.push(facing); center + u32::from(facing) },
            |_| false,
            |_| panic!("clear-to-move must short-circuit when outside radar"),
        );
        assert!(!result.found);
        assert_eq!(observed, LOAD_APPROACH_FACING_ORDER);
        assert_eq!(result.move_to_cell, 101);
        assert_eq!(result.direction, DirType::NORTH);
    }

    #[test]
    fn destination_assignment_obeys_early_return_and_takeoff_rules() {
        let unchanged = destination_assignment_plan(true, true, true, true, true);
        assert!(!unchanged.delegate_to_foot_class);
        assert!(!unchanged.process_takeoff);

        let fixed_wing = destination_assignment_plan(true, false, true, true, false);
        assert!(fixed_wing.delegate_to_foot_class);
        assert!(fixed_wing.process_takeoff);
        assert!(fixed_wing.clear_status);

        let landing_rotor = destination_assignment_plan(true, false, false, false, true);
        assert!(landing_rotor.delegate_to_foot_class);
        assert!(!landing_rotor.process_takeoff);

        let invalid = destination_assignment_plan(false, false, true, true, true);
        assert!(!invalid.process_takeoff);
    }

    #[test]
    fn fixed_wing_takeoff_sets_full_speed_and_completes_at_flight_level() {
        let before = process_take_off(true, 100, 256, DirType::EAST, DirType::NORTH);
        assert_eq!(before.speed, Some(255));
        assert!(!before.complete);
        assert!(before.is_taking_off);
        let at_level = process_take_off(true, 256, 256, DirType::EAST, DirType::NORTH);
        assert!(at_level.complete);
        assert!(at_level.is_taking_off); // matches the source's fixed-wing branch.
    }

    #[test]
    fn helicopter_takeoff_emits_exact_height_stage_actions() {
        let ground = process_take_off(false, 0, 256, DirType::EAST, DirType::NORTH_WEST);
        assert_eq!(ground.close_door_with_frames, Some((5, 4)));
        assert!(ground.copy_secondary_to_primary);
        let half = process_take_off(false, 128, 256, DirType::EAST, DirType::NORTH_WEST);
        assert_eq!(half.primary_desired, Some(DirType::EAST));
        let two_thirds = process_take_off(false, 171, 256, DirType::EAST, DirType::NORTH_WEST);
        assert_eq!(two_thirds.speed, Some(0x20));
        assert_eq!(two_thirds.secondary_desired, Some(DirType::NORTH_WEST));
        let four_fifths = process_take_off(false, 205, 256, DirType::EAST, DirType::NORTH_WEST);
        assert_eq!(four_fifths.speed, Some(0x40));
        let top = process_take_off(false, 256, 256, DirType::EAST, DirType::NORTH_WEST);
        assert_eq!(top.speed, Some(255));
        assert!(top.complete);
        assert!(!top.is_taking_off);
    }

    #[test]
    fn fixed_wing_landing_turns_only_when_far_and_applies_bias_speed() {
        let far = process_landing(
            true, 100, 256, 0x0200, DirType::SOUTH_EAST, 80, Fixed16::from_ratio(2, 1),
        );
        assert_eq!(far.secondary_desired, Some(DirType::SOUTH_EAST));
        assert_eq!(far.speed, Some(40));
        assert!(far.is_landing);
        let near = process_landing(
            true, 100, 256, 0x0100, DirType::EAST, 80, Fixed16::ONE,
        );
        assert_eq!(near.secondary_desired, None);
    }

    #[test]
    fn landing_completion_and_helicopter_midpoint_speed_match_source() {
        let fixed_landed = process_landing(
            true, 0, 256, 0, DirType::NORTH, 80, Fixed16::ONE,
        );
        assert!(fixed_landed.complete);
        assert_eq!(fixed_landed.speed, Some(0));
        let rotor_half = process_landing(
            false, 128, 256, 0, DirType::NORTH, 80, Fixed16::ONE,
        );
        assert!(!rotor_half.complete);
        assert_eq!(rotor_half.speed, Some(0));
        let rotor_ground = process_landing(
            false, 0, 256, 0, DirType::NORTH, 80, Fixed16::ONE,
        );
        assert!(rotor_ground.complete);
        assert_eq!(rotor_ground.speed, None);
    }

    #[test]
    fn shape_number_uses_upstream_rotation_tables_and_door_override() {
        let mut body = [0_i32; 256];
        for (index, slot) in body.iter_mut().enumerate() { *slot = index as i32 * 4; }
        let mut facing16 = [0_u8; 256];
        let mut facing32 = [0_u8; 256];
        facing16[64] = 3;
        facing32[64] = 9;
        assert_eq!(shape_number(32, DirType::EAST, &body, &facing16, &facing32, true, 0), Ok(36));
        assert_eq!(shape_number(16, DirType::EAST, &body, &facing16, &facing32, true, 0), Ok(12));
        // Dir_To_8 is ((unsigned char)(direction | 0x10)) >> 5.
        assert_eq!(shape_number(8, DirType::EAST, &body, &facing16, &facing32, true, 0), Ok(8));
        assert_eq!(shape_number(32, DirType::EAST, &body, &facing16, &facing32, false, 3), Ok(35));
        assert_eq!(shape_number(24, DirType::EAST, &body, &facing16, &facing32, true, 0), Ok(0));
    }

    #[test]
    fn shape_number_reports_short_source_lookup_tables_instead_of_panicking() {
        let body = [0_i32; 8];
        let facing16 = [0_u8; 256];
        let mut facing32 = [0_u8; 256];
        facing32[0] = 9;
        assert_eq!(
            shape_number(32, DirType::NORTH, &body, &facing16, &facing32, true, 0),
            Err(ShapeNumberError::BodyShapeIndexOutOfRange { index: 9 })
        );
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
