//! Generic typed cell grid for the scenario editor.
//!
//! Source contract: the C# CellGrid<T> stores one nullable value per cell,
//! indexes row-major, emits a cell-change event for changed values, emits one
//! clear event for Clear(), and CopyTo validates total length rather than
//! requiring identical width/height.

use rts_core::{CellCoord, MapDimensions};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CellGridEvent<T> {
    CellChanged { index: usize, coordinate: CellCoord, previous: Option<T>, current: Option<T> },
    Cleared,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellGridError {
    CellOutOfBounds(CellCoord),
    IndexOutOfBounds(usize),
    CopyLengthMismatch { source: usize, destination: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellGrid<T> {
    dimensions: MapDimensions,
    cells: Vec<Option<T>>,
}

impl<T> CellGrid<T> {
    pub fn new(dimensions: MapDimensions) -> Self {
        Self { dimensions, cells: std::iter::repeat_with(|| None).take(dimensions.cell_count()).collect() }
    }

    pub const fn dimensions(&self) -> MapDimensions { self.dimensions }
    pub fn len(&self) -> usize { self.cells.len() }
    pub fn is_empty(&self) -> bool { self.cells.is_empty() }
    pub fn values(&self) -> &[Option<T>] { &self.cells }

    pub fn get(&self, coordinate: CellCoord) -> Option<&T> {
        self.dimensions.index(coordinate).and_then(|index| self.cells.get(index)).and_then(Option::as_ref)
    }

    pub fn get_index(&self, index: usize) -> Option<&T> {
        self.cells.get(index).and_then(Option::as_ref)
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = &Option<T>> { self.cells.iter() }

    pub fn set(&mut self, coordinate: CellCoord, value: Option<T>) -> Result<Option<CellGridEvent<T>>, CellGridError>
    where T: Clone + PartialEq {
        let index = self.dimensions.index(coordinate).ok_or(CellGridError::CellOutOfBounds(coordinate))?;
        if self.cells[index] == value { return Ok(None); }
        let previous = std::mem::replace(&mut self.cells[index], value.clone());
        Ok(Some(CellGridEvent::CellChanged { index, coordinate, previous, current: value }))
    }

    pub fn set_index(&mut self, index: usize, value: Option<T>) -> Result<Option<CellGridEvent<T>>, CellGridError>
    where T: Clone + PartialEq {
        let coordinate = self.dimensions.coord(index).ok_or(CellGridError::IndexOutOfBounds(index))?;
        self.set(coordinate, value)
    }

    /// Clears all values and returns one aggregate Cleared event.
    pub fn clear(&mut self) -> Option<CellGridEvent<T>> {
        if self.cells.iter().all(Option::is_none) { return None; }
        self.cells.iter_mut().for_each(|cell| *cell = None);
        Some(CellGridEvent::Cleared)
    }

    /// Copies by flattened row-major order. As in the source, only total
    /// lengths must match; dimensions may differ when their areas are equal.
    pub fn copy_to(&self, destination: &mut Self) -> Result<(), CellGridError>
    where T: Clone {
        if self.len() != destination.len() {
            return Err(CellGridError::CopyLengthMismatch { source: self.len(), destination: destination.len() });
        }
        destination.cells.clone_from_slice(&self.cells);
        Ok(())
    }

    pub fn into_values(self) -> Vec<Option<T>> { self.cells }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dims(w: u16, h: u16) -> MapDimensions { MapDimensions::new(w, h).unwrap() }

    #[test]
    fn set_get_and_events_use_row_major_indices() {
        let mut grid = CellGrid::new(dims(4, 3));
        let event = grid.set(CellCoord::new(2, 1), Some(17_u16)).unwrap().unwrap();
        assert_eq!(event, CellGridEvent::CellChanged {
            index: 6, coordinate: CellCoord::new(2, 1), previous: None, current: Some(17),
        });
        assert_eq!(grid.get(CellCoord::new(2, 1)), Some(&17));
        assert_eq!(grid.get_index(6), Some(&17));
    }

    #[test]
    fn assigning_same_value_emits_no_change() {
        let mut grid = CellGrid::new(dims(2, 2));
        grid.set(CellCoord::new(0, 0), Some(4_u8)).unwrap();
        assert_eq!(grid.set(CellCoord::new(0, 0), Some(4_u8)).unwrap(), None);
    }

    #[test]
    fn clear_emits_one_event_and_is_idempotent() {
        let mut grid = CellGrid::new(dims(2, 2));
        grid.set(CellCoord::new(0, 0), Some(1_u8)).unwrap();
        grid.set(CellCoord::new(1, 1), Some(2_u8)).unwrap();
        assert_eq!(grid.clear(), Some(CellGridEvent::Cleared));
        assert_eq!(grid.clear(), None);
        assert!(grid.values().iter().all(Option::is_none));
    }

    #[test]
    fn copy_uses_flattened_length_not_shape() {
        let mut source = CellGrid::new(dims(2, 3));
        let mut destination = CellGrid::new(dims(3, 2));
        source.set_index(4, Some(9_u8)).unwrap();
        source.copy_to(&mut destination).unwrap();
        assert_eq!(destination.get_index(4), Some(&9));
    }

    #[test]
    fn rejects_invalid_coordinates_indexes_and_copy_lengths() {
        let mut grid = CellGrid::<u8>::new(dims(2, 2));
        assert_eq!(grid.set(CellCoord::new(2, 0), Some(1)), Err(CellGridError::CellOutOfBounds(CellCoord::new(2, 0))));
        assert_eq!(grid.set_index(4, Some(1)), Err(CellGridError::IndexOutOfBounds(4)));
        let source = CellGrid::<u8>::new(dims(2, 2));
        let mut other = CellGrid::<u8>::new(dims(3, 2));
        assert_eq!(source.copy_to(&mut other), Err(CellGridError::CopyLengthMismatch { source: 4, destination: 6 }));
    }
}
