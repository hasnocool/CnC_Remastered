//! Detach_This_From_All from REDALERT/TRACKER.CPP.

use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DetachCollection {
    Houses, Teams, TeamTypes, Units, Vessels, Aircraft, Buildings,
    Bullets, Infantry, Anims, Triggers, TriggerTypes,
}

/// World access boundary for the legacy global object arrays and singletons.
pub trait DetachWorld {
    fn target_legal(&self, target: u32) -> bool;
    fn detach_collection(&mut self, collection: DetachCollection, target: u32, all: bool);
    fn detach_map(&mut self, target: u32, all: bool);
    fn detach_logic(&mut self, target: u32, all: bool);
    fn detach_chronal_vortex(&mut self, target: u32);
    fn is_trigger_type(&self, target: u32) -> bool;
    fn trigger_count(&self) -> usize;
    fn trigger_class_target(&self, index: usize) -> Option<u32>;
    fn trigger_target(&self, index: usize) -> u32;
    fn delete_trigger(&mut self, index: usize);
}

const DETACH_ORDER: [DetachCollection; 10] = [
    DetachCollection::Houses,
    DetachCollection::Teams,
    DetachCollection::TeamTypes,
    DetachCollection::Units,
    DetachCollection::Vessels,
    DetachCollection::Aircraft,
    DetachCollection::Buildings,
    DetachCollection::Bullets,
    DetachCollection::Infantry,
    DetachCollection::Anims,
];

/// Sweep every object registry and singleton in the original order. Dependent
/// triggers are recursively detached and deleted before trigger registries
/// receive their own detach call.
pub fn detach_this_from_all(world: &mut impl DetachWorld, target: u32, all: bool) {
    let mut visited = HashSet::new();
    detach_recursive(world, target, all, &mut visited);
}

fn detach_recursive(
    world: &mut impl DetachWorld,
    target: u32,
    all: bool,
    visited: &mut HashSet<u32>,
) {
    if !world.target_legal(target) || !visited.insert(target) { return; }

    for collection in DETACH_ORDER {
        world.detach_collection(collection, target, all);
    }
    world.detach_map(target, all);
    world.detach_logic(target, all);
    world.detach_chronal_vortex(target);

    if world.is_trigger_type(target) {
        let mut index = 0;
        while index < world.trigger_count() {
            if world.trigger_class_target(index) == Some(target) {
                let dependent_target = world.trigger_target(index);
                detach_recursive(world, dependent_target, true, visited);
                world.delete_trigger(index);
            } else {
                index += 1;
            }
        }
    }

    world.detach_collection(DetachCollection::Triggers, target, all);
    world.detach_collection(DetachCollection::TriggerTypes, target, all);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug)]
    struct Trigger { class_target: u32, target: u32 }

    #[derive(Default)]
    struct FakeWorld {
        calls: Vec<(String, u32, bool)>,
        legal: HashSet<u32>,
        trigger_types: HashSet<u32>,
        triggers: Vec<Trigger>,
    }

    impl DetachWorld for FakeWorld {
        fn target_legal(&self, target: u32) -> bool { self.legal.contains(&target) }
        fn detach_collection(&mut self, c: DetachCollection, target: u32, all: bool) {
            self.calls.push((format!("{c:?}"), target, all));
        }
        fn detach_map(&mut self, target: u32, all: bool) { self.calls.push(("Map".into(), target, all)); }
        fn detach_logic(&mut self, target: u32, all: bool) { self.calls.push(("Logic".into(), target, all)); }
        fn detach_chronal_vortex(&mut self, target: u32) { self.calls.push(("ChronalVortex".into(), target, false)); }
        fn is_trigger_type(&self, target: u32) -> bool { self.trigger_types.contains(&target) }
        fn trigger_count(&self) -> usize { self.triggers.len() }
        fn trigger_class_target(&self, index: usize) -> Option<u32> { self.triggers.get(index).map(|t| t.class_target) }
        fn trigger_target(&self, index: usize) -> u32 { self.triggers[index].target }
        fn delete_trigger(&mut self, index: usize) { self.triggers.remove(index); }
    }

    #[test]
    fn illegal_targets_are_ignored() {
        let mut world = FakeWorld::default();
        detach_this_from_all(&mut world, 0, true);
        assert!(world.calls.is_empty());
    }

    #[test]
    fn detaches_all_registries_in_source_order() {
        let mut world = FakeWorld::default();
        world.legal.insert(10);
        detach_this_from_all(&mut world, 10, false);
        let names = world.calls.iter().map(|c| c.0.as_str()).collect::<Vec<_>>();
        assert_eq!(names, [
            "Houses", "Teams", "TeamTypes", "Units", "Vessels", "Aircraft",
            "Buildings", "Bullets", "Infantry", "Anims", "Map", "Logic",
            "ChronalVortex", "Triggers", "TriggerTypes",
        ]);
        assert!(world.calls.iter().all(|c| c.2 == false || c.0 == "ChronalVortex"));
    }

    #[test]
    fn trigger_type_removal_recursively_detaches_dependents_before_delete() {
        let mut world = FakeWorld::default();
        world.legal.extend([20, 30]);
        world.trigger_types.insert(20);
        world.triggers.push(Trigger { class_target: 20, target: 30 });
        detach_this_from_all(&mut world, 20, true);
        assert!(world.triggers.is_empty());
        assert!(world.calls.iter().any(|c| c.0 == "Houses" && c.1 == 30 && c.2));
        let first_trigger_detach = world.calls.iter().position(|c| c.0 == "Triggers" && c.1 == 20).unwrap();
        let recursive_map = world.calls.iter().position(|c| c.0 == "Map" && c.1 == 30).unwrap();
        assert!(recursive_map < first_trigger_detach);
    }

    #[test]
    fn recursive_cycles_are_safely_visited_once() {
        let mut world = FakeWorld::default();
        world.legal.insert(7);
        world.trigger_types.insert(7);
        world.triggers.push(Trigger { class_target: 7, target: 7 });
        detach_this_from_all(&mut world, 7, true);
        assert_eq!(world.calls.iter().filter(|c| c.0 == "Houses" && c.1 == 7).count(), 1);
        assert!(world.triggers.is_empty());
    }
}
