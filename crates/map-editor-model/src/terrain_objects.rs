//! Map-owned terrain objects and their derived logical occupancy index.
//!
//! A terrain is one object anchored at an origin. Its source OccupancyMask
//! determines which cells it owns; render overlap is intentionally unrelated.

use crate::{MapDocument, TerrainPlacementDefinition};
use rts_core::CellCoord;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TerrainObjectId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerrainObject {
    pub id: TerrainObjectId,
    pub origin: CellCoord,
    pub terrain_type: u16,
    pub icon: u8,
    /// None represents the source editor's Trigger.None value.
    pub trigger: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerrainObjectError {
    OutOfBounds(CellCoord),
    Occupied(CellCoord),
    DuplicateObjectId(TerrainObjectId),
    UnknownObject(TerrainObjectId),
    OccupancyMismatch(CellCoord),
    IdExhausted,
}

impl MapDocument {
    /// All placed terrain objects, in stable ID order.
    pub fn terrain_objects(&self) -> impl Iterator<Item = &TerrainObject> {
        self.terrain_objects.values()
    }

    pub fn terrain_object(&self, id: TerrainObjectId) -> Option<&TerrainObject> {
        self.terrain_objects.get(&id)
    }

    /// Returns the object occupying a cell, if any.
    pub fn terrain_object_at(&self, coordinate: CellCoord) -> Option<&TerrainObject> {
        let index = self.dimensions.index(coordinate)?;
        let id = self.terrain_occupancy.get(index).copied().flatten()?;
        self.terrain_objects.get(&id)
    }

    pub fn terrain_occupancy_owner(&self, coordinate: CellCoord) -> Option<TerrainObjectId> {
        let index = self.dimensions.index(coordinate)?;
        self.terrain_occupancy.get(index).copied().flatten()
    }

    /// Adds one terrain object and atomically populates its source occupancy.
    /// The cell-layer terrain values are maintained as a compatibility mirror.
    pub fn add_terrain_object(
        &mut self,
        origin: CellCoord,
        definition: &TerrainPlacementDefinition,
        icon: u8,
        trigger: Option<String>,
    ) -> Result<TerrainObjectId, TerrainObjectError> {
        let id = TerrainObjectId(self.next_terrain_object_id);
        let next_id = self.next_terrain_object_id.checked_add(1)
            .ok_or(TerrainObjectError::IdExhausted)?;
        self.insert_terrain_object(id, origin, definition, icon, trigger, false)?;
        self.next_terrain_object_id = next_id;
        Ok(id)
    }

    /// Restores a serialized object while allowing its matching legacy cell
    /// mirror to already be present. Conflicting mirrors still fail.
    pub fn restore_terrain_object(
        &mut self,
        id: TerrainObjectId,
        origin: CellCoord,
        definition: &TerrainPlacementDefinition,
        icon: u8,
        trigger: Option<String>,
    ) -> Result<(), TerrainObjectError> {
        let next_id = id.0.checked_add(1).ok_or(TerrainObjectError::IdExhausted)?;
        self.insert_terrain_object(id, origin, definition, icon, trigger, true)?;
        self.next_terrain_object_id = self.next_terrain_object_id.max(next_id);
        Ok(())
    }

    fn insert_terrain_object(
        &mut self,
        id: TerrainObjectId,
        origin: CellCoord,
        definition: &TerrainPlacementDefinition,
        icon: u8,
        trigger: Option<String>,
        allow_matching_mirror: bool,
    ) -> Result<(), TerrainObjectError> {
        if self.terrain_objects.contains_key(&id) {
            return Err(TerrainObjectError::DuplicateObjectId(id));
        }
        let cells: Vec<_> = definition.occupancy.occupied_cells(origin).collect();
        let expected = definition.occupancy.occupied_cells(CellCoord::new(0, 0)).count();
        if cells.len() != expected {
            return Err(TerrainObjectError::OutOfBounds(origin));
        }
        let mut indices = Vec::with_capacity(cells.len());
        for coordinate in &cells {
            let index = self.dimensions.index(*coordinate)
                .ok_or(TerrainObjectError::OutOfBounds(*coordinate))?;
            if self.terrain_occupancy[index].is_some() {
                return Err(TerrainObjectError::Occupied(*coordinate));
            }
            let layers = self.cell(*coordinate).ok_or(TerrainObjectError::OutOfBounds(*coordinate))?;
            let matching_legacy = allow_matching_mirror && layers.terrain == Some(definition.id);
            if layers.terrain.is_some() && !matching_legacy {
                return Err(TerrainObjectError::Occupied(*coordinate));
            }
            indices.push(index);
        }

        // No mutation occurs until every footprint cell has been validated.
        let object = TerrainObject {
            id,
            origin,
            terrain_type: definition.id,
            icon,
            trigger,
        };
        for (coordinate, index) in cells.iter().zip(indices) {
            self.terrain_occupancy[index] = Some(id);
            let mut layers = *self.cell(*coordinate).expect("prevalidated terrain cell");
            layers.terrain = Some(definition.id);
            layers.terrain_icon = icon;
            self.set_terrain_object_cell(*coordinate, layers).expect("prevalidated terrain cell");
        }
        self.terrain_objects.insert(id, object);
        Ok(())
    }

    /// Removes the object and only the occupancy cells owned by its identity.
    pub fn remove_terrain_object(
        &mut self,
        id: TerrainObjectId,
        definition: &TerrainPlacementDefinition,
    ) -> Result<TerrainObject, TerrainObjectError> {
        let object = self.terrain_objects.get(&id)
            .cloned().ok_or(TerrainObjectError::UnknownObject(id))?;
        if object.terrain_type != definition.id {
            return Err(TerrainObjectError::UnknownObject(id));
        }
        let cells: Vec<_> = definition.occupancy.occupied_cells(object.origin).collect();
        let expected = definition.occupancy.occupied_cells(CellCoord::new(0, 0)).count();
        if cells.len() != expected {
            return Err(TerrainObjectError::OutOfBounds(object.origin));
        }
        let mut indices = Vec::with_capacity(cells.len());
        for coordinate in &cells {
            let index = self.dimensions.index(*coordinate)
                .ok_or(TerrainObjectError::OutOfBounds(*coordinate))?;
            if self.terrain_occupancy[index] != Some(id) {
                return Err(TerrainObjectError::OccupancyMismatch(*coordinate));
            }
            let layers = self.cell(*coordinate).ok_or(TerrainObjectError::OutOfBounds(*coordinate))?;
            if layers.terrain != Some(definition.id) || layers.terrain_icon != object.icon {
                return Err(TerrainObjectError::OccupancyMismatch(*coordinate));
            }
            indices.push(index);
        }

        for (coordinate, index) in cells.iter().zip(indices) {
            self.terrain_occupancy[index] = None;
            let mut layers = *self.cell(*coordinate).expect("prevalidated terrain cell");
            layers.terrain = None;
            layers.terrain_icon = 0;
            self.set_terrain_object_cell(*coordinate, layers).expect("prevalidated terrain cell");
        }
        self.terrain_objects.remove(&id);
        Ok(object)
    }

    /// Moves an object without changing its identity or trigger. The complete
    /// destination is validated before the old footprint is released.
    pub fn move_terrain_object(
        &mut self,
        id: TerrainObjectId,
        destination: CellCoord,
        definition: &TerrainPlacementDefinition,
    ) -> Result<(), TerrainObjectError> {
        let object = self.terrain_objects.get(&id)
            .cloned().ok_or(TerrainObjectError::UnknownObject(id))?;
        if object.terrain_type != definition.id {
            return Err(TerrainObjectError::UnknownObject(id));
        }
        let old_cells: Vec<_> = definition.occupancy.occupied_cells(object.origin).collect();
        let new_cells: Vec<_> = definition.occupancy.occupied_cells(destination).collect();
        let expected = definition.occupancy.occupied_cells(CellCoord::new(0, 0)).count();
        if old_cells.len() != expected || new_cells.len() != expected {
            return Err(TerrainObjectError::OutOfBounds(destination));
        }
        let mut old_indices = Vec::with_capacity(expected);
        for coordinate in &old_cells {
            let index = self.dimensions.index(*coordinate)
                .ok_or(TerrainObjectError::OutOfBounds(*coordinate))?;
            if self.terrain_occupancy[index] != Some(id) {
                return Err(TerrainObjectError::OccupancyMismatch(*coordinate));
            }
            old_indices.push(index);
        }
        let mut new_indices = Vec::with_capacity(expected);
        for coordinate in &new_cells {
            let index = self.dimensions.index(*coordinate)
                .ok_or(TerrainObjectError::OutOfBounds(*coordinate))?;
            if self.terrain_occupancy[index].is_some_and(|owner| owner != id) {
                return Err(TerrainObjectError::Occupied(*coordinate));
            }
            let layers = self.cell(*coordinate).ok_or(TerrainObjectError::OutOfBounds(*coordinate))?;
            if layers.terrain.is_some() && self.terrain_occupancy[index] != Some(id) {
                return Err(TerrainObjectError::Occupied(*coordinate));
            }
            new_indices.push(index);
        }

        let target_next_id = if target.is_some() {
            Some(id.0.checked_add(1).ok_or(TerrainObjectError::IdExhausted)?)
        } else {
            None
        };

        for (coordinate, index) in old_cells.iter().zip(old_indices) {
            if !new_indices.contains(&index) {
                self.terrain_occupancy[index] = None;
                let mut layers = *self.cell(*coordinate).expect("prevalidated old cell");
                layers.terrain = None;
                layers.terrain_icon = 0;
                self.set_terrain_object_cell(*coordinate, layers).expect("prevalidated old cell");
            }
        }
        for (coordinate, index) in new_cells.iter().zip(new_indices) {
            self.terrain_occupancy[index] = Some(id);
            let mut layers = *self.cell(*coordinate).expect("prevalidated destination cell");
            layers.terrain = Some(definition.id);
            layers.terrain_icon = object.icon;
            self.set_terrain_object_cell(*coordinate, layers).expect("prevalidated destination cell");
        }
        self.terrain_objects.get_mut(&id).expect("object retained").origin = destination;
        Ok(())
    }

    /// Applies an expected-to-target object transition after validating both
    /// footprints. Used by object-level undo/redo so cells, ownership index
    /// and object metadata change as one operation.
    pub(crate) fn transition_terrain_object(
        &mut self,
        expected: Option<&TerrainObject>,
        target: Option<&TerrainObject>,
        definition: &TerrainPlacementDefinition,
    ) -> Result<(), TerrainObjectError> {
        if expected.is_none() && target.is_none() {
            return Ok(());
        }
        let id = expected.map(|object| object.id).or_else(|| target.map(|object| object.id))
            .expect("one side of a terrain transition has an object");
        if expected.is_some_and(|object| object.id != id)
            || target.is_some_and(|object| object.id != id)
            || expected.is_some_and(|object| object.terrain_type != definition.id)
            || target.is_some_and(|object| object.terrain_type != definition.id)
        {
            return Err(TerrainObjectError::UnknownObject(id));
        }
        match (self.terrain_objects.get(&id), expected) {
            (None, None) => {}
            (Some(actual), Some(expected)) if actual == expected => {}
            _ => return Err(TerrainObjectError::UnknownObject(id)),
        }

        let old_cells: Vec<_> = expected
            .map(|object| definition.occupancy.occupied_cells(object.origin).collect())
            .unwrap_or_default();
        let new_cells: Vec<_> = target
            .map(|object| definition.occupancy.occupied_cells(object.origin).collect())
            .unwrap_or_default();
        let footprint_size = definition.occupancy.occupied_cells(CellCoord::new(0, 0)).count();
        if expected.is_some() && old_cells.len() != footprint_size {
            return Err(TerrainObjectError::OutOfBounds(expected.unwrap().origin));
        }
        if target.is_some() && new_cells.len() != footprint_size {
            return Err(TerrainObjectError::OutOfBounds(target.unwrap().origin));
        }

        let mut old_indices = Vec::with_capacity(old_cells.len());
        for coordinate in &old_cells {
            let index = self.dimensions.index(*coordinate)
                .ok_or(TerrainObjectError::OutOfBounds(*coordinate))?;
            if self.terrain_occupancy[index] != Some(id) {
                return Err(TerrainObjectError::OccupancyMismatch(*coordinate));
            }
            let layers = self.cell(*coordinate).ok_or(TerrainObjectError::OutOfBounds(*coordinate))?;
            if layers.terrain != Some(definition.id)
                || Some(layers.terrain_icon) != expected.map(|object| object.icon)
            {
                return Err(TerrainObjectError::OccupancyMismatch(*coordinate));
            }
            old_indices.push(index);
        }

        let mut new_indices = Vec::with_capacity(new_cells.len());
        for coordinate in &new_cells {
            let index = self.dimensions.index(*coordinate)
                .ok_or(TerrainObjectError::OutOfBounds(*coordinate))?;
            if self.terrain_occupancy[index].is_some_and(|owner| owner != id) {
                return Err(TerrainObjectError::Occupied(*coordinate));
            }
            let layers = self.cell(*coordinate).ok_or(TerrainObjectError::OutOfBounds(*coordinate))?;
            if layers.terrain.is_some() && self.terrain_occupancy[index] != Some(id) {
                return Err(TerrainObjectError::Occupied(*coordinate));
            }
            new_indices.push(index);
        }

        for (coordinate, index) in old_cells.iter().zip(old_indices) {
            if !new_indices.contains(&index) {
                self.terrain_occupancy[index] = None;
                let mut layers = *self.cell(*coordinate).expect("prevalidated old cell");
                layers.terrain = None;
                layers.terrain_icon = 0;
                self.set_terrain_object_cell(*coordinate, layers).expect("prevalidated old cell");
            }
        }
        for (coordinate, index) in new_cells.iter().zip(new_indices) {
            self.terrain_occupancy[index] = Some(id);
            let mut layers = *self.cell(*coordinate).expect("prevalidated target cell");
            layers.terrain = Some(definition.id);
            layers.terrain_icon = target.expect("target footprint has an object").icon;
            self.set_terrain_object_cell(*coordinate, layers).expect("prevalidated target cell");
        }
        if let Some(target) = target {
            self.terrain_objects.insert(id, target.clone());
            self.next_terrain_object_id = self.next_terrain_object_id.max(
                target_next_id.expect("target object has next ID")
            );
        } else {
            self.terrain_objects.remove(&id);
        }
        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CellLayers, OccupancyMask};
    use rts_core::{GameVariant, MapDimensions};

    fn map() -> MapDocument {
        MapDocument::new(GameVariant::TiberianDawn, MapDimensions::new(6, 5).unwrap()).unwrap()
    }
    fn definition() -> TerrainPlacementDefinition {
        TerrainPlacementDefinition {
            id: 18,
            occupancy: OccupancyMask::new(2, 2, vec![true, false, true, true]).unwrap(),
            transformable: true,
        }
    }

    #[test]
    fn stores_one_object_and_indexes_its_occupied_cells() {
        let mut map = map();
        let def = definition();
        let id = map.add_terrain_object(CellCoord::new(1, 1), &def, 22, Some("TRIGGER_A".into())).unwrap();
        assert_eq!(map.terrain_objects().count(), 1);
        let object = map.terrain_object(id).unwrap();
        assert_eq!(object.origin, CellCoord::new(1, 1));
        assert_eq!(object.terrain_type, 18);
        assert_eq!(object.icon, 22);
        assert_eq!(object.trigger.as_deref(), Some("TRIGGER_A"));
        assert_eq!(map.terrain_object_at(CellCoord::new(1, 1)).unwrap().id, id);
        assert!(map.terrain_object_at(CellCoord::new(2, 1)).is_none());
        assert_eq!(map.cell(CellCoord::new(1, 2)).unwrap().terrain, Some(18));
    }

    #[test]
    fn rejects_overlapping_object_atomically() {
        let mut map = map();
        let def = definition();
        map.add_terrain_object(CellCoord::new(1, 1), &def, 22, None).unwrap();
        assert!(matches!(
            map.add_terrain_object(CellCoord::new(2, 1), &def, 22, None),
            Err(TerrainObjectError::Occupied(_))
        ));
        assert_eq!(map.terrain_objects().count(), 1);
    }

    #[test]
    fn remove_clears_only_owned_footprint() {
        let mut map = map();
        let def = definition();
        let id = map.add_terrain_object(CellCoord::new(1, 1), &def, 22, None).unwrap();
        map.remove_terrain_object(id, &def).unwrap();
        assert_eq!(map.terrain_objects().count(), 0);
        assert!(map.terrain_object_at(CellCoord::new(1, 1)).is_none());
        assert!(map.cells().iter().all(|cell| cell.terrain.is_none()));
    }

    #[test]
    fn move_preserves_identity_and_trigger_and_is_atomic_on_conflict() {
        let mut map = map();
        let def = definition();
        let id = map.add_terrain_object(CellCoord::new(1, 1), &def, 22, Some("A".into())).unwrap();
        map.add_terrain_object(CellCoord::new(4, 1), &def, 22, None).unwrap();
        assert!(matches!(
            map.move_terrain_object(id, CellCoord::new(3, 1), &def),
            Err(TerrainObjectError::Occupied(_))
        ));
        assert_eq!(map.terrain_object(id).unwrap().origin, CellCoord::new(1, 1));
        map.move_terrain_object(id, CellCoord::new(1, 3), &def).unwrap();
        assert_eq!(map.terrain_object(id).unwrap().origin, CellCoord::new(1, 3));
        assert_eq!(map.terrain_object(id).unwrap().trigger.as_deref(), Some("A"));
    }

    #[test]
    fn restores_object_over_its_legacy_cell_mirror() {
        let mut map = map();
        let def = definition();
        for cell in def.occupancy.occupied_cells(CellCoord::new(1, 1)) {
            map.set_cell(cell, CellLayers { terrain: Some(18), terrain_icon: 22, ..Default::default() }).unwrap();
        }
        map.restore_terrain_object(TerrainObjectId(8), CellCoord::new(1, 1), &def, 22, None).unwrap();
        assert_eq!(map.terrain_object(TerrainObjectId(8)).unwrap().origin, CellCoord::new(1, 1));
    }
}
