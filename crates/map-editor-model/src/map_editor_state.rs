//! Owning aggregate that connects the editable map document to its spatial indexes.
//!
//! The wrapper keeps serialized cell data separate from runtime/editor spatial
//! indexes while providing one mutation boundary for placed buildings and walls.

use std::collections::{BTreeMap, HashMap};
use rts_core::{CellCoord, GameVariant, MapDimensions};
use crate::{
    MapDocument, MapDocumentError, MapSpatialIndex, OccupierId, OverlapRect,
    SpatialIndexError, OverlayInvalidation, MapUpdateError, MapUpdateWork,
    OverlayTypeDefinition, PlacedOverlay, SmudgeTypeDefinition, PlacedSmudge, EditorColor, SmudgeTypeFlag, resource_icon, wall_icon,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapEditorState {
    document: MapDocument,
    spatial: MapSpatialIndex,
    overlay_types: BTreeMap<i8, OverlayTypeDefinition>,
    overlays: HashMap<CellCoord, (PlacedOverlay, Option<OccupierId>)>,
    smudge_types: BTreeMap<i8, SmudgeTypeDefinition>,
    smudges: HashMap<CellCoord, PlacedSmudge>,
    building_bib_cells: HashMap<OccupierId, Vec<CellCoord>>,
    next_internal_occupier: u64,
}

impl MapEditorState {
    pub fn new(variant: GameVariant, dimensions: MapDimensions) -> Result<Self, MapDocumentError> {
        Ok(Self {
            document: MapDocument::new(variant, dimensions)?,
            spatial: MapSpatialIndex::new(dimensions),
            overlay_types: BTreeMap::new(),
            overlays: HashMap::new(),
            smudge_types: BTreeMap::new(),
            smudges: HashMap::new(),
            building_bib_cells: HashMap::new(),
            next_internal_occupier: u64::MAX,
        })
    }

    pub fn document(&self) -> &MapDocument { &self.document }
    pub fn document_mut(&mut self) -> &mut MapDocument { &mut self.document }

    pub fn begin_update(&mut self) { self.document.begin_update(); }

    /// Ends one update batch and immediately applies its coalesced derived icons.
    pub fn end_update(&mut self) -> Result<Option<MapUpdateWork>, MapUpdateError> {
        let work = self.document.end_update()?;
        if let Some(ref pending) = work { self.apply_update_work(pending); }
        Ok(work)
    }
    pub fn spatial(&self) -> &MapSpatialIndex { &self.spatial }

    pub fn register_overlay_type(&mut self, definition: OverlayTypeDefinition) {
        self.overlay_types.insert(definition.id, definition);
    }

    pub fn register_smudge_type(&mut self, definition: SmudgeTypeDefinition) {
        self.smudge_types.insert(definition.id, definition);
    }

    pub fn smudge_at(&self, cell: CellCoord) -> Option<&PlacedSmudge> {
        self.smudges.get(&cell)
    }

    pub fn building_bib_cells(&self, id: OccupierId) -> Option<&[CellCoord]> {
        self.building_bib_cells.get(&id).map(Vec::as_slice)
    }

    pub fn overlay_at(&self, cell: CellCoord) -> Option<&PlacedOverlay> {
        self.overlays.get(&cell).map(|(overlay, _)| overlay)
    }

    /// Replaces one placed overlay and synchronizes wall occupancy before
    /// publishing the cell change. Failed spatial registration leaves the
    /// prior overlay and indexes intact.
    pub fn set_overlay(
        &mut self,
        cell: CellCoord,
        overlay: Option<PlacedOverlay>,
    ) -> Result<Option<MapUpdateWork>, MapEditorStateError> {
        if !self.document.dimensions().contains(cell) {
            return Err(MapEditorStateError::CellOutOfBounds(cell));
        }
        let old = self.overlays.get(&cell).map(|(placed, _)| placed.clone());
        if old == overlay { return Ok(None); }

        let old_type = old.as_ref().and_then(|value| self.overlay_types.get(&value.type_id));
        let new_type = overlay.as_ref().map(|value| {
            self.overlay_types.get(&value.type_id).ok_or(MapEditorStateError::UnknownOverlayType(value.type_id))
        }).transpose()?;

        let mut staged = self.spatial.clone();
        if let Some((_, Some(id))) = self.overlays.get(&cell) {
            if staged.is_wall(*id) { staged.remove_wall(*id); }
        }
        let new_id = if new_type.is_some_and(|definition| definition.is_wall()) {
            let id = Self::allocate_overlay_occupier(&mut self.next_internal_occupier, &staged)?;
            let definition = new_type.expect("checked above");
            staged.add_wall(id, cell, &definition.occupy_mask())
                .map_err(MapEditorStateError::Spatial)?;
            Some(id)
        } else { None };

        self.spatial = staged;
        if let (Some(placed), Some(id)) = (overlay.clone(), new_id) {
            self.overlays.insert(cell, (placed, Some(id)));
        } else if let Some(placed) = overlay.clone() {
            self.overlays.insert(cell, (placed, None));
        } else {
            self.overlays.remove(&cell);
        }

        let mut work = None;
        if old_type.is_some_and(|definition| definition.is_resource())
            || new_type.is_some_and(|definition| definition.is_resource()) {
            work = self.document.invalidate_overlay(cell, OverlayInvalidation::Resources)
                .map_err(MapEditorStateError::Update)?;
        }
        if old_type.is_some_and(|definition| definition.is_wall())
            || new_type.is_some_and(|definition| definition.is_wall()) {
            let wall_work = self.document.invalidate_overlay(cell, OverlayInvalidation::Walls)
                .map_err(MapEditorStateError::Update)?;
            work = merge_update_work(work, wall_work);
        }
        if let Some(ref derived_work) = work {
            self.apply_update_work(derived_work);
        }
        Ok(work)
    }

    /// Applies source-derived overlay icon updates for work returned by the
    /// outermost end_update call. Immediate, non-batched edits apply this
    /// automatically from set_overlay.
    pub fn apply_update_work(&mut self, work: &MapUpdateWork) {
        let dimensions = self.document.dimensions();
        let resource_cells = work.resources.clone();
        for cell in resource_cells {
            let Some((placed, _)) = self.overlays.get(&cell) else { continue; };
            let Some(definition) = self.overlay_types.get(&placed.type_id) else { continue; };
            if !definition.is_resource() { continue; }
            let is_gem = definition.is_gem();
            let mut adjacent = 0_u8;
            for dy in -1_i32..=1 {
                for dx in -1_i32..=1 {
                    if dx == 0 && dy == 0 { continue; }
                    let x = i32::from(cell.x) + dx;
                    let y = i32::from(cell.y) + dy;
                    if x < 0 || y < 0 || x >= i32::from(dimensions.width()) || y >= i32::from(dimensions.height()) {
                        continue;
                    }
                    let neighbor = CellCoord::new(x as u16, y as u16);
                    if self.overlays.get(&neighbor).is_some_and(|(overlay, _)| {
                        self.overlay_types.get(&overlay.type_id).is_some_and(|kind| kind.is_resource())
                    }) {
                        adjacent += 1;
                    }
                }
            }
            if let Some((overlay, _)) = self.overlays.get_mut(&cell) {
                if let Some(icon) = resource_icon(is_gem, adjacent) { overlay.icon = icon; }
            }
        }

        let wall_cells = work.walls.clone();
        for cell in wall_cells {
            let Some((placed, _)) = self.overlays.get(&cell) else { continue; };
            let type_id = placed.type_id;
            let Some(definition) = self.overlay_types.get(&type_id) else { continue; };
            if !definition.is_wall() { continue; }
            let has_same_wall = |x: i32, y: i32, overlays: &HashMap<CellCoord, (PlacedOverlay, Option<OccupierId>)>, types: &BTreeMap<i8, OverlayTypeDefinition>| {
                if x < 0 || y < 0 || x >= i32::from(dimensions.width()) || y >= i32::from(dimensions.height()) { return false; }
                overlays.get(&CellCoord::new(x as u16, y as u16)).is_some_and(|(overlay, _)| {
                    overlay.type_id == type_id && types.get(&overlay.type_id).is_some_and(|kind| kind.is_wall())
                })
            };
            let x = i32::from(cell.x);
            let y = i32::from(cell.y);
            let icon = wall_icon(
                has_same_wall(x, y - 1, &self.overlays, &self.overlay_types),
                has_same_wall(x + 1, y, &self.overlays, &self.overlay_types),
                has_same_wall(x, y + 1, &self.overlays, &self.overlay_types),
                has_same_wall(x - 1, y, &self.overlays, &self.overlay_types),
            );
            if let Some((overlay, _)) = self.overlays.get_mut(&cell) { overlay.icon = icon; }
        }
    }

    fn allocate_overlay_occupier(next_id: &mut u64, spatial: &MapSpatialIndex) -> Result<OccupierId, MapEditorStateError> {
        loop {
            let candidate = OccupierId(*next_id);
            *next_id = (*next_id).checked_sub(1)
                .ok_or(MapEditorStateError::OccupierIdsExhausted)?;
            if !spatial.contains_occupier(candidate) { return Ok(candidate); }
        }
    }

    /// Registers a building and creates its source-defined bib smudges.
    /// Bib placements are staged before spatial registration and committed
    /// only after the building has been accepted by all spatial indexes.
    pub fn add_building_with_bibs(
        &mut self,
        id: OccupierId,
        anchor: CellCoord,
        full_mask: &[(u16, u16)],
        base_mask: &[(u16, u16)],
        overlap_bounds: OverlapRect,
        has_bib: bool,
        building_width: u16,
        building_height: u16,
        tint: EditorColor,
    ) -> Result<(), SpatialIndexError> {
        let mut staged_bibs = Vec::new();
        let selected_flags = match building_width {
            2 => Some(SmudgeTypeFlag::Bib3.bits()),
            3 => Some(SmudgeTypeFlag::Bib2.bits()),
            4 => Some(SmudgeTypeFlag::Bib1.bits()),
            _ => None,
        };
        if has_bib {
            if let Some(flags) = selected_flags {
                if let Some(definition) = self.smudge_types.values().find(|kind| kind.flags == flags) {
                    for row in 0..definition.height {
                        for column in 0..definition.width {
                            let x = u32::from(anchor.x) + u32::from(column);
                            let y = u32::from(anchor.y) + u32::from(building_height) + u32::from(row);
                            if y == 0 || x >= u32::from(self.document.dimensions().width())
                                || y - 1 >= u32::from(self.document.dimensions().height()) {
                                continue;
                            }
                            let cell = CellCoord::new(x as u16, (y - 1) as u16);
                            staged_bibs.push((cell, PlacedSmudge {
                                type_id: definition.id,
                                icon: (row * definition.width + column) as i32,
                                data: 0,
                                tint,
                            }));
                        }
                    }
                }
            }
        }

        self.spatial.add_building(id, anchor, full_mask, base_mask, overlap_bounds)?;
        let cells = staged_bibs.iter().map(|(cell, _)| *cell).collect::<Vec<_>>();
        for (cell, smudge) in staged_bibs { self.smudges.insert(cell, smudge); }
        self.building_bib_cells.insert(id, cells);
        let _ = self.document.request_overlapper_rebuild();
        Ok(())
    }

    pub fn add_building(
        &mut self,
        id: OccupierId,
        anchor: CellCoord,
        full_mask: &[(u16, u16)],
        base_mask: &[(u16, u16)],
        overlap_bounds: OverlapRect,
    ) -> Result<(), SpatialIndexError> {
        self.spatial.add_building(id, anchor, full_mask, base_mask, overlap_bounds)?;
        let _ = self.document.request_overlapper_rebuild();
        Ok(())
    }

    pub fn add_wall(
        &mut self,
        id: OccupierId,
        anchor: CellCoord,
        mask: &[(u16, u16)],
    ) -> Result<(), SpatialIndexError> {
        self.spatial.add_wall(id, anchor, mask)?;
        Ok(())
    }

    pub fn remove_building(&mut self, id: OccupierId) -> bool {
        self.remove_building_bibs(id);
        let removed = self.spatial.remove_building(id);
        if removed { let _ = self.document.request_overlapper_rebuild(); }
        removed
    }

    /// Mirrors Map.RemoveBibs: only currently Bib-flagged smudges are removed.
    /// The ownership set is cleared even when a cell was replaced by another
    /// smudge type after the building was placed.
    pub fn remove_building_bibs(&mut self, id: OccupierId) {
        if let Some(cells) = self.building_bib_cells.remove(&id) {
            for cell in cells {
                let is_bib = self.smudges.get(&cell).and_then(|smudge| self.smudge_types.get(&smudge.type_id))
                    .is_some_and(SmudgeTypeDefinition::is_bib);
                if is_bib { self.smudges.remove(&cell); }
            }
        }
    }

    pub fn remove_wall(&mut self, id: OccupierId) -> bool {
        let cell = self.overlays.iter().find_map(|(cell, (_, overlay_id))| (*overlay_id == Some(id)).then_some(*cell));
        let removed = self.spatial.remove_wall(id);
        if removed {
            if let Some(cell) = cell {
                self.overlays.remove(&cell);
                if let Ok(work) = self.document.invalidate_overlay(cell, OverlayInvalidation::Walls) {
                    if let Some(ref pending) = work { self.apply_update_work(pending); }
                }
            }
        }
        removed
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MapEditorStateError {
    CellOutOfBounds(CellCoord),
    UnknownOverlayType(i8),
    Spatial(SpatialIndexError),
    Update(MapUpdateError),
    OccupierIdsExhausted,
}

fn merge_update_work(
    first: Option<MapUpdateWork>,
    second: Option<MapUpdateWork>,
) -> Option<MapUpdateWork> {
    match (first, second) {
        (None, None) => None,
        (Some(work), None) | (None, Some(work)) => Some(work),
        (Some(mut left), Some(right)) => {
            for cell in right.resources { if !left.resources.contains(&cell) { left.resources.push(cell); } }
            for cell in right.walls { if !left.walls.contains(&cell) { left.walls.push(cell); } }
            left.rebuild_overlappers |= right.rebuild_overlappers;
            Some(left)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dimensions() -> MapDimensions { MapDimensions::new(16, 16).unwrap() }

    #[test]
    fn overlay_cell_changes_mirror_walls_and_invalidate_wall_neighbors() {
        let mut state = MapEditorState::new(GameVariant::RedAlert, dimensions()).unwrap();
        state.register_overlay_type(OverlayTypeDefinition { id: 4, name: "wall".into(), flags: 4 });
        state.document_mut().begin_update();
        state.set_overlay(CellCoord::new(3, 3), Some(PlacedOverlay::new(4))).unwrap();
        let id = state.overlays.get(&CellCoord::new(3, 3)).unwrap().1.unwrap();
        assert!(state.spatial().is_wall(id));
        assert_eq!(state.spatial().buildings().occupant_at(CellCoord::new(3, 3)), Some(id));
        assert_eq!(state.spatial().technos().occupant_at(CellCoord::new(3, 3)), Some(id));
        let work = state.document_mut().end_update().unwrap().unwrap();
        assert_eq!(work.walls.len(), 9);
    }

    #[test]
    fn replacing_wall_with_non_wall_removes_both_occupancy_entries() {
        let mut state = MapEditorState::new(GameVariant::TiberianDawn, dimensions()).unwrap();
        state.register_overlay_type(OverlayTypeDefinition { id: 1, name: "wall".into(), flags: 4 });
        state.register_overlay_type(OverlayTypeDefinition { id: 2, name: "crate".into(), flags: 8 });
        state.set_overlay(CellCoord::new(5, 5), Some(PlacedOverlay::new(1))).unwrap();
        let old_id = state.overlays.get(&CellCoord::new(5, 5)).unwrap().1.unwrap();
        state.set_overlay(CellCoord::new(5, 5), Some(PlacedOverlay::new(2))).unwrap();
        assert!(!state.spatial().buildings().contains(old_id));
        assert!(!state.spatial().technos().contains(old_id));
        assert_eq!(state.overlay_at(CellCoord::new(5, 5)).unwrap().type_id, 2);
    }

    #[test]
    fn removing_wall_id_zero_does_not_remove_non_wall_overlay() {
        let mut state = MapEditorState::new(GameVariant::RedAlert, dimensions()).unwrap();
        state.register_overlay_type(OverlayTypeDefinition { id: 1, name: "crate".into(), flags: 8 });
        state.set_overlay(CellCoord::new(2, 2), Some(PlacedOverlay::new(1))).unwrap();
        state.add_wall(OccupierId(0), CellCoord::new(8, 8), &[(0, 0)]).unwrap();
        assert!(state.remove_wall(OccupierId(0)));
        assert_eq!(state.overlay_at(CellCoord::new(2, 2)).unwrap().type_id, 1);
    }

    #[test]
    fn unknown_overlay_type_does_not_mutate_existing_overlay() {
        let mut state = MapEditorState::new(GameVariant::TiberianDawn, dimensions()).unwrap();
        state.register_overlay_type(OverlayTypeDefinition { id: 1, name: "wall".into(), flags: 4 });
        state.set_overlay(CellCoord::new(2, 2), Some(PlacedOverlay::new(1))).unwrap();
        assert_eq!(
            state.set_overlay(CellCoord::new(2, 2), Some(PlacedOverlay::new(99))),
            Err(MapEditorStateError::UnknownOverlayType(99))
        );
        assert_eq!(state.overlay_at(CellCoord::new(2, 2)).unwrap().type_id, 1);
    }

    #[test]
    fn resource_icons_recompute_from_eight_neighbor_resource_counts() {
        let mut state = MapEditorState::new(GameVariant::TiberianDawn, dimensions()).unwrap();
        state.register_overlay_type(OverlayTypeDefinition { id: 1, name: "ore".into(), flags: 1 });
        state.set_overlay(CellCoord::new(5, 5), Some(PlacedOverlay::new(1))).unwrap();
        assert_eq!(state.overlay_at(CellCoord::new(5, 5)).unwrap().icon, 0);
        state.set_overlay(CellCoord::new(6, 5), Some(PlacedOverlay::new(1))).unwrap();
        assert_eq!(state.overlay_at(CellCoord::new(5, 5)).unwrap().icon, 1);
        assert_eq!(state.overlay_at(CellCoord::new(6, 5)).unwrap().icon, 1);
    }

    #[test]
    fn wall_icons_use_same_type_cardinal_neighbors_only() {
        let mut state = MapEditorState::new(GameVariant::RedAlert, dimensions()).unwrap();
        state.register_overlay_type(OverlayTypeDefinition { id: 1, name: "wall-a".into(), flags: 4 });
        state.register_overlay_type(OverlayTypeDefinition { id: 2, name: "wall-b".into(), flags: 4 });
        state.set_overlay(CellCoord::new(5, 5), Some(PlacedOverlay::new(1))).unwrap();
        state.set_overlay(CellCoord::new(5, 4), Some(PlacedOverlay::new(1))).unwrap();
        state.set_overlay(CellCoord::new(6, 5), Some(PlacedOverlay::new(2))).unwrap();
        assert_eq!(state.overlay_at(CellCoord::new(5, 5)).unwrap().icon, 1);
        assert_eq!(state.overlay_at(CellCoord::new(5, 4)).unwrap().icon, 4);
    }

    #[test]
    fn batched_overlay_changes_apply_icons_when_update_work_is_consumed() {
        let mut state = MapEditorState::new(GameVariant::TiberianDawn, dimensions()).unwrap();
        state.register_overlay_type(OverlayTypeDefinition { id: 1, name: "ore".into(), flags: 1 });
        state.begin_update();
        state.set_overlay(CellCoord::new(5, 5), Some(PlacedOverlay::new(1))).unwrap();
        state.set_overlay(CellCoord::new(6, 5), Some(PlacedOverlay::new(1))).unwrap();
        let work = state.end_update().unwrap().unwrap();
        assert_eq!(state.overlay_at(CellCoord::new(5, 5)).unwrap().icon, 1);
    }

    #[test]
    fn building_bib_uses_width_selected_type_and_source_row_major_icons() {
        let mut state = MapEditorState::new(GameVariant::TiberianDawn, dimensions()).unwrap();
        state.register_smudge_type(SmudgeTypeDefinition {
            id: 10, name: "BIB3".into(), flags: SmudgeTypeFlag::Bib3.bits(), width: 2, height: 1,
        });
        state.add_building_with_bibs(
            OccupierId(70), CellCoord::new(4, 4), &[(0, 0), (1, 0)], &[(0, 0)],
            OverlapRect::new(0, 0, 8, 8), true, 2, 2, EditorColor::WHITE,
        ).unwrap();
        assert_eq!(state.building_bib_cells(OccupierId(70)).unwrap(), &[CellCoord::new(4, 5), CellCoord::new(5, 5)]);
        assert_eq!(state.smudge_at(CellCoord::new(4, 5)).unwrap().type_id, 10);
        assert_eq!(state.smudge_at(CellCoord::new(4, 5)).unwrap().icon, 0);
        assert_eq!(state.smudge_at(CellCoord::new(5, 5)).unwrap().icon, 1);
        assert!(state.remove_building(OccupierId(70)));
        assert!(state.smudge_at(CellCoord::new(4, 5)).is_none());
        assert!(state.building_bib_cells(OccupierId(70)).is_none());
    }

    #[test]
    fn removing_building_bibs_preserves_replacement_non_bib_smudges() {
        let mut state = MapEditorState::new(GameVariant::RedAlert, dimensions()).unwrap();
        state.register_smudge_type(SmudgeTypeDefinition {
            id: 10, name: "BIB2".into(), flags: SmudgeTypeFlag::Bib2.bits(), width: 1, height: 1,
        });
        state.register_smudge_type(SmudgeTypeDefinition {
            id: 11, name: "CRATER".into(), flags: SmudgeTypeFlag::None.bits(), width: 1, height: 1,
        });
        state.add_building_with_bibs(
            OccupierId(71), CellCoord::new(3, 3), &[(0, 0)], &[(0, 0)],
            OverlapRect::new(0, 0, 8, 8), true, 3, 2, EditorColor::WHITE,
        ).unwrap();
        let bib_cell = CellCoord::new(3, 4);
        assert_eq!(state.smudge_at(bib_cell).unwrap().type_id, 10);
        state.smudges.insert(bib_cell, PlacedSmudge::new(11));
        assert!(state.remove_building(OccupierId(71)));
        assert_eq!(state.smudge_at(bib_cell).unwrap().type_id, 11);
    }

    #[test]
    fn building_without_bib_or_unsupported_width_creates_no_bib_cells() {
        let mut state = MapEditorState::new(GameVariant::TiberianDawn, dimensions()).unwrap();
        state.register_smudge_type(SmudgeTypeDefinition {
            id: 10, name: "BIB1".into(), flags: SmudgeTypeFlag::Bib1.bits(), width: 4, height: 1,
        });
        state.add_building_with_bibs(
            OccupierId(72), CellCoord::new(1, 1), &[(0, 0)], &[(0, 0)],
            OverlapRect::new(0, 0, 8, 8), false, 4, 2, EditorColor::WHITE,
        ).unwrap();
        assert!(state.building_bib_cells(OccupierId(72)).unwrap().is_empty());
    }

    #[test]
    fn aggregate_routes_building_and_wall_registration_to_distinct_indexes() {
        let mut state = MapEditorState::new(GameVariant::RedAlert, dimensions()).unwrap();
        state.add_building(
            OccupierId(1), CellCoord::new(2, 2),
            &[(0, 0), (1, 0), (0, 1), (1, 1), (0, 2)],
            &[(0, 0), (1, 0), (0, 1), (1, 1)],
            OverlapRect::new(32, 32, 48, 48),
        ).unwrap();
        state.add_wall(OccupierId(2), CellCoord::new(8, 8), &[(0, 0)]).unwrap();

        assert_eq!(state.spatial().buildings().occupant_at(CellCoord::new(2, 4)), Some(OccupierId(1)));
        assert_eq!(state.spatial().technos().occupant_at(CellCoord::new(2, 4)), None);
        assert_eq!(state.spatial().buildings().occupant_at(CellCoord::new(8, 8)), Some(OccupierId(2)));
        assert_eq!(state.spatial().technos().occupant_at(CellCoord::new(8, 8)), Some(OccupierId(2)));
        assert!(state.spatial().overlappers().contains(crate::OverlapperId(1)));
        assert!(!state.spatial().overlappers().contains(crate::OverlapperId(2)));
    }

    #[test]
    fn failed_overlap_registration_rolls_back_occupancy_indexes() {
        let mut state = MapEditorState::new(GameVariant::TiberianDawn, dimensions()).unwrap();
        assert!(matches!(
            state.add_building(
                OccupierId(9), CellCoord::new(1, 1), &[(0, 0)], &[(0, 0)],
                OverlapRect::new(0, 0, 0, 1),
            ),
            Err(SpatialIndexError::Overlapper(_))
        ));
        assert!(!state.spatial().buildings().contains(OccupierId(9)));
        assert!(!state.spatial().technos().contains(OccupierId(9)));
    }

    #[test]
    fn building_removal_requests_overlapper_rebuild() {
        let mut state = MapEditorState::new(GameVariant::TiberianDawn, dimensions()).unwrap();
        state.add_building(
            OccupierId(3), CellCoord::new(1, 1), &[(0, 0)], &[(0, 0)],
            OverlapRect::new(8, 8, 8, 8),
        ).unwrap();
        state.document_mut().begin_update();
        assert!(state.remove_building(OccupierId(3)));
        let work = state.document_mut().end_update().unwrap().unwrap();
        assert!(work.rebuild_overlappers);
    }
}
