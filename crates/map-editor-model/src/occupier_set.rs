//! Source-derived mask occupancy index from the pinned C# map editor.
//! Occupancy is separate from rectangle overlap; additions are preflighted atomically.
use std::collections::BTreeMap;
use rts_core::{CellCoord, MapDimensions};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OccupierId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OccupierError {
    DuplicateObject(OccupierId),
    CellOutOfBounds { x: u32, y: u32 },
    CellOccupied { cell: CellCoord, by: OccupierId },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OccupierSet {
    dimensions: MapDimensions,
    cells: Vec<Option<OccupierId>>,
    objects: BTreeMap<OccupierId, Vec<CellCoord>>,
}

impl OccupierSet {
    pub fn new(dimensions: MapDimensions) -> Self {
        Self { dimensions, cells: vec![None; dimensions.cell_count()], objects: BTreeMap::new() }
    }
    pub const fn dimensions(&self) -> MapDimensions { self.dimensions }
    pub fn len(&self) -> usize { self.objects.len() }
    pub fn is_empty(&self) -> bool { self.objects.is_empty() }
    pub fn contains(&self, id: OccupierId) -> bool { self.objects.contains_key(&id) }
    pub fn occupant_at(&self, cell: CellCoord) -> Option<OccupierId> {
        self.dimensions.index(cell).and_then(|i| self.cells.get(i).copied().flatten())
    }
    pub fn occupied_cells(&self, id: OccupierId) -> Option<&[CellCoord]> {
        self.objects.get(&id).map(Vec::as_slice)
    }

    /// Adds anchor-relative true mask points. Offsets preserve the source [x,y] orientation.
    /// The operation changes neither index if any point is invalid or occupied.
    pub fn add(&mut self, id: OccupierId, anchor: CellCoord, mask_points: &[(u16, u16)]) -> Result<(), OccupierError> {
        if self.objects.contains_key(&id) { return Err(OccupierError::DuplicateObject(id)); }
        let mut unique = Vec::with_capacity(mask_points.len());
        for &(dx, dy) in mask_points {
            let x = u32::from(anchor.x) + u32::from(dx);
            let y = u32::from(anchor.y) + u32::from(dy);
            if x >= u32::from(self.dimensions.width()) || y >= u32::from(self.dimensions.height()) {
                return Err(OccupierError::CellOutOfBounds { x, y });
            }
            let cell = CellCoord::new(x as u16, y as u16);
            if !unique.contains(&cell) { unique.push(cell); }
        }
        let cells = unique;
        for &cell in &cells {
            if let Some(by) = self.occupant_at(cell) {
                return Err(OccupierError::CellOccupied { cell, by });
            }
        }
        for &cell in &cells {
            let index = self.dimensions.index(cell).expect("preflight validated cell");
            self.cells[index] = Some(id);
        }
        self.objects.insert(id, cells);
        Ok(())
    }

    pub fn remove(&mut self, id: OccupierId) -> bool {
        let Some(cells) = self.objects.remove(&id) else { return false };
        for cell in cells {
            if let Some(index) = self.dimensions.index(cell) {
                if self.cells[index] == Some(id) { self.cells[index] = None; }
            }
        }
        true
    }
    pub fn clear(&mut self) -> Vec<OccupierId> {
        let removed = self.objects.keys().copied().collect();
        self.objects.clear();
        self.cells.fill(None);
        removed
    }
    pub fn objects(&self) -> impl ExactSizeIterator<Item = OccupierId> + '_ {
        self.objects.keys().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn dims() -> MapDimensions { MapDimensions::new(5, 4).unwrap() }

    #[test]
    fn anchor_relative_mask_is_indexed() {
        let mut set = OccupierSet::new(dims());
        set.add(OccupierId(7), CellCoord::new(1, 1), &[(0, 0), (1, 0), (0, 1)]).unwrap();
        assert_eq!(set.occupant_at(CellCoord::new(1, 1)), Some(OccupierId(7)));
        assert_eq!(set.occupant_at(CellCoord::new(2, 1)), Some(OccupierId(7)));
        assert_eq!(set.occupant_at(CellCoord::new(1, 2)), Some(OccupierId(7)));
        assert_eq!(set.occupied_cells(OccupierId(7)).unwrap().len(), 3);
    }
    #[test]
    fn failed_add_is_atomic_on_collision() {
        let mut set = OccupierSet::new(dims());
        set.add(OccupierId(1), CellCoord::new(2, 1), &[(0, 0)]).unwrap();
        assert_eq!(set.add(OccupierId(2), CellCoord::new(1, 1), &[(0, 0), (1, 0)]),
            Err(OccupierError::CellOccupied { cell: CellCoord::new(2, 1), by: OccupierId(1) }));
        assert!(!set.contains(OccupierId(2)));
        assert_eq!(set.occupant_at(CellCoord::new(1, 1)), None);
    }
    #[test]
    fn rejects_out_of_bounds_and_duplicate_ids() {
        let mut set = OccupierSet::new(dims());
        assert!(matches!(set.add(OccupierId(1), CellCoord::new(4, 3), &[(1, 0)]), Err(OccupierError::CellOutOfBounds(_))));
        set.add(OccupierId(1), CellCoord::new(0, 0), &[(0, 0)]).unwrap();
        assert_eq!(set.add(OccupierId(1), CellCoord::new(1, 0), &[(0, 0)]), Err(OccupierError::DuplicateObject(OccupierId(1))));
    }
    #[test]
    fn remove_and_clear_keep_indexes_consistent() {
        let mut set = OccupierSet::new(dims());
        set.add(OccupierId(2), CellCoord::new(0, 0), &[(0, 0), (1, 0)]).unwrap();
        set.add(OccupierId(1), CellCoord::new(3, 2), &[(0, 0)]).unwrap();
        assert!(set.remove(OccupierId(2)));
        assert_eq!(set.occupant_at(CellCoord::new(1, 0)), None);
        assert_eq!(set.clear(), vec![OccupierId(1)]);
        assert!(set.is_empty());
    }
    #[test]
    fn duplicate_mask_points_are_deduplicated() {
        let mut set = OccupierSet::new(dims());
        set.add(OccupierId(1), CellCoord::new(1, 1), &[(0, 0), (0, 0)]).unwrap();
        assert_eq!(set.occupied_cells(OccupierId(1)).unwrap().len(), 1);
    }
}
