//! Red Alert TemplateClass from REDALERT/TEMPLATE.CPP.
//!
//! Template instances use the engine's object pool and update the map cells
//! covered by a template type's icon-set map. The adapter keeps pool, map,
//! radar visibility, overlay/smudge, and redraw operations in the host.

pub const MAP_CELL_WIDTH: i32 = 64;
pub const TEMPLATE_CLEAR_TYPE: i16 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TemplateTypeInfo {
    pub type_id: i16,
    pub width: u8,
    pub height: u8,
    pub is_clear_template: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateMark {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TemplateCellState {
    pub template_type: i16,
    pub template_icon: i16,
}

pub trait TemplateWorld {
    type TemplateId: Copy + Eq;

    fn free_all_templates(&mut self);
    fn allocate_template_slot(&mut self) -> Option<Self::TemplateId>;
    fn set_template_active(&mut self, id: Self::TemplateId, active: bool);
    fn free_template_slot(&mut self, id: Self::TemplateId, bypass_unmark: bool);
    fn unlimbo_template(&mut self, id: Self::TemplateId, cell: u32);
    fn limbo_template(&mut self, id: Self::TemplateId);

    fn object_mark(&mut self, id: Self::TemplateId, mark: TemplateMark) -> bool;
    fn icon_set_map(&self, type_id: i16) -> Option<Vec<i16>>;
    fn origin_cell(&self, id: Self::TemplateId) -> Option<u32>;
    fn cell_in_radar(&self, cell: u32) -> bool;
    fn template_at(&self, cell: u32) -> TemplateCellState;
    fn set_template_at(&mut self, cell: u32, state: TemplateCellState);
    fn clear_overlay(&mut self, cell: u32);
    fn clear_smudge(&mut self, cell: u32);
    fn redraw_objects(&mut self, cell: u32);
    fn recalculate_attributes(&mut self, cell: u32);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TemplateClass<Id: Copy + Eq> {
    pub id: Id,
    pub active: bool,
    pub class: TemplateTypeInfo,
}

impl<Id: Copy + Eq> TemplateClass<Id> {
    /// TemplateClass::Init resets the static template object pool.
    pub fn init(world: &mut impl TemplateWorld<TemplateId = Id>) {
        world.free_all_templates();
    }

    /// Counterpart to operator new plus the constructor's IsActive assignment.
    /// A missing position represents the source's sentinel coordinate.
    pub fn allocate(
        class: TemplateTypeInfo,
        position: Option<u32>,
        world: &mut impl TemplateWorld<TemplateId = Id>,
    ) -> Option<Self> {
        let id = world.allocate_template_slot()?;
        world.set_template_active(id, true);
        let object = Self { id, active: true, class };
        if let Some(cell) = position {
            world.unlimbo_template(id, cell);
        }
        Some(object)
    }

    /// TemplateClass::Mark. Returns false when the instance is inactive, its
    /// icon set is unavailable, or the base object declines the mark operation.
    pub fn mark(
        &mut self,
        mark: TemplateMark,
        world: &mut impl TemplateWorld<TemplateId = Id>,
    ) -> bool {
        if !self.active {
            return false;
        }
        let Some(icon_map) = world.icon_set_map(self.class.type_id) else {
            return false;
        };
        if !world.object_mark(self.id, mark) {
            return false;
        }
        let Some(origin) = world.origin_cell(self.id) else {
            return false;
        };

        for y in 0..i32::from(self.class.height) {
            for x in 0..i32::from(self.class.width) {
                let map_index = (y * i32::from(self.class.width) + x) as usize;
                let Some(&icon) = icon_map.get(map_index) else {
                    continue;
                };
                if icon == -1 {
                    continue;
                }

                let raw_cell = i64::from(origin)
                    + i64::from(y) * i64::from(MAP_CELL_WIDTH)
                    + i64::from(x);
                if raw_cell < 0 || raw_cell > i64::from(u32::MAX) {
                    continue;
                }
                let cell = raw_cell as u32;
                if !world.cell_in_radar(cell) {
                    continue;
                }

                match mark {
                    TemplateMark::Up => {
                        let existing = world.template_at(cell);
                        if existing.template_type == self.class.type_id
                            && existing.template_icon == icon
                        {
                            world.set_template_at(cell, TemplateCellState {
                                template_type: TEMPLATE_CLEAR_TYPE,
                                template_icon: 0,
                            });
                        }
                    }
                    TemplateMark::Down => {
                        let state = if self.class.is_clear_template {
                            TemplateCellState {
                                template_type: TEMPLATE_CLEAR_TYPE,
                                template_icon: 0,
                            }
                        } else {
                            TemplateCellState {
                                template_type: self.class.type_id,
                                template_icon: icon,
                            }
                        };
                        world.set_template_at(cell, state);
                        world.clear_overlay(cell);
                        world.clear_smudge(cell);
                    }
                }

                world.redraw_objects(cell);
                world.recalculate_attributes(cell);
            }
        }

        if mark == TemplateMark::Down {
            // The C++ code sets its no-up guard before delete this. The pool
            // adapter receives bypass_unmark=true to prevent destructor-driven
            // mark-up from clearing the just-installed map cells.
            world.set_template_active(self.id, false);
            world.free_template_slot(self.id, true);
            self.active = false;
        }
        true
    }

    /// Explicit counterpart to the inline destructor plus operator delete.
    pub fn destroy(
        &mut self,
        game_active: bool,
        world: &mut impl TemplateWorld<TemplateId = Id>,
    ) {
        if !self.active {
            return;
        }
        if game_active {
            world.limbo_template(self.id);
        }
        world.set_template_active(self.id, false);
        world.free_template_slot(self.id, false);
        self.active = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeMap, BTreeSet};

    #[derive(Default)]
    struct MockWorld {
        next_id: u32,
        freed_all: usize,
        active: BTreeSet<u32>,
        free_calls: Vec<(u32, bool)>,
        unlimbo_calls: Vec<(u32, u32)>,
        limbo_calls: Vec<u32>,
        marked: Vec<(u32, TemplateMark)>,
        icon_maps: BTreeMap<i16, Vec<i16>>,
        origins: BTreeMap<u32, u32>,
        radar_cells: BTreeSet<u32>,
        cells: BTreeMap<u32, TemplateCellState>,
        overlays_cleared: Vec<u32>,
        smudges_cleared: Vec<u32>,
        redrawn: Vec<u32>,
        recalculated: Vec<u32>,
        base_mark_result: bool,
    }

    impl TemplateWorld for MockWorld {
        type TemplateId = u32;

        fn free_all_templates(&mut self) { self.freed_all += 1; }
        fn allocate_template_slot(&mut self) -> Option<u32> {
            let id = self.next_id;
            self.next_id += 1;
            Some(id)
        }
        fn set_template_active(&mut self, id: u32, active: bool) {
            if active { self.active.insert(id); } else { self.active.remove(&id); }
        }
        fn free_template_slot(&mut self, id: u32, bypass_unmark: bool) {
            self.free_calls.push((id, bypass_unmark));
        }
        fn unlimbo_template(&mut self, id: u32, cell: u32) {
            self.origins.insert(id, cell);
            self.unlimbo_calls.push((id, cell));
        }
        fn limbo_template(&mut self, id: u32) { self.limbo_calls.push(id); }
        fn object_mark(&mut self, id: u32, mark: TemplateMark) -> bool {
            self.marked.push((id, mark));
            self.base_mark_result
        }
        fn icon_set_map(&self, type_id: i16) -> Option<Vec<i16>> {
            self.icon_maps.get(&type_id).cloned()
        }
        fn origin_cell(&self, id: u32) -> Option<u32> { self.origins.get(&id).copied() }
        fn cell_in_radar(&self, cell: u32) -> bool { self.radar_cells.contains(&cell) }
        fn template_at(&self, cell: u32) -> TemplateCellState {
            self.cells.get(&cell).copied().unwrap_or(TemplateCellState {
                template_type: TEMPLATE_CLEAR_TYPE,
                template_icon: 0,
            })
        }
        fn set_template_at(&mut self, cell: u32, state: TemplateCellState) {
            self.cells.insert(cell, state);
        }
        fn clear_overlay(&mut self, cell: u32) { self.overlays_cleared.push(cell); }
        fn clear_smudge(&mut self, cell: u32) { self.smudges_cleared.push(cell); }
        fn redraw_objects(&mut self, cell: u32) { self.redrawn.push(cell); }
        fn recalculate_attributes(&mut self, cell: u32) { self.recalculated.push(cell); }
    }

    fn class() -> TemplateTypeInfo {
        TemplateTypeInfo { type_id: 7, width: 3, height: 2, is_clear_template: false }
    }

    fn world() -> MockWorld {
        let mut value = MockWorld { base_mark_result: true, ..MockWorld::default() };
        value.icon_maps.insert(7, vec![1, -1, 2, 3, 4, 5]);
        for cell in [100, 101, 102, 164, 165, 166] {
            value.radar_cells.insert(cell);
        }
        value
    }

    #[test]
    fn init_clears_static_template_pool() {
        let mut host = world();
        TemplateClass::<u32>::init(&mut host);
        assert_eq!(host.freed_all, 1);
    }

    #[test]
    fn allocation_marks_slot_active_and_unlimbos_non_sentinel_position() {
        let mut host = world();
        let template = TemplateClass::allocate(class(), Some(100), &mut host).unwrap();
        assert_eq!(template.id, 0);
        assert!(template.active);
        assert!(host.active.contains(&0));
        assert_eq!(host.unlimbo_calls, vec![(0, 100)]);

        let no_position = TemplateClass::allocate(class(), None, &mut host).unwrap();
        assert_eq!(no_position.id, 1);
        assert_eq!(host.unlimbo_calls.len(), 1);
    }

    #[test]
    fn mark_down_writes_only_real_icon_cells_and_clears_overlay_and_smudge() {
        let mut host = world();
        let mut template = TemplateClass::allocate(class(), Some(100), &mut host).unwrap();
        assert!(template.mark(TemplateMark::Down, &mut host));
        assert_eq!(host.cells.get(&100), Some(&TemplateCellState { template_type: 7, template_icon: 1 }));
        assert!(!host.cells.contains_key(&101));
        assert_eq!(host.cells.get(&102), Some(&TemplateCellState { template_type: 7, template_icon: 2 }));
        assert_eq!(host.cells.get(&164), Some(&TemplateCellState { template_type: 7, template_icon: 3 }));
        assert_eq!(host.cells.get(&165), Some(&TemplateCellState { template_type: 7, template_icon: 4 }));
        assert_eq!(host.cells.get(&166), Some(&TemplateCellState { template_type: 7, template_icon: 5 }));
        assert_eq!(host.overlays_cleared, vec![100, 102, 164, 165, 166]);
        assert_eq!(host.smudges_cleared, host.overlays_cleared);
        assert_eq!(host.redrawn.len(), 5);
        assert_eq!(host.recalculated.len(), 5);
        assert!(!template.active);
        assert_eq!(host.free_calls, vec![(0, true)]);
    }

    #[test]
    fn mark_up_clears_only_a_matching_template_type_and_icon() {
        let mut host = world();
        host.cells.insert(100, TemplateCellState { template_type: 7, template_icon: 1 });
        host.cells.insert(102, TemplateCellState { template_type: 9, template_icon: 2 });
        host.cells.insert(164, TemplateCellState { template_type: 7, template_icon: 99 });
        let mut template = TemplateClass::allocate(class(), Some(100), &mut host).unwrap();
        assert!(template.mark(TemplateMark::Up, &mut host));
        assert_eq!(host.cells.get(&100), Some(&TemplateCellState { template_type: TEMPLATE_CLEAR_TYPE, template_icon: 0 }));
        assert_eq!(host.cells.get(&102), Some(&TemplateCellState { template_type: 9, template_icon: 2 }));
        assert_eq!(host.cells.get(&164), Some(&TemplateCellState { template_type: 7, template_icon: 99 }));
        assert!(host.overlays_cleared.is_empty());
        assert!(host.smudges_cleared.is_empty());
        assert_eq!(host.free_calls.len(), 0);
    }

    #[test]
    fn radar_visibility_and_map_bounds_limit_cell_mutations() {
        let mut host = world();
        host.radar_cells.remove(&165);
        let mut template = TemplateClass::allocate(class(), Some(100), &mut host).unwrap();
        assert!(template.mark(TemplateMark::Down, &mut host));
        assert!(!host.cells.contains_key(&165));
        assert!(!host.overlays_cleared.contains(&165));
        assert_eq!(host.redrawn.len(), 4);
    }

    #[test]
    fn base_mark_failure_and_missing_icon_map_do_not_change_map() {
        let mut host = world();
        host.base_mark_result = false;
        let mut template = TemplateClass::allocate(class(), Some(100), &mut host).unwrap();
        assert!(!template.mark(TemplateMark::Down, &mut host));
        assert!(host.cells.is_empty());
        assert!(host.free_calls.is_empty());

        host.base_mark_result = true;
        host.icon_maps.remove(&7);
        assert!(!template.mark(TemplateMark::Down, &mut host));
        assert!(host.cells.is_empty());
    }

    #[test]
    fn clear_template_places_clear_tiles_instead_of_stamping_icons() {
        let mut host = world();
        let clear = TemplateTypeInfo { is_clear_template: true, ..class() };
        let mut template = TemplateClass::allocate(clear, Some(100), &mut host).unwrap();
        assert!(template.mark(TemplateMark::Down, &mut host));
        assert_eq!(host.cells.get(&100), Some(&TemplateCellState { template_type: 0, template_icon: 0 }));
        assert_eq!(host.cells.get(&102), Some(&TemplateCellState { template_type: 0, template_icon: 0 }));
    }

    #[test]
    fn ordinary_destroy_limbos_when_game_active_then_frees_pool_slot() {
        let mut host = world();
        let mut template = TemplateClass::allocate(class(), Some(100), &mut host).unwrap();
        template.destroy(true, &mut host);
        assert_eq!(host.limbo_calls, vec![0]);
        assert_eq!(host.free_calls, vec![(0, false)]);
        assert!(!host.active.contains(&0));
    }

    #[test]
    fn destroy_outside_game_skips_limbo_and_inactive_destroy_is_idempotent() {
        let mut host = world();
        let mut template = TemplateClass::allocate(class(), None, &mut host).unwrap();
        template.destroy(false, &mut host);
        template.destroy(true, &mut host);
        assert!(host.limbo_calls.is_empty());
        assert_eq!(host.free_calls, vec![(0, false)]);
    }

    #[test]
    fn inactive_template_cannot_be_marked() {
        let mut host = world();
        let mut template = TemplateClass::allocate(class(), None, &mut host).unwrap();
        template.active = false;
        assert!(!template.mark(TemplateMark::Down, &mut host));
        assert!(host.marked.is_empty());
    }
}
