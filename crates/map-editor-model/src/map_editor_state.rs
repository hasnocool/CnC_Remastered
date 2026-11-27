//! Owning aggregate that connects the editable map document to its spatial indexes.
//!
//! The wrapper keeps serialized cell data separate from runtime/editor spatial
//! indexes while providing one mutation boundary for placed buildings and walls.

use rts_core::{CellCoord, GameVariant, MapDimensions};
use crate::{
    MapDocument, MapDocumentError, MapSpatialIndex, OccupierId, OverlapRect,
    SpatialIndexError,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapEditorState {
    document: MapDocument,
    spatial: MapSpatialIndex,
}

impl MapEditorState {
    pub fn new(variant: GameVariant, dimensions: MapDimensions) -> Result<Self, MapDocumentError> {
        Ok(Self {
            document: MapDocument::new(variant, dimensions)?,
            spatial: MapSpatialIndex::new(dimensions),
        })
    }

    pub fn document(&self) -> &MapDocument { &self.document }
    pub fn document_mut(&mut self) -> &mut MapDocument { &mut self.document }
    pub fn spatial(&self) -> &MapSpatialIndex { &self.spatial }

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
        self.spatial.remove_wall(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dimensions() -> MapDimensions { MapDimensions::new(16, 16).unwrap() }

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
