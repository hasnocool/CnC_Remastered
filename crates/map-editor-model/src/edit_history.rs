//! Transactional cell-edit history for the map editor.
use crate::{CellLayers, MapDocument, MapDocumentError};
use rts_core::CellCoord;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CellEdit { coordinate: CellCoord, before: CellLayers, after: CellLayers }
#[derive(Debug, Clone, PartialEq, Eq)]
struct CellEditBatch { edits: Vec<CellEdit> }

/// Bounded, linear undo/redo history. A batch is one indivisible command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapEditHistory { undo: Vec<CellEditBatch>, redo: Vec<CellEditBatch>, capacity: usize }

impl Default for MapEditHistory { fn default() -> Self { Self::new(256) } }

impl MapEditHistory {
    pub fn new(capacity: usize) -> Self {
        Self { undo: Vec::with_capacity(capacity), redo: Vec::new(), capacity }
    }
    pub fn can_undo(&self) -> bool { !self.undo.is_empty() }
    pub fn can_redo(&self) -> bool { !self.redo.is_empty() }
    pub fn undo_depth(&self) -> usize { self.undo.len() }
    pub fn redo_depth(&self) -> usize { self.redo.len() }

    /// Whether the next undo changes a template reference or logical icon.
    pub fn undo_changes_templates(&self) -> bool {
        self.undo.last().is_some_and(|command| command.edits.iter().any(|edit| {
            edit.before.template != edit.after.template
                || edit.before.template_icon != edit.after.template_icon
        }))
    }

    /// Whether the next redo changes a template reference or logical icon.
    pub fn redo_changes_templates(&self) -> bool {
        self.redo.last().is_some_and(|command| command.edits.iter().any(|edit| {
            edit.before.template != edit.after.template
                || edit.before.template_icon != edit.after.template_icon
        }))
    }

    pub fn apply_cell(&mut self, document: &mut MapDocument, coordinate: CellCoord, after: CellLayers) -> Result<bool, MapDocumentError> {
        Ok(self.apply_cells(document, &[(coordinate, after)])? != 0)
    }

    /// Atomically applies unique cell replacements and records them as one command.
    /// Returns the number of cells changed; no-op entries are ignored.
    pub fn apply_cells(&mut self, document: &mut MapDocument, replacements: &[(CellCoord, CellLayers)]) -> Result<usize, MapDocumentError> {
        let mut edits: Vec<CellEdit> = Vec::with_capacity(replacements.len());
        let mut seen = Vec::with_capacity(replacements.len());
        for (coordinate, after) in replacements {
            if seen.contains(coordinate) {
                return Err(MapDocumentError::DuplicateCellEdit(*coordinate));
            }
            seen.push(*coordinate);
            let before = document.cell(*coordinate).copied()
                .ok_or(MapDocumentError::CellOutOfBounds(*coordinate))?;
            if before != *after { edits.push(CellEdit { coordinate: *coordinate, before, after: *after }); }
        }
        let changed = edits.len();
        if changed == 0 { return Ok(0); }

        // All coordinates and before states have been checked before mutation.
        for edit in &edits {
            document.set_cell(edit.coordinate, edit.after).expect("prevalidated transaction coordinate");
        }
        self.redo.clear();
        if self.capacity == 0 {
            self.undo.clear();
        } else {
            self.undo.push(CellEditBatch { edits });
            if self.undo.len() > self.capacity { self.undo.remove(0); }
        }
        Ok(changed)
    }

    pub fn undo(&mut self, document: &mut MapDocument) -> Result<bool, MapDocumentError> {
        let Some(command) = self.undo.pop() else { return Ok(false); };
        let invalid = command.edits.iter().find_map(|edit| {
            match document.cell(edit.coordinate) {
                None => Some((edit.coordinate, true)),
                Some(actual) if *actual != edit.after => Some((edit.coordinate, false)),
                Some(_) => None,
            }
        });
        if let Some((coordinate, out_of_bounds)) = invalid {
            self.undo.push(command);
            return Err(if out_of_bounds {
                MapDocumentError::CellOutOfBounds(coordinate)
            } else {
                MapDocumentError::CellStateConflict(coordinate)
            });
        }
        for edit in &command.edits {
            document.set_cell(edit.coordinate, edit.before).expect("prevalidated undo coordinate");
        }
        self.redo.push(command);
        Ok(true)
    }

    pub fn redo(&mut self, document: &mut MapDocument) -> Result<bool, MapDocumentError> {
        let Some(command) = self.redo.pop() else { return Ok(false); };
        let invalid = command.edits.iter().find_map(|edit| {
            match document.cell(edit.coordinate) {
                None => Some((edit.coordinate, true)),
                Some(actual) if *actual != edit.before => Some((edit.coordinate, false)),
                Some(_) => None,
            }
        });
        if let Some((coordinate, out_of_bounds)) = invalid {
            self.redo.push(command);
            return Err(if out_of_bounds {
                MapDocumentError::CellOutOfBounds(coordinate)
            } else {
                MapDocumentError::CellStateConflict(coordinate)
            });
        }
        for edit in &command.edits {
            document.set_cell(edit.coordinate, edit.after).expect("prevalidated redo coordinate");
        }
        self.undo.push(command);
        Ok(true)
    }

    pub fn clear(&mut self) { self.undo.clear(); self.redo.clear(); }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rts_core::{GameVariant, MapDimensions};
    fn document() -> MapDocument {
        MapDocument::new(GameVariant::TiberianDawn, MapDimensions::new(4, 4).unwrap()).unwrap()
    }
    #[test]
    fn duplicate_after_noop_is_still_rejected() {
        let mut map = document();
        let mut history = MapEditHistory::default();
        let at = CellCoord::new(0, 0);
        let changed = CellLayers { terrain: Some(5), ..CellLayers::default() };
        assert_eq!(
            history.apply_cells(&mut map, &[(at, CellLayers::default()), (at, changed)]),
            Err(MapDocumentError::DuplicateCellEdit(at))
        );
        assert_eq!(map.cell(at), Some(&CellLayers::default()));
        assert_eq!(history.undo_depth(), 0);
    }

    #[test]
    fn template_cache_hints_follow_undo_and_redo_command() {
        let mut map = document();
        let mut history = MapEditHistory::default();
        let at = CellCoord::new(0, 0);
        let template = CellLayers { template: Some(4), template_icon: 2, ..CellLayers::default() };
        assert!(!history.undo_changes_templates());
        assert!(!history.redo_changes_templates());
        history.apply_cell(&mut map, at, template).unwrap();
        assert!(history.undo_changes_templates());
        assert!(!history.redo_changes_templates());
        history.undo(&mut map).unwrap();
        assert!(!history.undo_changes_templates());
        assert!(history.redo_changes_templates());
        history.redo(&mut map).unwrap();
        assert!(history.undo_changes_templates());
    }

    #[test]
    fn undo_redo_restore_all_cell_layers() {
        let mut map=document(); let mut h=MapEditHistory::default(); let at=CellCoord::new(1,2);
        let value=CellLayers{template:Some(17),template_icon:9,terrain:Some(4),overlay:Some(12),smudge:Some(3)};
        assert!(h.apply_cell(&mut map,at,value).unwrap()); assert_eq!(map.cell(at),Some(&value));
        assert!(h.undo(&mut map).unwrap()); assert_eq!(map.cell(at),Some(&CellLayers::default()));
        assert!(h.redo(&mut map).unwrap()); assert_eq!(map.cell(at),Some(&value));
    }
    #[test]
    fn batch_is_one_atomic_undo_redo_command() {
        let mut map=document(); let mut h=MapEditHistory::default(); let a=CellCoord::new(0,0); let b=CellCoord::new(1,0);
        let v=CellLayers{terrain:Some(7),..CellLayers::default()};
        assert_eq!(h.apply_cells(&mut map,&[(a,v),(b,v)]).unwrap(),2); assert_eq!(h.undo_depth(),1);
        h.undo(&mut map).unwrap(); assert_eq!(map.cell(a),Some(&CellLayers::default())); assert_eq!(map.cell(b),Some(&CellLayers::default()));
        h.redo(&mut map).unwrap(); assert_eq!(map.cell(a),Some(&v)); assert_eq!(map.cell(b),Some(&v));
    }
    #[test]
    fn undo_conflict_rejects_entire_batch() {
        let mut map=document(); let mut h=MapEditHistory::default(); let a=CellCoord::new(0,0); let b=CellCoord::new(1,0);
        let v=CellLayers{terrain:Some(7),..CellLayers::default()}; h.apply_cells(&mut map,&[(a,v),(b,v)]).unwrap();
        map.set_cell(b,CellLayers{overlay:Some(9),..v}).unwrap();
        assert_eq!(h.undo(&mut map),Err(MapDocumentError::CellStateConflict(b)));
        assert_eq!(map.cell(a),Some(&v)); assert_eq!(map.cell(b).unwrap().overlay,Some(9)); assert!(h.can_undo());
    }
    #[test]
    fn redo_conflict_rejects_entire_batch() {
        let mut map=document(); let mut h=MapEditHistory::default(); let a=CellCoord::new(0,0); let b=CellCoord::new(1,0);
        let v=CellLayers{terrain:Some(7),..CellLayers::default()}; h.apply_cells(&mut map,&[(a,v),(b,v)]).unwrap(); h.undo(&mut map).unwrap();
        map.set_cell(b,CellLayers{overlay:Some(9),..CellLayers::default()}).unwrap();
        assert_eq!(h.redo(&mut map),Err(MapDocumentError::CellStateConflict(b)));
        assert_eq!(map.cell(a),Some(&CellLayers::default())); assert_eq!(map.cell(b).unwrap().overlay,Some(9)); assert!(h.can_redo());
    }
    #[test]
    fn invalid_batch_does_not_mutate_document_or_history() {
        let mut map=document(); let mut h=MapEditHistory::default(); let a=CellCoord::new(0,0); let v=CellLayers{terrain:Some(1),..CellLayers::default()};
        assert_eq!(h.apply_cells(&mut map,&[(a,v),(a,v)]),Err(MapDocumentError::DuplicateCellEdit(a)));
        assert_eq!(h.apply_cells(&mut map,&[(a,v),(CellCoord::new(4,0),v)]),Err(MapDocumentError::CellOutOfBounds(CellCoord::new(4,0))));
        assert_eq!(map.cell(a),Some(&CellLayers::default())); assert_eq!(h.undo_depth(),0);
    }
    #[test]
    fn no_ops_capacity_and_redo_invalidation_work() {
        let mut map=document(); let mut h=MapEditHistory::new(1); let a=CellCoord::new(0,0);
        assert_eq!(h.apply_cells(&mut map,&[(a,CellLayers::default())]).unwrap(),0);
        let v=CellLayers{overlay:Some(8),..CellLayers::default()}; assert_eq!(h.apply_cells(&mut map,&[(a,v)]).unwrap(),1);
        h.undo(&mut map).unwrap(); h.apply_cell(&mut map,CellCoord::new(1,0),v).unwrap(); assert!(!h.can_redo());
        let mut disabled=MapEditHistory::new(0); assert_eq!(disabled.apply_cell(&mut map,a,CellLayers::default()).unwrap(),true); assert_eq!(disabled.undo_depth(),0);
    }
}
