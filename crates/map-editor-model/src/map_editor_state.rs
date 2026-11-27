//! Owning aggregate that connects the editable map document to its spatial indexes.
//!
//! The wrapper keeps serialized cell data separate from runtime/editor spatial
//! indexes while providing one mutation boundary for placed buildings and walls.

use std::collections::{BTreeMap, HashMap};
use rts_core::{CellCoord, GameVariant, MapDimensions};
use crate::{
    MapDocument, MapDocumentError, MapSpatialIndex, OccupierId, OverlapRect,
    SpatialIndexError, OverlayInvalidation, MapUpdateError, MapUpdateWork,
    OverlayTypeDefinition, PlacedOverlay,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapEditorState {
    document: MapDocument,
    spatial: MapSpatialIndex,
    overlay_types: BTreeMap<i8, OverlayTypeDefinition>,
    overlays: HashMap<CellCoord, (PlacedOverlay, Option<OccupierId>)>,
    next_internal_occupier: u64,
}

impl MapEditorState {
    pub fn new(variant: GameVariant, dimensions: MapDimensions) -> Result<Self, MapDocumentError> {
        Ok(Self {
            document: MapDocument::new(variant, dimensions)?,
            spatial: MapSpatialIndex::new(dimensions),
            overlay_types: BTreeMap::new(),
            overlays: HashMap::new(),
            next_internal_occupier: u64::MAX,
        })
    }

    pub fn document(&self) -> &MapDocument { &self.document }
    pub fn document_mut(&mut self) -> &mut MapDocument { &mut self.document }
    pub fn spatial(&self) -> &MapSpatialIndex { &self.spatial }

    pub fn register_overlay_type(&mut self, definition: OverlayTypeDefinition) {
        self.overlay_types.insert(definition.id, definition);
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
        Ok(work)
    }

    fn allocate_overlay_occupier(next_id: &mut u64, spatial: &MapSpatialIndex) -> Result<OccupierId, MapEditorStateError> {
        loop {
            let candidate = OccupierId(*next_id);
            *next_id = (*next_id).checked_sub(1)
                .ok_or(MapEditorStateError::OccupierIdsExhausted)?;
            if !spatial.contains_occupier(candidate) { return Ok(candidate); }
        }
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
        let removed = self.spatial.remove_building(id);
        if removed { let _ = self.document.request_overlapper_rebuild(); }
        removed
    }

    pub fn remove_wall(&mut self, id: OccupierId) -> bool {
        let removed = self.spatial.remove_wall(id);
        if removed {
            if let Some(cell) = self.overlays.iter().find_map(|(cell, (_, overlay_id))| (*overlay_id == Some(id)).then_some(*cell)) {
                self.overlays.remove(&cell);
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
