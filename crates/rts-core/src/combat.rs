//! Tiberian Dawn combat damage logic ported from TIBERIANDAWN/COMBAT.CPP.
//!
//! The original implementation operates on global map/object registries.
//! This module keeps the exact combat decisions while exposing those engine
//! operations through `CombatWorld`, allowing a game runtime or a test world
//! to provide the backing state.

use crate::coordinate::Coordinate;
use crate::facing::Facing;
use crate::misc_asm::{fixed_to_cardinal, MiscAsmVariant};

pub const ARMOR_NONE: usize = 0;
pub const ARMOR_WOOD: usize = 1;
pub const ARMOR_ALUMINUM: usize = 2;
pub const ARMOR_STEEL: usize = 3;
pub const ARMOR_CONCRETE: usize = 4;
pub const ARMOR_COUNT: usize = 5;

pub const WARHEAD_NONE: i8 = -1;
pub const WARHEAD_SA: i8 = 0;
pub const WARHEAD_HE: i8 = 1;

/// Maximum unique objects collected by the original stack-local explosion list.
pub const MAX_EXPLOSION_OBJECTS: usize = 32;

/// Tiberian Dawn's `WarheadTypeClass` damage data from TYPE.H.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TiberianDawnWarhead {
    pub id: i8,
    pub spread_factor: u32,
    pub is_wall_destroyer: bool,
    pub is_wood_destroyer: bool,
    pub is_tiberium_destroyer: bool,
    /// Tiberian Dawn stores each armor modifier as an unsigned 8.8 fixed value.
    pub modifiers: [u32; ARMOR_COUNT],
}

impl TiberianDawnWarhead {
    pub const fn new(id: i8, modifiers: [u32; ARMOR_COUNT]) -> Self {
        Self {
            id,
            spread_factor: 0,
            is_wall_destroyer: false,
            is_wood_destroyer: false,
            is_tiberium_destroyer: false,
            modifiers,
        }
    }

    pub const fn with_spread_factor(mut self, spread_factor: u32) -> Self {
        self.spread_factor = spread_factor;
        self
    }
}

/// Invalid armor values are rejected instead of indexing beyond the source table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatError {
    InvalidArmorIndex(usize),
}

/// Exact port of the four-argument Tiberian Dawn `Modify_Damage` overload.
///
/// Returns zero for inert special state, zero raw damage or an absent warhead.
/// The legacy code's final right shift is retained as written; in particular,
/// this does not force a minimum damage value of one.
pub fn modify_damage(
    damage: i32,
    warhead: Option<&TiberianDawnWarhead>,
    armor: usize,
    distance: i32,
    special_is_inert: bool,
) -> Result<i32, CombatError> {
    if special_is_inert || damage == 0 || warhead.is_none() {
        return Ok(0);
    }
    let warhead = warhead.expect("checked above");
    let modifier = *warhead
        .modifiers
        .get(armor)
        .ok_or(CombatError::InvalidArmorIndex(armor))?;

    let mut result = fixed_to_cardinal(
        MiscAsmVariant::TiberianDawn,
        damage as u32,
        modifier,
    ) as i32;

    if result != 0 {
        let spread_distance = distance
            .wrapping_shr(warhead.spread_factor)
            .clamp(0, 16) as u32;
        result >>= spread_distance;
    }
    Ok(result)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatObjectKind {
    Building,
    Aircraft,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OverlayCombatInfo {
    pub is_wall: bool,
    pub is_wooden: bool,
}

/// Minimal world operations required by the original explosion routine.
///
/// `Object` is a stable object handle. Implementations should return the
/// linked-list successor for each cell occupant and keep those handles valid
/// throughout one call to `explosion_damage`.
pub trait CombatWorld {
    type Object: Copy + Eq;

    fn coord_cell(&self, coordinate: Coordinate) -> Option<u32>;
    fn cell_occupier(&self, cell: u32) -> Option<Self::Object>;
    fn adjacent_cell(&self, cell: u32, direction: Facing) -> Option<u32>;
    fn next_occupier(&self, object: Self::Object) -> Option<Self::Object>;

    fn is_to_damage(&self, object: Self::Object) -> bool;
    fn set_to_damage(&mut self, object: Self::Object, value: bool);

    fn object_kind(&self, object: Self::Object) -> CombatObjectKind;
    fn is_down(&self, object: Self::Object) -> bool;
    fn is_in_limbo(&self, object: Self::Object) -> bool;
    fn center_coordinate(&self, object: Self::Object) -> Coordinate;

    fn take_damage(
        &mut self,
        object: Self::Object,
        damage: i32,
        distance: u32,
        warhead: i8,
        source: Option<Self::Object>,
    );

    fn reduce_tiberium(&mut self, cell: u32, amount: u32);
    fn overlay_combat_info(&self, cell: u32) -> Option<OverlayCombatInfo>;
    fn reduce_wall(&mut self, cell: u32, strength: u32);
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ExplosionReport {
    pub collected_objects: usize,
    pub damaged_objects: usize,
    pub tiberium_reduction: u32,
    pub wall_damaged: bool,
}

fn collect_cell_objects<W: CombatWorld>(
    world: &mut W,
    cell: u32,
    source: Option<W::Object>,
    objects: &mut Vec<W::Object>,
) {
    let mut current = world.cell_occupier(cell);
    while let Some(object) = current {
        // Read the successor before any object state is changed.
        current = world.next_occupier(object);

        if !world.is_to_damage(object) && Some(object) != source {
            world.set_to_damage(object, true);
            objects.push(object);
            if objects.len() >= MAX_EXPLOSION_OBJECTS {
                return;
            }
        }
    }
}

/// Exact behavioral port of Tiberian Dawn's `Explosion_Damage`.
///
/// The source engine supplies `ICON_LEPTON_W`; it is an argument here so the
/// collision radius follows the actual runtime build instead of guessing a
/// display-scale constant. The source uses one and a half icon widths.
pub fn explosion_damage<W: CombatWorld>(
    world: &mut W,
    coordinate: Coordinate,
    strength: u32,
    source: Option<W::Object>,
    warhead: Option<&TiberianDawnWarhead>,
    special_is_inert: bool,
    icon_lepton_width: u32,
) -> ExplosionReport {
    let Some(warhead) = warhead else {
        return ExplosionReport::default();
    };
    if strength == 0 || special_is_inert || warhead.id == WARHEAD_NONE {
        return ExplosionReport::default();
    }

    let Some(cell) = world.coord_cell(coordinate) else {
        return ExplosionReport::default();
    };

    // The first occupant is retained for the source's direct building-hit test.
    let impact_occupier = world.cell_occupier(cell);
    let mut objects = Vec::with_capacity(MAX_EXPLOSION_OBJECTS);

    collect_cell_objects(world, cell, source, &mut objects);

    // FACING_NONE is the center cell; the following eight values are the
    // compass directions in the same order as FacingType in DEFINES.H.
    for direction in Facing::ALL {
        if objects.len() >= MAX_EXPLOSION_OBJECTS {
            break;
        }
        if let Some(adjacent) = world.adjacent_cell(cell, direction) {
            collect_cell_objects(world, adjacent, source, &mut objects);
        }
    }

    let range = icon_lepton_width.wrapping_add(icon_lepton_width >> 1);
    let mut report = ExplosionReport {
        collected_objects: objects.len(),
        ..ExplosionReport::default()
    };

    for object in objects {
        // The C++ routine clears this transient de-duplication flag before
        // evaluating the target and before calling its damage handler.
        world.set_to_damage(object, false);

        let distance = if world.object_kind(object) == CombatObjectKind::Building
            && impact_occupier == Some(object)
        {
            0
        } else {
            coordinate.distance(world.center_coordinate(object))
        };

        if world.is_down(object) && !world.is_in_limbo(object) && distance < range {
            // The C++ local is a signed int initialized from unsigned strength.
            let mut damage = strength as i32;
            if warhead.id == WARHEAD_HE
                && world.object_kind(object) == CombatObjectKind::Aircraft
            {
                damage = damage.wrapping_mul(2);
            }
            if damage != 0 {
                world.take_damage(object, damage, distance, warhead.id, source);
                report.damaged_objects += 1;
            }
        }
    }

    let tiberium_reduction = strength / 10;
    world.reduce_tiberium(cell, tiberium_reduction);
    report.tiberium_reduction = tiberium_reduction;

    if let Some(overlay) = world.overlay_combat_info(cell) {
        if overlay.is_wall
            && (warhead.is_wall_destroyer || (warhead.is_wood_destroyer && overlay.is_wooden))
        {
            world.reduce_wall(cell, strength);
            report.wall_damaged = true;
        }
    }

    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct DamageEvent {
        amount: i32,
        distance: u32,
        warhead: i8,
        source: Option<usize>,
    }

    #[derive(Debug, Clone)]
    struct MockObject {
        kind: CombatObjectKind,
        coordinate: Coordinate,
        down: bool,
        in_limbo: bool,
        marked: bool,
        next: Option<usize>,
        damage: Vec<DamageEvent>,
    }

    #[derive(Default)]
    struct MockWorld {
        objects: Vec<MockObject>,
        cell_heads: BTreeMap<u32, usize>,
        adjacency: BTreeMap<(u32, u8), u32>,
        overlays: BTreeMap<u32, OverlayCombatInfo>,
        tiberium: Vec<(u32, u32)>,
        walls: Vec<(u32, u32)>,
    }

    impl MockWorld {
        fn add_object(
            &mut self,
            cell: u32,
            kind: CombatObjectKind,
            coordinate: Coordinate,
            down: bool,
            in_limbo: bool,
        ) -> usize {
            let id = self.objects.len();
            self.objects.push(MockObject {
                kind,
                coordinate,
                down,
                in_limbo,
                marked: false,
                next: None,
                damage: Vec::new(),
            });
            match self.cell_heads.get(&cell).copied() {
                None => { self.cell_heads.insert(cell, id); }
                Some(mut current) => {
                    while let Some(next) = self.objects[current].next {
                        current = next;
                    }
                    self.objects[current].next = Some(id);
                }
            }
            id
        }

        fn add_adjacent(&mut self, center: u32, facing: Facing, adjacent: u32) {
            self.adjacency.insert((center, facing.ordinal()), adjacent);
        }
    }

    impl CombatWorld for MockWorld {
        type Object = usize;

        fn coord_cell(&self, _coordinate: Coordinate) -> Option<u32> { Some(10) }
        fn cell_occupier(&self, cell: u32) -> Option<Self::Object> {
            self.cell_heads.get(&cell).copied()
        }
        fn adjacent_cell(&self, cell: u32, direction: Facing) -> Option<u32> {
            self.adjacency.get(&(cell, direction.ordinal())).copied()
        }
        fn next_occupier(&self, object: Self::Object) -> Option<Self::Object> {
            self.objects.get(object).and_then(|entry| entry.next)
        }
        fn is_to_damage(&self, object: Self::Object) -> bool {
            self.objects.get(object).is_some_and(|entry| entry.marked)
        }
        fn set_to_damage(&mut self, object: Self::Object, value: bool) {
            self.objects[object].marked = value;
        }
        fn object_kind(&self, object: Self::Object) -> CombatObjectKind {
            self.objects[object].kind
        }
        fn is_down(&self, object: Self::Object) -> bool { self.objects[object].down }
        fn is_in_limbo(&self, object: Self::Object) -> bool { self.objects[object].in_limbo }
        fn center_coordinate(&self, object: Self::Object) -> Coordinate {
            self.objects[object].coordinate
        }
        fn take_damage(
            &mut self,
            object: Self::Object,
            damage: i32,
            distance: u32,
            warhead: i8,
            source: Option<Self::Object>,
        ) {
            self.objects[object].damage.push(DamageEvent {
                amount: damage, distance, warhead, source,
            });
        }
        fn reduce_tiberium(&mut self, cell: u32, amount: u32) {
            self.tiberium.push((cell, amount));
        }
        fn overlay_combat_info(&self, cell: u32) -> Option<OverlayCombatInfo> {
            self.overlays.get(&cell).copied()
        }
        fn reduce_wall(&mut self, cell: u32, strength: u32) {
            self.walls.push((cell, strength));
        }
    }

    fn warhead(id: i8) -> TiberianDawnWarhead {
        TiberianDawnWarhead::new(id, [0x100; ARMOR_COUNT])
    }

    fn coordinate(x: i16, y: i16) -> Coordinate {
        Coordinate::from_leptons(x, y)
    }

    #[test]
    fn modify_damage_uses_tiberian_dawn_8_8_modifier_then_spread_shift() {
        let profile = TiberianDawnWarhead::new(WARHEAD_SA, [0x100, 0x80, 0x100, 0x100, 0x100])
            .with_spread_factor(2);
        assert_eq!(modify_damage(100, Some(&profile), ARMOR_WOOD, 8, false), Ok(12));
        assert_eq!(modify_damage(100, Some(&profile), ARMOR_NONE, 0, false), Ok(100));
        assert_eq!(modify_damage(100, Some(&profile), ARMOR_WOOD, 8, true), Ok(0));
        assert_eq!(modify_damage(100, None, ARMOR_COUNT, 8, false), Ok(0));
    }

    #[test]
    fn modify_damage_preserves_zero_after_final_distance_attenuation() {
        let profile = warhead(WARHEAD_SA);
        assert_eq!(modify_damage(1, Some(&profile), ARMOR_NONE, 16, false), Ok(0));
    }

    #[test]
    fn modify_damage_rejects_invalid_armor_only_when_damage_is_applicable() {
        let profile = warhead(WARHEAD_SA);
        assert_eq!(
            modify_damage(100, Some(&profile), ARMOR_COUNT, 0, false),
            Err(CombatError::InvalidArmorIndex(ARMOR_COUNT))
        );
        assert_eq!(modify_damage(0, Some(&profile), ARMOR_COUNT, 0, false), Ok(0));
    }

    #[test]
    fn explosion_hits_impact_building_and_doubles_he_aircraft_damage() {
        let mut world = MockWorld::default();
        let building = world.add_object(10, CombatObjectKind::Building, coordinate(900, 900), true, false);
        let source = world.add_object(10, CombatObjectKind::Other, coordinate(0, 0), true, false);
        let aircraft = world.add_object(11, CombatObjectKind::Aircraft, coordinate(300, 0), true, false);
        world.add_adjacent(10, Facing::East, 11);
        world.overlays.insert(10, OverlayCombatInfo { is_wall: true, is_wooden: true });
        let profile = TiberianDawnWarhead {
            is_wall_destroyer: false,
            is_wood_destroyer: true,
            ..warhead(WARHEAD_HE)
        };

        let report = explosion_damage(
            &mut world, coordinate(0, 0), 30, Some(source), Some(&profile), false, 256,
        );

        assert_eq!(report.collected_objects, 2);
        assert_eq!(report.damaged_objects, 2);
        assert_eq!(world.objects[building].damage[0].amount, 30);
        assert_eq!(world.objects[building].damage[0].distance, 0);
        assert_eq!(world.objects[aircraft].damage[0].amount, 60);
        assert_eq!(world.objects[aircraft].damage[0].distance, 300);
        assert!(!world.objects[building].marked);
        assert!(!world.objects[aircraft].marked);
        assert_eq!(world.tiberium, vec![(10, 3)]);
        assert_eq!(world.walls, vec![(10, 30)]);
        assert!(report.wall_damaged);
    }

    #[test]
    fn explosion_skips_source_down_limbo_and_out_of_range_objects() {
        let mut world = MockWorld::default();
        let source = world.add_object(10, CombatObjectKind::Other, coordinate(0, 0), true, false);
        let down = world.add_object(10, CombatObjectKind::Other, coordinate(0, 0), false, false);
        let limbo = world.add_object(10, CombatObjectKind::Other, coordinate(0, 0), true, true);
        let distant = world.add_object(10, CombatObjectKind::Other, coordinate(2_000, 0), true, false);

        let report = explosion_damage(
            &mut world, coordinate(0, 0), 20, Some(source), Some(&warhead(WARHEAD_SA)), false, 256,
        );
        assert_eq!(report.collected_objects, 3);
        assert_eq!(report.damaged_objects, 0);
        assert!(world.objects.iter().all(|object| object.damage.is_empty()));
        assert!(world.objects.iter().all(|object| !object.marked));
        assert_eq!(world.tiberium, vec![(10, 2)]);
        assert!(world.walls.is_empty());
        let _ = (down, limbo, distant);
    }

    #[test]
    fn explosion_collection_is_deduplicated_and_capped_at_32() {
        let mut world = MockWorld::default();
        let mut ids = Vec::new();
        for _ in 0..33 {
            ids.push(world.add_object(10, CombatObjectKind::Other, coordinate(0, 0), true, false));
        }
        let report = explosion_damage(
            &mut world, coordinate(0, 0), 1, None, Some(&warhead(WARHEAD_SA)), false, 256,
        );
        assert_eq!(report.collected_objects, MAX_EXPLOSION_OBJECTS);
        assert_eq!(report.damaged_objects, MAX_EXPLOSION_OBJECTS);
        for id in ids.iter().take(MAX_EXPLOSION_OBJECTS) {
            assert_eq!(world.objects[*id].damage.len(), 1);
            assert!(!world.objects[*id].marked);
        }
        assert!(world.objects[*ids.last().unwrap()].damage.is_empty());
        assert!(!world.objects[*ids.last().unwrap()].marked);
    }

    #[test]
    fn explosion_does_not_mutate_world_for_inert_or_invalid_impact() {
        let mut world = MockWorld::default();
        let profile = warhead(WARHEAD_SA);
        let report = explosion_damage(
            &mut world, coordinate(0, 0), 30, None, Some(&profile), true, 256,
        );
        assert_eq!(report, ExplosionReport::default());
        assert!(world.tiberium.is_empty());
        assert!(world.walls.is_empty());
    }

    #[test]
    fn wood_destroyer_does_not_reduce_steel_wall() {
        let mut world = MockWorld::default();
        world.overlays.insert(10, OverlayCombatInfo { is_wall: true, is_wooden: false });
        let profile = TiberianDawnWarhead {
            is_wood_destroyer: true,
            ..warhead(WARHEAD_SA)
        };
        let report = explosion_damage(
            &mut world, coordinate(0, 0), 10, None, Some(&profile), false, 256,
        );
        assert!(!report.wall_damaged);
        assert!(world.walls.is_empty());
    }
}
