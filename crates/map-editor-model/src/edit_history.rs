//! Transactional cell-edit history for the map editor.
//!
//! Commands retain complete before/after cell layers so undo and redo restore
//! template IDs, logical template icon slots, terrain, overlays, and smudges
//! atomically. The history is intentionally independent of UI and renderer
//! crates.

use crate::{CellLayers, MapDocument, MapDocumentError};
use rts_core::CellCoord;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CellEdit {
    coordinate: CellCoord,
    before: CellLayers,
    after: CellLayers,
}

/// Bounded, linear undo/redo history for map-cell edits.
///
/// Applying a new edit after undo discards the redo branch. No-op edits are
/// ignored and therefore do not create empty undo steps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapEditHistory {
    undo: Vec<CellEdit>,
    redo: Vec<CellEdit>,
    capacity: usize,
}

impl Default for MapEditHistory {
    fn default() -> Self {
        Self::new(256)
    }
}

impl MapEditHistory {
    /// Creates a history retaining at most `capacity` undo operations.
    pub fn new(capacity: usize) -> Self {
        Self {
            undo: Vec::with_capacity(capacity),
            redo: Vec::new(),
            capacity,
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    pub fn undo_depth(&self) -> usize {
        self.undo.len()
    }

    pub fn redo_depth(&self) -> usize {
        self.redo.len()
    }

    /// Applies a cell replacement and records its inverse operation.
    pub fn apply_cell(
        &mut self,
        document: &mut MapDocument,
        coordinate: CellCoord,
        after: CellLayers,
    ) -> Result<bool, MapDocumentError> {
        let before = document
            .cell(coordinate)
            .copied()
            .ok_or(MapDocumentError::CellOutOfBounds(coordinate))?;

        if before == after {
            return Ok(false);
        }

        document.set_cell(coordinate, after)?;
        self.redo.clear();

        if self.capacity == 0 {
            self.undo.clear();
            return Ok(true);
        }

        self.undo.push(CellEdit {
            coordinate,
            before,
            after,
        });

        if self.undo.len() > self.capacity {
            self.undo.remove(0);
        }

        Ok(true)
    }

    /// Reverts the most recent edit. Returns false when history is empty.
    pub fn undo(
        &mut self,
        document: &mut MapDocument,
    ) -> Result<bool, MapDocumentError> {
        let Some(edit) = self.undo.pop() else {
            return Ok(false);
        };

        // Reinsert the command if the target document rejects the operation.
        if let Err(error) = document.set_cell(edit.coordinate, edit.before) {
            self.undo.push(edit);
            return Err(error);
        }

        self.redo.push(edit);
        Ok(true)
    }

    /// Reapplies the most recently reverted edit.
    pub fn redo(
        &mut self,
        document: &mut MapDocument,
    ) -> Result<bool, MapDocumentError> {
        let Some(edit) = self.redo.pop() else {
            return Ok(false);
        };

        if let Err(error) = document.set_cell(edit.coordinate, edit.after) {
            self.redo.push(edit);
            return Err(error);
        }

        self.undo.push(edit);
        Ok(true)
    }

    /// Clears both stacks, for example after loading or replacing a document.
    pub fn clear(&mut self) {
        self.undo.clear();
        self.redo.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rts_core::{GameVariant, MapDimensions};

    fn document() -> MapDocument {
        MapDocument::new(
            GameVariant::TiberianDawn,
            MapDimensions::new(4, 4).expect("valid dimensions"),
        )
        .expect("valid map")
    }

    #[test]
    fn undo_and_redo_restore_all_cell_layers() {
        let mut map = document();
        let mut history = MapEditHistory::default();
        let coordinate = CellCoord::new(1, 2);
        let layers = CellLayers {
            template: Some(17),
            template_icon: 9,
            terrain: Some(4),
            overlay: Some(12),
            smudge: Some(3),
        };

        assert!(history.apply_cell(&mut map, coordinate, layers).unwrap());
        assert_eq!(map.cell(coordinate), Some(&layers));
        assert!(history.undo(&mut map).unwrap());
        assert_eq!(map.cell(coordinate), Some(&CellLayers::default()));
        assert!(history.redo(&mut map).unwrap());
        assert_eq!(map.cell(coordinate), Some(&layers));
    }

    #[test]
    fn no_op_does_not_create_history() {
        let mut map = document();
        let mut history = MapEditHistory::default();

        assert!(!history
            .apply_cell(&mut map, CellCoord::new(0, 0), CellLayers::default())
            .unwrap());
        assert_eq!(history.undo_depth(), 0);
        assert!(!history.can_undo());
    }

    #[test]
    fn new_edit_discards_redo_branch() {
        let mut map = document();
        let mut history = MapEditHistory::default();
        let first = CellLayers { template: Some(1), ..CellLayers::default() };
        let second = CellLayers { template: Some(2), ..CellLayers::default() };

        history.apply_cell(&mut map, CellCoord::new(0, 0), first).unwrap();
        history.undo(&mut map).unwrap();
        history.apply_cell(&mut map, CellCoord::new(1, 0), second).unwrap();

        assert!(!history.can_redo());
        assert_eq!(history.undo_depth(), 1);
    }

    #[test]
    fn rejects_out_of_bounds_edit_without_mutating_history() {
        let mut map = document();
        let mut history = MapEditHistory::default();
        let coordinate = CellCoord::new(4, 0);

        assert_eq!(
            history.apply_cell(&mut map, coordinate, CellLayers::default()),
            Err(MapDocumentError::CellOutOfBounds(coordinate))
        );
        assert_eq!(history.undo_depth(), 0);
    }

    #[test]
    fn zero_capacity_applies_without_retaining_history() {
        let mut map = document();
        let mut history = MapEditHistory::new(0);
        let layers = CellLayers { overlay: Some(8), ..CellLayers::default() };

        assert!(history.apply_cell(&mut map, CellCoord::new(0, 0), layers).unwrap());
        assert_eq!(history.undo_depth(), 0);
        assert!(!history.can_undo());
    }

    #[test]
    fn capacity_discards_oldest_undo_entry() {
        let mut map = document();
        let mut history = MapEditHistory::new(1);
        let first = CellCoord::new(0, 0);
        let second = CellCoord::new(1, 0);

        history.apply_cell(&mut map, first, CellLayers { template: Some(1), ..CellLayers::default() }).unwrap();
        history.apply_cell(&mut map, second, CellLayers { template: Some(2), ..CellLayers::default() }).unwrap();

        assert_eq!(history.undo_depth(), 1);
        history.undo(&mut map).unwrap();
        assert_eq!(map.cell(second), Some(&CellLayers::default()));
        assert_eq!(map.cell(first), Some(&CellLayers { template: Some(1), ..CellLayers::default() }));
    }
}
