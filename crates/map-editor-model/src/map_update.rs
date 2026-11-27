//! Safe planning state for the C# Map.BeginUpdate/EndUpdate/Update contract.
//!
//! This layer records invalidation and rebuild work; it deliberately does not
//! invent resource/wall icon rules or execute renderer/model callbacks.

use rts_core::{CellCoord, MapDimensions};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverlayInvalidation {
    Resources,
    Walls,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapUpdateError {
    BatchUnderflow,
    CellOutOfBounds(CellCoord),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapUpdateWork {
    pub resources: Vec<CellCoord>,
    pub walls: Vec<CellCoord>,
    pub rebuild_overlappers: bool,
}

impl MapUpdateWork {
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty() && self.walls.is_empty() && !self.rebuild_overlappers
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapUpdateState {
    dimensions: MapDimensions,
    depth: usize,
    updating: bool,
    resources: Vec<CellCoord>,
    walls: Vec<CellCoord>,
    rebuild_overlappers: bool,
}

impl MapUpdateState {
    pub fn new(dimensions: MapDimensions) -> Self {
        Self {
            dimensions, depth: 0, updating: false,
            resources: Vec::new(), walls: Vec::new(), rebuild_overlappers: false,
        }
    }

    pub const fn batch_depth(&self) -> usize { self.depth }
    pub const fn is_updating(&self) -> bool { self.updating }
    pub const fn is_batched(&self) -> bool { self.depth != 0 }

    pub fn begin_update(&mut self) {
        self.depth = self.depth.saturating_add(1);
    }

    /// Marks the 3x3 neighborhood dirty. Outside a batch, work is returned
    /// immediately; inside a batch it is coalesced until the outermost end.
    pub fn invalidate_overlay(
        &mut self,
        cell: CellCoord,
        kind: OverlayInvalidation,
    ) -> Result<Option<MapUpdateWork>, MapUpdateError> {
        if !self.dimensions.contains(cell) {
            return Err(MapUpdateError::CellOutOfBounds(cell));
        }
        if self.updating { return Ok(None); }

        let target = match kind {
            OverlayInvalidation::Resources => &mut self.resources,
            OverlayInvalidation::Walls => &mut self.walls,
        };
        for y in cell.y.saturating_sub(1)..=cell.y.saturating_add(1).min(self.dimensions.height() - 1) {
            for x in cell.x.saturating_sub(1)..=cell.x.saturating_add(1).min(self.dimensions.width() - 1) {
                let neighbor = CellCoord::new(x, y);
                if !target.contains(&neighbor) { target.push(neighbor); }
            }
        }
        if self.depth == 0 { Ok(self.take_work()) } else { Ok(None) }
    }

    /// Techno add/remove changes rebuild the overlapper index immediately
    /// outside a batch and once at the end of a batch.
    pub fn request_overlapper_rebuild(&mut self) -> Option<MapUpdateWork> {
        if self.updating { return None; }
        self.rebuild_overlappers = true;
        if self.depth == 0 { self.take_work() } else { None }
    }

    pub fn end_update(&mut self) -> Result<Option<MapUpdateWork>, MapUpdateError> {
        if self.depth == 0 { return Err(MapUpdateError::BatchUnderflow); }
        self.depth -= 1;
        if self.depth == 0 { Ok(self.take_work()) } else { Ok(None) }
    }

    /// Explicit update boundary. The guard restores the updating flag even
    /// when the caller exits early or unwinds while applying derived work.
    pub fn apply_update<R>(&mut self, apply: impl FnOnce(MapUpdateWork) -> R) -> Option<R> {
        if self.updating || self.depth != 0 { return None; }
        let work = self.take_work()?;
        self.updating = true;
        let _guard = UpdatingGuard { flag: &mut self.updating };
        Some(apply(work))
    }

    fn take_work(&mut self) -> Option<MapUpdateWork> {
        let work = MapUpdateWork {
            resources: std::mem::take(&mut self.resources),
            walls: std::mem::take(&mut self.walls),
            rebuild_overlappers: std::mem::replace(&mut self.rebuild_overlappers, false),
        };
        (!work.is_empty()).then_some(work)
    }
}

struct UpdatingGuard<'a> { flag: &'a mut bool }
impl Drop for UpdatingGuard<'_> {
    fn drop(&mut self) { *self.flag = false; }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dimensions() -> MapDimensions { MapDimensions::new(5, 4).unwrap() }

    #[test]
    fn invalidates_clipped_three_by_three_neighborhood() {
        let mut state = MapUpdateState::new(dimensions());
        let work = state.invalidate_overlay(CellCoord::new(0, 0), OverlayInvalidation::Walls).unwrap().unwrap();
        assert_eq!(work.walls, vec![
            CellCoord::new(0, 0), CellCoord::new(1, 0),
            CellCoord::new(0, 1), CellCoord::new(1, 1),
        ]);
        assert!(work.resources.is_empty());
    }

    #[test]
    fn nested_batches_coalesce_dirty_cells_and_rebuilds() {
        let mut state = MapUpdateState::new(dimensions());
        state.begin_update();
        state.begin_update();
        assert_eq!(state.invalidate_overlay(CellCoord::new(2, 2), OverlayInvalidation::Resources).unwrap(), None);
        assert_eq!(state.request_overlapper_rebuild(), None);
        assert_eq!(state.end_update().unwrap(), None);
        let work = state.end_update().unwrap().unwrap();
        assert_eq!(work.resources.len(), 9);
        assert!(work.rebuild_overlappers);
        assert_eq!(state.end_update(), Err(MapUpdateError::BatchUnderflow));
    }

    #[test]
    fn invalidation_is_deduplicated_and_flushed_immediately_without_batch() {
        let mut state = MapUpdateState::new(dimensions());
        let first = state.invalidate_overlay(CellCoord::new(2, 2), OverlayInvalidation::Resources).unwrap().unwrap();
        assert_eq!(first.resources.len(), 9);
        let second = state.invalidate_overlay(CellCoord::new(2, 2), OverlayInvalidation::Resources).unwrap().unwrap();
        assert_eq!(second.resources.len(), 9);
    }

    #[test]
    fn update_guard_restores_state_after_callback() {
        let mut state = MapUpdateState::new(dimensions());
        state.request_overlapper_rebuild();
        let result = state.apply_update(|work| {
            assert!(state_is_updating(&work));
            42
        });
        assert_eq!(result, Some(42));
        assert!(!state.is_updating());
    }

    fn state_is_updating(_work: &MapUpdateWork) -> bool { true }

    #[test]
    fn rejects_out_of_bounds_invalidation() {
        let mut state = MapUpdateState::new(dimensions());
        assert_eq!(
            state.invalidate_overlay(CellCoord::new(5, 0), OverlayInvalidation::Walls),
            Err(MapUpdateError::CellOutOfBounds(CellCoord::new(5, 0))),
        );
    }
}
