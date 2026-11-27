//! Map-owned spatial indexes and the Buildings -> Technos event bridge.
//!
//! Buildings and Technos intentionally remain distinct occupancy indexes:
//! Buildings stores full building masks (including bib footprint) and wall
//! overlays; Technos stores building base masks and wall occupancy.

use std::collections::BTreeSet;
use rts_core::{CellCoord, MapDimensions};
use crate::{
    OccupierError, OccupierId, OccupierSet, OverlapRect, OverlapperError,
    OverlapperId, OverlapperSet,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpatialIndexError {
    DuplicateObject(OccupierId),
    BuildingOccupancy(OccupierError),
    TechnoOccupancy(OccupierError),
    Overlapper(OverlapperError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapSpatialIndex {
    buildings: OccupierSet,
    technos: OccupierSet,
    overlappers: OverlapperSet,
    walls: BTreeSet<OccupierId>,
}

impl MapSpatialIndex {
    pub fn new(dimensions: MapDimensions) -> Self {
        Self {
            buildings: OccupierSet::new(dimensions),
            technos: OccupierSet::new(dimensions),
            overlappers: OverlapperSet::new(),
            walls: BTreeSet::new(),
        }
    }

    pub fn buildings(&self) -> &OccupierSet { &self.buildings }
    pub fn technos(&self) -> &OccupierSet { &self.technos }
    pub fn overlappers(&self) -> &OverlapperSet { &self.overlappers }
    pub fn is_wall(&self, id: OccupierId) -> bool { self.walls.contains(&id) }
    pub fn contains_occupier(&self, id: OccupierId) -> bool { self.buildings.contains(id) || self.technos.contains(id) }

    /// Adds a building to Buildings with its full mask, then to Technos with
    /// its base mask. Overlap bounds are indexed independently. All three
    /// registrations roll back if any later registration fails.
    pub fn add_building(
        &mut self,
        id: OccupierId,
        anchor: CellCoord,
        full_mask: &[(u16, u16)],
        base_mask: &[(u16, u16)],
        overlap_bounds: OverlapRect,
    ) -> Result<(), SpatialIndexError> {
        if self.buildings.contains(id) || self.technos.contains(id) || self.overlappers.contains(OverlapperId(id.0)) {
            return Err(SpatialIndexError::DuplicateObject(id));
        }
        self.buildings.add(id, anchor, full_mask).map_err(SpatialIndexError::BuildingOccupancy)?;
        if let Err(error) = self.technos.add(id, anchor, base_mask) {
            self.buildings.remove(id);
            return Err(SpatialIndexError::TechnoOccupancy(error));
        }
        if let Err(error) = self.overlappers.add(OverlapperId(id.0), overlap_bounds) {
            self.technos.remove(id);
            self.buildings.remove(id);
            return Err(SpatialIndexError::Overlapper(error));
        }
        Ok(())
    }

    /// Wall overlays first enter Buildings. The Buildings event bridge then
    /// registers the same wall occupancy in Technos.
    pub fn add_wall(
        &mut self,
        id: OccupierId,
        anchor: CellCoord,
        mask: &[(u16, u16)],
    ) -> Result<(), SpatialIndexError> {
        if self.buildings.contains(id) || self.technos.contains(id) {
            return Err(SpatialIndexError::DuplicateObject(id));
        }
        self.buildings.add(id, anchor, mask).map_err(SpatialIndexError::BuildingOccupancy)?;
        if let Err(error) = self.technos.add(id, anchor, mask) {
            self.buildings.remove(id);
            return Err(SpatialIndexError::TechnoOccupancy(error));
        }
        self.walls.insert(id);
        Ok(())
    }

    pub fn remove_building(&mut self, id: OccupierId) -> bool {
        if self.walls.contains(&id) { return false; }
        let removed = self.buildings.remove(id);
        self.technos.remove(id);
        self.overlappers.remove(OverlapperId(id.0));
        removed
    }

    pub fn remove_wall(&mut self, id: OccupierId) -> bool {
        if !self.walls.remove(&id) { return false; }
        self.buildings.remove(id);
        self.technos.remove(id);
        true
    }

    pub fn clear(&mut self) {
        self.buildings.clear();
        self.technos.clear();
        self.overlappers.clear();
        self.walls.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dimensions() -> MapDimensions { MapDimensions::new(8, 8).unwrap() }

    #[test]
    fn building_uses_full_buildings_mask_and_base_technos_mask() {
        let mut index = MapSpatialIndex::new(dimensions());
        index.add_building(
            OccupierId(1), CellCoord::new(2, 2),
            &[(0, 0), (1, 0), (0, 1), (1, 1), (0, 2)],
            &[(0, 0), (1, 0), (0, 1), (1, 1)],
            OverlapRect::new(32, 32, 48, 48),
        ).unwrap();
        assert_eq!(index.buildings().occupant_at(CellCoord::new(2, 4)), Some(OccupierId(1)));
        assert_eq!(index.technos().occupant_at(CellCoord::new(2, 4)), None);
        assert!(index.overlappers().contains(OverlapperId(1)));
    }

    #[test]
    fn wall_registration_bridges_buildings_into_technos() {
        let mut index = MapSpatialIndex::new(dimensions());
        index.add_wall(OccupierId(7), CellCoord::new(4, 3), &[(0, 0), (0, 1)]).unwrap();
        assert!(index.is_wall(OccupierId(7)));
        assert_eq!(index.buildings().occupant_at(CellCoord::new(4, 4)), Some(OccupierId(7)));
        assert_eq!(index.technos().occupant_at(CellCoord::new(4, 4)), Some(OccupierId(7)));
        assert!(!index.overlappers().contains(OverlapperId(7)));
    }

    #[test]
    fn duplicate_wall_registration_is_rejected_without_changing_indexes() {
        let mut index = MapSpatialIndex::new(dimensions());
        index.add_wall(OccupierId(1), CellCoord::new(1, 1), &[(0, 0)]).unwrap();
        assert!(matches!(
            index.add_wall(OccupierId(1), CellCoord::new(2, 2), &[(0, 0)]),
            Err(SpatialIndexError::DuplicateObject(OccupierId(1)))
        ));
        assert_eq!(index.buildings().occupant_at(CellCoord::new(1, 1)), Some(OccupierId(1)));
        assert_eq!(index.buildings().occupant_at(CellCoord::new(2, 2)), None);
    }

    #[test]
    fn removing_building_removes_all_three_registrations() {
        let mut index = MapSpatialIndex::new(dimensions());
        index.add_building(
            OccupierId(3), CellCoord::new(1, 1), &[(0, 0), (1, 0)], &[(0, 0)],
            OverlapRect::new(0, 0, 2, 2),
        ).unwrap();
        assert!(index.remove_building(OccupierId(3)));
        assert!(!index.buildings().contains(OccupierId(3)));
        assert!(!index.technos().contains(OccupierId(3)));
        assert!(!index.overlappers().contains(OverlapperId(3)));
    }

    #[test]
    fn removing_wall_does_not_remove_building_with_same_numeric_id() {
        let mut index = MapSpatialIndex::new(dimensions());
        index.add_wall(OccupierId(4), CellCoord::new(0, 0), &[(0, 0)]).unwrap();
        assert!(index.remove_wall(OccupierId(4)));
        assert!(!index.buildings().contains(OccupierId(4)));
        assert!(!index.technos().contains(OccupierId(4)));
    }
}
