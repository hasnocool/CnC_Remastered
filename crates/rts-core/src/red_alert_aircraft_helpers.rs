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
    rearm_delay: i32,
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
        let _ = second_shot; // used by the upstream Rearm_Delay callback, not the result itself.
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
            true, true, false, 10,
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
            false, false, true, 30,
        );
        assert_eq!(single.path, AircraftFireAtPath::CameraSnapshot);
        assert_eq!(single.sight_requests, [AircraftSightRequest { house_id: 7, sight_range: 9 }]);
        assert!(single.clear_ammo);
        assert_eq!(single.rearm_timer_after_call, Some(30));
        assert!(!single.call_base_fire);

        let multiplayer = aircraft_fire_at_plan(
            false, true, true, 7, false, &[7, 9, 12], Some(7),
            false, false, false, 40,
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
            true, true, false, 0,
        );
        assert_eq!(plan.path, AircraftFireAtPath::StandardWeapon);
        assert_eq!(plan.sight_requests, [AircraftSightRequest { house_id: 8, sight_range: 1 }]);
        assert!(plan.call_base_fire);
        assert!(plan.return_bullet);
        assert_eq!(plan.dropping_bullet_speed, Some((40, "MPH_MEDIUM_SLOW")));
        let no_bullet = aircraft_fire_at_plan(
            false, false, false, 7, false, &[], None,
            false, true, false, 0,
        );
        assert!(no_bullet.sight_requests.is_empty());
        assert!(!no_bullet.return_bullet);
        assert_eq!(no_bullet.dropping_bullet_speed, None);
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
