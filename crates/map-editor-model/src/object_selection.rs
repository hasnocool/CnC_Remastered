//! Scenario-editor object selection and manipulation ports for
//! TIBERIANDAWN/MAPEDSEL.CPP and REDALERT/MAPEDSEL.CPP.
//!
//! Upstream baseline: electronicarts/CnC_Remastered_Collection commit
//! f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae.
//!
//! The editor's object graph, gadgets, map and rendering calls are exposed as
//! ordered plans. A host applies the plans on its editor thread; this module
//! keeps variant-specific menu and ownership rules deterministic.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorVariant { TiberianDawn, RedAlert }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorSession { SinglePlayer, Multiplayer }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EditorObjectId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorObjectKind {
    Vessel, Unit, Infantry, Aircraft, Building, Terrain, Overlay, Other,
}

impl EditorObjectKind {
    pub const fn is_techno(self) -> bool {
        matches!(self, Self::Vessel | Self::Unit | Self::Infantry | Self::Aircraft | Self::Building)
    }
    pub const fn is_infantry(self) -> bool { matches!(self, Self::Infantry) }
    pub const fn is_building(self) -> bool { matches!(self, Self::Building) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EditorObjectSnapshot {
    pub id: EditorObjectId,
    pub kind: EditorObjectKind,
    pub cell: i16,
    pub coordinate: u32,
    pub owner: u8,
    pub mission: u8,
    /// Legacy Health_Ratio() return value; Red Alert multiplies by 256.
    pub health_ratio: i32,
    pub strength: i32,
    pub primary_facing: u8,
    pub turret_equipped: bool,
    pub allowed_to_sell: bool,
    pub to_rebuild: bool,
    pub is_in_ai_base: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionAction {
    UnselectAll, Select(EditorObjectId), SetDefaultMouse, OverrideNormalMouse,
    ClearHiddenPage, FlagFullRedraw, RebuildPopupControls,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectObjectPlan {
    pub return_code: i32,
    pub selected_object: Option<EditorObjectId>,
    pub actions: Vec<SelectionAction>,
}

/// Port of Select_Object. The host supplies the result of cell hit-testing.
pub fn select_object_plan(
    current_selection: &[EditorObjectId],
    clicked_object: Option<EditorObjectId>,
) -> SelectObjectPlan {
    let mut actions = Vec::new();
    let mut selected_object = current_selection.first().copied();
    match clicked_object {
        None => {
            if !current_selection.is_empty() {
                actions.extend([SelectionAction::UnselectAll, SelectionAction::RebuildPopupControls]);
            }
            selected_object = None;
        }
        Some(object) => {
            if selected_object != Some(object) {
                actions.extend([
                    SelectionAction::UnselectAll,
                    SelectionAction::Select(object),
                    SelectionAction::SetDefaultMouse,
                    SelectionAction::OverrideNormalMouse,
                    SelectionAction::RebuildPopupControls,
                ]);
                selected_object = Some(object);
            }
        }
    }
    actions.extend([SelectionAction::ClearHiddenPage, SelectionAction::FlagFullRedraw]);
    SelectObjectPlan {
        return_code: if clicked_object.is_some() { 0 } else { -1 },
        selected_object,
        actions,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TacticalViewport {
    pub tactical_left_cell: i32,
    pub tactical_top_cell: i32,
    pub width_cells: i32,
    pub height_cells: i32,
    pub map_width_cells: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TacticalPosition { pub x_cell: i32, pub y_cell: i32 }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectNextAction {
    UnselectAll, Select(EditorObjectId), SetDefaultMouse, OverrideNormalMouse,
    RebuildPopupControls, IncrementScenarioInit, SetTacticalPosition(TacticalPosition),
    DecrementScenarioInit, ClearHiddenPage, FlagFullRedraw,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectNextPlan {
    pub selected_object: EditorObjectId,
    pub viewport: TacticalPosition,
    pub actions: Vec<SelectNextAction>,
}

/// Port of Select_Next. Current selection must exist, just as in the original
/// routine which dereferences CurrentObject[0] if no next object was found.
pub fn select_next_plan(
    current_object: EditorObjectSnapshot,
    next_object: Option<EditorObjectSnapshot>,
    viewport: TacticalViewport,
) -> SelectNextPlan {
    let selected = next_object.unwrap_or(current_object);
    let mut actions = Vec::new();
    if let Some(next) = next_object {
        actions.extend([SelectNextAction::UnselectAll, SelectNextAction::Select(next.id)]);
    }
    actions.extend([
        SelectNextAction::SetDefaultMouse,
        SelectNextAction::OverrideNormalMouse,
        SelectNextAction::RebuildPopupControls,
    ]);

    let map_width = viewport.map_width_cells.max(1);
    let object_x = i32::from(selected.cell).rem_euclid(map_width);
    let object_y = i32::from(selected.cell).div_euclid(map_width);
    let mut new_x = viewport.tactical_left_cell;
    let mut new_y = viewport.tactical_top_cell;
    if object_x < new_x {
        new_x = object_x;
    } else if object_x >= new_x + viewport.width_cells {
        new_x = object_x - viewport.width_cells + 1;
    }
    if object_y < new_y {
        new_y = object_y;
    } else if object_y >= new_y + viewport.height_cells {
        new_y = object_y - viewport.height_cells + 1;
    }
    let viewport_position = TacticalPosition { x_cell: new_x, y_cell: new_y };
    actions.extend([
        SelectNextAction::IncrementScenarioInit,
        SelectNextAction::SetTacticalPosition(viewport_position),
        SelectNextAction::DecrementScenarioInit,
        SelectNextAction::ClearHiddenPage,
        SelectNextAction::FlagFullRedraw,
    ]);
    SelectNextPlan { selected_object: selected.id, viewport: viewport_position, actions }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HouseButton { Gdi, Nod, Neutral, Multi(u8), HouseList }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopupControl {
    House(HouseButton), MissionList, HealthGauge, HealthText, FacingDial,
    BaseGauge, BaseLabel, MapArea, Sellable, Rebuildable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopupControlState { TurnOn, TurnOff }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PopupControlsPlan {
    pub remove_controls: Vec<PopupControl>,
    pub add_controls: Vec<PopupControl>,
    pub selected_house: Option<u8>,
    pub selected_mission_index: Option<usize>,
    pub health_gauge_value: Option<i32>,
    pub health_text: Option<String>,
    pub facing: Option<u8>,
    pub sellable_state: Option<PopupControlState>,
    pub rebuildable_state: Option<PopupControlState>,
}

fn house_bit(mask: u32, house: u8) -> bool {
    house < 32 && (mask & (1_u32 << house)) != 0
}

/// Port of both variants' Popup_Controls. mission_ids is the shared
/// MapEditMissions table. Tiberian Dawn filters faction buttons by ownable mask;
/// Red Alert uses a house-list gadget and building sell/rebuild toggles.
pub fn popup_controls_plan(
    variant: EditorVariant,
    session: EditorSession,
    object: Option<EditorObjectSnapshot>,
    mission_ids: &[u8],
    ownable_mask: u32,
) -> PopupControlsPlan {
    let remove_controls = match variant {
        EditorVariant::TiberianDawn => vec![
            PopupControl::House(HouseButton::Gdi),
            PopupControl::House(HouseButton::Nod),
            PopupControl::House(HouseButton::Neutral),
            PopupControl::House(HouseButton::Multi(1)),
            PopupControl::House(HouseButton::Multi(2)),
            PopupControl::House(HouseButton::Multi(3)),
            PopupControl::House(HouseButton::Multi(4)),
            PopupControl::MissionList, PopupControl::HealthGauge, PopupControl::HealthText,
            PopupControl::FacingDial, PopupControl::BaseGauge, PopupControl::BaseLabel,
            PopupControl::MapArea,
        ],
        EditorVariant::RedAlert => vec![
            PopupControl::House(HouseButton::HouseList),
            PopupControl::MissionList, PopupControl::HealthGauge, PopupControl::HealthText,
            PopupControl::FacingDial, PopupControl::BaseGauge, PopupControl::BaseLabel,
            PopupControl::MapArea, PopupControl::Sellable, PopupControl::Rebuildable,
        ],
    };
    let base_controls = || vec![PopupControl::BaseGauge, PopupControl::BaseLabel, PopupControl::MapArea];
    let Some(object) = object.filter(|value| value.kind.is_techno()) else {
        return PopupControlsPlan {
            remove_controls, add_controls: base_controls(), selected_house: None,
            selected_mission_index: None, health_gauge_value: None, health_text: None,
            facing: None, sellable_state: None, rebuildable_state: None,
        };
    };

    let mission_index = mission_ids.iter().position(|mission| *mission == object.mission).unwrap_or(0);
    let health_value = match variant {
        EditorVariant::TiberianDawn => object.health_ratio,
        EditorVariant::RedAlert => object.health_ratio.wrapping_mul(256),
    };
    let mut add_controls = Vec::new();
    let mut sellable_state = None;
    let mut rebuildable_state = None;

    match variant {
        EditorVariant::TiberianDawn => {
            let candidate_houses: &[u8] = if session == EditorSession::Multiplayer {
                &[2, 4, 5, 6, 7]
            } else {
                &[2, 1, 0]
            };
            for &house in candidate_houses {
                if house_bit(ownable_mask, house) {
                    add_controls.push(PopupControl::House(match house {
                        0 => HouseButton::Gdi,
                        1 => HouseButton::Nod,
                        2 => HouseButton::Neutral,
                        other => HouseButton::Multi(other - 3),
                    }));
                }
            }
            match object.kind {
                EditorObjectKind::Unit | EditorObjectKind::Infantry | EditorObjectKind::Aircraft => {
                    add_controls.extend([
                        PopupControl::MissionList, PopupControl::HealthGauge,
                        PopupControl::HealthText, PopupControl::FacingDial,
                    ]);
                }
                EditorObjectKind::Building => {
                    add_controls.extend([PopupControl::HealthGauge, PopupControl::HealthText]);
                    if object.turret_equipped { add_controls.push(PopupControl::FacingDial); }
                }
                _ => {}
            }
        }
        EditorVariant::RedAlert => {
            match object.kind {
                EditorObjectKind::Vessel | EditorObjectKind::Unit
                | EditorObjectKind::Infantry | EditorObjectKind::Aircraft => {
                    add_controls.extend([
                        PopupControl::HealthGauge, PopupControl::House(HouseButton::HouseList),
                        PopupControl::MissionList, PopupControl::HealthText, PopupControl::FacingDial,
                    ]);
                }
                EditorObjectKind::Building => {
                    add_controls.extend([
                        PopupControl::HealthGauge, PopupControl::House(HouseButton::HouseList),
                        PopupControl::HealthText, PopupControl::Sellable, PopupControl::Rebuildable,
                    ]);
                    sellable_state = Some(if object.allowed_to_sell {
                        PopupControlState::TurnOn
                    } else { PopupControlState::TurnOff });
                    rebuildable_state = Some(if object.to_rebuild {
                        PopupControlState::TurnOn
                    } else { PopupControlState::TurnOff });
                    if object.turret_equipped { add_controls.push(PopupControl::FacingDial); }
                }
                _ => {}
            }
        }
    }
    // The map area is always last so it cannot intercept the gadget input.
    add_controls.extend([PopupControl::BaseGauge, PopupControl::BaseLabel, PopupControl::MapArea]);
    let has_mission_list = add_controls.contains(&PopupControl::MissionList);
    let has_health_gauge = add_controls.contains(&PopupControl::HealthGauge);
    let has_health_text = add_controls.contains(&PopupControl::HealthText);
    let has_facing = add_controls.contains(&PopupControl::FacingDial);
    PopupControlsPlan {
        remove_controls, add_controls, selected_house: Some(object.owner),
        selected_mission_index: has_mission_list.then_some(mission_index),
        health_gauge_value: has_health_gauge.then_some(health_value),
        health_text: has_health_text.then(|| object.strength.to_string()),
        facing: has_facing.then_some(object.primary_facing),
        sellable_state, rebuildable_state,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GrabState { pub object: EditorObjectId, pub offset_cells: i16 }

/// Port of Grab_Object: retain the object's cell offset from ZoneCell.
pub fn grab_object_plan(selected: Option<EditorObjectSnapshot>, zone_cell: i16) -> Option<GrabState> {
    selected.map(|object| GrabState {
        object: object.id,
        offset_cells: object.cell.wrapping_sub(zone_cell),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseNodeCoordinate { FullCoordinate(u32), Cell(i16) }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoveObjectAction {
    MarkUp,
    ClearOldInfantryOccupancy(u32),
    UpdateBaseNode(BaseNodeCoordinate),
    SetObjectCoordinate(u32),
    MarkDown,
    SetNewInfantryOccupancy(Option<u32>),
    SetDefaultMouse,
    OverrideNormalMouse,
    FlagFullRedraw,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveGrabbedObjectPlan {
    pub return_code: i32,
    pub new_coordinate: Option<u32>,
    pub actions: Vec<MoveObjectAction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveGrabbedObjectInput {
    pub variant: EditorVariant,
    pub object: EditorObjectSnapshot,
    pub zone_cell: i16,
    pub grab_offset: i16,
    pub infantry_spot_is_free: bool,
    pub closest_free_spot: Option<u32>,
    pub target_cell_center: u32,
    pub can_enter_target_cell: bool,
    pub building_is_in_base: bool,
}

/// Port of Move_Grabbed_Object. Cell-space checks and Closest_Free_Spot are
/// evaluated by the host; this function preserves lift/place/occupancy order.
pub fn move_grabbed_object_plan(input: MoveGrabbedObjectInput) -> MoveGrabbedObjectPlan {
    let mut actions = vec![MoveObjectAction::MarkUp];
    let infantry = input.object.kind.is_infantry();
    let mut destination = None;
    if infantry {
        if input.infantry_spot_is_free {
            destination = input.closest_free_spot;
            actions.push(MoveObjectAction::ClearOldInfantryOccupancy(input.object.coordinate));
        }
    } else {
        let destination_cell = input.zone_cell.wrapping_add(input.grab_offset);
        let coordinate = if matches!(input.object.kind, EditorObjectKind::Building | EditorObjectKind::Terrain) {
            input.target_cell_center & 0xFF00_FF00
        } else {
            input.target_cell_center
        };
        if input.can_enter_target_cell { destination = Some(coordinate); }
        if destination.is_some() && input.object.kind.is_building() && input.building_is_in_base {
            let base_coordinate = match input.variant {
                EditorVariant::TiberianDawn => BaseNodeCoordinate::FullCoordinate(coordinate),
                EditorVariant::RedAlert => BaseNodeCoordinate::Cell(destination_cell),
            };
            actions.push(MoveObjectAction::UpdateBaseNode(base_coordinate));
        }
    }
    if let Some(coordinate) = destination {
        actions.push(MoveObjectAction::SetObjectCoordinate(coordinate));
    }
    actions.push(MoveObjectAction::MarkDown);
    if infantry {
        // The source calls Set_Occupy_Bit even if its new coordinate is NULL.
        actions.push(MoveObjectAction::SetNewInfantryOccupancy(destination));
    }
    actions.extend([
        MoveObjectAction::SetDefaultMouse,
        MoveObjectAction::OverrideNormalMouse,
        MoveObjectAction::FlagFullRedraw,
    ]);
    MoveGrabbedObjectPlan {
        return_code: if destination.is_some() { 0 } else { -1 },
        new_coordinate: destination,
        actions,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangeHouseInput {
    pub variant: EditorVariant,
    pub object: Option<EditorObjectSnapshot>,
    pub new_house: u8,
    pub house_exists: bool,
    pub new_house_is_player: bool,
    pub ownable_mask: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangeHousePlan {
    pub changed: bool,
    pub new_owner: Option<u8>,
    /// Red Alert rewrites IsOwnedByPlayer; Tiberian Dawn leaves it unchanged.
    pub is_owned_by_player: Option<bool>,
}

/// Port of Change_House. Tiberian Dawn checks the object ownable mask; Red
/// Alert's check is commented out and instead updates IsOwnedByPlayer.
pub fn change_house_plan(input: ChangeHouseInput) -> ChangeHousePlan {
    let Some(object) = input.object else {
        return ChangeHousePlan { changed: false, new_owner: None, is_owned_by_player: None };
    };
    if !object.kind.is_techno()
        || (object.kind.is_building() && object.is_in_ai_base)
        || !input.house_exists
    {
        return ChangeHousePlan { changed: false, new_owner: None, is_owned_by_player: None };
    }
    if input.variant == EditorVariant::TiberianDawn && !house_bit(input.ownable_mask, input.new_house) {
        return ChangeHousePlan { changed: false, new_owner: None, is_owned_by_player: None };
    }
    ChangeHousePlan {
        changed: true,
        new_owner: Some(input.new_house),
        is_owned_by_player: (input.variant == EditorVariant::RedAlert).then_some(input.new_house_is_player),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn object(kind: EditorObjectKind) -> EditorObjectSnapshot {
        EditorObjectSnapshot {
            id: EditorObjectId(7), kind, cell: 130, coordinate: 0x1234_5678,
            owner: 2, mission: 11, health_ratio: 128, strength: 60,
            primary_facing: 96, turret_equipped: true, allowed_to_sell: true,
            to_rebuild: false, is_in_ai_base: false,
        }
    }

    #[test]
    fn empty_click_unselects_but_same_object_click_preserves_selection() {
        let empty = select_object_plan(&[EditorObjectId(1)], None);
        assert_eq!(empty.return_code, -1);
        assert_eq!(empty.selected_object, None);
        assert!(empty.actions.contains(&SelectionAction::UnselectAll));
        assert!(empty.actions.contains(&SelectionAction::RebuildPopupControls));
        assert_eq!(empty.actions.last(), Some(&SelectionAction::FlagFullRedraw));

        let same = select_object_plan(&[EditorObjectId(1)], Some(EditorObjectId(1)));
        assert_eq!(same.return_code, 0);
        assert_eq!(same.selected_object, Some(EditorObjectId(1)));
        assert!(!same.actions.contains(&SelectionAction::UnselectAll));
        assert!(!same.actions.contains(&SelectionAction::RebuildPopupControls));
    }

    #[test]
    fn different_object_click_unselects_selects_and_resets_mouse() {
        let plan = select_object_plan(&[EditorObjectId(1)], Some(EditorObjectId(2)));
        assert_eq!(plan.actions, vec![
            SelectionAction::UnselectAll, SelectionAction::Select(EditorObjectId(2)),
            SelectionAction::SetDefaultMouse, SelectionAction::OverrideNormalMouse,
            SelectionAction::RebuildPopupControls, SelectionAction::ClearHiddenPage,
            SelectionAction::FlagFullRedraw,
        ]);
    }

    #[test]
    fn select_next_recenters_object_outside_tactical_viewport() {
        let current = object(EditorObjectKind::Unit);
        let next = EditorObjectSnapshot { id: EditorObjectId(8), cell: 25 * 128 + 40, ..current };
        let viewport = TacticalViewport {
            tactical_left_cell: 10, tactical_top_cell: 10,
            width_cells: 20, height_cells: 15, map_width_cells: 128,
        };
        let plan = select_next_plan(current, Some(next), viewport);
        assert_eq!(plan.selected_object, EditorObjectId(8));
        assert_eq!(plan.viewport, TacticalPosition { x_cell: 21, y_cell: 16 });
        assert!(plan.actions.contains(&SelectNextAction::IncrementScenarioInit));
        assert!(plan.actions.contains(&SelectNextAction::DecrementScenarioInit));
    }

    #[test]
    fn select_next_keeps_current_object_when_there_is_no_next_object() {
        let current = object(EditorObjectKind::Unit);
        let viewport = TacticalViewport {
            tactical_left_cell: 0, tactical_top_cell: 0,
            width_cells: 40, height_cells: 30, map_width_cells: 128,
        };
        let plan = select_next_plan(current, None, viewport);
        assert_eq!(plan.selected_object, current.id);
        assert!(!plan.actions.contains(&SelectNextAction::UnselectAll));
        assert!(!plan.actions.contains(&SelectNextAction::Select(current.id)));
    }

    #[test]
    fn popup_for_empty_or_non_techno_object_keeps_base_controls_only() {
        for selected in [None, Some(object(EditorObjectKind::Terrain)), Some(object(EditorObjectKind::Overlay))] {
            let plan = popup_controls_plan(EditorVariant::RedAlert, EditorSession::SinglePlayer, selected, &[], 0);
            assert_eq!(plan.add_controls, vec![
                PopupControl::BaseGauge, PopupControl::BaseLabel, PopupControl::MapArea,
            ]);
            assert_eq!(plan.health_gauge_value, None);
        }
    }

    #[test]
    fn tiberian_dawn_popup_filters_owner_buttons_by_mask_and_session() {
        let mask = (1 << 0) | (1 << 2) | (1 << 4) | (1 << 6);
        let multiplayer = popup_controls_plan(
            EditorVariant::TiberianDawn, EditorSession::Multiplayer,
            Some(object(EditorObjectKind::Unit)), &[10, 11], mask,
        );
        assert_eq!(
            multiplayer.add_controls.iter().filter(|c| matches!(c, PopupControl::House(_))).copied().collect::<Vec<_>>(),
            vec![PopupControl::House(HouseButton::Neutral),
                 PopupControl::House(HouseButton::Multi(1)),
                 PopupControl::House(HouseButton::Multi(3))]
        );
        let single = popup_controls_plan(
            EditorVariant::TiberianDawn, EditorSession::SinglePlayer,
            Some(object(EditorObjectKind::Unit)), &[10, 11], mask,
        );
        assert_eq!(
            single.add_controls.iter().filter(|c| matches!(c, PopupControl::House(_))).copied().collect::<Vec<_>>(),
            vec![PopupControl::House(HouseButton::Neutral), PopupControl::House(HouseButton::Gdi)]
        );
    }

    #[test]
    fn red_alert_popup_uses_house_list_and_building_sale_rebuild_toggles() {
        let mut building = object(EditorObjectKind::Building);
        building.allowed_to_sell = false;
        building.to_rebuild = true;
        let plan = popup_controls_plan(EditorVariant::RedAlert, EditorSession::SinglePlayer, Some(building), &[], 0);
        assert!(plan.add_controls.contains(&PopupControl::House(HouseButton::HouseList)));
        assert!(plan.add_controls.contains(&PopupControl::Sellable));
        assert!(plan.add_controls.contains(&PopupControl::Rebuildable));
        assert_eq!(plan.sellable_state, Some(PopupControlState::TurnOff));
        assert_eq!(plan.rebuildable_state, Some(PopupControlState::TurnOn));
        assert_eq!(plan.health_gauge_value, Some(128 * 256));
        assert_eq!(plan.facing, Some(96));
        assert_eq!(plan.add_controls.last(), Some(&PopupControl::MapArea));
    }

    #[test]
    fn popup_strength_and_mission_selection_preserve_variant_scaling() {
        let td = popup_controls_plan(
            EditorVariant::TiberianDawn, EditorSession::SinglePlayer,
            Some(object(EditorObjectKind::Aircraft)), &[3, 11], 0,
        );
        let ra = popup_controls_plan(
            EditorVariant::RedAlert, EditorSession::SinglePlayer,
            Some(object(EditorObjectKind::Vessel)), &[3, 11], 0,
        );
        assert_eq!(td.health_gauge_value, Some(128));
        assert_eq!(ra.health_gauge_value, Some(128 * 256));
        assert_eq!(td.selected_mission_index, Some(1));
        assert_eq!(ra.selected_mission_index, Some(1));
    }

    #[test]
    fn grab_object_retains_offset_from_zone_cell() {
        let state = grab_object_plan(Some(object(EditorObjectKind::Unit)), 100).unwrap();
        assert_eq!(state.object, EditorObjectId(7));
        assert_eq!(state.offset_cells, 30);
        assert_eq!(grab_object_plan(None, 0), None);
    }

    #[test]
    fn infantry_move_uses_closest_free_spot_and_updates_occupancy() {
        let plan = move_grabbed_object_plan(MoveGrabbedObjectInput {
            variant: EditorVariant::RedAlert, object: object(EditorObjectKind::Infantry),
            zone_cell: 100, grab_offset: 30, infantry_spot_is_free: true,
            closest_free_spot: Some(0xABCD_0123), target_cell_center: 0,
            can_enter_target_cell: true, building_is_in_base: false,
        });
        assert_eq!(plan.return_code, 0);
        assert_eq!(plan.new_coordinate, Some(0xABCD_0123));
        assert!(plan.actions.contains(&MoveObjectAction::ClearOldInfantryOccupancy(0x1234_5678)));
        assert!(plan.actions.contains(&MoveObjectAction::SetNewInfantryOccupancy(Some(0xABCD_0123))));
    }

    #[test]
    fn failed_infantry_move_marks_down_and_sets_null_occupancy() {
        let plan = move_grabbed_object_plan(MoveGrabbedObjectInput {
            variant: EditorVariant::TiberianDawn, object: object(EditorObjectKind::Infantry),
            zone_cell: 100, grab_offset: 30, infantry_spot_is_free: false,
            closest_free_spot: None, target_cell_center: 0,
            can_enter_target_cell: false, building_is_in_base: false,
        });
        assert_eq!(plan.return_code, -1);
        assert_eq!(plan.new_coordinate, None);
        assert!(plan.actions.contains(&MoveObjectAction::MarkDown));
        assert!(plan.actions.contains(&MoveObjectAction::SetNewInfantryOccupancy(None)));
    }

    #[test]
    fn building_move_updates_variant_specific_base_node_location() {
        let td = move_grabbed_object_plan(MoveGrabbedObjectInput {
            variant: EditorVariant::TiberianDawn, object: object(EditorObjectKind::Building),
            zone_cell: 100, grab_offset: 30, infantry_spot_is_free: false,
            closest_free_spot: None, target_cell_center: 0x1234_5678,
            can_enter_target_cell: true, building_is_in_base: true,
        });
        assert!(td.actions.contains(&MoveObjectAction::UpdateBaseNode(BaseNodeCoordinate::FullCoordinate(0x1200_5600))));
        let ra = move_grabbed_object_plan(MoveGrabbedObjectInput {
            variant: EditorVariant::RedAlert, object: object(EditorObjectKind::Building),
            zone_cell: 100, grab_offset: 30, infantry_spot_is_free: false,
            closest_free_spot: None, target_cell_center: 0x1234_5678,
            can_enter_target_cell: true, building_is_in_base: true,
        });
        assert!(ra.actions.contains(&MoveObjectAction::UpdateBaseNode(BaseNodeCoordinate::Cell(130))));
    }

    #[test]
    fn invalid_non_infantry_destination_is_rejected_after_mark_up() {
        let plan = move_grabbed_object_plan(MoveGrabbedObjectInput {
            variant: EditorVariant::RedAlert, object: object(EditorObjectKind::Unit),
            zone_cell: 100, grab_offset: 30, infantry_spot_is_free: false,
            closest_free_spot: None, target_cell_center: 0x1234_5678,
            can_enter_target_cell: false, building_is_in_base: false,
        });
        assert_eq!(plan.return_code, -1);
        assert_eq!(plan.actions.first(), Some(&MoveObjectAction::MarkUp));
        assert_eq!(plan.actions.get(1), Some(&MoveObjectAction::MarkDown));
    }

    #[test]
    fn tiberian_dawn_change_house_checks_ownable_mask_without_player_flag_write() {
        let input = ChangeHouseInput {
            variant: EditorVariant::TiberianDawn, object: Some(object(EditorObjectKind::Unit)),
            new_house: 2, house_exists: true, new_house_is_player: false, ownable_mask: 1 << 2,
        };
        let result = change_house_plan(input);
        assert!(result.changed);
        assert_eq!(result.new_owner, Some(2));
        assert_eq!(result.is_owned_by_player, None);
        assert!(!change_house_plan(ChangeHouseInput { ownable_mask: 0, ..input }).changed);
    }

    #[test]
    fn red_alert_change_house_skips_ownable_mask_and_updates_player_flag() {
        let input = ChangeHouseInput {
            variant: EditorVariant::RedAlert, object: Some(object(EditorObjectKind::Unit)),
            new_house: 7, house_exists: true, new_house_is_player: true, ownable_mask: 0,
        };
        let result = change_house_plan(input);
        assert!(result.changed);
        assert_eq!(result.is_owned_by_player, Some(true));
    }

    #[test]
    fn both_variants_reject_base_buildings_non_techno_objects_and_missing_houses() {
        for variant in [EditorVariant::TiberianDawn, EditorVariant::RedAlert] {
            let mut building = object(EditorObjectKind::Building);
            building.is_in_ai_base = true;
            let input = ChangeHouseInput {
                variant, object: Some(building), new_house: 1, house_exists: true,
                new_house_is_player: false, ownable_mask: u32::MAX,
            };
            assert!(!change_house_plan(input).changed);
            assert!(!change_house_plan(ChangeHouseInput { house_exists: false, ..input }).changed);
            assert!(!change_house_plan(ChangeHouseInput {
                object: Some(object(EditorObjectKind::Terrain)), ..input
            }).changed);
        }
    }
}
