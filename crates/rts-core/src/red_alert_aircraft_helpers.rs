//! Independent AircraftClass routines ported from REDALERT/AIRCRAFT.CPP.
//!
//! Upstream baseline: electronicarts/CnC_Remastered_Collection
//! commit f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae.
//!
//! This is intentionally a partial translation unit. It ports routines whose
//! behavior can be isolated without the full simulation object graph; aircraft
//! missions, radio, combat, movement state machines, rendering, INI and save /
//! load integration remain outstanding and are not claimed as complete.

use crate::facing::{DirType, FacingClass};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AircraftCellOccupier {
    None,
    NonTechno,
    Techno {
        allied_to_aircraft: bool,
        cloaked_from_aircraft: bool,
        is_building: bool,
        building_is_invisible: bool,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AircraftMoveResult {
    No,
    Ok,
}

/// Port of AircraftClass::Can_Enter_Cell. The clearance callback represents
/// CellClass::Is_Clear_To_Move(SPEED_TRACK, false, false) and is invoked only
/// under the exact occupant-dependent conditions used by the source.
pub fn can_enter_cell<F>(
    in_radar: bool,
    occupier: AircraftCellOccupier,
    scenario_init: i32,
    game_is_normal: bool,
    aircraft_is_player_owned: bool,
    cell_is_mapped: bool,
    is_clear_to_move: F,
) -> AircraftMoveResult
where
    F: FnOnce() -> bool,
{
    if !in_radar {
        return AircraftMoveResult::No;
    }

    let check_cell_clearance = match occupier {
        AircraftCellOccupier::None | AircraftCellOccupier::NonTechno => true,
        AircraftCellOccupier::Techno {
            allied_to_aircraft,
            cloaked_from_aircraft,
            is_building,
            building_is_invisible,
        } => {
            allied_to_aircraft
                || (!cloaked_from_aircraft
                    && scenario_init == 0
                    && (!is_building || !building_is_invisible))
        }
    };
    if check_cell_clearance && !is_clear_to_move() {
        return AircraftMoveResult::No;
    }
    if game_is_normal && aircraft_is_player_owned && !cell_is_mapped {
        return AircraftMoveResult::No;
    }
    AircraftMoveResult::Ok
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AircraftTrafficEntry {
    pub id: u32,
    pub present: bool,
    pub in_limbo: bool,
    pub current_cell: u32,
    /// Decoded cell target corresponding to NavCom. Other target kinds should
    /// be represented as None by the engine adapter.
    pub navcom_cell: Option<u32>,
}

/// Port of AircraftClass::Cell_Seems_Ok. Strict mode includes this aircraft in
/// the collision scan; ordinary mode ignores only the current aircraft ID.
pub fn cell_seems_ok(
    cell: u32,
    self_id: u32,
    strict: bool,
    aircraft: &[AircraftTrafficEntry],
) -> bool {
    !aircraft.iter().any(|entry| {
        entry.present
            && (strict || entry.id != self_id)
            && !entry.in_limbo
            && (entry.current_cell == cell || entry.navcom_cell == Some(cell))
    })
}

/// Callback-driven port of AircraftClass::Good_Fire_Location.
/// Returns a candidate cell rather than constructing the C++ TARGET encoding;
/// the engine wrapper can convert the selected cell to the appropriate target.
pub fn good_fire_location<M, C, E, R, V, O, D, P>(
    target_is_legal: bool,
    target_coordinate: u32,
    aircraft_coordinate: u32,
    moving_target_nav_coordinate: Option<u32>,
    weapon_range: i32,
    flight_level_leptons: i32,
    game_is_normal: bool,
    mut coordinate_move: M,
    mut coordinate_to_cell: C,
    mut subtract_flight_height: E,
    mut in_radar: R,
    mut is_visible: V,
    mut cell_seems_ok_strict: O,
    mut distance: D,
    mut choose_closest: P,
) -> Option<u32>
where
    M: FnMut(u32, DirType, i32) -> u32,
    C: FnMut(u32) -> u32,
    E: FnMut(u32, i32) -> u32,
    R: FnMut(u32) -> bool,
    V: FnMut(u32) -> bool,
    O: FnMut(u32) -> bool,
    D: FnMut(u32, Option<u32>, u32) -> i32,
    P: FnMut(u8) -> bool,
{
    if !target_is_legal {
        return None;
    }

    let mut best: Option<(u32, i32)> = None;
    let mut second_best: Option<(u32, i32)> = None;
    let mut radius = weapon_range - 0x0100;
    while radius > 0x0100 {
        for facing in (0_u16..255).step_by(16) {
            let coordinate = coordinate_move(
                target_coordinate,
                DirType(facing as u8),
                radius,
            );
            let cell = coordinate_to_cell(coordinate);
            let actual_coordinate = subtract_flight_height(coordinate, flight_level_leptons);
            let actual_cell = coordinate_to_cell(actual_coordinate);
            if in_radar(actual_cell)
                && (!game_is_normal || is_visible(cell))
                && cell_seems_ok_strict(cell)
            {
                let range_origin = moving_target_nav_coordinate.unwrap_or(aircraft_coordinate);
                let candidate_distance = distance(coordinate, moving_target_nav_coordinate, range_origin);
                if best.is_none_or(|(_, best_distance)| candidate_distance < best_distance) {
                    second_best = best;
                    best = Some((cell, candidate_distance));
                }
            }
        }
        if best.is_some() {
            break;
        }
        radius -= 0x0100;
    }

    let (best_cell, _) = best?;
    let second_cell = second_best.map_or(best_cell, |(cell, _)| cell);
    if choose_closest(50) {
        Some(best_cell)
    } else {
        Some(second_cell)
    }
}

/// Callback-driven port of AircraftClass::New_LZ's concentric-ring scan.
/// Returns a replacement cell only when the previous target was legal but no
/// longer usable and a valid new landing cell is found. None means preserve the
/// original target exactly as the C++ routine does.
pub fn find_new_landing_zone<M, C, R, L, A>(
    old_target_is_legal: bool,
    old_location_is_usable: bool,
    center_coordinate: u32,
    scan_radius_leptons: i32,
    cell_leptons: i32,
    icon_leptons: i32,
    mut random_modifier: R,
    mut coordinate_move: M,
    mut coordinate_to_cell: C,
    mut in_radar: L,
    mut landing_zone_is_clear: A,
    mut cell_seems_ok: impl FnMut(u32) -> bool,
) -> Option<u32>
where
    M: FnMut(u32, DirType, i32) -> u32,
    C: FnMut(u32) -> u32,
    R: FnMut() -> u8,
    L: FnMut(u32) -> bool,
    A: FnMut(u32) -> bool,
{
    if !old_target_is_legal || old_location_is_usable || cell_leptons <= 0 {
        return None;
    }
    let radius_count = scan_radius_leptons / cell_leptons;
    for radius in 0..radius_count {
        let modifier = random_modifier();
        let distance = radius * icon_leptons;
        let mut last_cell = None;
        for facing in 0_u8..8 {
            // Facing_Dir(FacingType(face + modifier)) casts the shifted
            // integer back to the byte-sized DirType, preserving wraparound.
            let direction = DirType((u16::from(facing + modifier) << 5) as u8);
            let moved = coordinate_move(center_coordinate, direction, distance);
            let new_cell = coordinate_to_cell(moved);
            if in_radar(new_cell) {
                if last_cell != Some(new_cell)
                    && landing_zone_is_clear(new_cell)
                    && cell_seems_ok(new_cell)
                {
                    return Some(new_cell);
                }
                last_cell = Some(new_cell);
            }
        }
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LandingCellOccupier {
    Empty,
    ThisAircraft,
    ContactedObject,
    OtherObject,
}

/// Port of AircraftClass::Is_LZ_Clear. The movement-clearance callback is used
/// only for empty cells; occupied cells are accepted only for self or the
/// exact object currently in radio contact.
pub fn landing_zone_is_clear<F>(
    target_is_legal: bool,
    cell_in_radar: bool,
    occupier: LandingCellOccupier,
    is_clear_to_move: F,
) -> bool
where
    F: FnOnce() -> bool,
{
    if !target_is_legal || !cell_in_radar {
        return false;
    }
    match occupier {
        LandingCellOccupier::ThisAircraft | LandingCellOccupier::ContactedObject => true,
        LandingCellOccupier::OtherObject => false,
        LandingCellOccupier::Empty => is_clear_to_move(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LandingBuildingCandidate {
    pub present: bool,
    pub in_limbo: bool,
    pub owned_by_aircraft_house: bool,
    pub cell: u32,
    pub distance: i32,
    pub is_preferred_support_building: bool,
}

/// Port of AircraftClass::Good_LZ. The engine supplies the ordered building
/// scan and source-computed distance; preferred support buildings have their
/// distance divided by four, with first-in-list tie behavior preserved.
pub fn good_landing_zone(current_cell: u32, buildings: &[LandingBuildingCandidate]) -> u32 {
    let mut best_cell = current_cell;
    let mut best_distance: Option<i32> = None;
    for building in buildings {
        if !building.present || building.in_limbo || !building.owned_by_aircraft_house {
            continue;
        }
        let distance = if building.is_preferred_support_building {
            building.distance / 4
        } else {
            building.distance
        };
        if best_distance.is_none_or(|best| distance < best) {
            best_distance = Some(distance);
            best_cell = building.cell;
        }
    }
    best_cell
}

/// The adjacent-cell directions used by AircraftClass::Exit_Object, indexed
/// by its clockwise FacingType scan order N through NW.
pub const PASSENGER_EXIT_FACING_ORDER: [u8; 8] = [4, 5, 3, 7, 1, 0, 6, 2];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExitObjectPlan {
    pub candidate_cell: u32,
    pub spawn_coordinate: u32,
    pub facing: DirType,
    pub assign_move_mission: bool,
    pub assign_candidate_destination: bool,
    pub send_unload_after_radio_roger: bool,
    pub look_incremental: bool,
}

/// Port of AircraftClass::Exit_Object as an action plan. The search order,
/// spawn coordinate, direction, and radio handshake are preserved. No valid
/// cell or failed Unlimbo returns None instead of indexing past the source's
/// eight-entry direction array.
pub fn plan_passenger_exit<C, U>(
    aircraft_cell: u32,
    aircraft_coordinate: u32,
    adjacent_cell: C,
    mut can_enter_cell: impl FnMut(u32) -> bool,
    unlimbo: U,
    radio_hello_was_roger: bool,
) -> Option<ExitObjectPlan>
where
    C: Fn(u32, u8) -> u32,
    U: FnOnce(u32, DirType) -> bool,
{
    let mut selected = None;
    for facing in PASSENGER_EXIT_FACING_ORDER {
        let cell = adjacent_cell(aircraft_cell, facing);
        if can_enter_cell(cell) {
            selected = Some((cell, facing));
            break;
        }
    }
    let (candidate_cell, facing) = selected?;
    let direction = DirType((u16::from(facing) << 5) as u8);
    if !unlimbo(aircraft_coordinate, direction) {
        return None;
    }
    Some(ExitObjectPlan {
        candidate_cell,
        spawn_coordinate: aircraft_coordinate,
        facing: direction,
        assign_move_mission: true,
        assign_candidate_destination: true,
        send_unload_after_radio_roger: radio_hello_was_roger,
        look_incremental: false,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvacuatingPassenger {
    pub passenger_id: u32,
    pub house_id: u32,
    pub counts_as_civ_evacuee: bool,
    pub team_id: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeOfWorldPlan {
    pub lock_aircraft: bool,
    pub mark_civ_evacuation_for_houses: Vec<u32>,
    pub mark_teams_to_leave: Vec<u32>,
    pub delete_passengers: Vec<u32>,
    pub mark_aircraft_team_to_leave: bool,
    pub stun_aircraft: bool,
    pub delete_aircraft: bool,
}

/// Port of AircraftClass::Edge_Of_World_AI. Retreating aircraft that leave the
/// radar map release all attached passengers, trigger eligible civilian
/// evacuation flags, mark passenger/aircraft teams as leaving, then stun and
/// delete themselves. Aircraft still on-map are locked in place.
pub fn edge_of_world_plan(
    aircraft_is_in_radar: bool,
    mission_is_retreat: bool,
    aircraft_has_team: bool,
    passengers: &[EvacuatingPassenger],
) -> EdgeOfWorldPlan {
    if aircraft_is_in_radar {
        return EdgeOfWorldPlan {
            lock_aircraft: true,
            mark_civ_evacuation_for_houses: Vec::new(),
            mark_teams_to_leave: Vec::new(),
            delete_passengers: Vec::new(),
            mark_aircraft_team_to_leave: false,
            stun_aircraft: false,
            delete_aircraft: false,
        };
    }
    if !mission_is_retreat {
        return EdgeOfWorldPlan {
            lock_aircraft: false,
            mark_civ_evacuation_for_houses: Vec::new(),
            mark_teams_to_leave: Vec::new(),
            delete_passengers: Vec::new(),
            mark_aircraft_team_to_leave: false,
            stun_aircraft: false,
            delete_aircraft: false,
        };
    }
    let mut plan = EdgeOfWorldPlan {
        lock_aircraft: false,
        mark_civ_evacuation_for_houses: Vec::new(),
        mark_teams_to_leave: Vec::new(),
        delete_passengers: Vec::with_capacity(passengers.len()),
        mark_aircraft_team_to_leave: aircraft_has_team,
        stun_aircraft: true,
        delete_aircraft: true,
    };
    for passenger in passengers {
        if passenger.counts_as_civ_evacuee {
            plan.mark_civ_evacuation_for_houses.push(passenger.house_id);
        }
        if let Some(team_id) = passenger.team_id {
            plan.mark_teams_to_leave.push(team_id);
        }
        plan.delete_passengers.push(passenger.passenger_id);
    }
    plan
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RotationAiResult {
    /// Number of source Mark(MARK_CHANGE_REDRAW) calls for each facing.
    pub primary_redraw_marks: u8,
    pub secondary_redraw_marks: u8,
}

/// Port of AircraftClass::Rotation_AI, including the source's fixed-wing copy
/// of PrimaryFacing before SecondaryFacing is independently advanced.
pub fn rotation_ai(
    primary: &mut FacingClass,
    secondary: &mut FacingClass,
    is_fixed_wing: bool,
    rotation_rate: i32,
) -> RotationAiResult {
    let mut result = RotationAiResult::default();
    if primary.is_rotating() {
        result.primary_redraw_marks += 1;
        if primary.rotation_adjust(rotation_rate) {
            result.primary_redraw_marks += 1;
        }
    }
    if is_fixed_wing {
        *secondary = *primary;
    }
    if secondary.is_rotating() {
        result.secondary_redraw_marks += 1;
        if secondary.rotation_adjust(rotation_rate) {
            result.secondary_redraw_marks += 1;
        }
    }
    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AircraftFireCheck {
    Ok,
    Ammo,
    Rearm,
    Range,
    Facing,
    Other(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AircraftFireRequest {
    pub is_passenger_aircraft: bool,
    pub has_attached_passenger: bool,
    pub has_primary_weapon: bool,
    pub primary_weapon_is_camera: bool,
    pub primary_bullet_is_parachuted: bool,
    pub has_ammo: bool,
    pub arm_timer: i32,
    pub distance_to_target: i32,
    pub aircraft_center_is_in_radar: bool,
    pub is_fixed_wing: bool,
    /// Signed byte-angle difference from the aircraft's primary facing to the
    /// target direction, equivalent to PrimaryFacing.Difference(Direction(TarCom)).
    pub facing_difference: i32,
}

/// Port of AircraftClass::Can_Fire. The base-class check is lazy and invoked
/// only for ordinary weapons; camera/parachute cargo attacks use their special
/// arm/range/radar rules before that path.
pub fn can_fire_aircraft<F>(
    request: AircraftFireRequest,
    base_can_fire: F,
) -> AircraftFireCheck
where
    F: FnOnce() -> AircraftFireCheck,
{
    if request.is_passenger_aircraft && !request.has_attached_passenger {
        return AircraftFireCheck::Ammo;
    }
    let fudge = request.is_passenger_aircraft || (
        request.has_primary_weapon && request.primary_bullet_is_parachuted
    );
    if fudge
        && !request.primary_weapon_is_camera
        && !request.has_ammo
        && !request.is_passenger_aircraft
    {
        return AircraftFireCheck::Ammo;
    }
    if request.primary_weapon_is_camera
        || (fudge && request.is_passenger_aircraft && request.has_attached_passenger)
    {
        if request.arm_timer != 0 {
            return AircraftFireCheck::Rearm;
        }
        let range = if request.primary_weapon_is_camera { 0x0380 } else { 0x0200 };
        if request.distance_to_target < range && request.aircraft_center_is_in_radar {
            return AircraftFireCheck::Ok;
        }
        return AircraftFireCheck::Range;
    }

    let base_result = base_can_fire();
    if base_result == AircraftFireCheck::Ok && request.is_fixed_wing {
        let facing_limit = if fudge { 16 } else { 8 };
        if request.facing_difference.abs() > facing_limit {
            return AircraftFireCheck::Facing;
        }
    }
    base_result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParadropCargoPlan {
    pub reattach_passenger: bool,
    pub play_parachute_sound: bool,
    pub remove_passenger_from_team: bool,
    pub assign_passenger_mission: Option<&'static str>,
    pub arm_after_call: i32,
}

/// Port of AircraftClass::Paradrop_Cargo. A failed parachute attempt restores
/// the passenger and retains the rearm timer; success clears Arm and only
/// changes passenger mission/team state when the aircraft belongs to a team.
pub const fn paradrop_cargo_plan(
    passenger_present: bool,
    paradrop_succeeded: bool,
    aircraft_has_team: bool,
    passenger_house_is_human: bool,
    current_arm: i32,
) -> ParadropCargoPlan {
    if !passenger_present {
        return ParadropCargoPlan {
            reattach_passenger: false,
            play_parachute_sound: false,
            remove_passenger_from_team: false,
            assign_passenger_mission: None,
            arm_after_call: current_arm,
        };
    }
    if !paradrop_succeeded {
        return ParadropCargoPlan {
            reattach_passenger: true,
            play_parachute_sound: false,
            remove_passenger_from_team: false,
            assign_passenger_mission: None,
            arm_after_call: current_arm,
        };
    }
    ParadropCargoPlan {
        reattach_passenger: false,
        play_parachute_sound: true,
        remove_passenger_from_team: aircraft_has_team,
        assign_passenger_mission: if aircraft_has_team {
            Some(if passenger_house_is_human { "GUARD" } else { "HUNT" })
        } else {
            None
        },
        arm_after_call: 0,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AircraftVoicePrompt {
    Attack,
    Move,
    Select,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AircraftVoicePlan {
    pub sample: &'static str,
    pub should_play: bool,
    pub volume: Fixed16,
    pub speaker_id: i32,
}

/// Source response voice tables. Selection index is supplied from the
/// variant's Sim_Random_Pick call; disabled voice still consumes that choice.
pub fn aircraft_voice_plan(
    prompt: AircraftVoicePrompt,
    selection_index: usize,
    allow_voice: bool,
    aircraft_id: i32,
) -> Option<AircraftVoicePlan> {
    const ATTACK: [&str; 2] = ["VOC_AFFIRM", "VOC_ACKNOWL"];
    const MOVE: [&str; 2] = ["VOC_ACKNOWL", "VOC_AFFIRM"];
    const SELECT: [&str; 6] = [
        "VOC_VEHIC", "VOC_REPORT", "VOC_YESSIR",
        "VOC_YESSIR", "VOC_YESSIR", "VOC_AWAIT",
    ];
    let sample = match prompt {
        AircraftVoicePrompt::Attack => *ATTACK.get(selection_index)?,
        AircraftVoicePrompt::Move => *MOVE.get(selection_index)?,
        AircraftVoicePrompt::Select => *SELECT.get(selection_index)?,
    };
    Some(AircraftVoicePlan {
        sample,
        should_play: allow_voice,
        volume: Fixed16::ONE,
        speaker_id: aircraft_id.wrapping_add(1).wrapping_neg(),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AircraftAction {
    None,
    SelfAction,
    Move,
    NoMove,
    Attack,
    Select,
    Enter,
    Other(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AircraftObjectActionContext {
    pub has_passengers: bool,
    pub height_above_ground: bool,
    pub is_tethered: bool,
    pub has_primary_weapon: bool,
    pub player_controls_house: bool,
    pub target_is_ally: bool,
    pub target_is_building: bool,
    /// Result of Transmit_Message(RADIO_CAN_LOAD, target) when that check is
    /// applicable; the caller should not send it for irrelevant target types.
    pub radio_can_load_approved: bool,
    pub is_fixed_wing: bool,
    /// Result of the cell overload, evaluated by the engine only when needed.
    pub fallback_cell_action: AircraftAction,
    pub target_is_repair_building: bool,
    pub repair_building_in_radio_contact: bool,
    pub repair_building_has_attached_object: bool,
}

/// Port of AircraftClass::What_Action(ObjectClass const *). The caller supplies
/// the base-class action and the results of target/radio queries; this function
/// preserves the C++ override order and the conditional cell-action fallback.
pub fn what_action_object(
    mut action: AircraftAction,
    context: AircraftObjectActionContext,
) -> AircraftAction {
    if action == AircraftAction::SelfAction
        && (!context.has_passengers || context.height_above_ground || context.is_tethered)
    {
        action = AircraftAction::None;
    }
    if action == AircraftAction::Attack && !context.has_primary_weapon {
        action = AircraftAction::None;
    }
    if context.player_controls_house
        && context.target_is_ally
        && context.target_is_building
        && context.radio_can_load_approved
    {
        action = AircraftAction::Enter;
    }
    if context.is_fixed_wing && action == AircraftAction::Move {
        action = AircraftAction::NoMove;
    }
    if action == AircraftAction::None {
        action = context.fallback_cell_action;
    }
    if context.player_controls_house
        && action == AircraftAction::Select
        && context.target_is_building
        && context.target_is_repair_building
        && !context.repair_building_in_radio_contact
        && !context.repair_building_has_attached_object
    {
        action = AircraftAction::Enter;
    }
    action
}

/// Port of AircraftClass::What_Action(CELL).
pub fn what_action_cell(
    mut action: AircraftAction,
    cell_is_visible_to_player: bool,
    has_primary_weapon: bool,
    is_fixed_wing: bool,
) -> AircraftAction {
    if matches!(action, AircraftAction::Move | AircraftAction::Attack)
        && !cell_is_visible_to_player
    {
        action = AircraftAction::NoMove;
    }
    if action == AircraftAction::Attack && !has_primary_weapon {
        action = AircraftAction::None;
    }
    if is_fixed_wing && action == AircraftAction::Move {
        action = AircraftAction::NoMove;
    }
    action
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AircraftSightRequest {
    pub house_id: u32,
    pub sight_range: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AircraftFireAtPath {
    ParadropCargo,
    CameraSnapshot,
    StandardWeapon,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AircraftFireAtPlan {
    pub path: AircraftFireAtPath,
    pub sight_requests: Vec<AircraftSightRequest>,
    pub clear_ammo: bool,
    pub rearm_timer_after_call: Option<i32>,
    pub call_base_fire: bool,
    pub return_bullet: bool,
    pub dropping_bullet_speed: Option<(u8, &'static str)>,
}

/// Port of AircraftClass::Fire_At. Sight recipients are supplied by the
/// multiplayer/session adapter after applying the source's player-house
/// alliance checks. The plan contains no renderer, network or sound calls.
pub fn aircraft_fire_at_plan(
    has_attached_passenger: bool,
    primary_weapon_is_camera: bool,
    is_glyphx_multiplayer: bool,
    own_house_id: u32,
    own_house_is_ally_to_player: bool,
    glyphx_visible_allied_house_ids: &[u32],
    owner_house_id_if_human: Option<u32>,
    base_bullet_created: bool,
    base_bullet_is_dropping: bool,
    second_shot: bool,
    rearm_delay_first_shot: i32,
    rearm_delay_second_shot: i32,
) -> AircraftFireAtPlan {
    if has_attached_passenger {
        return AircraftFireAtPlan {
            path: AircraftFireAtPath::ParadropCargo,
            sight_requests: Vec::new(),
            clear_ammo: false,
            rearm_timer_after_call: None,
            call_base_fire: false,
            return_bullet: false,
            dropping_bullet_speed: None,
        };
    }
    if primary_weapon_is_camera {
        let sight_requests = if is_glyphx_multiplayer {
            glyphx_visible_allied_house_ids.iter().copied()
                .map(|house_id| AircraftSightRequest { house_id, sight_range: 9 })
                .collect()
        } else if own_house_is_ally_to_player {
            vec![AircraftSightRequest { house_id: own_house_id, sight_range: 9 }]
        } else {
            Vec::new()
        };
        let rearm_delay = if second_shot { rearm_delay_second_shot } else { rearm_delay_first_shot };
        return AircraftFireAtPlan {
            path: AircraftFireAtPath::CameraSnapshot,
            sight_requests,
            clear_ammo: true,
            rearm_timer_after_call: Some(rearm_delay),
            call_base_fire: false,
            return_bullet: false,
            dropping_bullet_speed: None,
        };
    }

    let mut sight_requests = Vec::new();
    if base_bullet_created {
        if let Some(owner_house_id) = owner_house_id_if_human {
            sight_requests.push(AircraftSightRequest { house_id: owner_house_id, sight_range: 1 });
        }
    }
    AircraftFireAtPlan {
        path: AircraftFireAtPath::StandardWeapon,
        sight_requests,
        clear_ammo: false,
        rearm_timer_after_call: None,
        call_base_fire: true,
        return_bullet: base_bullet_created,
        dropping_bullet_speed: if base_bullet_created && base_bullet_is_dropping {
            Some((40, "MPH_MEDIUM_SLOW"))
        } else {
            None
        },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LandingTakeoffAiPlan {
    pub height: i32,
    pub is_landing: bool,
    pub is_taking_off: bool,
    pub speed_override: Option<i32>,
    pub clear_navcom: bool,
    pub scatter: bool,
    pub destroyed: bool,
    pub remove_before_destruction: bool,
    pub redraw_mark_count: u8,
    pub remove_from_old_layer: bool,
    pub submit_to_new_layer: bool,
    pub establish_tether: bool,
    pub remove_tether: bool,
    pub perform_look: bool,
    pub radio_over_out: bool,
    pub takeoff_started_due_to_blocked_landing_zone: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LandingTakeoffAiInput {
    pub door_is_closed: bool,
    pub is_landing: bool,
    pub is_taking_off: bool,
    pub height: i32,
    pub pixel_to_lepton_step: i32,
    pub flight_level: i32,
    pub fixed_wing: bool,
    pub mission_is_enter: bool,
    pub old_layer_is_ground: bool,
    pub current_cell_matches_navcom: bool,
    pub navcom_is_legal: bool,
    pub navcom_targets_contact_object: bool,
    pub in_radio_contact: bool,
    pub im_in_reply_was_roger: bool,
    pub landing_zone_is_clear: bool,
    pub navcom_differs_from_contact: bool,
}

/// Port of AircraftClass::Landing_Takeoff_AI. The layer resolver is evaluated
/// after the height/flag changes, mirroring In_Which_Layer(); the caller applies
/// returned map, radio, damage and rendering operations on its simulation thread.
pub fn landing_takeoff_ai<F>(
    input: LandingTakeoffAiInput,
    mut new_layer_is_ground: F,
) -> LandingTakeoffAiPlan
where
    F: FnMut(i32, bool, bool) -> bool,
{
    let mut plan = LandingTakeoffAiPlan {
        height: input.height,
        is_landing: input.is_landing,
        is_taking_off: input.is_taking_off,
        ..LandingTakeoffAiPlan::default()
    };
    if !input.door_is_closed || (!input.is_landing && !input.is_taking_off) {
        return plan;
    }

    if plan.is_landing {
        plan.redraw_mark_count += 1; // MARK_UP
        plan.height -= input.pixel_to_lepton_step;
        if plan.height <= 0 {
            plan.height = 0;
            plan.is_landing = false;
            plan.speed_override = Some(0);
            if input.current_cell_matches_navcom {
                plan.clear_navcom = true;
            }
            if input.fixed_wing && !input.mission_is_enter {
                plan.destroyed = true;
                plan.remove_before_destruction = true;
                return plan; // The C++ method returns before MARK_DOWN.
            }
            if input.navcom_is_legal
                && input.navcom_targets_contact_object
                && input.in_radio_contact
                && !input.im_in_reply_was_roger
            {
                plan.scatter = true;
            }
        }
        plan.redraw_mark_count += 1; // MARK_DOWN
    }

    if plan.is_taking_off {
        plan.redraw_mark_count += 1; // MARK_UP
        plan.height += input.pixel_to_lepton_step;
        if plan.height >= input.flight_level {
            plan.height = input.flight_level;
            plan.is_taking_off = false;
        }
        plan.redraw_mark_count += 1; // MARK_DOWN
    }

    let now_ground = new_layer_is_ground(plan.height, plan.is_landing, plan.is_taking_off);
    if now_ground != input.old_layer_is_ground {
        if now_ground && !plan.is_taking_off && !input.fixed_wing {
            if !input.landing_zone_is_clear {
                plan.is_taking_off = true;
                plan.redraw_mark_count += 2; // MARK_UP and MARK_DOWN around the corrective lift.
                plan.height += input.pixel_to_lepton_step;
                plan.takeoff_started_due_to_blocked_landing_zone = true;
                return plan;
            }
        }

        plan.remove_from_old_layer = true;
        plan.submit_to_new_layer = true;
        if now_ground {
            plan.clear_navcom = true;
            plan.establish_tether = true;
            plan.perform_look = true;
        } else {
            plan.remove_tether = true;
            if input.in_radio_contact
                && input.navcom_is_legal
                && input.navcom_differs_from_contact
            {
                plan.radio_over_out = true;
            }
        }
    }
    plan
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotorPart {
    Right,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RotorDrawRequest {
    pub part: RotorPart,
    pub shape_number: i32,
    pub x: i32,
    pub y: i32,
    pub centered: bool,
    pub window_relative: bool,
    pub ghost: bool,
    pub fading: bool,
    pub predator: bool,
    pub unit_shadow: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RotorDrawPlan {
    pub shape_number: i32,
    pub ghost: bool,
    pub fading: bool,
    pub predator: bool,
    pub draw_requests: Vec<RotorDrawRequest>,
}

/// Port of AircraftClass::Draw_Rotors. The caller supplies the exact
/// Lepton_To_Pixel and Move_Point implementations; this function preserves the
/// original fetch-stage modulo, flags, transport stretch table, and draw order.
pub fn draw_rotors_plan<P, M>(
    is_transport: bool,
    height: i32,
    fetch_stage: i32,
    x: i32,
    y: i32,
    secondary_facing: DirType,
    lepton_to_pixel: P,
    mut move_point: M,
) -> RotorDrawPlan
where
    P: FnOnce(i32) -> i32,
    M: FnMut((i16, i16), DirType, i32) -> (i16, i16),
{
    let (shape_number, ghost, fading, predator) = if height == 0 {
        (fetch_stage.rem_euclid(8) + 4, true, false, false)
    } else {
        (fetch_stage.rem_euclid(4), false, true, true)
    };
    let base_y = y - lepton_to_pixel(height);
    let common = |part, position: (i32, i32)| RotorDrawRequest {
        part,
        shape_number,
        x: position.0,
        y: position.1 - 2,
        centered: true,
        window_relative: true,
        ghost,
        fading,
        predator,
        unit_shadow: true,
    };
    let mut draw_requests = Vec::with_capacity(if is_transport { 2 } else { 1 });

    if is_transport {
        const STRETCH: [i32; 8] = [8, 9, 10, 9, 8, 9, 10, 9];
        let facing_index = usize::from(secondary_facing.as_facing() as u8);
        let stretch = STRETCH[facing_index];
        let first_start = (x as i16, base_y as i16);
        let right_position = move_point(first_start, secondary_facing, stretch);
        draw_requests.push(common(RotorPart::Right, (i32::from(right_position.0), i32::from(right_position.1))));

        let left_position = move_point(
            right_position,
            secondary_facing.wrapping_add(128),
            stretch * 2,
        );
        draw_requests.push(common(RotorPart::Left, (i32::from(left_position.0), i32::from(left_position.1))));
    } else {
        draw_requests.push(common(RotorPart::Right, (x, base_y)));
    }

    RotorDrawPlan {
        shape_number,
        ghost,
        fading,
        predator,
        draw_requests,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AircraftDamageResult {
    None,
    Half,
    Destroyed,
    Other(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AircraftDamagePlan {
    /// Damage passed to FootClass::Take_Damage after the aircraft's in-flight
    /// halving rule.
    pub adjusted_damage: i32,
    pub kill_cargo: bool,
    pub announce_death: bool,
    pub fireball_coordinate: Option<u32>,
    pub attempt_crew_paradrop: bool,
    pub delete_aircraft: bool,
}

/// Port of AircraftClass::Take_Damage's aircraft-specific wrapper. The base
/// Take_Damage result and the combined crew/chance/landing-cell eligibility
/// check are supplied by the engine adapter.
pub fn aircraft_damage_plan(
    damage: i32,
    height: i32,
    base_result: AircraftDamageResult,
    target_coordinate: u32,
    high_coordinate_mask: u32,
    is_crew_aircraft: bool,
    crew_paradrop_chance_and_cell_are_valid: bool,
) -> AircraftDamagePlan {
    let adjusted_damage = if height != 0 { damage / 2 } else { damage };
    if base_result == AircraftDamageResult::Destroyed {
        AircraftDamagePlan {
            adjusted_damage,
            kill_cargo: true,
            announce_death: true,
            fireball_coordinate: (target_coordinate & high_coordinate_mask == 0)
                .then_some(target_coordinate),
            attempt_crew_paradrop: is_crew_aircraft && crew_paradrop_chance_and_cell_are_valid,
            delete_aircraft: true,
        }
    } else {
        AircraftDamagePlan {
            adjusted_damage,
            kill_cargo: false,
            announce_death: false,
            fireball_coordinate: None,
            attempt_crew_paradrop: false,
            delete_aircraft: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AircraftMission {
    Hunt,
    Guard,
    Retreat,
    Unload,
    Enter,
    Move,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdleModeDestination {
    Clear,
    CurrentCell,
    AssignedCell(u32),
    GoodLandingZone(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdleModeContext {
    pub fixed_wing: bool,
    pub is_ground_layer: bool,
    pub is_loaner: bool,
    pub has_primary_weapon: bool,
    pub has_ammo: bool,
    pub has_team: bool,
    pub team_has_entered_map: bool,
    pub house_is_human: bool,
    pub has_attached_passengers: bool,
    pub is_landing: bool,
    /// Accepted docking building after the source's contact/Find_Docking_Bay
    /// and RADIO_HELLO checks. A failed or missing result is None.
    pub accepted_docking_building_cell: Option<u32>,
    pub good_landing_zone_cell: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdleModePlan {
    /// None means the source returned early because the aircraft remained
    /// attached to a team that controls its current mission.
    pub mission: Option<AircraftMission>,
    pub destination: Option<IdleModeDestination>,
    pub clear_target: bool,
    pub remove_from_team: bool,
    pub reset_status: bool,
    pub process_takeoff: bool,
    pub commence: bool,
}

/// Port of AircraftClass::Enter_Idle_Mode with engine queries pre-resolved in
/// IdleModeContext. Team removal and radio/docking handshakes are represented
/// as actions; source-disabled FIXIT_CARRIER branches are intentionally absent.
pub fn enter_idle_mode_plan(context: IdleModeContext) -> IdleModePlan {
    let mut mission = if context.fixed_wing && context.is_loaner && context.has_primary_weapon {
        AircraftMission::Hunt
    } else {
        AircraftMission::Guard
    };
    let mut destination = None;
    let mut clear_target = false;
    let mut remove_from_team = false;
    let mut reset_status = false;
    let mut process_takeoff = false;

    if context.fixed_wing {
        if context.is_ground_layer {
            if context.is_loaner {
                mission = AircraftMission::Retreat;
            } else {
                destination = Some(IdleModeDestination::Clear);
                clear_target = true;
                mission = AircraftMission::Guard;
            }
        } else {
            let remove_entered_team = context.has_team
                && context.team_has_entered_map
                && ((context.is_loaner && context.house_is_human)
                    || (!context.house_is_human && !context.has_ammo));
            remove_from_team = remove_entered_team;
            let team_remains = context.has_team && !remove_entered_team;
            if team_remains {
                return IdleModePlan {
                    mission: None,
                    destination: None,
                    clear_target: false,
                    remove_from_team,
                    reset_status: false,
                    process_takeoff: false,
                    commence: false,
                };
            }
            if !(context.is_loaner && context.has_primary_weapon && !context.has_ammo) {
                if !context.is_loaner {
                    destination = Some(IdleModeDestination::Clear);
                    if context.accepted_docking_building_cell.is_some() {
                        mission = AircraftMission::Enter;
                        reset_status = true;
                        process_takeoff = context.is_landing;
                    } else {
                        mission = AircraftMission::Retreat;
                    }
                }
            }
        }
    } else if context.is_ground_layer {
        if context.is_loaner {
            if context.has_attached_passengers {
                mission = if context.has_team {
                    AircraftMission::Guard
                } else {
                    AircraftMission::Unload
                };
            } else {
                mission = AircraftMission::Retreat;
            }
        } else {
            destination = Some(IdleModeDestination::Clear);
            clear_target = true;
            mission = AircraftMission::Guard;
        }
    } else if context.has_attached_passengers {
        if context.is_loaner {
            if context.has_team {
                mission = AircraftMission::Guard;
            } else {
                mission = AircraftMission::Unload;
                destination = Some(IdleModeDestination::GoodLandingZone(context.good_landing_zone_cell));
            }
        } else {
            destination = Some(IdleModeDestination::GoodLandingZone(context.good_landing_zone_cell));
            mission = AircraftMission::Move;
        }
    } else {
        let remove_entered_team = context.has_team
            && context.team_has_entered_map
            && ((context.is_loaner && context.house_is_human)
                || (!context.house_is_human && !context.has_ammo));
        remove_from_team = remove_entered_team;
        let team_remains = context.has_team && !remove_entered_team;
        if context.has_primary_weapon {
            if context.is_loaner {
                if !context.has_ammo {
                    if team_remains {
                        remove_from_team = true;
                    }
                    mission = AircraftMission::Retreat;
                } else if !team_remains {
                    mission = AircraftMission::Hunt;
                }
            } else {
                destination = Some(IdleModeDestination::Clear);
                if context.accepted_docking_building_cell.is_some() {
                    mission = AircraftMission::Enter;
                } else {
                    destination = Some(IdleModeDestination::GoodLandingZone(context.good_landing_zone_cell));
                    mission = AircraftMission::Move;
                }
            }
        } else {
            if team_remains {
                return IdleModePlan {
                    mission: None,
                    destination: None,
                    clear_target: false,
                    remove_from_team,
                    reset_status: false,
                    process_takeoff: false,
                    commence: false,
                };
            }
            destination = Some(IdleModeDestination::GoodLandingZone(context.good_landing_zone_cell));
            mission = AircraftMission::Move;
        }
    }

    IdleModePlan {
        mission: Some(mission),
        destination,
        clear_target,
        remove_from_team,
        reset_status,
        process_takeoff,
        commence: true,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AircraftScatterPlan {
    pub enter_idle_mode: bool,
    pub is_landing: bool,
    pub is_taking_off: bool,
}

/// Port of AircraftClass::Scatter's gates and takeoff transition. The idle-mode
/// mission selection is supplied by enter_idle_mode_plan.
pub const fn aircraft_scatter_plan(
    mission_allows_scatter: bool,
    fixed_wing: bool,
    is_landing: bool,
    height: i32,
    is_taking_off: bool,
) -> AircraftScatterPlan {
    if !mission_allows_scatter || fixed_wing {
        return AircraftScatterPlan {
            enter_idle_mode: false,
            is_landing,
            is_taking_off,
        };
    }
    if is_landing || height == 0 {
        AircraftScatterPlan {
            enter_idle_mode: true,
            is_landing: false,
            is_taking_off: true,
        }
    } else {
        AircraftScatterPlan {
            enter_idle_mode: true,
            is_landing,
            is_taking_off,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessFlyToPlan {
    pub destination_coordinate: u32,
    pub primary_facing: DirType,
    pub speed_updates: Vec<u32>,
    pub returned_distance: i32,
}

/// Port of AircraftClass::Process_Fly_To. Destination resolution (building
/// docking coordinate versus ordinary coordinate), distance and direction are
/// supplied by the variant's map/target adapters.
pub fn process_fly_to<D, F>(
    target_coordinate: u32,
    building_docking_coordinate: Option<u32>,
    aircraft_coordinate: u32,
    fixed_wing: bool,
    slowdown_requested: bool,
    current_speed: u32,
    distance_between: D,
    direction_to: F,
) -> ProcessFlyToPlan
where
    D: FnOnce(u32, u32) -> i32,
    F: FnOnce(u32, u32) -> DirType,
{
    let destination_coordinate = building_docking_coordinate.unwrap_or(target_coordinate);
    let mut distance = distance_between(aircraft_coordinate, destination_coordinate);
    let primary_facing = direction_to(aircraft_coordinate, destination_coordinate);
    let slowdown = slowdown_requested && !fixed_wing;
    let mut speed_updates = Vec::with_capacity(2);
    if slowdown {
        let speed = (distance.min(0x0300) / 3).clamp(0x0020, 0x00FF) as u32;
        if current_speed != speed {
            speed_updates.push(speed);
        }
    }
    if distance < 0x0010 {
        if slowdown {
            speed_updates.push(0);
        }
        distance = 0;
    }
    ProcessFlyToPlan {
        destination_coordinate,
        primary_facing,
        speed_updates,
        returned_distance: distance,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AircraftHuntStage {
    LookForTarget,
    TakeOff,
    FlyToTarget,
    DropBombs,
    Regroup,
    Unknown(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AircraftHuntFireResult {
    Ok,
    Ammo,
    Range,
    Facing,
    Cant,
    Illegal,
    Rearm,
    Other(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AircraftThreatKind {
    Tiberium,
    Normal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HuntDelay {
    Ticks(i32),
    NormalPlusRandom2 { normal_delay: i32, random_value: i32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HuntFireTarget {
    ComputedForwardPoint,
    CurrentTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HuntFireAction {
    pub target: HuntFireTarget,
    pub weapon_index: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AircraftHuntPlan {
    pub assign_destination_to_target: bool,
    pub threat_search_order: Vec<AircraftThreatKind>,
    pub assign_target_from_tiberium_threat: bool,
    pub assign_target_from_normal_threat: bool,
    pub next_stage: Option<AircraftHuntStage>,
    pub mission_to_assign: Option<AircraftMission>,
    pub enter_idle_mode: bool,
    pub process_takeoff: bool,
    pub set_speed: Option<u8>,
    pub break_radio_contact: bool,
    pub remove_from_team: bool,
    pub commence: bool,
    pub notify_target_incoming: bool,
    pub fire_actions: Vec<HuntFireAction>,
    pub attacks_remaining_delta: i8,
    pub set_attacks_remaining_zero: bool,
    pub rearm_delay_return: bool,
    pub delay: HuntDelay,
    pub return_immediately: bool,
}

/// Inputs are resolved snapshots from the simulation loop. Target acquisition,
/// team membership, range/facing checks and Fire_At are expressed as actions;
/// this function itself is a deterministic state transition with no global
/// reads, random calls, radio operations or blocking I/O.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AircraftHuntContext {
    pub fixed_wing: bool,
    pub stage: AircraftHuntStage,
    pub mission_is_attack: bool,
    pub target_is_legal: bool,
    pub target_differs_from_navcom: bool,
    pub team_is_valid: bool,
    pub game_is_normal: bool,
    pub tiberium_threat_is_legal: bool,
    pub normal_threat_is_legal: bool,
    pub takeoff_completed: bool,
    pub in_radio_contact: bool,
    pub fire_result: AircraftHuntFireResult,
    pub target_is_in_weapon_range: bool,
    pub passenger: bool,
    pub primary_facing_is_rotating: bool,
    pub target_is_legal_after_search: bool,
    pub primary_weapon_is_camera: bool,
    pub primary_bullet_is_homing: bool,
    pub two_shooter: bool,
    pub ammo: i32,
    pub attacks_remaining: i32,
    pub has_attached_passengers: bool,
    pub is_loaner: bool,
    pub ticks_per_second: i32,
    pub normal_delay: i32,
    pub random_delay_0_to_2: i32,
}

impl AircraftHuntPlan {
    fn new(normal_delay: i32, random_delay_0_to_2: i32) -> Self {
        Self {
            assign_destination_to_target: false,
            threat_search_order: Vec::new(),
            assign_target_from_tiberium_threat: false,
            assign_target_from_normal_threat: false,
            next_stage: None,
            mission_to_assign: None,
            enter_idle_mode: false,
            process_takeoff: false,
            set_speed: None,
            break_radio_contact: false,
            remove_from_team: false,
            commence: false,
            notify_target_incoming: false,
            fire_actions: Vec::new(),
            attacks_remaining_delta: 0,
            set_attacks_remaining_zero: false,
            rearm_delay_return: false,
            delay: HuntDelay::NormalPlusRandom2 {
                normal_delay,
                random_value: random_delay_0_to_2,
            },
            return_immediately: false,
        }
    }
}

/// Port of AircraftClass::Mission_Hunt. Fixed-wing status transitions and the
/// rotorcraft target/mission handoff follow the source's switch ordering.
pub fn aircraft_mission_hunt(context: AircraftHuntContext) -> AircraftHuntPlan {
    let mut plan = AircraftHuntPlan::new(
        context.normal_delay,
        context.random_delay_0_to_2.clamp(0, 2),
    );

    if !context.fixed_wing {
        if context.ammo == 0 {
            plan.remove_from_team = context.team_is_valid;
            plan.enter_idle_mode = true;
        } else {
            if !context.target_is_legal {
                if !context.game_is_normal {
                    plan.threat_search_order.push(AircraftThreatKind::Tiberium);
                    plan.assign_target_from_tiberium_threat = true;
                }
                plan.threat_search_order.push(AircraftThreatKind::Normal);
                plan.assign_target_from_normal_threat = true;
                if !context.target_is_legal_after_search {
                    plan.enter_idle_mode = true;
                    plan.delay = HuntDelay::Ticks(1);
                    plan.return_immediately = true;
                    return plan;
                }
            }
            plan.mission_to_assign = Some(AircraftMission::Attack);
            plan.delay = HuntDelay::Ticks(1);
            plan.return_immediately = true;
        }
        return plan;
    }

    if context.target_differs_from_navcom {
        plan.assign_destination_to_target = true;
    }

    match context.stage {
        AircraftHuntStage::LookForTarget => {
            if context.target_is_legal {
                plan.next_stage = Some(AircraftHuntStage::TakeOff);
                plan.delay = HuntDelay::Ticks(1);
                plan.return_immediately = true;
            } else if !context.team_is_valid {
                if !context.game_is_normal {
                    plan.threat_search_order.push(AircraftThreatKind::Tiberium);
                    plan.assign_target_from_tiberium_threat = true;
                }
                if context.game_is_normal || !context.tiberium_threat_is_legal {
                    plan.threat_search_order.push(AircraftThreatKind::Normal);
                    plan.assign_target_from_normal_threat = true;
                }
                if !context.target_is_legal_after_search {
                    plan.enter_idle_mode = true;
                }
            }
        }
        AircraftHuntStage::TakeOff => {
            plan.process_takeoff = true;
            if context.takeoff_completed {
                plan.set_speed = Some(u8::MAX);
                plan.break_radio_contact = context.in_radio_contact;
                plan.next_stage = Some(AircraftHuntStage::FlyToTarget);
            }
            plan.delay = HuntDelay::Ticks(1);
            plan.return_immediately = true;
        }
        AircraftHuntStage::FlyToTarget => {
            match context.fire_result {
                AircraftHuntFireResult::Facing => {
                    if context.target_is_in_weapon_range || context.passenger {
                        plan.delay = HuntDelay::Ticks(context.ticks_per_second.saturating_mul(2));
                        plan.return_immediately = true;
                    } else if !context.primary_facing_is_rotating && context.target_is_legal {
                        plan.next_stage = None; // caller re-aims primary facing at TarCom.
                    }
                }
                AircraftHuntFireResult::Ammo => {
                    plan.next_stage = Some(AircraftHuntStage::Regroup);
                }
                AircraftHuntFireResult::Cant | AircraftHuntFireResult::Illegal => {
                    plan.next_stage = Some(if context.mission_is_attack {
                        AircraftHuntStage::Regroup
                    } else {
                        AircraftHuntStage::LookForTarget
                    });
                }
                AircraftHuntFireResult::Ok => {
                    plan.next_stage = Some(AircraftHuntStage::DropBombs);
                    plan.delay = HuntDelay::Ticks(1);
                    plan.return_immediately = true;
                }
                AircraftHuntFireResult::Range
                | AircraftHuntFireResult::Rearm
                | AircraftHuntFireResult::Other(_) => {
                    // The source turns toward a legal target when not already rotating.
                    if !context.primary_facing_is_rotating && context.target_is_legal {
                        plan.next_stage = None;
                    }
                }
            }
            if !plan.return_immediately {
                plan.delay = HuntDelay::Ticks(context.ticks_per_second / 2);
                plan.return_immediately = true;
            }
        }
        AircraftHuntStage::DropBombs => {
            match context.fire_result {
                AircraftHuntFireResult::Ok => {
                    plan.next_stage = if context.primary_weapon_is_camera {
                        Some(AircraftHuntStage::Regroup)
                    } else {
                        None
                    };
                    plan.notify_target_incoming = !context.primary_weapon_is_camera;
                    let target = if context.primary_bullet_is_homing {
                        HuntFireTarget::CurrentTarget
                    } else {
                        HuntFireTarget::ComputedForwardPoint
                    };
                    plan.fire_actions.push(HuntFireAction { target, weapon_index: 0 });
                    if context.two_shooter {
                        plan.fire_actions.push(HuntFireAction { target, weapon_index: 0 });
                    }
                    plan.rearm_delay_return = true;
                    plan.return_immediately = true;
                }
                AircraftHuntFireResult::Range | AircraftHuntFireResult::Facing => {
                    plan.next_stage = Some(AircraftHuntStage::FlyToTarget);
                    plan.delay = HuntDelay::Ticks(context.ticks_per_second.saturating_mul(4));
                    plan.return_immediately = true;
                }
                AircraftHuntFireResult::Illegal => {
                    plan.next_stage = Some(if context.mission_is_attack {
                        AircraftHuntStage::Regroup
                    } else {
                        AircraftHuntStage::LookForTarget
                    });
                }
                AircraftHuntFireResult::Cant => {
                    plan.next_stage = Some(AircraftHuntStage::Regroup);
                }
                AircraftHuntFireResult::Ammo => {
                    plan.attacks_remaining_delta = -1;
                    plan.next_stage = Some(AircraftHuntStage::Regroup);
                }
                AircraftHuntFireResult::Rearm | AircraftHuntFireResult::Other(_) => {}
            }
            if !plan.return_immediately {
                plan.delay = HuntDelay::Ticks(1);
                plan.return_immediately = true;
            }
        }
        AircraftHuntStage::Regroup => {
            let mut attacks_remaining_zero = context.attacks_remaining == 0;
            if context.ammo == 0 {
                attacks_remaining_zero = true;
                plan.set_attacks_remaining_zero = true;
                plan.remove_from_team = context.team_is_valid;
                plan.enter_idle_mode = true;
            }
            let must_leave_or_idle = context.mission_is_attack
                || context.primary_weapon_is_camera
                || (attacks_remaining_zero && !context.has_attached_passengers);
            if must_leave_or_idle {
                if context.is_loaner {
                    plan.remove_from_team |= context.team_is_valid;
                    plan.mission_to_assign = Some(AircraftMission::Retreat);
                    plan.commence = true;
                } else if !context.team_is_valid {
                    plan.enter_idle_mode = true;
                }
                plan.commence = true;
            } else {
                plan.next_stage = Some(AircraftHuntStage::LookForTarget);
            }
        }
        AircraftHuntStage::Unknown(_) => {}
    }
    plan
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AircraftRetreatStage {
    TakeOff,
    FaceMapEdge,
    KeepFlying,
    Unknown(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AircraftRetreatContext {
    pub fixed_wing: bool,
    pub height: i32,
    pub flight_level: i32,
    pub status: AircraftRetreatStage,
    pub takeoff_completed: bool,
    /// Source House::Control.Edge enum. Low two bits encode friendly edge.
    pub friendly_map_edge: u8,
    pub normal_delay: i32,
    pub random_delay_0_to_2: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AircraftRetreatPlan {
    pub height_after_tick: i32,
    pub next_stage: Option<AircraftRetreatStage>,
    pub process_takeoff: bool,
    pub set_speed: Option<u8>,
    pub primary_desired: Option<DirType>,
    pub secondary_desired: Option<DirType>,
    pub delay: HuntDelay,
    pub return_immediately: bool,
}

/// Port of AircraftClass::Mission_Retreat. Fixed-wing aircraft climb one
/// source height unit per call then wait; rotorcraft take off and steer toward
/// the owning house's friendly map edge before preserving the flight path.
pub fn aircraft_mission_retreat(context: AircraftRetreatContext) -> AircraftRetreatPlan {
    if context.fixed_wing {
        if context.height < context.flight_level {
            return AircraftRetreatPlan {
                height_after_tick: context.height + 1,
                next_stage: None,
                process_takeoff: false,
                set_speed: None,
                primary_desired: None,
                secondary_desired: None,
                delay: HuntDelay::Ticks(3),
                return_immediately: true,
            };
        }
        return AircraftRetreatPlan {
            height_after_tick: context.height,
            next_stage: None,
            process_takeoff: false,
            set_speed: None,
            primary_desired: None,
            secondary_desired: None,
            delay: HuntDelay::Ticks(context.normal_delay.saturating_mul(10)),
            return_immediately: true,
        };
    }

    let mut plan = AircraftRetreatPlan {
        height_after_tick: context.height,
        next_stage: None,
        process_takeoff: false,
        set_speed: None,
        primary_desired: None,
        secondary_desired: None,
        delay: HuntDelay::NormalPlusRandom2 {
            normal_delay: context.normal_delay,
            random_value: context.random_delay_0_to_2.clamp(0, 2),
        },
        return_immediately: false,
    };
    match context.status {
        AircraftRetreatStage::TakeOff => {
            plan.process_takeoff = true;
            if context.takeoff_completed {
                plan.next_stage = Some(AircraftRetreatStage::FaceMapEdge);
            }
            plan.delay = HuntDelay::Ticks(1);
            plan.return_immediately = true;
        }
        AircraftRetreatStage::FaceMapEdge => {
            plan.set_speed = Some(u8::MAX);
            let direction = DirType((u16::from(context.friendly_map_edge & 0x03) << 6) as u8);
            plan.primary_desired = Some(direction);
            plan.secondary_desired = Some(direction);
            plan.next_stage = Some(AircraftRetreatStage::KeepFlying);
        }
        AircraftRetreatStage::KeepFlying | AircraftRetreatStage::Unknown(_) => {}
    }
    plan
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AircraftMovementAiPlan {
    pub assign_move_mission: bool,
    pub mark_up_before_physics: bool,
    pub mark_down_after_physics: bool,
    pub mark_change_redraw_before_physics: bool,
    pub mark_change_redraw_after_physics_if_moved: bool,
    pub perform_physics: bool,
}

/// Port of AircraftClass::Movement_AI. Physics itself remains a simulation
/// service; the plan retains the source's ground/air redraw marking order.
pub fn aircraft_movement_ai(
    navcom_is_legal: bool,
    mission_is_guard: bool,
    mission_queue_is_none: bool,
    speed: u32,
    current_layer_is_ground: bool,
) -> AircraftMovementAiPlan {
    let perform_physics = speed != 0;
    AircraftMovementAiPlan {
        assign_move_mission: navcom_is_legal && mission_is_guard && mission_queue_is_none,
        mark_up_before_physics: perform_physics && current_layer_is_ground,
        mark_down_after_physics: perform_physics && current_layer_is_ground,
        mark_change_redraw_before_physics: perform_physics && !current_layer_is_ground,
        mark_change_redraw_after_physics_if_moved: perform_physics && !current_layer_is_ground,
        perform_physics,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AircraftGuardBuildingPurpose {
    Repair,
    Rearm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AircraftGuardContext {
    pub height: i32,
    pub flight_level: i32,
    pub team_is_valid: bool,
    pub navcom_is_legal: bool,
    pub has_primary_weapon: bool,
    pub house_is_human: bool,
    pub house_available_money: i32,
    pub health_ratio: Fixed16,
    pub condition_yellow: Fixed16,
    pub in_radio_contact: bool,
    pub height_is_grounded: bool,
    pub contact_is_repair_building: bool,
    pub repair_building_available: bool,
    pub ammo: i32,
    pub weapon_is_equipped: bool,
    pub helipad_available: bool,
    pub target_is_legal: bool,
    pub scatter_is_allowed: bool,
    pub house_is_attacked: bool,
    pub juicy_target_is_legal: bool,
    pub ticks_per_second: i32,
    pub normal_delay: i32,
    pub base_guard_delay: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AircraftGuardPlan {
    pub destination: Option<IdleModeDestination>,
    pub clear_target: bool,
    pub target_juicy_threat: bool,
    pub mission_to_assign: Option<AircraftMission>,
    pub enter_idle_mode: bool,
    pub scatter: bool,
    pub delegate_to_base_guard: bool,
    pub delay: HuntDelay,
    pub return_immediately: bool,
}

/// Port of AircraftClass::Mission_Guard. Building search/radio acceptance is
/// resolved before calling; branch precedence and return delays are explicit.
pub fn aircraft_mission_guard(context: AircraftGuardContext) -> AircraftGuardPlan {
    let normal = HuntDelay::Ticks(context.normal_delay);
    if context.height == context.flight_level {
        if context.team_is_valid {
            return AircraftGuardPlan {
                destination: None,
                clear_target: false,
                target_juicy_threat: false,
                mission_to_assign: if context.navcom_is_legal { Some(AircraftMission::Move) } else { None },
                enter_idle_mode: false,
                scatter: false,
                delegate_to_base_guard: false,
                delay: normal,
                return_immediately: true,
            };
        }
        if !context.has_primary_weapon {
            return AircraftGuardPlan {
                destination: Some(IdleModeDestination::CurrentCell),
                clear_target: false,
                target_juicy_threat: false,
                mission_to_assign: Some(AircraftMission::Move),
                enter_idle_mode: false,
                scatter: false,
                delegate_to_base_guard: false,
                delay: HuntDelay::Ticks(1),
                return_immediately: true,
            };
        }
        return AircraftGuardPlan {
            destination: None,
            clear_target: false,
            target_juicy_threat: false,
            mission_to_assign: None,
            enter_idle_mode: !context.team_is_valid,
            scatter: false,
            delegate_to_base_guard: false,
            delay: HuntDelay::Ticks(1),
            return_immediately: true,
        };
    }

    if context.house_is_human {
        return AircraftGuardPlan {
            destination: None,
            clear_target: false,
            target_juicy_threat: false,
            mission_to_assign: None,
            enter_idle_mode: false,
            scatter: false,
            delegate_to_base_guard: false,
            delay: normal,
            return_immediately: true,
        };
    }

    let damaged = context.house_available_money >= 100
        && context.health_ratio <= context.condition_yellow;
    let should_search_repair = damaged
        && (!context.in_radio_contact
            || (context.height_is_grounded && !context.contact_is_repair_building));
    if should_search_repair && context.repair_building_available {
        return AircraftGuardPlan {
            destination: Some(IdleModeDestination::Clear),
            clear_target: true,
            target_juicy_threat: false,
            mission_to_assign: Some(AircraftMission::Enter),
            enter_idle_mode: false,
            scatter: false,
            delegate_to_base_guard: false,
            delay: HuntDelay::Ticks(1),
            return_immediately: true,
        };
    }

    if context.ammo == 0 && context.weapon_is_equipped && !context.in_radio_contact
        && context.helipad_available
    {
        return AircraftGuardPlan {
            destination: Some(IdleModeDestination::Clear),
            clear_target: true,
            target_juicy_threat: false,
            mission_to_assign: Some(AircraftMission::Enter),
            enter_idle_mode: false,
            scatter: false,
            delegate_to_base_guard: false,
            delay: HuntDelay::Ticks(1),
            return_immediately: true,
        };
    }

    if context.target_is_legal {
        return AircraftGuardPlan {
            destination: None,
            clear_target: false,
            target_juicy_threat: false,
            mission_to_assign: Some(AircraftMission::Attack),
            enter_idle_mode: false,
            scatter: false,
            delegate_to_base_guard: false,
            delay: HuntDelay::Ticks(1),
            return_immediately: true,
        };
    }

    if !context.weapon_is_equipped {
        return AircraftGuardPlan {
            destination: None,
            clear_target: false,
            target_juicy_threat: false,
            mission_to_assign: None,
            enter_idle_mode: false,
            scatter: false,
            delegate_to_base_guard: false,
            delay: HuntDelay::Ticks(context.ticks_per_second.saturating_mul(3)),
            return_immediately: true,
        };
    }

    if context.height_is_grounded && !context.in_radio_contact {
        return AircraftGuardPlan {
            destination: None,
            clear_target: false,
            target_juicy_threat: false,
            mission_to_assign: None,
            enter_idle_mode: false,
            scatter: context.scatter_is_allowed,
            delegate_to_base_guard: false,
            delay: HuntDelay::Ticks(context.ticks_per_second.saturating_mul(3)),
            return_immediately: true,
        };
    }

    if !context.house_is_attacked && context.juicy_target_is_legal {
        return AircraftGuardPlan {
            destination: None,
            clear_target: false,
            target_juicy_threat: true,
            mission_to_assign: Some(AircraftMission::Attack),
            enter_idle_mode: false,
            scatter: false,
            delegate_to_base_guard: true,
            delay: HuntDelay::Ticks(context.base_guard_delay),
            return_immediately: true,
        };
    }

    AircraftGuardPlan {
        destination: None,
        clear_target: false,
        target_juicy_threat: false,
        mission_to_assign: None,
        enter_idle_mode: false,
        scatter: false,
        delegate_to_base_guard: true,
        delay: HuntDelay::Ticks(context.base_guard_delay),
        return_immediately: true,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AircraftGuardAreaContext {
    pub height: i32,
    pub flight_level: i32,
    pub team_is_valid: bool,
    pub house_is_human: bool,
    pub height_is_grounded: bool,
    pub in_radio_contact: bool,
    pub scatter_is_allowed: bool,
    pub target_is_legal: bool,
    pub ticks_per_second: i32,
    pub base_guard_area_delay: i32,
}

pub fn aircraft_mission_guard_area(context: AircraftGuardAreaContext) -> AircraftGuardPlan {
    if context.height == context.flight_level {
        return AircraftGuardPlan {
            destination: None,
            clear_target: false,
            target_juicy_threat: false,
            mission_to_assign: None,
            enter_idle_mode: !context.team_is_valid,
            scatter: false,
            delegate_to_base_guard: false,
            delay: HuntDelay::Ticks(1),
            return_immediately: true,
        };
    }
    if context.house_is_human {
        return AircraftGuardPlan {
            destination: None,
            clear_target: false,
            target_juicy_threat: false,
            mission_to_assign: None,
            enter_idle_mode: false,
            scatter: false,
            delegate_to_base_guard: false,
            delay: HuntDelay::Ticks(context.ticks_per_second),
            return_immediately: true,
        };
    }
    if context.height_is_grounded && !context.in_radio_contact {
        return AircraftGuardPlan {
            destination: None,
            clear_target: false,
            target_juicy_threat: false,
            mission_to_assign: None,
            enter_idle_mode: false,
            scatter: context.scatter_is_allowed,
            delegate_to_base_guard: false,
            delay: HuntDelay::Ticks(context.ticks_per_second.saturating_mul(3)),
            return_immediately: true,
        };
    }
    if context.target_is_legal {
        return AircraftGuardPlan {
            destination: None,
            clear_target: false,
            target_juicy_threat: false,
            mission_to_assign: Some(AircraftMission::Attack),
            enter_idle_mode: false,
            scatter: false,
            delegate_to_base_guard: false,
            delay: HuntDelay::Ticks(1),
            return_immediately: true,
        };
    }
    AircraftGuardPlan {
        destination: None,
        clear_target: false,
        target_juicy_threat: false,
        mission_to_assign: None,
        enter_idle_mode: false,
        scatter: false,
        delegate_to_base_guard: true,
        delay: HuntDelay::Ticks(context.base_guard_area_delay),
        return_immediately: true,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AircraftAttackStage {
    ValidateAttackZone,
    PickAttackLocation,
    TakeOff,
    FlyToPosition,
    FireAtTarget,
    FireAtTargetSecondShot,
    ReturnToBase,
    Unknown(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AircraftAttackFireResult {
    Ok,
    Cloaked,
    Rearm,
    Facing,
    InRange,
    Ammo,
    Cant,
    Illegal,
    Other(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AircraftAttackContext {
    pub fixed_wing: bool,
    pub stage: AircraftAttackStage,
    pub target_is_legal: bool,
    pub navcom_is_legal: bool,
    pub selected_attack_cell: Option<u32>,
    pub takeoff_completed: bool,
    pub in_radio_contact_with_building: bool,
    pub direction_to_navcom: DirType,
    pub secondary_facing_current: DirType,
    pub direction_to_target: DirType,
    pub fire_direction_from_aircraft_to_navcom: DirType,
    pub distance_to_navcom: i32,
    pub fire_result: AircraftAttackFireResult,
    pub ammo: i32,
    pub target_is_in_weapon_range: bool,
    pub is_loaner: bool,
    pub house_is_human: bool,
    pub curley_shuffle: bool,
    pub normal_delay: i32,
    pub random_delay_0_to_2: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AircraftAttackPlan {
    pub delegate_to_hunt: bool,
    pub next_stage: Option<AircraftAttackStage>,
    pub assign_destination_to_attack_cell: Option<u32>,
    pub clear_destination: bool,
    pub clear_target: bool,
    pub process_takeoff: bool,
    pub set_primary_current: Option<DirType>,
    pub primary_desired: Option<DirType>,
    pub secondary_desired: Option<DirType>,
    pub break_radio_contact: bool,
    pub do_uncloak: bool,
    pub fire_target: bool,
    pub notify_target_incoming: bool,
    pub enter_idle_mode: bool,
    pub delay: HuntDelay,
    pub return_immediately: bool,
}

impl AircraftAttackPlan {
    fn new(normal_delay: i32, random_delay_0_to_2: i32) -> Self {
        Self {
            delegate_to_hunt: false,
            next_stage: None,
            assign_destination_to_attack_cell: None,
            clear_destination: false,
            clear_target: false,
            process_takeoff: false,
            set_primary_current: None,
            primary_desired: None,
            secondary_desired: None,
            break_radio_contact: false,
            do_uncloak: false,
            fire_target: false,
            notify_target_incoming: false,
            enter_idle_mode: false,
            delay: HuntDelay::NormalPlusRandom2 {
                normal_delay,
                random_value: random_delay_0_to_2.clamp(0, 2),
            },
            return_immediately: false,
        }
    }
}

/// Port of AircraftClass::Mission_Attack for rotorcraft. Fixed-wing aircraft
/// use the source Mission_Hunt state machine; target movement, facing,
/// incoming markers, weapon execution and radio are expressed as actions.
pub fn aircraft_mission_attack(context: AircraftAttackContext) -> AircraftAttackPlan {
    let mut plan = AircraftAttackPlan::new(
        context.normal_delay,
        context.random_delay_0_to_2,
    );
    if context.fixed_wing {
        plan.delegate_to_hunt = true;
        plan.return_immediately = true;
        return plan;
    }

    match context.stage {
        AircraftAttackStage::ValidateAttackZone => {
            plan.next_stage = Some(if context.target_is_legal {
                AircraftAttackStage::PickAttackLocation
            } else {
                AircraftAttackStage::ReturnToBase
            });
        }
        AircraftAttackStage::PickAttackLocation => {
            if context.target_is_legal {
                plan.assign_destination_to_attack_cell = context.selected_attack_cell;
                plan.next_stage = Some(if context.selected_attack_cell.is_some() {
                    AircraftAttackStage::TakeOff
                } else {
                    AircraftAttackStage::ReturnToBase
                });
            } else {
                plan.next_stage = Some(AircraftAttackStage::ReturnToBase);
            }
        }
        AircraftAttackStage::TakeOff => {
            if !context.target_is_legal {
                plan.next_stage = Some(AircraftAttackStage::ReturnToBase);
            } else {
                plan.process_takeoff = true;
                if context.takeoff_completed {
                    plan.next_stage = Some(AircraftAttackStage::FlyToPosition);
                    plan.break_radio_contact = context.in_radio_contact_with_building;
                    let difference = context.direction_to_navcom.signed_difference(
                        context.secondary_facing_current,
                    ) as i32;
                    let skew = difference.clamp(-128, 128);
                    plan.set_primary_current = Some(context.secondary_facing_current.wrapping_add(skew));
                }
                plan.delay = HuntDelay::Ticks(1);
                plan.return_immediately = true;
            }
        }
        AircraftAttackStage::FlyToPosition => {
            if !context.target_is_legal {
                plan.next_stage = Some(AircraftAttackStage::ReturnToBase);
            } else if !context.navcom_is_legal {
                plan.next_stage = Some(AircraftAttackStage::PickAttackLocation);
                plan.delay = HuntDelay::Ticks(1);
                plan.return_immediately = true;
                return plan;
            } else if context.distance_to_navcom < 0x0200 {
                plan.secondary_desired = Some(context.direction_to_target);
                if context.distance_to_navcom < 0x0010 {
                    plan.next_stage = Some(AircraftAttackStage::FireAtTarget);
                    plan.clear_destination = true;
                }
            } else {
                plan.secondary_desired = Some(context.fire_direction_from_aircraft_to_navcom);
                plan.delay = HuntDelay::Ticks(1);
                plan.return_immediately = true;
            }
            if !plan.return_immediately {
                plan.delay = HuntDelay::Ticks(1);
                plan.return_immediately = true;
            }
        }
        AircraftAttackStage::FireAtTarget | AircraftAttackStage::FireAtTargetSecondShot => {
            if !context.target_is_legal {
                plan.next_stage = Some(AircraftAttackStage::ReturnToBase);
                plan.delay = HuntDelay::Ticks(1);
                plan.return_immediately = true;
                return plan;
            }
            plan.primary_desired = Some(context.direction_to_target);
            plan.secondary_desired = Some(context.direction_to_target);
            let second_shot = context.stage == AircraftAttackStage::FireAtTargetSecondShot;
            match context.fire_result {
                AircraftAttackFireResult::Cloaked => {
                    plan.do_uncloak = true;
                }
                AircraftAttackFireResult::Ok => {
                    plan.fire_target = true;
                    plan.notify_target_incoming = true;
                    if !second_shot {
                        plan.next_stage = Some(AircraftAttackStage::FireAtTargetSecondShot);
                    } else if context.ammo != 0 {
                        plan.next_stage = Some(if context.curley_shuffle {
                            AircraftAttackStage::PickAttackLocation
                        } else {
                            AircraftAttackStage::FireAtTarget
                        });
                    } else {
                        plan.next_stage = Some(AircraftAttackStage::ReturnToBase);
                    }
                }
                AircraftAttackFireResult::Rearm => {}
                AircraftAttackFireResult::Facing if !second_shot => {}
                AircraftAttackFireResult::Facing => {
                    plan.next_stage = Some(if context.ammo == 0 {
                        AircraftAttackStage::ReturnToBase
                    } else if !context.target_is_in_weapon_range {
                        AircraftAttackStage::PickAttackLocation
                    } else if context.curley_shuffle {
                        AircraftAttackStage::PickAttackLocation
                    } else {
                        AircraftAttackStage::FireAtTarget
                    });
                }
                AircraftAttackFireResult::InRange
                | AircraftAttackFireResult::Ammo
                | AircraftAttackFireResult::Cant
                | AircraftAttackFireResult::Illegal
                | AircraftAttackFireResult::Other(_) => {
                    plan.next_stage = Some(if context.ammo == 0 {
                        AircraftAttackStage::ReturnToBase
                    } else if !second_shot {
                        AircraftAttackStage::FireAtTargetSecondShot
                    } else if !context.target_is_in_weapon_range || context.curley_shuffle {
                        AircraftAttackStage::PickAttackLocation
                    } else {
                        AircraftAttackStage::FireAtTarget
                    });
                }
            }
            plan.delay = HuntDelay::Ticks(1);
            plan.return_immediately = true;
        }
        AircraftAttackStage::ReturnToBase => {
            plan.clear_target = context.ammo == 0 && (context.is_loaner || context.house_is_human);
            plan.clear_destination = true;
            plan.enter_idle_mode = true;
        }
        AircraftAttackStage::Unknown(_) => {}
    }
    plan
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AircraftMoveStage {
    FixedWingTakeOff,
    FixedWingFlyTowardTarget,
    ValidateLandingZone,
    TakeOff,
    FlyToLandingZone,
    Land,
    Unknown(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AircraftMoveContext {
    pub fixed_wing: bool,
    pub stage: AircraftMoveStage,
    pub is_loaner: bool,
    pub team_is_valid: bool,
    pub navcom_is_legal: bool,
    pub landing_zone_is_clear: bool,
    pub landing_cell_is_available: bool,
    pub replacement_landing_cell: Option<u32>,
    pub takeoff_completed: bool,
    pub in_radio_contact_with_building: bool,
    pub current_cell_is_contacted_building: bool,
    pub distance_to_navcom: i32,
    pub navcom_direction: DirType,
    pub target_is_legal: bool,
    pub direction_to_target: DirType,
    pub pose_direction: DirType,
    pub replacement_landing_target_is_cell: bool,
    pub landing_completes_as_crash: bool,
    pub docking_building_is_accepted: bool,
    pub good_landing_zone_cell: u32,
    pub navcom_is_cell_target: bool,
    pub is_taking_off: bool,
    pub process_landing_completes: bool,
    pub mission_queue_is_none: bool,
    pub normal_delay: i32,
    pub random_delay_0_to_2: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AircraftMovePlan {
    pub next_stage: Option<AircraftMoveStage>,
    pub destination: Option<IdleModeDestination>,
    pub clear_destination: bool,
    pub update_team_mission_target: bool,
    pub mission_to_assign: Option<AircraftMission>,
    pub enter_idle_mode: bool,
    pub commence: bool,
    pub process_takeoff: bool,
    pub process_fly_to: bool,
    pub process_landing: bool,
    pub stop_taking_off: bool,
    pub set_speed: Option<u8>,
    pub primary_desired: Option<DirType>,
    pub secondary_desired: Option<DirType>,
    pub break_radio_contact: bool,
    pub force_crash_and_delete: bool,
    pub delay: HuntDelay,
    pub return_immediately: bool,
}

impl AircraftMovePlan {
    fn new(normal_delay: i32, random_delay_0_to_2: i32) -> Self {
        Self {
            next_stage: None,
            destination: None,
            clear_destination: false,
            update_team_mission_target: false,
            mission_to_assign: None,
            enter_idle_mode: false,
            commence: false,
            process_takeoff: false,
            process_fly_to: false,
            process_landing: false,
            stop_taking_off: false,
            set_speed: None,
            primary_desired: None,
            secondary_desired: None,
            break_radio_contact: false,
            force_crash_and_delete: false,
            delay: HuntDelay::NormalPlusRandom2 {
                normal_delay,
                random_value: random_delay_0_to_2.clamp(0, 2),
            },
            return_immediately: false,
        }
    }
}

/// Port of AircraftClass::Mission_Move for fixed-wing and rotorcraft. The
/// simulation supplies map/target/building outcomes; this function describes
/// state and side-effect requests in original source order.
pub fn aircraft_mission_move(context: AircraftMoveContext) -> AircraftMovePlan {
    let mut plan = AircraftMovePlan::new(
        context.normal_delay,
        context.random_delay_0_to_2,
    );

    if context.fixed_wing {
        match context.stage {
            AircraftMoveStage::FixedWingTakeOff => {
                plan.process_takeoff = true;
                if context.takeoff_completed {
                    plan.stop_taking_off = true;
                    plan.set_speed = Some(u8::MAX);
                    plan.break_radio_contact = context.in_radio_contact_with_building
                        && context.current_cell_is_contacted_building;
                    plan.next_stage = Some(AircraftMoveStage::FixedWingFlyTowardTarget);
                }
                plan.delay = HuntDelay::Ticks(1);
                plan.return_immediately = true;
            }
            AircraftMoveStage::FixedWingFlyTowardTarget => {
                plan.primary_desired = Some(context.navcom_direction);
                if context.distance_to_navcom < 0x00C0 {
                    if !context.is_loaner {
                        if context.docking_building_is_accepted {
                            plan.clear_destination = true;
                            plan.mission_to_assign = Some(AircraftMission::Enter);
                        } else {
                            plan.destination = Some(IdleModeDestination::GoodLandingZone(
                                context.good_landing_zone_cell,
                            ));
                            if context.navcom_is_cell_target {
                                plan.process_landing = true;
                                if context.landing_completes_as_crash {
                                    plan.force_crash_and_delete = true;
                                    plan.delay = HuntDelay::Ticks(1);
                                    plan.return_immediately = true;
                                    return plan;
                                }
                                plan.delay = HuntDelay::Ticks(500);
                                plan.return_immediately = true;
                                return plan;
                            }
                            plan.mission_to_assign = Some(AircraftMission::Move);
                        }
                        plan.commence = true;
                    } else if !context.team_is_valid {
                        plan.enter_idle_mode = true;
                    }
                    plan.delay = HuntDelay::Ticks(1);
                    plan.return_immediately = true;
                } else {
                    plan.delay = HuntDelay::Ticks(5);
                    plan.return_immediately = true;
                }
            }
            _ => {
                plan.delay = HuntDelay::Ticks(5);
                plan.return_immediately = true;
            }
        }
        return plan;
    }

    match context.stage {
        AircraftMoveStage::ValidateLandingZone => {
            if !context.navcom_is_legal {
                plan.enter_idle_mode = true;
            } else if !context.landing_zone_is_clear || !context.landing_cell_is_available {
                plan.destination = context.replacement_landing_cell.map(IdleModeDestination::AssignedCell);
                plan.update_team_mission_target = context.team_is_valid;
            } else {
                plan.next_stage = Some(AircraftMoveStage::TakeOff);
            }
        }
        AircraftMoveStage::TakeOff => {
            if !context.navcom_is_legal {
                plan.next_stage = Some(AircraftMoveStage::ValidateLandingZone);
            } else {
                plan.process_takeoff = true;
                if context.takeoff_completed {
                    plan.break_radio_contact = context.in_radio_contact_with_building
                        && context.current_cell_is_contacted_building;
                    plan.next_stage = Some(AircraftMoveStage::FlyToLandingZone);
                }
                plan.delay = HuntDelay::Ticks(1);
                plan.return_immediately = true;
            }
        }
        AircraftMoveStage::FlyToLandingZone => {
            if context.landing_zone_is_clear {
                plan.process_fly_to = true;
                if context.distance_to_navcom < 0x0080 {
                    plan.secondary_desired = Some(if context.target_is_legal {
                        context.direction_to_target
                    } else {
                        context.pose_direction
                    });
                    if context.distance_to_navcom < 0x0010 {
                        plan.next_stage = Some(AircraftMoveStage::Land);
                    }
                } else {
                    plan.secondary_desired = Some(context.navcom_direction);
                }
            } else {
                plan.destination = context.replacement_landing_cell.map(IdleModeDestination::AssignedCell);
                plan.update_team_mission_target = context.team_is_valid;
                if !context.navcom_is_legal {
                    plan.next_stage = Some(AircraftMoveStage::Land);
                }
            }
            plan.delay = HuntDelay::Ticks(1);
            plan.return_immediately = true;
        }
        AircraftMoveStage::Land => {
            if context.is_taking_off {
                plan.destination = context.replacement_landing_cell.map(IdleModeDestination::AssignedCell);
                plan.update_team_mission_target = context.team_is_valid;
                plan.next_stage = Some(AircraftMoveStage::TakeOff);
            }
            plan.process_landing = true;
            if context.process_landing_completes && context.mission_queue_is_none {
                plan.enter_idle_mode = true;
            }
            plan.delay = HuntDelay::Ticks(1);
            plan.return_immediately = true;
        }
        _ => {}
    }
    plan
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AircraftUnloadStage {
    SearchForLandingZone,
    FlyToLandingZone,
    LandOnLandingZone,
    UnloadPassengers,
    TakeOff,
    Unknown(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnloadDestination {
    Cell(u32),
    Waypoint(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AircraftUnloadContext {
    pub fixed_wing: bool,
    pub stage: AircraftUnloadStage,
    pub height: i32,
    pub flight_level: i32,
    pub navcom_is_legal: bool,
    pub aircraft_coordinate_matches_navcom: bool,
    pub landing_zone_is_clear: bool,
    pub attached_passenger_team_origin: Option<i32>,
    pub reinforcement_waypoint: i32,
    pub replacement_landing_cell: Option<u32>,
    pub team_is_valid: bool,
    pub is_tethered: bool,
    pub has_attached_passenger: bool,
    pub has_attached_passenger_after_exit: bool,
    pub exit_object_succeeded: bool,
    pub is_taking_off: bool,
    pub takeoff_completed: bool,
    pub landing_completed: bool,
    pub distance_to_navcom: i32,
    pub pose_direction: DirType,
    pub primary_facing_desired: DirType,
    pub in_radio_contact_with_origin_building: bool,
    pub normal_delay: i32,
    pub random_delay_0_to_2: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AircraftUnloadPlan {
    pub delegate_to_hunt: bool,
    pub next_stage: Option<AircraftUnloadStage>,
    pub destination: Option<UnloadDestination>,
    pub update_team_mission_target: bool,
    pub process_takeoff: bool,
    pub process_fly_to: bool,
    pub process_landing: bool,
    pub set_secondary_facing: Option<DirType>,
    pub break_radio_contact: bool,
    pub detach_passenger: bool,
    pub pick_up_aircraft_from_map: bool,
    pub attempt_exit_object: bool,
    pub delete_passenger_on_exit_failure: bool,
    pub place_aircraft_on_map: bool,
    pub enter_idle_mode: bool,
    pub delay: HuntDelay,
    pub return_immediately: bool,
}

/// Port of AircraftClass::Mission_Unload. Fixed-wing transport delegates to
/// Mission_Hunt after copying NavCom to TarCom; rotorcraft preserves the
/// landing-zone, takeoff, unload, and retry stages.
pub fn aircraft_mission_unload(context: AircraftUnloadContext) -> AircraftUnloadPlan {
    let mut plan = AircraftUnloadPlan {
        delegate_to_hunt: context.fixed_wing,
        next_stage: None,
        destination: None,
        update_team_mission_target: false,
        process_takeoff: false,
        process_fly_to: false,
        process_landing: false,
        set_secondary_facing: None,
        break_radio_contact: false,
        detach_passenger: false,
        pick_up_aircraft_from_map: false,
        attempt_exit_object: false,
        delete_passenger_on_exit_failure: false,
        place_aircraft_on_map: false,
        enter_idle_mode: false,
        delay: HuntDelay::NormalPlusRandom2 {
            normal_delay: context.normal_delay,
            random_value: context.random_delay_0_to_2.clamp(0, 2),
        },
        return_immediately: false,
    };
    if context.fixed_wing {
        plan.return_immediately = true;
        return plan;
    }

    match context.stage {
        AircraftUnloadStage::SearchForLandingZone => {
            if context.height == 0
                && (context.navcom_is_legal || context.aircraft_coordinate_matches_navcom)
            {
                plan.next_stage = Some(AircraftUnloadStage::UnloadPassengers);
            } else if !context.landing_zone_is_clear {
                plan.destination = if let Some(origin) = context.attached_passenger_team_origin.filter(|origin| *origin != -1) {
                    Some(UnloadDestination::Waypoint(origin))
                } else {
                    context.replacement_landing_cell.map(UnloadDestination::Cell)
                };
                plan.update_team_mission_target = plan.destination.is_some() && context.team_is_valid;
            } else if context.height == context.flight_level {
                plan.next_stage = Some(AircraftUnloadStage::FlyToLandingZone);
            } else {
                plan.next_stage = Some(AircraftUnloadStage::TakeOff);
            }
        }
        AircraftUnloadStage::FlyToLandingZone => {
            if context.landing_zone_is_clear {
                plan.process_fly_to = true;
                if context.distance_to_navcom < 0x0100 {
                    plan.set_secondary_facing = Some(context.pose_direction);
                    if context.distance_to_navcom < 0x0010 {
                        plan.next_stage = Some(AircraftUnloadStage::LandOnLandingZone);
                    }
                    plan.delay = HuntDelay::Ticks(1);
                    plan.return_immediately = true;
                } else {
                    plan.set_secondary_facing = Some(context.primary_facing_desired);
                    plan.delay = HuntDelay::Ticks(5);
                    plan.return_immediately = true;
                }
            } else {
                plan.next_stage = Some(AircraftUnloadStage::SearchForLandingZone);
            }
        }
        AircraftUnloadStage::LandOnLandingZone => {
            if context.is_taking_off {
                plan.next_stage = Some(AircraftUnloadStage::TakeOff);
            } else {
                plan.process_landing = true;
                if context.landing_completed {
                    plan.next_stage = Some(AircraftUnloadStage::UnloadPassengers);
                }
            }
            plan.delay = HuntDelay::Ticks(1);
            plan.return_immediately = true;
        }
        AircraftUnloadStage::UnloadPassengers => {
            if !context.is_tethered {
                if context.has_attached_passenger {
                    plan.detach_passenger = true;
                    plan.pick_up_aircraft_from_map = true;
                    plan.attempt_exit_object = true;
                    plan.delete_passenger_on_exit_failure = !context.exit_object_succeeded;
                    plan.place_aircraft_on_map = true;
                    if !context.has_attached_passenger_after_exit {
                        plan.enter_idle_mode = true;
                    }
                } else {
                    plan.enter_idle_mode = true;
                }
            }
        }
        AircraftUnloadStage::TakeOff => {
            plan.process_takeoff = true;
            if context.takeoff_completed {
                if context.has_attached_passenger {
                    plan.next_stage = Some(AircraftUnloadStage::SearchForLandingZone);
                    plan.break_radio_contact = context.in_radio_contact_with_origin_building;
                } else {
                    plan.enter_idle_mode = true;
                }
            }
            plan.delay = HuntDelay::Ticks(1);
            plan.return_immediately = true;
        }
        AircraftUnloadStage::Unknown(_) => {}
    }
    plan
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
    fn can_enter_cell_applies_radar_clearance_and_fog_rules_in_order() {
        use AircraftCellOccupier::{None, NonTechno, Techno};
        assert_eq!(can_enter_cell(false, None, 0, true, true, false, || panic!("out of radar")), AircraftMoveResult::No);
        assert_eq!(can_enter_cell(true, NonTechno, 0, false, false, false, || false), AircraftMoveResult::No);
        assert_eq!(can_enter_cell(true, None, 0, false, false, true, || true), AircraftMoveResult::Ok);
        assert_eq!(can_enter_cell(true, None, 0, true, true, false, || true), AircraftMoveResult::No);
    }

    #[test]
    fn can_enter_cell_preserves_enemy_occupier_visibility_exceptions() {
        use AircraftCellOccupier::Techno;
        let enemy = Techno {
            allied_to_aircraft: false,
            cloaked_from_aircraft: false,
            is_building: false,
            building_is_invisible: false,
        };
        assert_eq!(can_enter_cell(true, enemy, 0, false, false, true, || false), AircraftMoveResult::No);
        assert_eq!(
            can_enter_cell(true, Techno { cloaked_from_aircraft: true, ..enemy }, 0, false, false, true, || panic!("cloaked occupier bypasses cell check")),
            AircraftMoveResult::Ok
        );
        assert_eq!(
            can_enter_cell(true, Techno { is_building: true, building_is_invisible: true, ..enemy }, 0, false, false, true, || panic!("invisible building bypasses cell check")),
            AircraftMoveResult::Ok
        );
        assert_eq!(
            can_enter_cell(true, Techno { is_building: true, building_is_invisible: true, ..enemy }, 1, false, false, true, || panic!("scenario init bypasses cell check")),
            AircraftMoveResult::Ok
        );
    }

    #[test]
    fn can_enter_cell_rechecks_clearance_for_allied_occupiers() {
        let allied = AircraftCellOccupier::Techno {
            allied_to_aircraft: true,
            cloaked_from_aircraft: false,
            is_building: false,
            building_is_invisible: false,
        };
        assert_eq!(can_enter_cell(true, allied, 0, false, false, true, || false), AircraftMoveResult::No);
        assert_eq!(can_enter_cell(true, allied, 0, false, false, true, || true), AircraftMoveResult::Ok);
    }

    #[test]
    fn cell_seems_ok_ignores_self_unless_strict_and_checks_occupied_or_reserved_cells() {
        let entries = [
            AircraftTrafficEntry { id: 1, present: true, in_limbo: false, current_cell: 10, navcom_cell: None },
            AircraftTrafficEntry { id: 2, present: true, in_limbo: false, current_cell: 20, navcom_cell: Some(30) },
            AircraftTrafficEntry { id: 3, present: true, in_limbo: true, current_cell: 40, navcom_cell: Some(50) },
            AircraftTrafficEntry { id: 4, present: false, in_limbo: false, current_cell: 60, navcom_cell: Some(70) },
        ];
        assert!(cell_seems_ok(10, 1, false, &entries));
        assert!(!cell_seems_ok(10, 1, true, &entries));
        assert!(!cell_seems_ok(30, 1, false, &entries));
        assert!(cell_seems_ok(40, 1, false, &entries));
        assert!(cell_seems_ok(60, 1, false, &entries));
    }

    #[test]
    fn landing_zone_requires_legal_radar_cell_and_only_accepts_known_occupants() {
        use LandingCellOccupier::*;
        assert!(!landing_zone_is_clear(false, true, ThisAircraft, || panic!("invalid target")));
        assert!(!landing_zone_is_clear(true, false, Empty, || panic!("off-radar cell")));
        assert!(landing_zone_is_clear(true, true, ThisAircraft, || panic!("self is accepted")));
        assert!(landing_zone_is_clear(true, true, ContactedObject, || panic!("contact is accepted")));
        assert!(!landing_zone_is_clear(true, true, OtherObject, || panic!("other occupant blocks")));
        assert!(landing_zone_is_clear(true, true, Empty, || true));
        assert!(!landing_zone_is_clear(true, true, Empty, || false));
    }

    #[test]
    fn good_landing_zone_prefers_nearest_friendly_building_with_support_bias() {
        let buildings = [
            LandingBuildingCandidate {
                present: true, in_limbo: false, owned_by_aircraft_house: false,
                cell: 10, distance: 1, is_preferred_support_building: true,
            },
            LandingBuildingCandidate {
                present: true, in_limbo: true, owned_by_aircraft_house: true,
                cell: 20, distance: 1, is_preferred_support_building: true,
            },
            LandingBuildingCandidate {
                present: true, in_limbo: false, owned_by_aircraft_house: true,
                cell: 30, distance: 80, is_preferred_support_building: false,
            },
            LandingBuildingCandidate {
                present: true, in_limbo: false, owned_by_aircraft_house: true,
                cell: 40, distance: 200, is_preferred_support_building: true,
            },
        ];
        // Preferred support distance is 200 / 4 = 50, so the closer building wins.
        assert_eq!(good_landing_zone(99, &buildings), 40);
    }

    #[test]
    fn good_landing_zone_preserves_first_tie_and_falls_back_to_current_cell() {
        let tied = [
            LandingBuildingCandidate {
                present: true, in_limbo: false, owned_by_aircraft_house: true,
                cell: 30, distance: 20, is_preferred_support_building: false,
            },
            LandingBuildingCandidate {
                present: true, in_limbo: false, owned_by_aircraft_house: true,
                cell: 40, distance: 80, is_preferred_support_building: true,
            },
        ];
        assert_eq!(good_landing_zone(99, &tied), 30);
        assert_eq!(good_landing_zone(99, &[]), 99);
    }

    #[test]
    fn new_lz_preserves_original_when_invalid_or_still_usable() {
        let mut scans = 0;
        let mut move_cell = |_: u32, _: DirType, _: i32| { scans += 1; 0 };
        let none = find_new_landing_zone(
            false, false, 100, 1024, 256, 256, || 0,
            &mut move_cell, |coord| coord, |_| true, |_| true, |_| true,
        );
        assert_eq!(none, None);
        let none = find_new_landing_zone(
            true, true, 100, 1024, 256, 256, || 0,
            &mut move_cell, |coord| coord, |_| true, |_| true, |_| true,
        );
        assert_eq!(none, None);
        assert_eq!(scans, 0);
    }

    #[test]
    fn new_lz_scans_eight_facings_in_each_concentric_radius() {
        let mut observed = Vec::new();
        let result = find_new_landing_zone(
            true, false, 1_000, 768, 256, 256,
            || 0,
            |center, direction, distance| {
                observed.push((direction.0, distance));
                center + u32::from(direction.0) + distance as u32
            },
            |coord| coord,
            |_| true,
            |cell| cell == 1_032,
            |_| true,
        );
        assert_eq!(result, Some(1_032));
        assert_eq!(observed.len(), 2);
        assert_eq!(observed[0], (0, 0));
        assert_eq!(observed[1], (32, 0));
    }

    #[test]
    fn new_lz_wraps_facing_modifier_and_skips_duplicate_cells_per_ring() {
        let mut seen = Vec::new();
        let result = find_new_landing_zone(
            true, false, 0, 256, 256, 128,
            || 7,
            |_, direction, _| {
                seen.push(direction.0);
                12 // Every direction resolves to the same cell.
            },
            |coord| coord,
            |_| true,
            |_| false,
            |_| true,
        );
        assert_eq!(result, None);
        assert_eq!(seen, [224, 0, 32, 64, 96, 128, 160, 192]);
    }

    #[test]
    fn new_lz_checks_radar_before_landing_and_traffic_predicates() {
        let mut clear_checks = 0;
        let result = find_new_landing_zone(
            true, false, 0, 256, 256, 256,
            || 0,
            |_, _, _| 4,
            |coord| coord,
            |_| false,
            |_| { clear_checks += 1; true },
            |_| true,
        );
        assert_eq!(result, None);
        assert_eq!(clear_checks, 0);
    }

    #[test]
    fn good_fire_location_scans_outer_ring_and_chooses_closest_pair() {
        let mut visited = Vec::new();
        let chosen = good_fire_location(
            true, 1000, 500, None, 1024, 128, false,
            |target, dir, radius| {
                visited.push((dir.0, radius));
                target + radius as u32 + u32::from(dir.0)
            },
            |coord| coord,
            |coord, flight| coord - flight as u32,
            |_| true,
            |_| true,
            |_| true,
            |coord, _, _| (coord as i32 - 1500).abs(),
            |_| false,
        );
        // Outer ring is radius=768. Best and second-best are retained by
        // strict distance comparison, then the 50% branch selects second.
        assert_eq!(visited.len(), 16);
        assert!(visited.iter().all(|(_, radius)| *radius == 768));
        assert!(chosen.is_some());
    }

    #[test]
    fn good_fire_location_uses_target_navigation_coordinate_for_moving_targets() {
        let mut received = Vec::new();
        let result = good_fire_location(
            true, 0, 50, Some(75), 512, 10, false,
            |target, _, _| target,
            |_| 1,
            |coord, _| coord,
            |_| true,
            |_| true,
            |_| true,
            |_, alternate, origin| {
                received.push((alternate, origin));
                1
            },
            |_| true,
        );
        assert!(result.is_some());
        assert!(received.iter().all(|(alternate, origin)| *alternate == Some(75) && *origin == 75));
    }

    #[test]
    fn good_fire_location_checks_visibility_only_in_normal_games() {
        let mut visibility_checks = 0;
        let result = good_fire_location(
            true, 0, 0, None, 512, 0, false,
            |target, _, _| target,
            |_| 1,
            |coord, _| coord,
            |_| true,
            |_| { visibility_checks += 1; false },
            |_| true,
            |_, _, _| 1,
            |_| true,
        );
        assert!(result.is_some());
        assert_eq!(visibility_checks, 0);
    }

    #[test]
    fn good_fire_location_returns_none_without_search_for_invalid_target_or_range() {
        let mut moves = 0;
        let invalid = good_fire_location(
            false, 0, 0, None, 2048, 0, false,
            |coord, _, _| { moves += 1; coord },
            |_| 0,
            |coord, _| coord,
            |_| true,
            |_| true,
            |_| true,
            |_, _, _| 0,
            |_| true,
        );
        assert_eq!(invalid, None);
        let too_short = good_fire_location(
            true, 0, 0, None, 512, 0, false,
            |coord, _, _| { moves += 1; coord },
            |_| 0,
            |coord, _| coord,
            |_| true,
            |_| true,
            |_| true,
            |_, _, _| 0,
            |_| true,
        );
        assert_eq!(too_short, None);
        assert_eq!(moves, 0);
    }

    #[test]
    fn passenger_exit_uses_source_direction_order_and_radio_handshake() {
        assert_eq!(PASSENGER_EXIT_FACING_ORDER, [4, 5, 3, 7, 1, 0, 6, 2]);
        let mut tested = Vec::new();
        let mut spawn = None;
        let result = plan_passenger_exit(
            100,
            0x1234_5678,
            |cell, facing| { tested.push(facing); cell + u32::from(facing) },
            |cell| cell == 105,
            |coordinate, direction| { spawn = Some((coordinate, direction)); true },
            true,
        ).unwrap();
        assert_eq!(tested, [4, 5]);
        assert_eq!(result.candidate_cell, 105);
        assert_eq!(result.spawn_coordinate, 0x1234_5678);
        assert_eq!(result.facing, DirType(160));
        assert_eq!(spawn, Some((0x1234_5678, DirType(160))));
        assert!(result.assign_move_mission);
        assert!(result.assign_candidate_destination);
        assert!(result.send_unload_after_radio_roger);
        assert!(!result.look_incremental);
    }

    #[test]
    fn passenger_exit_fails_safely_without_cell_or_unlimbo() {
        let none_found = plan_passenger_exit(0, 1, |_, _| 0, |_| false, |_, _| true, false);
        assert_eq!(none_found, None);
        let unlimbo_failed = plan_passenger_exit(0, 1, |_, _| 5, |_| true, |_, _| false, false);
        assert_eq!(unlimbo_failed, None);
    }

    #[test]
    fn edge_of_world_locks_aircraft_while_it_remains_inside_radar() {
        let plan = edge_of_world_plan(
            true, true, true,
            &[EvacuatingPassenger { passenger_id: 70, house_id: 7, counts_as_civ_evacuee: true, team_id: Some(3) }],
        );
        assert!(plan.lock_aircraft);
        assert!(plan.mark_civ_evacuation_for_houses.is_empty());
        assert!(plan.delete_passengers.is_empty());
        assert!(!plan.delete_aircraft);
    }

    #[test]
    fn retreating_aircraft_leaving_map_fulfills_evacuation_and_deletes_cargo() {
        let plan = edge_of_world_plan(
            false,
            true,
            true,
            &[
                EvacuatingPassenger { passenger_id: 100, house_id: 10, counts_as_civ_evacuee: true, team_id: Some(2) },
                EvacuatingPassenger { passenger_id: 110, house_id: 11, counts_as_civ_evacuee: false, team_id: None },
            ],
        );
        assert_eq!(plan.mark_civ_evacuation_for_houses, [10]);
        assert_eq!(plan.mark_teams_to_leave, [2]);
        assert_eq!(plan.delete_passengers, [100, 110]);
        assert!(plan.mark_aircraft_team_to_leave);
        assert!(plan.stun_aircraft);
        assert!(plan.delete_aircraft);
    }

    #[test]
    fn non_retreating_aircraft_outside_radar_is_not_deleted_by_this_handler() {
        let plan = edge_of_world_plan(false, false, true, &[]);
        assert!(!plan.lock_aircraft);
        assert!(!plan.stun_aircraft);
        assert!(!plan.delete_aircraft);
        assert!(!plan.mark_aircraft_team_to_leave);
    }

    #[test]
    fn rotation_ai_advances_primary_and_secondary_and_marks_zone_changes() {
        use crate::facing::FacingClass;
        let mut primary = FacingClass::with_direction(DirType::NORTH);
        primary.set_desired(DirType::EAST);
        let mut secondary = FacingClass::with_direction(DirType::SOUTH);
        secondary.set_desired(DirType::WEST);
        let result = rotation_ai(&mut primary, &mut secondary, false, 16);
        assert_eq!(result.primary_redraw_marks, 2);
        assert_eq!(result.secondary_redraw_marks, 2);
        assert_eq!(primary.current(), DirType(16));
        assert_eq!(secondary.current(), DirType(144));
    }

    #[test]
    fn fixed_wing_rotation_copies_primary_then_advances_secondary_again() {
        use crate::facing::FacingClass;
        let mut primary = FacingClass::with_direction(DirType::NORTH);
        primary.set_desired(DirType::EAST);
        let mut secondary = FacingClass::with_direction(DirType::SOUTH);
        let result = rotation_ai(&mut primary, &mut secondary, true, 16);
        assert_eq!(primary.current(), DirType(16));
        assert_eq!(secondary.current(), DirType(32));
        assert_eq!(result.primary_redraw_marks, 2);
        assert_eq!(result.secondary_redraw_marks, 2);
    }

    #[test]
    fn can_fire_cargo_and_camera_paths_obey_arm_range_and_radar_checks() {
        let mut base_calls = 0;
        let passenger_empty = AircraftFireRequest {
            is_passenger_aircraft: true, has_attached_passenger: false,
            has_primary_weapon: false, primary_weapon_is_camera: false,
            primary_bullet_is_parachuted: false, has_ammo: false,
            arm_timer: 0, distance_to_target: 0, aircraft_center_is_in_radar: true,
            is_fixed_wing: false, facing_difference: 0,
        };
        assert_eq!(can_fire_aircraft(passenger_empty, || { base_calls += 1; AircraftFireCheck::Ok }), AircraftFireCheck::Ammo);

        let camera = AircraftFireRequest {
            is_passenger_aircraft: false, has_attached_passenger: false,
            has_primary_weapon: true, primary_weapon_is_camera: true,
            primary_bullet_is_parachuted: false, has_ammo: false,
            arm_timer: 0, distance_to_target: 0x037F, aircraft_center_is_in_radar: true,
            is_fixed_wing: false, facing_difference: 0,
        };
        assert_eq!(can_fire_aircraft(camera, || { base_calls += 1; AircraftFireCheck::Other(1) }), AircraftFireCheck::Ok);
        assert_eq!(can_fire_aircraft(AircraftFireRequest { arm_timer: 1, ..camera }, || panic!("rearm path skips base")), AircraftFireCheck::Rearm);
        assert_eq!(can_fire_aircraft(AircraftFireRequest { distance_to_target: 0x0380, ..camera }, || panic!("camera path skips base")), AircraftFireCheck::Range);
        assert_eq!(can_fire_aircraft(AircraftFireRequest { aircraft_center_is_in_radar: false, ..camera }, || panic!("radar path skips base")), AircraftFireCheck::Range);
        assert_eq!(base_calls, 0);
    }

    #[test]
    fn can_fire_parachuted_bullet_requires_ammo_and_fixed_wing_facing() {
        let parachute = AircraftFireRequest {
            is_passenger_aircraft: false, has_attached_passenger: false,
            has_primary_weapon: true, primary_weapon_is_camera: false,
            primary_bullet_is_parachuted: true, has_ammo: false,
            arm_timer: 0, distance_to_target: 0, aircraft_center_is_in_radar: true,
            is_fixed_wing: true, facing_difference: 0,
        };
        assert_eq!(can_fire_aircraft(parachute, || AircraftFireCheck::Ok), AircraftFireCheck::Ammo);
        let armed = AircraftFireRequest { has_ammo: true, facing_difference: 17, ..parachute };
        assert_eq!(can_fire_aircraft(armed, || AircraftFireCheck::Ok), AircraftFireCheck::Facing);
        assert_eq!(can_fire_aircraft(AircraftFireRequest { facing_difference: 16, ..armed }, || AircraftFireCheck::Ok), AircraftFireCheck::Ok);
        assert_eq!(can_fire_aircraft(AircraftFireRequest { is_fixed_wing: false, facing_difference: 127, ..armed }, || AircraftFireCheck::Ok), AircraftFireCheck::Ok);
    }

    #[test]
    fn can_fire_ordinary_weapons_delegate_and_apply_fixed_wing_facing_limit() {
        let request = AircraftFireRequest {
            is_passenger_aircraft: false, has_attached_passenger: false,
            has_primary_weapon: true, primary_weapon_is_camera: false,
            primary_bullet_is_parachuted: false, has_ammo: true,
            arm_timer: 0, distance_to_target: 10, aircraft_center_is_in_radar: true,
            is_fixed_wing: true, facing_difference: -9,
        };
        assert_eq!(can_fire_aircraft(request, || AircraftFireCheck::Ok), AircraftFireCheck::Facing);
        assert_eq!(can_fire_aircraft(AircraftFireRequest { facing_difference: 8, ..request }, || AircraftFireCheck::Ok), AircraftFireCheck::Ok);
        assert_eq!(can_fire_aircraft(request, || AircraftFireCheck::Other(7)), AircraftFireCheck::Other(7));
    }

    #[test]
    fn paradrop_cargo_reattaches_failures_and_clears_arm_after_success() {
        let absent = paradrop_cargo_plan(false, false, true, true, 9);
        assert_eq!(absent.arm_after_call, 9);
        let failed = paradrop_cargo_plan(true, false, true, true, 9);
        assert!(failed.reattach_passenger);
        assert!(!failed.play_parachute_sound);
        assert_eq!(failed.arm_after_call, 9);
        let success = paradrop_cargo_plan(true, true, true, true, 9);
        assert!(!success.reattach_passenger);
        assert!(success.play_parachute_sound);
        assert!(success.remove_passenger_from_team);
        assert_eq!(success.assign_passenger_mission, Some("GUARD"));
        assert_eq!(success.arm_after_call, 0);
        let no_team = paradrop_cargo_plan(true, true, false, false, 9);
        assert_eq!(no_team.assign_passenger_mission, None);
        assert!(!no_team.remove_passenger_from_team);
    }

    #[test]
    fn aircraft_voice_tables_preserve_random_weighting_and_voice_gate() {
        assert_eq!(aircraft_voice_plan(AircraftVoicePrompt::Attack, 0, true, 4).unwrap().sample, "VOC_AFFIRM");
        assert_eq!(aircraft_voice_plan(AircraftVoicePrompt::Attack, 1, true, 4).unwrap().sample, "VOC_ACKNOWL");
        assert_eq!(aircraft_voice_plan(AircraftVoicePrompt::Move, 0, true, 4).unwrap().sample, "VOC_ACKNOWL");
        assert_eq!(aircraft_voice_plan(AircraftVoicePrompt::Select, 2, true, 4).unwrap().sample, "VOC_YESSIR");
        assert_eq!(aircraft_voice_plan(AircraftVoicePrompt::Select, 3, true, 4).unwrap().sample, "VOC_YESSIR");
        let muted = aircraft_voice_plan(AircraftVoicePrompt::Select, 5, false, 4).unwrap();
        assert_eq!(muted.sample, "VOC_AWAIT");
        assert!(!muted.should_play);
        assert_eq!(muted.volume.raw(), Fixed16::ONE.raw());
        assert_eq!(muted.speaker_id, -5);
        assert_eq!(aircraft_voice_plan(AircraftVoicePrompt::Select, 6, true, 0), None);
    }

    #[test]
    fn what_action_cell_applies_visibility_weapon_and_fixed_wing_rules_in_order() {
        use AircraftAction::*;
        assert_eq!(what_action_cell(Move, false, true, false), NoMove);
        assert_eq!(what_action_cell(Attack, false, true, false), NoMove);
        assert_eq!(what_action_cell(Attack, true, false, false), None);
        assert_eq!(what_action_cell(Move, true, true, true), NoMove);
        assert_eq!(what_action_cell(Move, true, true, false), Move);
        assert_eq!(what_action_cell(Select, false, false, true), Select);
    }

    #[test]
    fn what_action_object_blocks_self_attack_and_fixed_wing_move_as_source_does() {
        use AircraftAction::*;
        let context = AircraftObjectActionContext {
            has_passengers: false,
            height_above_ground: false,
            is_tethered: false,
            has_primary_weapon: false,
            player_controls_house: false,
            target_is_ally: false,
            target_is_building: false,
            radio_can_load_approved: false,
            is_fixed_wing: true,
            fallback_cell_action: Other(9),
            target_is_repair_building: false,
            repair_building_in_radio_contact: false,
            repair_building_has_attached_object: false,
        };
        assert_eq!(what_action_object(SelfAction, context), Other(9));
        assert_eq!(what_action_object(Attack, context), Other(9));
        assert_eq!(what_action_object(Move, context), NoMove);
    }

    #[test]
    fn what_action_object_allied_load_and_repair_building_overrides_are_conditional() {
        use AircraftAction::*;
        let base = AircraftObjectActionContext {
            has_passengers: true,
            height_above_ground: false,
            is_tethered: false,
            has_primary_weapon: true,
            player_controls_house: true,
            target_is_ally: true,
            target_is_building: true,
            radio_can_load_approved: true,
            is_fixed_wing: false,
            fallback_cell_action: Select,
            target_is_repair_building: false,
            repair_building_in_radio_contact: false,
            repair_building_has_attached_object: false,
        };
        assert_eq!(what_action_object(Select, base), Enter);
        let repair = AircraftObjectActionContext {
            radio_can_load_approved: false,
            target_is_repair_building: true,
            ..base
        };
        assert_eq!(what_action_object(Select, repair), Enter);
        assert_eq!(
            what_action_object(Select, AircraftObjectActionContext { repair_building_in_radio_contact: true, ..repair }),
            Select
        );
        assert_eq!(
            what_action_object(Select, AircraftObjectActionContext { repair_building_has_attached_object: true, ..repair }),
            Select
        );
    }

    #[test]
    fn fire_at_prioritizes_paradrop_cargo_over_camera_and_base_weapon() {
        let plan = aircraft_fire_at_plan(
            true, true, true, 1, true, &[1, 2], Some(1),
            true, true, false, 10, 20,
        );
        assert_eq!(plan.path, AircraftFireAtPath::ParadropCargo);
        assert!(plan.sight_requests.is_empty());
        assert!(!plan.clear_ammo);
        assert!(!plan.call_base_fire);
        assert!(!plan.return_bullet);
        assert_eq!(plan.dropping_bullet_speed, None);
    }

    #[test]
    fn camera_fire_consumes_ammo_rearms_and_reveals_correct_houses() {
        let single = aircraft_fire_at_plan(
            false, true, false, 7, true, &[7, 8], Some(7),
            false, false, true, 20, 30,
        );
        assert_eq!(single.path, AircraftFireAtPath::CameraSnapshot);
        assert_eq!(single.sight_requests, [AircraftSightRequest { house_id: 7, sight_range: 9 }]);
        assert!(single.clear_ammo);
        assert_eq!(single.rearm_timer_after_call, Some(30));
        assert!(!single.call_base_fire);

        let multiplayer = aircraft_fire_at_plan(
            false, true, true, 7, false, &[7, 9, 12], Some(7),
            false, false, false, 40, 50,
        );
        assert_eq!(multiplayer.sight_requests, [
            AircraftSightRequest { house_id: 7, sight_range: 9 },
            AircraftSightRequest { house_id: 9, sight_range: 9 },
            AircraftSightRequest { house_id: 12, sight_range: 9 },
        ]);
    }

    #[test]
    fn standard_fire_reveals_for_human_owner_and_speeds_dropping_bullets() {
        let plan = aircraft_fire_at_plan(
            false, false, false, 7, false, &[], Some(8),
            true, true, false, 0, 0,
        );
        assert_eq!(plan.path, AircraftFireAtPath::StandardWeapon);
        assert_eq!(plan.sight_requests, [AircraftSightRequest { house_id: 8, sight_range: 1 }]);
        assert!(plan.call_base_fire);
        assert!(plan.return_bullet);
        assert_eq!(plan.dropping_bullet_speed, Some((40, "MPH_MEDIUM_SLOW")));
        let no_bullet = aircraft_fire_at_plan(
            false, false, false, 7, false, &[], None,
            false, true, false, 0, 0,
        );
        assert!(no_bullet.sight_requests.is_empty());
        assert!(!no_bullet.return_bullet);
        assert_eq!(no_bullet.dropping_bullet_speed, None);
    }

    #[test]
    fn landing_takeoff_ai_is_gated_by_closed_door_and_transition_flags() {
        let input = LandingTakeoffAiInput {
            door_is_closed: false, is_landing: true, is_taking_off: false,
            height: 20, pixel_to_lepton_step: 10, flight_level: 100,
            fixed_wing: false, mission_is_enter: false, old_layer_is_ground: false,
            current_cell_matches_navcom: false, navcom_is_legal: false,
            navcom_targets_contact_object: false, in_radio_contact: false,
            im_in_reply_was_roger: false, landing_zone_is_clear: true,
            navcom_differs_from_contact: false,
        };
        let plan = landing_takeoff_ai(input, |_, _, _| true);
        assert_eq!(plan.height, 20);
        assert!(plan.is_landing);
        assert_eq!(plan.redraw_mark_count, 0);
        assert!(!plan.submit_to_new_layer);
        let idle = landing_takeoff_ai(
            LandingTakeoffAiInput { door_is_closed: true, is_landing: false, is_taking_off: false, ..input },
            |_, _, _| true,
        );
        assert_eq!(idle.redraw_mark_count, 0);
    }

    #[test]
    fn fixed_wing_landing_crashes_outside_enter_mission_before_mark_down() {
        let input = LandingTakeoffAiInput {
            door_is_closed: true, is_landing: true, is_taking_off: false,
            height: 5, pixel_to_lepton_step: 10, flight_level: 100,
            fixed_wing: true, mission_is_enter: false, old_layer_is_ground: false,
            current_cell_matches_navcom: true, navcom_is_legal: true,
            navcom_targets_contact_object: true, in_radio_contact: true,
            im_in_reply_was_roger: false, landing_zone_is_clear: true,
            navcom_differs_from_contact: false,
        };
        let plan = landing_takeoff_ai(input, |_, _, _| true);
        assert_eq!(plan.height, 0);
        assert!(!plan.is_landing);
        assert_eq!(plan.speed_override, Some(0));
        assert!(plan.clear_navcom);
        assert!(plan.destroyed);
        assert!(plan.remove_before_destruction);
        assert!(!plan.scatter);
        assert_eq!(plan.redraw_mark_count, 1);
    }

    #[test]
    fn helicopter_landing_scatter_occurs_only_when_contact_rejects_im_in() {
        let input = LandingTakeoffAiInput {
            door_is_closed: true, is_landing: true, is_taking_off: false,
            height: 4, pixel_to_lepton_step: 8, flight_level: 100,
            fixed_wing: false, mission_is_enter: false, old_layer_is_ground: false,
            current_cell_matches_navcom: false, navcom_is_legal: true,
            navcom_targets_contact_object: true, in_radio_contact: true,
            im_in_reply_was_roger: false, landing_zone_is_clear: true,
            navcom_differs_from_contact: false,
        };
        let plan = landing_takeoff_ai(input, |_, _, _| true);
        assert!(plan.scatter);
        assert!(!plan.destroyed);
        assert_eq!(plan.redraw_mark_count, 2);
    }

    #[test]
    fn takeoff_height_clamps_to_flight_level_and_unanchors_on_layer_change() {
        let input = LandingTakeoffAiInput {
            door_is_closed: true, is_landing: false, is_taking_off: true,
            height: 95, pixel_to_lepton_step: 10, flight_level: 100,
            fixed_wing: true, mission_is_enter: false, old_layer_is_ground: true,
            current_cell_matches_navcom: false, navcom_is_legal: true,
            navcom_targets_contact_object: false, in_radio_contact: true,
            im_in_reply_was_roger: false, landing_zone_is_clear: true,
            navcom_differs_from_contact: true,
        };
        let plan = landing_takeoff_ai(input, |height, _, _| height > 0);
        assert_eq!(plan.height, 100);
        assert!(!plan.is_taking_off);
        assert_eq!(plan.redraw_mark_count, 2);
        assert!(plan.remove_from_old_layer);
        assert!(plan.submit_to_new_layer);
        assert!(plan.remove_tether);
        assert!(plan.radio_over_out);
        assert!(!plan.establish_tether);
    }

    #[test]
    fn unsafe_helicopter_ground_transition_starts_corrective_takeoff_instead() {
        let input = LandingTakeoffAiInput {
            door_is_closed: true, is_landing: true, is_taking_off: false,
            height: 10, pixel_to_lepton_step: 5, flight_level: 100,
            fixed_wing: false, mission_is_enter: false, old_layer_is_ground: false,
            current_cell_matches_navcom: false, navcom_is_legal: false,
            navcom_targets_contact_object: false, in_radio_contact: false,
            im_in_reply_was_roger: false, landing_zone_is_clear: false,
            navcom_differs_from_contact: false,
        };
        let plan = landing_takeoff_ai(input, |height, _, _| height == 0);
        assert!(plan.is_landing);
        assert!(plan.is_taking_off);
        assert_eq!(plan.height, 10); // landing -5, corrective lift +5
        assert!(plan.takeoff_started_due_to_blocked_landing_zone);
        assert!(!plan.remove_from_old_layer);
        assert_eq!(plan.redraw_mark_count, 4);
    }

    #[test]
    fn rotor_draw_uses_idle_ground_frames_and_ghost_flag() {
        let plan = draw_rotors_plan(
            false, 0, 10, 100, 200, DirType::EAST,
            |height| height / 2,
            |position, _, _| position,
        );
        assert_eq!(plan.shape_number, 6);
        assert!(plan.ghost);
        assert!(!plan.fading);
        assert!(!plan.predator);
        assert_eq!(plan.draw_requests.len(), 1);
        let draw = plan.draw_requests[0];
        assert_eq!(draw.part, RotorPart::Right);
        assert_eq!((draw.x, draw.y), (100, 198));
        assert!(draw.centered && draw.window_relative && draw.unit_shadow);
    }

    #[test]
    fn rotor_draw_uses_four_frame_airborne_cycle_and_fade_predator_flags() {
        let plan = draw_rotors_plan(
            false, 40, 7, 100, 200, DirType::EAST,
            |height| height / 2,
            |position, _, _| position,
        );
        assert_eq!(plan.shape_number, 3);
        assert!(!plan.ghost);
        assert!(plan.fading);
        assert!(plan.predator);
        assert_eq!((plan.draw_requests[0].x, plan.draw_requests[0].y), (100, 178));
    }

    #[test]
    fn transport_rotors_use_facing_stretch_and_draw_right_then_left() {
        let mut calls = Vec::new();
        let plan = draw_rotors_plan(
            true, 0, 3, 20, 30, DirType::EAST,
            |_| 0,
            |position, direction, distance| {
                calls.push((position, direction, distance));
                if calls.len() == 1 {
                    (position.0 + distance as i16, position.1)
                } else {
                    (position.0 - distance as i16, position.1)
                }
            },
        );
        assert_eq!(plan.shape_number, 7);
        assert_eq!(plan.draw_requests.len(), 2);
        assert_eq!(plan.draw_requests[0].part, RotorPart::Right);
        assert_eq!(plan.draw_requests[1].part, RotorPart::Left);
        assert_eq!((plan.draw_requests[0].x, plan.draw_requests[0].y), (30, 28));
        assert_eq!((plan.draw_requests[1].x, plan.draw_requests[1].y), (10, 28));
        assert_eq!(calls[0], ((20, 30), DirType::EAST, 10));
        assert_eq!(calls[1], ((30, 30), DirType::WEST, 20));
    }

    #[test]
    fn aircraft_damage_is_halved_only_while_airborne_before_base_result() {
        let flying = aircraft_damage_plan(
            9, 1, AircraftDamageResult::Half, 100, 0x8000_0000, false, false,
        );
        assert_eq!(flying.adjusted_damage, 4);
        assert!(!flying.delete_aircraft);
        let landed = aircraft_damage_plan(
            9, 0, AircraftDamageResult::None, 100, 0x8000_0000, false, false,
        );
        assert_eq!(landed.adjusted_damage, 9);
    }

    #[test]
    fn destroyed_aircraft_kill_cargo_announce_and_conditionally_create_fireball() {
        let plan = aircraft_damage_plan(
            20, 0, AircraftDamageResult::Destroyed, 0x1234_5678,
            0x8000_0000, false, true,
        );
        assert!(plan.kill_cargo);
        assert!(plan.announce_death);
        assert_eq!(plan.fireball_coordinate, Some(0x1234_5678));
        assert!(!plan.attempt_crew_paradrop);
        assert!(plan.delete_aircraft);
        let high = aircraft_damage_plan(
            20, 1, AircraftDamageResult::Destroyed, 0x9234_5678,
            0x8000_0000, true, true,
        );
        assert_eq!(high.adjusted_damage, 10);
        assert_eq!(high.fireball_coordinate, None);
        assert!(high.attempt_crew_paradrop);
    }

    #[test]
    fn non_destroying_aircraft_damage_has_no_crash_aftermath() {
        for result in [AircraftDamageResult::None, AircraftDamageResult::Half, AircraftDamageResult::Other(7)] {
            let plan = aircraft_damage_plan(15, 1, result, 1, 0x8000_0000, true, true);
            assert_eq!(plan.adjusted_damage, 7);
            assert!(!plan.kill_cargo);
            assert!(!plan.announce_death);
            assert_eq!(plan.fireball_coordinate, None);
            assert!(!plan.attempt_crew_paradrop);
            assert!(!plan.delete_aircraft);
        }
    }

    fn idle_context() -> IdleModeContext {
        IdleModeContext {
            fixed_wing: false,
            is_ground_layer: false,
            is_loaner: false,
            has_primary_weapon: true,
            has_ammo: true,
            has_team: false,
            team_has_entered_map: false,
            house_is_human: true,
            has_attached_passengers: false,
            is_landing: false,
            accepted_docking_building_cell: None,
            good_landing_zone_cell: 900,
        }
    }

    #[test]
    fn idle_mode_fixed_wing_uses_retreat_on_ground_and_enter_or_retreat_in_air() {
        let ground_loaner = enter_idle_mode_plan(IdleModeContext {
            fixed_wing: true, is_ground_layer: true, is_loaner: true, ..idle_context()
        });
        assert_eq!(ground_loaner.mission, Some(AircraftMission::Retreat));

        let ground_owned = enter_idle_mode_plan(IdleModeContext {
            fixed_wing: true, is_ground_layer: true, ..idle_context()
        });
        assert_eq!(ground_owned.mission, Some(AircraftMission::Guard));
        assert_eq!(ground_owned.destination, Some(IdleModeDestination::Clear));
        assert!(ground_owned.clear_target);

        let air_dock = enter_idle_mode_plan(IdleModeContext {
            fixed_wing: true, accepted_docking_building_cell: Some(12), is_landing: true,
            ..idle_context()
        });
        assert_eq!(air_dock.mission, Some(AircraftMission::Enter));
        assert!(air_dock.reset_status);
        assert!(air_dock.process_takeoff);

        let air_no_dock = enter_idle_mode_plan(IdleModeContext {
            fixed_wing: true, ..idle_context()
        });
        assert_eq!(air_no_dock.mission, Some(AircraftMission::Retreat));
    }

    #[test]
    fn idle_mode_fixed_wing_loaner_weapon_and_team_rules_preserve_early_return() {
        let armed_loaner = enter_idle_mode_plan(IdleModeContext {
            fixed_wing: true, is_loaner: true, has_primary_weapon: true,
            ..idle_context()
        });
        assert_eq!(armed_loaner.mission, Some(AircraftMission::Hunt));

        let team_not_entered = enter_idle_mode_plan(IdleModeContext {
            fixed_wing: true, has_team: true, team_has_entered_map: false,
            ..idle_context()
        });
        assert_eq!(team_not_entered.mission, None);
        assert!(!team_not_entered.commence);

        let team_entered = enter_idle_mode_plan(IdleModeContext {
            fixed_wing: true, is_loaner: true, has_team: true,
            team_has_entered_map: true, house_is_human: true,
            ..idle_context()
        });
        assert!(team_entered.remove_from_team);
        assert!(team_entered.commence);
    }

    #[test]
    fn idle_mode_helicopter_ground_and_cargo_branches_select_guard_unload_or_retreat() {
        let own_ground = enter_idle_mode_plan(IdleModeContext {
            is_ground_layer: true, ..idle_context()
        });
        assert_eq!(own_ground.mission, Some(AircraftMission::Guard));
        assert!(own_ground.clear_target);

        let loaner_empty = enter_idle_mode_plan(IdleModeContext {
            is_ground_layer: true, is_loaner: true, ..idle_context()
        });
        assert_eq!(loaner_empty.mission, Some(AircraftMission::Retreat));

        let loaner_cargo_team = enter_idle_mode_plan(IdleModeContext {
            is_ground_layer: true, is_loaner: true, has_team: true,
            has_attached_passengers: true, ..idle_context()
        });
        assert_eq!(loaner_cargo_team.mission, Some(AircraftMission::Guard));

        let loaner_cargo_solo = enter_idle_mode_plan(IdleModeContext {
            is_ground_layer: true, is_loaner: true, has_attached_passengers: true,
            ..idle_context()
        });
        assert_eq!(loaner_cargo_solo.mission, Some(AircraftMission::Unload));
    }

    #[test]
    fn idle_mode_airborne_heli_assigns_lz_or_docking_target_based_on_role() {
        let normal_cargo = enter_idle_mode_plan(IdleModeContext {
            has_attached_passengers: true, ..idle_context()
        });
        assert_eq!(normal_cargo.mission, Some(AircraftMission::Move));
        assert_eq!(normal_cargo.destination, Some(IdleModeDestination::GoodLandingZone(900)));

        let loaner_cargo = enter_idle_mode_plan(IdleModeContext {
            is_loaner: true, has_attached_passengers: true, ..idle_context()
        });
        assert_eq!(loaner_cargo.mission, Some(AircraftMission::Unload));
        assert_eq!(loaner_cargo.destination, Some(IdleModeDestination::GoodLandingZone(900)));

        let owned_weapon_dock = enter_idle_mode_plan(IdleModeContext {
            accepted_docking_building_cell: Some(77), ..idle_context()
        });
        assert_eq!(owned_weapon_dock.mission, Some(AircraftMission::Enter));
        assert_eq!(owned_weapon_dock.destination, Some(IdleModeDestination::Clear));

        let owned_weapon_no_dock = enter_idle_mode_plan(idle_context());
        assert_eq!(owned_weapon_no_dock.mission, Some(AircraftMission::Move));
        assert_eq!(owned_weapon_no_dock.destination, Some(IdleModeDestination::GoodLandingZone(900)));
    }

    #[test]
    fn idle_mode_helicopter_team_removal_and_no_weapon_early_return_match_source() {
        let loaner_empty_ammo = enter_idle_mode_plan(IdleModeContext {
            is_loaner: true, has_team: true, team_has_entered_map: false,
            has_ammo: false, ..idle_context()
        });
        assert_eq!(loaner_empty_ammo.mission, Some(AircraftMission::Retreat));
        assert!(loaner_empty_ammo.remove_from_team);

        let no_weapon_team = enter_idle_mode_plan(IdleModeContext {
            has_primary_weapon: false, has_team: true, ..idle_context()
        });
        assert_eq!(no_weapon_team.mission, None);
        assert!(!no_weapon_team.commence);

        let loaner_ammo_team = enter_idle_mode_plan(IdleModeContext {
            is_loaner: true, has_team: true, has_ammo: true, ..idle_context()
        });
        assert_eq!(loaner_ammo_team.mission, Some(AircraftMission::Guard));
        assert!(!loaner_ammo_team.remove_from_team);
    }

    #[test]
    fn scatter_respects_mission_and_fixed_wing_gates_and_starts_rotor_takeoff() {
        let forbidden = aircraft_scatter_plan(false, false, true, 0, false);
        assert!(!forbidden.enter_idle_mode);
        assert!(forbidden.is_landing);
        let fixed = aircraft_scatter_plan(true, true, false, 0, false);
        assert!(!fixed.enter_idle_mode);
        let grounded = aircraft_scatter_plan(true, false, false, 0, false);
        assert!(grounded.enter_idle_mode);
        assert!(!grounded.is_landing);
        assert!(grounded.is_taking_off);
        let airborne = aircraft_scatter_plan(true, false, true, 100, false);
        assert!(airborne.enter_idle_mode);
        assert!(!airborne.is_landing);
        assert!(airborne.is_taking_off);
    }

    #[test]
    fn process_fly_to_prefers_building_docking_coordinate_and_sets_heading() {
        let plan = process_fly_to(
            100, Some(200), 0, false, false, 0,
            |from, to| (to - from) as i32,
            |_, _| DirType::SOUTH_EAST,
        );
        assert_eq!(plan.destination_coordinate, 200);
        assert_eq!(plan.primary_facing, DirType::SOUTH_EAST);
        assert_eq!(plan.returned_distance, 200);
        assert!(plan.speed_updates.is_empty());
    }

    #[test]
    fn process_fly_to_slows_rotorcraft_with_bounded_distance_speed() {
        let plan = process_fly_to(
            0x1000, None, 0, false, true, 0,
            |_, _| 0x0180,
            |_, _| DirType::EAST,
        );
        assert_eq!(plan.speed_updates, [0x0080]);
        assert_eq!(plan.returned_distance, 0x0180);
        let far = process_fly_to(
            0x1000, None, 0, false, true, 0,
            |_, _| 0x1000,
            |_, _| DirType::EAST,
        );
        assert_eq!(far.speed_updates, [0x00FF]);
    }

    #[test]
    fn process_fly_to_stops_near_target_but_fixed_wing_disables_slowdown() {
        let near = process_fly_to(
            0, None, 0, false, true, 0x20,
            |_, _| 0x000F,
            |_, _| DirType::NORTH,
        );
        assert_eq!(near.returned_distance, 0);
        assert_eq!(near.speed_updates, [0]);
        let fixed = process_fly_to(
            0, None, 0, true, true, 0x80,
            |_, _| 0x0001,
            |_, _| DirType::NORTH,
        );
        assert_eq!(fixed.returned_distance, 0);
        assert!(fixed.speed_updates.is_empty());
    }

    fn hunt_context() -> AircraftHuntContext {
        AircraftHuntContext {
            fixed_wing: true,
            stage: AircraftHuntStage::LookForTarget,
            mission_is_attack: false,
            target_is_legal: false,
            target_differs_from_navcom: false,
            team_is_valid: false,
            game_is_normal: true,
            tiberium_threat_is_legal: false,
            normal_threat_is_legal: false,
            takeoff_completed: false,
            in_radio_contact: false,
            fire_result: AircraftHuntFireResult::Other(0),
            target_is_in_weapon_range: false,
            passenger: false,
            primary_facing_is_rotating: false,
            target_is_legal_after_search: false,
            primary_weapon_is_camera: false,
            primary_bullet_is_homing: false,
            two_shooter: false,
            ammo: 5,
            attacks_remaining: 2,
            has_attached_passengers: false,
            is_loaner: false,
            ticks_per_second: 15,
            normal_delay: 5,
            random_delay_0_to_2: 2,
        }
    }

    #[test]
    fn fixed_wing_hunt_acquires_and_prioritizes_targets_in_source_order() {
        let initial = aircraft_mission_hunt(AircraftHuntContext {
            target_is_legal: true,
            target_differs_from_navcom: true,
            ..hunt_context()
        });
        assert!(initial.assign_destination_to_target);
        assert_eq!(initial.next_stage, Some(AircraftHuntStage::TakeOff));
        assert_eq!(initial.delay, HuntDelay::Ticks(1));

        let multiplayer = aircraft_mission_hunt(AircraftHuntContext {
            game_is_normal: false,
            tiberium_threat_is_legal: false,
            target_is_legal_after_search: false,
            ..hunt_context()
        });
        assert_eq!(multiplayer.threat_search_order, [
            AircraftThreatKind::Tiberium,
            AircraftThreatKind::Normal,
        ]);
        assert!(multiplayer.assign_target_from_tiberium_threat);
        assert!(multiplayer.assign_target_from_normal_threat);
        assert!(multiplayer.enter_idle_mode);

        let tiberium_found = aircraft_mission_hunt(AircraftHuntContext {
            game_is_normal: false,
            tiberium_threat_is_legal: true,
            target_is_legal_after_search: true,
            ..hunt_context()
        });
        assert_eq!(tiberium_found.threat_search_order, [AircraftThreatKind::Tiberium]);
        assert!(!tiberium_found.assign_target_from_normal_threat);
    }

    #[test]
    fn fixed_wing_hunt_calls_takeoff_processor_until_completion() {
        let pending = aircraft_mission_hunt(AircraftHuntContext {
            stage: AircraftHuntStage::TakeOff,
            takeoff_completed: false,
            ..hunt_context()
        });
        assert!(pending.process_takeoff);
        assert_eq!(pending.set_speed, None);
        assert_eq!(pending.next_stage, None);
        assert_eq!(pending.delay, HuntDelay::Ticks(1));
        assert!(pending.return_immediately);
    }

    #[test]
    fn fixed_wing_hunt_takeoff_and_fly_to_target_obey_return_delays() {
        let takeoff = aircraft_mission_hunt(AircraftHuntContext {
            stage: AircraftHuntStage::TakeOff,
            takeoff_completed: true,
            in_radio_contact: true,
            ..hunt_context()
        });
        assert!(takeoff.process_takeoff);
        assert_eq!(takeoff.set_speed, Some(255));
        assert!(takeoff.break_radio_contact);
        assert_eq!(takeoff.next_stage, Some(AircraftHuntStage::FlyToTarget));
        assert_eq!(takeoff.delay, HuntDelay::Ticks(1));

        let facing_close = aircraft_mission_hunt(AircraftHuntContext {
            stage: AircraftHuntStage::FlyToTarget,
            fire_result: AircraftHuntFireResult::Facing,
            target_is_in_weapon_range: true,
            ..hunt_context()
        });
        assert_eq!(facing_close.delay, HuntDelay::Ticks(30));
        assert!(facing_close.return_immediately);

        let can_fire = aircraft_mission_hunt(AircraftHuntContext {
            stage: AircraftHuntStage::FlyToTarget,
            fire_result: AircraftHuntFireResult::Ok,
            ..hunt_context()
        });
        assert_eq!(can_fire.next_stage, Some(AircraftHuntStage::DropBombs));
        assert_eq!(can_fire.delay, HuntDelay::Ticks(1));
    }

    #[test]
    fn fixed_wing_hunt_fly_failure_routes_to_regroup_or_reacquisition() {
        for result in [AircraftHuntFireResult::Cant, AircraftHuntFireResult::Illegal] {
            let attack_mission = aircraft_mission_hunt(AircraftHuntContext {
                stage: AircraftHuntStage::FlyToTarget,
                mission_is_attack: true,
                fire_result: result,
                ..hunt_context()
            });
            assert_eq!(attack_mission.next_stage, Some(AircraftHuntStage::Regroup));
            let hunt_mission = aircraft_mission_hunt(AircraftHuntContext {
                stage: AircraftHuntStage::FlyToTarget,
                mission_is_attack: false,
                fire_result: result,
                ..hunt_context()
            });
            assert_eq!(hunt_mission.next_stage, Some(AircraftHuntStage::LookForTarget));
        }
        let no_ammo = aircraft_mission_hunt(AircraftHuntContext {
            stage: AircraftHuntStage::FlyToTarget,
            fire_result: AircraftHuntFireResult::Ammo,
            ..hunt_context()
        });
        assert_eq!(no_ammo.next_stage, Some(AircraftHuntStage::Regroup));
    }

    #[test]
    fn fixed_wing_hunt_drop_bombs_selects_homing_target_and_two_shot_count() {
        let plan = aircraft_mission_hunt(AircraftHuntContext {
            stage: AircraftHuntStage::DropBombs,
            fire_result: AircraftHuntFireResult::Ok,
            primary_bullet_is_homing: true,
            two_shooter: true,
            ..hunt_context()
        });
        assert_eq!(plan.fire_actions, [
            HuntFireAction { target: HuntFireTarget::CurrentTarget, weapon_index: 0 },
            HuntFireAction { target: HuntFireTarget::CurrentTarget, weapon_index: 0 },
        ]);
        assert!(plan.notify_target_incoming);
        assert!(plan.rearm_delay_return);
        assert!(plan.return_immediately);

        let camera = aircraft_mission_hunt(AircraftHuntContext {
            stage: AircraftHuntStage::DropBombs,
            fire_result: AircraftHuntFireResult::Ok,
            primary_weapon_is_camera: true,
            ..hunt_context()
        });
        assert_eq!(camera.next_stage, Some(AircraftHuntStage::Regroup));
        assert!(!camera.notify_target_incoming);
    }

    #[test]
    fn fixed_wing_hunt_drop_bombs_routes_range_ammo_and_invalid_results() {
        let range = aircraft_mission_hunt(AircraftHuntContext {
            stage: AircraftHuntStage::DropBombs,
            fire_result: AircraftHuntFireResult::Range,
            ..hunt_context()
        });
        assert_eq!(range.next_stage, Some(AircraftHuntStage::FlyToTarget));
        assert_eq!(range.delay, HuntDelay::Ticks(60));

        let ammo = aircraft_mission_hunt(AircraftHuntContext {
            stage: AircraftHuntStage::DropBombs,
            fire_result: AircraftHuntFireResult::Ammo,
            ..hunt_context()
        });
        assert_eq!(ammo.attacks_remaining_delta, -1);
        assert_eq!(ammo.next_stage, Some(AircraftHuntStage::Regroup));

        let cant = aircraft_mission_hunt(AircraftHuntContext {
            stage: AircraftHuntStage::DropBombs,
            fire_result: AircraftHuntFireResult::Cant,
            ..hunt_context()
        });
        assert_eq!(cant.next_stage, Some(AircraftHuntStage::Regroup));
    }

    #[test]
    fn fixed_wing_hunt_regroup_handles_empty_ammo_loaners_and_remaining_passengers() {
        let loaner = aircraft_mission_hunt(AircraftHuntContext {
            stage: AircraftHuntStage::Regroup,
            ammo: 0,
            team_is_valid: true,
            is_loaner: true,
            ..hunt_context()
        });
        assert!(loaner.set_attacks_remaining_zero);
        assert!(loaner.enter_idle_mode);
        assert!(loaner.remove_from_team);
        assert_eq!(loaner.mission_to_assign, Some(AircraftMission::Retreat));
        assert!(loaner.commence);

        let passenger = aircraft_mission_hunt(AircraftHuntContext {
            stage: AircraftHuntStage::Regroup,
            attacks_remaining: 0,
            has_attached_passengers: true,
            ..hunt_context()
        });
        assert_eq!(passenger.next_stage, Some(AircraftHuntStage::LookForTarget));
    }

    #[test]
    fn rotorcraft_hunt_reacquires_or_hands_off_to_attack() {
        let empty = aircraft_mission_hunt(AircraftHuntContext {
            fixed_wing: false,
            ammo: 0,
            team_is_valid: true,
            ..hunt_context()
        });
        assert!(empty.enter_idle_mode);
        assert!(empty.remove_from_team);
        assert!(!empty.return_immediately);

        let no_target = aircraft_mission_hunt(AircraftHuntContext {
            fixed_wing: false,
            target_is_legal: false,
            game_is_normal: false,
            target_is_legal_after_search: false,
            ..hunt_context()
        });
        assert_eq!(no_target.threat_search_order, [
            AircraftThreatKind::Tiberium,
            AircraftThreatKind::Normal,
        ]);
        assert!(no_target.enter_idle_mode);
        assert_eq!(no_target.delay, HuntDelay::Ticks(1));

        let attack = aircraft_mission_hunt(AircraftHuntContext {
            fixed_wing: false,
            target_is_legal_after_search: true,
            ..hunt_context()
        });
        assert_eq!(attack.mission_to_assign, Some(AircraftMission::Attack));
        assert_eq!(attack.delay, HuntDelay::Ticks(1));
    }

    #[test]
    fn fixed_wing_retreat_climbs_one_height_unit_until_flight_level() {
        let climb = aircraft_mission_retreat(AircraftRetreatContext {
            fixed_wing: true, height: 50, flight_level: 100,
            status: AircraftRetreatStage::KeepFlying, takeoff_completed: false,
            friendly_map_edge: 3, normal_delay: 7, random_delay_0_to_2: 2,
        });
        assert_eq!(climb.height_after_tick, 51);
        assert_eq!(climb.delay, HuntDelay::Ticks(3));
        assert!(climb.return_immediately);
        let cruise = aircraft_mission_retreat(AircraftRetreatContext {
            height: 100, ..AircraftRetreatContext {
                fixed_wing: true, height: 50, flight_level: 100,
                status: AircraftRetreatStage::KeepFlying, takeoff_completed: false,
                friendly_map_edge: 3, normal_delay: 7, random_delay_0_to_2: 2,
            }
        });
        assert_eq!(cruise.height_after_tick, 100);
        assert_eq!(cruise.delay, HuntDelay::Ticks(70));
    }

    #[test]
    fn rotorcraft_retreat_processes_takeoff_then_sets_friendly_edge_heading() {
        let pending = aircraft_mission_retreat(AircraftRetreatContext {
            fixed_wing: false, height: 0, flight_level: 100,
            status: AircraftRetreatStage::TakeOff, takeoff_completed: false,
            friendly_map_edge: 2, normal_delay: 5, random_delay_0_to_2: 1,
        });
        assert!(pending.process_takeoff);
        assert_eq!(pending.next_stage, None);
        assert_eq!(pending.delay, HuntDelay::Ticks(1));
        let complete = aircraft_mission_retreat(AircraftRetreatContext {
            takeoff_completed: true, ..AircraftRetreatContext {
                fixed_wing: false, height: 0, flight_level: 100,
                status: AircraftRetreatStage::TakeOff, takeoff_completed: false,
                friendly_map_edge: 2, normal_delay: 5, random_delay_0_to_2: 1,
            }
        });
        assert_eq!(complete.next_stage, Some(AircraftRetreatStage::FaceMapEdge));

        let face = aircraft_mission_retreat(AircraftRetreatContext {
            fixed_wing: false, height: 100, flight_level: 100,
            status: AircraftRetreatStage::FaceMapEdge, takeoff_completed: true,
            friendly_map_edge: 2, normal_delay: 5, random_delay_0_to_2: 1,
        });
        assert_eq!(face.set_speed, Some(255));
        assert_eq!(face.primary_desired, Some(DirType::SOUTH));
        assert_eq!(face.secondary_desired, Some(DirType::SOUTH));
        assert_eq!(face.next_stage, Some(AircraftRetreatStage::KeepFlying));
    }

    #[test]
    fn retreat_keep_flying_leaves_course_unchanged_and_uses_normal_delay() {
        let plan = aircraft_mission_retreat(AircraftRetreatContext {
            fixed_wing: false, height: 100, flight_level: 100,
            status: AircraftRetreatStage::KeepFlying, takeoff_completed: false,
            friendly_map_edge: 1, normal_delay: 6, random_delay_0_to_2: 8,
        });
        assert_eq!(plan.primary_desired, None);
        assert_eq!(plan.secondary_desired, None);
        assert_eq!(plan.delay, HuntDelay::NormalPlusRandom2 { normal_delay: 6, random_value: 2 });
        assert!(!plan.return_immediately);
    }

    #[test]
    fn movement_ai_marks_ground_and_airborne_physics_differently() {
        let ground = aircraft_movement_ai(true, true, true, 10, true);
        assert!(ground.assign_move_mission);
        assert!(ground.perform_physics);
        assert!(ground.mark_up_before_physics);
        assert!(ground.mark_down_after_physics);
        assert!(!ground.mark_change_redraw_before_physics);

        let airborne = aircraft_movement_ai(true, true, true, 10, false);
        assert!(airborne.mark_change_redraw_before_physics);
        assert!(airborne.mark_change_redraw_after_physics_if_moved);
        assert!(!airborne.mark_up_before_physics);
        let idle = aircraft_movement_ai(true, true, true, 0, true);
        assert!(!idle.perform_physics);
        assert!(!idle.mark_up_before_physics);
        let queued = aircraft_movement_ai(true, true, false, 10, true);
        assert!(!queued.assign_move_mission);
    }

    fn guard_context() -> AircraftGuardContext {
        AircraftGuardContext {
            height: 10, flight_level: 100, team_is_valid: false,
            navcom_is_legal: false, has_primary_weapon: true,
            house_is_human: false, house_available_money: 0,
            health_ratio: Fixed16::ONE, condition_yellow: Fixed16::from_ratio(1, 2),
            in_radio_contact: false, height_is_grounded: false,
            contact_is_repair_building: false, repair_building_available: false,
            ammo: 5, weapon_is_equipped: true, helipad_available: false,
            target_is_legal: false, scatter_is_allowed: true,
            house_is_attacked: true, juicy_target_is_legal: false,
            ticks_per_second: 15, normal_delay: 5, base_guard_delay: 7,
        }
    }

    #[test]
    fn guard_flight_level_team_and_weaponless_branches_return_early() {
        let team_nav = aircraft_mission_guard(AircraftGuardContext {
            height: 100, team_is_valid: true, navcom_is_legal: true,
            ..guard_context()
        });
        assert_eq!(team_nav.mission_to_assign, Some(AircraftMission::Move));
        assert_eq!(team_nav.delay, HuntDelay::Ticks(5));

        let weaponless = aircraft_mission_guard(AircraftGuardContext {
            height: 100, has_primary_weapon: false, ..guard_context()
        });
        assert_eq!(weaponless.mission_to_assign, Some(AircraftMission::Move));
        assert_eq!(weaponless.destination, Some(IdleModeDestination::CurrentCell));

        let armed_idle = aircraft_mission_guard(AircraftGuardContext {
            height: 100, has_primary_weapon: true, ..guard_context()
        });
        assert!(armed_idle.enter_idle_mode);
        assert_eq!(armed_idle.delay, HuntDelay::Ticks(1));
    }

    #[test]
    fn guard_player_house_waits_without_running_ai_guard_search() {
        let player = aircraft_mission_guard(AircraftGuardContext {
            house_is_human: true, ticks_per_second: 15, ..guard_context()
        });
        assert_eq!(player.delay, HuntDelay::Ticks(5));
        assert!(!player.delegate_to_base_guard);
        assert!(!player.enter_idle_mode);
    }

    #[test]
    fn guard_prioritizes_repair_then_rearm_then_current_target() {
        let repair = aircraft_mission_guard(AircraftGuardContext {
            house_available_money: 100,
            health_ratio: Fixed16::from_ratio(1, 4),
            condition_yellow: Fixed16::from_ratio(1, 2),
            repair_building_available: true,
            target_is_legal: true,
            ..guard_context()
        });
        assert_eq!(repair.mission_to_assign, Some(AircraftMission::Enter));
        assert!(repair.clear_target);

        let rearm = aircraft_mission_guard(AircraftGuardContext {
            ammo: 0, weapon_is_equipped: true, helipad_available: true,
            target_is_legal: true, ..guard_context()
        });
        assert_eq!(rearm.mission_to_assign, Some(AircraftMission::Enter));

        let attack = aircraft_mission_guard(AircraftGuardContext {
            target_is_legal: true, ..guard_context()
        });
        assert_eq!(attack.mission_to_assign, Some(AircraftMission::Attack));
    }

    #[test]
    fn guard_transport_no_weapon_and_grounded_helicopter_scatter_delays_match() {
        let transport = aircraft_mission_guard(AircraftGuardContext {
            weapon_is_equipped: false, ticks_per_second: 15, ..guard_context()
        });
        assert_eq!(transport.delay, HuntDelay::Ticks(45));
        assert!(!transport.scatter);

        let grounded = aircraft_mission_guard(AircraftGuardContext {
            height: 0, height_is_grounded: true, in_radio_contact: false,
            ticks_per_second: 15, ..guard_context()
        });
        assert!(grounded.scatter);
        assert_eq!(grounded.delay, HuntDelay::Ticks(45));
    }

    #[test]
    fn guard_hunts_juicy_targets_only_when_house_is_not_attacked() {
        let juicy = aircraft_mission_guard(AircraftGuardContext {
            house_is_attacked: false, juicy_target_is_legal: true, ..guard_context()
        });
        assert!(juicy.target_juicy_threat);
        assert_eq!(juicy.mission_to_assign, Some(AircraftMission::Attack));
        assert!(juicy.delegate_to_base_guard);
        assert!(juicy.return_immediately);
        let attacked = aircraft_mission_guard(AircraftGuardContext {
            house_is_attacked: true, juicy_target_is_legal: true, ..guard_context()
        });
        assert!(attacked.delegate_to_base_guard);
    }

    #[test]
    fn guard_area_respects_height_team_human_scatter_and_attack_order() {
        let high_team = aircraft_mission_guard_area(AircraftGuardAreaContext {
            height: 100, flight_level: 100, team_is_valid: true,
            house_is_human: false, height_is_grounded: false, in_radio_contact: false,
            scatter_is_allowed: true, target_is_legal: true, ticks_per_second: 15,
            base_guard_area_delay: 7,
        });
        assert!(!high_team.enter_idle_mode);
        assert_eq!(high_team.delay, HuntDelay::Ticks(1));

        let human = aircraft_mission_guard_area(AircraftGuardAreaContext {
            height: 10, flight_level: 100, team_is_valid: false,
            house_is_human: true, height_is_grounded: false, in_radio_contact: false,
            scatter_is_allowed: true, target_is_legal: true, ticks_per_second: 15,
            base_guard_area_delay: 7,
        });
        assert_eq!(human.delay, HuntDelay::Ticks(15));
        assert_eq!(human.mission_to_assign, None);

        let grounded = aircraft_mission_guard_area(AircraftGuardAreaContext {
            height: 0, flight_level: 100, team_is_valid: false,
            house_is_human: false, height_is_grounded: true, in_radio_contact: false,
            scatter_is_allowed: true, target_is_legal: true, ticks_per_second: 15,
            base_guard_area_delay: 7,
        });
        assert!(grounded.scatter);
        assert_eq!(grounded.delay, HuntDelay::Ticks(45));

        let attack = aircraft_mission_guard_area(AircraftGuardAreaContext {
            height: 50, flight_level: 100, team_is_valid: false,
            house_is_human: false, height_is_grounded: false, in_radio_contact: true,
            scatter_is_allowed: false, target_is_legal: true, ticks_per_second: 15,
            base_guard_area_delay: 7,
        });
        assert_eq!(attack.mission_to_assign, Some(AircraftMission::Attack));
    }

    fn attack_context() -> AircraftAttackContext {
        AircraftAttackContext {
            fixed_wing: false,
            stage: AircraftAttackStage::ValidateAttackZone,
            target_is_legal: true,
            navcom_is_legal: true,
            selected_attack_cell: Some(200),
            takeoff_completed: false,
            in_radio_contact_with_building: true,
            direction_to_navcom: DirType::EAST,
            secondary_facing_current: DirType::NORTH,
            direction_to_target: DirType::SOUTH,
            fire_direction_from_aircraft_to_navcom: DirType::NORTH_EAST,
            distance_to_navcom: 1000,
            fire_result: AircraftAttackFireResult::Other(0),
            ammo: 5,
            target_is_in_weapon_range: true,
            is_loaner: false,
            house_is_human: false,
            curley_shuffle: false,
            normal_delay: 5,
            random_delay_0_to_2: 1,
        }
    }

    #[test]
    fn fixed_wing_attack_delegates_to_hunt_mission() {
        let plan = aircraft_mission_attack(AircraftAttackContext {
            fixed_wing: true, ..attack_context()
        });
        assert!(plan.delegate_to_hunt);
        assert!(plan.return_immediately);
    }

    #[test]
    fn rotor_attack_validates_zone_and_selects_attack_position_before_takeoff() {
        let valid = aircraft_mission_attack(attack_context());
        assert_eq!(valid.next_stage, Some(AircraftAttackStage::PickAttackLocation));
        let invalid = aircraft_mission_attack(AircraftAttackContext {
            target_is_legal: false, ..attack_context()
        });
        assert_eq!(invalid.next_stage, Some(AircraftAttackStage::ReturnToBase));

        let pick = aircraft_mission_attack(AircraftAttackContext {
            stage: AircraftAttackStage::PickAttackLocation, ..attack_context()
        });
        assert_eq!(pick.assign_destination_to_attack_cell, Some(200));
        assert_eq!(pick.next_stage, Some(AircraftAttackStage::TakeOff));
        let no_cell = aircraft_mission_attack(AircraftAttackContext {
            stage: AircraftAttackStage::PickAttackLocation,
            selected_attack_cell: None, ..attack_context()
        });
        assert_eq!(no_cell.next_stage, Some(AircraftAttackStage::ReturnToBase));
    }

    #[test]
    fn rotor_attack_takeoff_skews_body_and_breaks_helipad_contact() {
        let pending = aircraft_mission_attack(AircraftAttackContext {
            stage: AircraftAttackStage::TakeOff,
            takeoff_completed: false, ..attack_context()
        });
        assert!(pending.process_takeoff);
        assert_eq!(pending.delay, HuntDelay::Ticks(1));
        assert_eq!(pending.next_stage, None);

        let complete = aircraft_mission_attack(AircraftAttackContext {
            stage: AircraftAttackStage::TakeOff,
            takeoff_completed: true, ..attack_context()
        });
        assert_eq!(complete.next_stage, Some(AircraftAttackStage::FlyToPosition));
        assert!(complete.break_radio_contact);
        assert_eq!(complete.set_primary_current, Some(DirType::EAST));
    }

    #[test]
    fn rotor_attack_flies_to_navcom_then_faces_target_and_stops_at_fire_point() {
        let approach = aircraft_mission_attack(AircraftAttackContext {
            stage: AircraftAttackStage::FlyToPosition,
            distance_to_navcom: 0x0300, ..attack_context()
        });
        assert_eq!(approach.secondary_desired, Some(DirType::NORTH_EAST));
        assert_eq!(approach.delay, HuntDelay::Ticks(1));

        let near = aircraft_mission_attack(AircraftAttackContext {
            stage: AircraftAttackStage::FlyToPosition,
            distance_to_navcom: 0x0100, ..attack_context()
        });
        assert_eq!(near.secondary_desired, Some(DirType::SOUTH));
        assert_eq!(near.next_stage, None);

        let arrived = aircraft_mission_attack(AircraftAttackContext {
            stage: AircraftAttackStage::FlyToPosition,
            distance_to_navcom: 0x000F, ..attack_context()
        });
        assert_eq!(arrived.next_stage, Some(AircraftAttackStage::FireAtTarget));
        assert!(arrived.clear_destination);

        let missing_navcom = aircraft_mission_attack(AircraftAttackContext {
            stage: AircraftAttackStage::FlyToPosition,
            navcom_is_legal: false, ..attack_context()
        });
        assert_eq!(missing_navcom.next_stage, Some(AircraftAttackStage::PickAttackLocation));
    }

    #[test]
    fn rotor_attack_firing_handles_cloak_rearm_first_and_second_shot() {
        let cloaked = aircraft_mission_attack(AircraftAttackContext {
            stage: AircraftAttackStage::FireAtTarget,
            fire_result: AircraftAttackFireResult::Cloaked, ..attack_context()
        });
        assert!(cloaked.do_uncloak);
        assert!(!cloaked.fire_target);

        let first = aircraft_mission_attack(AircraftAttackContext {
            stage: AircraftAttackStage::FireAtTarget,
            fire_result: AircraftAttackFireResult::Ok, ..attack_context()
        });
        assert!(first.fire_target && first.notify_target_incoming);
        assert_eq!(first.next_stage, Some(AircraftAttackStage::FireAtTargetSecondShot));

        let rearm = aircraft_mission_attack(AircraftAttackContext {
            stage: AircraftAttackStage::FireAtTarget,
            fire_result: AircraftAttackFireResult::Rearm, ..attack_context()
        });
        assert_eq!(rearm.next_stage, None);

        let second = aircraft_mission_attack(AircraftAttackContext {
            stage: AircraftAttackStage::FireAtTargetSecondShot,
            fire_result: AircraftAttackFireResult::Ok,
            ammo: 3,
            curley_shuffle: true,
            ..attack_context()
        });
        assert_eq!(second.next_stage, Some(AircraftAttackStage::PickAttackLocation));
    }

    #[test]
    fn rotor_attack_failed_second_shot_repositions_or_returns_to_base() {
        let reposition = aircraft_mission_attack(AircraftAttackContext {
            stage: AircraftAttackStage::FireAtTargetSecondShot,
            fire_result: AircraftAttackFireResult::Other(1),
            target_is_in_weapon_range: false,
            ammo: 3,
            ..attack_context()
        });
        assert_eq!(reposition.next_stage, Some(AircraftAttackStage::PickAttackLocation));
        let dry = aircraft_mission_attack(AircraftAttackContext {
            stage: AircraftAttackStage::FireAtTargetSecondShot,
            fire_result: AircraftAttackFireResult::Ammo,
            ammo: 0,
            ..attack_context()
        });
        assert_eq!(dry.next_stage, Some(AircraftAttackStage::ReturnToBase));
    }

    #[test]
    fn rotor_attack_return_clears_targets_for_empty_loaner_or_human_aircraft() {
        let human = aircraft_mission_attack(AircraftAttackContext {
            stage: AircraftAttackStage::ReturnToBase,
            ammo: 0, house_is_human: true, ..attack_context()
        });
        assert!(human.clear_target && human.clear_destination && human.enter_idle_mode);
        let computer_with_ammo = aircraft_mission_attack(AircraftAttackContext {
            stage: AircraftAttackStage::ReturnToBase,
            ammo: 1, house_is_human: false, is_loaner: false, ..attack_context()
        });
        assert!(!computer_with_ammo.clear_target);
        assert!(computer_with_ammo.clear_destination && computer_with_ammo.enter_idle_mode);
    }

    fn move_context() -> AircraftMoveContext {
        AircraftMoveContext {
            fixed_wing: false,
            stage: AircraftMoveStage::ValidateLandingZone,
            is_loaner: false,
            team_is_valid: false,
            navcom_is_legal: true,
            landing_zone_is_clear: true,
            landing_cell_is_available: true,
            replacement_landing_cell: Some(500),
            takeoff_completed: false,
            in_radio_contact_with_building: true,
            current_cell_is_contacted_building: true,
            distance_to_navcom: 1000,
            navcom_direction: DirType::EAST,
            target_is_legal: true,
            direction_to_target: DirType::SOUTH,
            pose_direction: DirType::NORTH,
            replacement_landing_target_is_cell: true,
            landing_completes_as_crash: false,
            docking_building_is_accepted: false,
            good_landing_zone_cell: 600,
            navcom_is_cell_target: false,
            is_taking_off: false,
            process_landing_completes: false,
            mission_queue_is_none: true,
            normal_delay: 5,
            random_delay_0_to_2: 1,
        }
    }

    #[test]
    fn rotor_move_validates_and_retargets_landing_zone_before_takeoff() {
        let invalid = aircraft_mission_move(AircraftMoveContext {
            navcom_is_legal: false, ..move_context()
        });
        assert!(invalid.enter_idle_mode);
        let blocked = aircraft_mission_move(AircraftMoveContext {
            landing_zone_is_clear: false, team_is_valid: true, ..move_context()
        });
        assert_eq!(blocked.destination, Some(IdleModeDestination::AssignedCell(500)));
        assert!(blocked.update_team_mission_target);
        let ready = aircraft_mission_move(move_context());
        assert_eq!(ready.next_stage, Some(AircraftMoveStage::TakeOff));
    }

    #[test]
    fn rotor_move_takeoff_waits_and_breaks_contact_only_with_origin_helipad() {
        let pending = aircraft_mission_move(AircraftMoveContext {
            stage: AircraftMoveStage::TakeOff, takeoff_completed: false, ..move_context()
        });
        assert!(pending.process_takeoff);
        assert_eq!(pending.delay, HuntDelay::Ticks(1));
        let complete = aircraft_mission_move(AircraftMoveContext {
            stage: AircraftMoveStage::TakeOff, takeoff_completed: true, ..move_context()
        });
        assert_eq!(complete.next_stage, Some(AircraftMoveStage::FlyToLandingZone));
        assert!(complete.break_radio_contact);
        let no_contact_match = aircraft_mission_move(AircraftMoveContext {
            stage: AircraftMoveStage::TakeOff, takeoff_completed: true,
            current_cell_is_contacted_building: false, ..move_context()
        });
        assert!(!no_contact_match.break_radio_contact);
    }

    #[test]
    fn rotor_move_flies_faces_target_or_pose_and_transitions_to_land() {
        let far = aircraft_mission_move(AircraftMoveContext {
            stage: AircraftMoveStage::FlyToLandingZone, distance_to_navcom: 0x0100,
            ..move_context()
        });
        assert!(far.process_fly_to);
        assert_eq!(far.secondary_desired, Some(DirType::EAST));
        let near_target = aircraft_mission_move(AircraftMoveContext {
            stage: AircraftMoveStage::FlyToLandingZone, distance_to_navcom: 0x0070,
            ..move_context()
        });
        assert_eq!(near_target.secondary_desired, Some(DirType::SOUTH));
        let near_pose = aircraft_mission_move(AircraftMoveContext {
            stage: AircraftMoveStage::FlyToLandingZone, distance_to_navcom: 0x0070,
            target_is_legal: false, ..move_context()
        });
        assert_eq!(near_pose.secondary_desired, Some(DirType::NORTH));
        let arrived = aircraft_mission_move(AircraftMoveContext {
            stage: AircraftMoveStage::FlyToLandingZone, distance_to_navcom: 0x000F,
            ..move_context()
        });
        assert_eq!(arrived.next_stage, Some(AircraftMoveStage::Land));
    }

    #[test]
    fn rotor_move_recovers_lost_lz_and_relaunches_if_landing_requires_takeoff() {
        let blocked = aircraft_mission_move(AircraftMoveContext {
            stage: AircraftMoveStage::FlyToLandingZone,
            landing_zone_is_clear: false, navcom_is_legal: false,
            team_is_valid: true, ..move_context()
        });
        assert_eq!(blocked.destination, Some(IdleModeDestination::AssignedCell(500)));
        assert!(blocked.update_team_mission_target);
        assert_eq!(blocked.next_stage, Some(AircraftMoveStage::Land));

        let relaunch = aircraft_mission_move(AircraftMoveContext {
            stage: AircraftMoveStage::Land, is_taking_off: true,
            process_landing_completes: true, team_is_valid: true, ..move_context()
        });
        assert_eq!(relaunch.next_stage, Some(AircraftMoveStage::TakeOff));
        assert!(relaunch.update_team_mission_target);
        assert!(relaunch.process_landing);
        assert!(!relaunch.enter_idle_mode); // source enters idle only after Process_Landing and queue check.
    }

    #[test]
    fn rotor_move_enters_idle_after_unqueued_landing_completes() {
        let landed = aircraft_mission_move(AircraftMoveContext {
            stage: AircraftMoveStage::Land,
            process_landing_completes: true,
            mission_queue_is_none: true,
            ..move_context()
        });
        assert!(landed.enter_idle_mode);
        assert_eq!(landed.delay, HuntDelay::Ticks(1));
    }

    #[test]
    fn fixed_wing_move_takeoff_and_docking_or_forced_landing_branches_match() {
        let pending = aircraft_mission_move(AircraftMoveContext {
            fixed_wing: true, stage: AircraftMoveStage::FixedWingTakeOff,
            takeoff_completed: false, ..move_context()
        });
        assert!(pending.process_takeoff);
        assert_eq!(pending.delay, HuntDelay::Ticks(1));

        let complete = aircraft_mission_move(AircraftMoveContext {
            fixed_wing: true, stage: AircraftMoveStage::FixedWingTakeOff,
            takeoff_completed: true, ..move_context()
        });
        assert!(complete.stop_taking_off);
        assert_eq!(complete.set_speed, Some(255));
        assert!(complete.break_radio_contact);
        assert_eq!(complete.next_stage, Some(AircraftMoveStage::FixedWingFlyTowardTarget));

        let dock = aircraft_mission_move(AircraftMoveContext {
            fixed_wing: true, stage: AircraftMoveStage::FixedWingFlyTowardTarget,
            distance_to_navcom: 0x00BF, docking_building_is_accepted: true,
            ..move_context()
        });
        assert!(dock.clear_destination);
        assert_eq!(dock.mission_to_assign, Some(AircraftMission::Enter));
        assert!(dock.commence);

        let no_dock = aircraft_mission_move(AircraftMoveContext {
            fixed_wing: true, stage: AircraftMoveStage::FixedWingFlyTowardTarget,
            distance_to_navcom: 0x00BF, docking_building_is_accepted: false,
            navcom_is_cell_target: false, ..move_context()
        });
        assert_eq!(no_dock.destination, Some(IdleModeDestination::GoodLandingZone(600)));
        assert_eq!(no_dock.mission_to_assign, Some(AircraftMission::Move));
    }

    #[test]
    fn fixed_wing_move_crashes_on_unserviced_cell_target_and_respects_loaner_team() {
        let crash = aircraft_mission_move(AircraftMoveContext {
            fixed_wing: true, stage: AircraftMoveStage::FixedWingFlyTowardTarget,
            distance_to_navcom: 0x00BF, navcom_is_cell_target: true,
            landing_completes_as_crash: true, ..move_context()
        });
        assert!(crash.process_landing);
        assert!(crash.force_crash_and_delete);

        let delay = aircraft_mission_move(AircraftMoveContext {
            fixed_wing: true, stage: AircraftMoveStage::FixedWingFlyTowardTarget,
            distance_to_navcom: 0x00BF, navcom_is_cell_target: true,
            landing_completes_as_crash: false, ..move_context()
        });
        assert_eq!(delay.delay, HuntDelay::Ticks(500));

        let loaner_team = aircraft_mission_move(AircraftMoveContext {
            fixed_wing: true, stage: AircraftMoveStage::FixedWingFlyTowardTarget,
            is_loaner: true, team_is_valid: true,
            distance_to_navcom: 0x00BF, ..move_context()
        });
        assert!(!loaner_team.enter_idle_mode);
    }

    fn unload_context() -> AircraftUnloadContext {
        AircraftUnloadContext {
            fixed_wing: false,
            stage: AircraftUnloadStage::SearchForLandingZone,
            height: 100,
            flight_level: 100,
            navcom_is_legal: true,
            aircraft_coordinate_matches_navcom: false,
            landing_zone_is_clear: true,
            attached_passenger_team_origin: None,
            reinforcement_waypoint: 3,
            replacement_landing_cell: Some(700),
            team_is_valid: false,
            is_tethered: false,
            has_attached_passenger: true,
            has_attached_passenger_after_exit: false,
            exit_object_succeeded: true,
            is_taking_off: false,
            takeoff_completed: false,
            landing_completed: false,
            distance_to_navcom: 500,
            pose_direction: DirType::NORTH_EAST,
            primary_facing_desired: DirType::EAST,
            in_radio_contact_with_origin_building: true,
            normal_delay: 5,
            random_delay_0_to_2: 1,
        }
    }

    #[test]
    fn fixed_wing_unload_delegates_to_hunt_after_target_copy() {
        let plan = aircraft_mission_unload(AircraftUnloadContext {
            fixed_wing: true, ..unload_context()
        });
        assert!(plan.delegate_to_hunt);
        assert!(plan.return_immediately);
    }

    #[test]
    fn unload_search_accepts_grounded_destination_or_selects_replacement_zone() {
        let grounded = aircraft_mission_unload(AircraftUnloadContext {
            height: 0, aircraft_coordinate_matches_navcom: true,
            ..unload_context()
        });
        assert_eq!(grounded.next_stage, Some(AircraftUnloadStage::UnloadPassengers));

        let team_waypoint = aircraft_mission_unload(AircraftUnloadContext {
            landing_zone_is_clear: false,
            attached_passenger_team_origin: Some(12),
            team_is_valid: true,
            ..unload_context()
        });
        assert_eq!(team_waypoint.destination, Some(UnloadDestination::Waypoint(12)));
        assert!(team_waypoint.update_team_mission_target);

        let fallback = aircraft_mission_unload(AircraftUnloadContext {
            landing_zone_is_clear: false,
            attached_passenger_team_origin: Some(-1),
            ..unload_context()
        });
        assert_eq!(fallback.destination, Some(UnloadDestination::Cell(700)));

        let airborne = aircraft_mission_unload(AircraftUnloadContext {
            height: 100, flight_level: 100, ..unload_context()
        });
        assert_eq!(airborne.next_stage, Some(AircraftUnloadStage::FlyToLandingZone));
        let below_level = aircraft_mission_unload(AircraftUnloadContext {
            height: 40, flight_level: 100, ..unload_context()
        });
        assert_eq!(below_level.next_stage, Some(AircraftUnloadStage::TakeOff));
    }

    #[test]
    fn unload_flight_and_landing_keep_source_thresholds_and_facing() {
        let far = aircraft_mission_unload(AircraftUnloadContext {
            stage: AircraftUnloadStage::FlyToLandingZone,
            distance_to_navcom: 0x0100,
            ..unload_context()
        });
        assert!(far.process_fly_to);
        assert_eq!(far.set_secondary_facing, Some(DirType::EAST));
        assert_eq!(far.delay, HuntDelay::Ticks(5));

        let near = aircraft_mission_unload(AircraftUnloadContext {
            stage: AircraftUnloadStage::FlyToLandingZone,
            distance_to_navcom: 0x00FF,
            ..unload_context()
        });
        assert_eq!(near.set_secondary_facing, Some(DirType::NORTH_EAST));
        assert_eq!(near.delay, HuntDelay::Ticks(1));

        let arrived = aircraft_mission_unload(AircraftUnloadContext {
            stage: AircraftUnloadStage::FlyToLandingZone,
            distance_to_navcom: 0x000F,
            ..unload_context()
        });
        assert_eq!(arrived.next_stage, Some(AircraftUnloadStage::LandOnLandingZone));

        let landed = aircraft_mission_unload(AircraftUnloadContext {
            stage: AircraftUnloadStage::LandOnLandingZone,
            landing_completed: true,
            ..unload_context()
        });
        assert!(landed.process_landing);
        assert_eq!(landed.next_stage, Some(AircraftUnloadStage::UnloadPassengers));
    }

    #[test]
    fn unload_passenger_detach_pickup_exit_and_map_restore_are_ordered() {
        let success = aircraft_mission_unload(AircraftUnloadContext {
            stage: AircraftUnloadStage::UnloadPassengers,
            has_attached_passenger: true,
            exit_object_succeeded: true,
            has_attached_passenger_after_exit: false,
            ..unload_context()
        });
        assert!(success.detach_passenger);
        assert!(success.pick_up_aircraft_from_map);
        assert!(success.attempt_exit_object);
        assert!(!success.delete_passenger_on_exit_failure);
        assert!(success.place_aircraft_on_map);
        assert!(success.enter_idle_mode);

        let failure = aircraft_mission_unload(AircraftUnloadContext {
            stage: AircraftUnloadStage::UnloadPassengers,
            has_attached_passenger: true,
            exit_object_succeeded: false,
            has_attached_passenger_after_exit: false,
            ..unload_context()
        });
        assert!(failure.delete_passenger_on_exit_failure);
        let tethered = aircraft_mission_unload(AircraftUnloadContext {
            stage: AircraftUnloadStage::UnloadPassengers,
            is_tethered: true, ..unload_context()
        });
        assert!(!tethered.detach_passenger);
        assert!(!tethered.enter_idle_mode);
    }

    #[test]
    fn unload_takeoff_retries_search_with_cargo_or_enters_idle_without_cargo() {
        let retry = aircraft_mission_unload(AircraftUnloadContext {
            stage: AircraftUnloadStage::TakeOff, takeoff_completed: true,
            has_attached_passenger: true, ..unload_context()
        });
        assert!(retry.process_takeoff);
        assert_eq!(retry.next_stage, Some(AircraftUnloadStage::SearchForLandingZone));
        assert!(retry.break_radio_contact);

        let done = aircraft_mission_unload(AircraftUnloadContext {
            stage: AircraftUnloadStage::TakeOff, takeoff_completed: true,
            has_attached_passenger: false, ..unload_context()
        });
        assert!(done.enter_idle_mode);
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
