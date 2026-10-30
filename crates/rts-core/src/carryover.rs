//! CarryoverClass snapshot and recreation behavior from REDALERT/CARRY.CPP.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CarryoverKind { None, Unit, Building, Infantry, Vessel, Other }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TechnoSnapshot {
    pub kind: CarryoverKind,
    pub type_id: u16,
    pub house: i32,
    pub strength: i32,
    /// Coord_Cell(techno->Coord) is evaluated by the caller and stored here.
    pub cell: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CarryoverRecord {
    pub kind: CarryoverKind,
    pub type_id: u16,
    pub cell: u16,
    pub strength: i32,
    pub house: i32,
}

impl Default for CarryoverRecord {
    fn default() -> Self {
        Self { kind: CarryoverKind::None, type_id: 0, cell: 0, strength: 0, house: -1 }
    }
}

impl CarryoverRecord {
    pub fn from_techno(techno: Option<TechnoSnapshot>) -> Self {
        let mut record = Self::default();
        if let Some(snapshot) = techno {
            record.kind = snapshot.kind;
            if matches!(snapshot.kind, CarryoverKind::Unit | CarryoverKind::Building | CarryoverKind::Infantry | CarryoverKind::Vessel) {
                record.type_id = snapshot.type_id;
            }
            record.house = snapshot.house;
            record.strength = snapshot.strength;
            record.cell = snapshot.cell;
        }
        record
    }

    /// Recreate and place the carried object. The original C++ routine
    /// performs its side effects but unconditionally returns false.
    pub fn create(&self, world: &mut impl CarryoverWorld) -> bool {
        let object = match self.kind {
            CarryoverKind::Unit => world.create_unit(self.type_id, self.house),
            CarryoverKind::Infantry => world.create_infantry(self.type_id, self.house),
            CarryoverKind::Building => world.create_building(self.type_id, self.house),
            CarryoverKind::Vessel => world.create_vessel(self.type_id, self.house),
            CarryoverKind::None | CarryoverKind::Other => None,
        };
        if let Some(object) = object {
            world.set_strength(object, self.strength);
            if self.kind == CarryoverKind::Infantry {
                let old_scenario_init = world.scenario_init();
                world.set_scenario_init(false);
                let _ = world.unlimbo(object, self.cell);
                world.set_scenario_init(old_scenario_init);
            } else {
                let _ = world.unlimbo(object, self.cell);
            }
        }
        false
    }
}

pub trait CarryoverWorld {
    type Object;
    fn create_unit(&mut self, type_id: u16, house: i32) -> Option<Self::Object>;
    fn create_infantry(&mut self, type_id: u16, house: i32) -> Option<Self::Object>;
    fn create_building(&mut self, type_id: u16, house: i32) -> Option<Self::Object>;
    fn create_vessel(&mut self, type_id: u16, house: i32) -> Option<Self::Object>;
    fn set_strength(&mut self, object: Self::Object, strength: i32);
    fn unlimbo(&mut self, object: Self::Object, cell: u16) -> bool;
    fn scenario_init(&self) -> bool;
    fn set_scenario_init(&mut self, enabled: bool);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct FakeWorld {
        calls: Vec<String>,
        scenario: bool,
        next: usize,
    }
    impl CarryoverWorld for FakeWorld {
        type Object = usize;
        fn create_unit(&mut self, id: u16, house: i32) -> Option<usize> {
            self.calls.push(format!("unit:{id}:{house}")); self.next += 1; Some(self.next)
        }
        fn create_infantry(&mut self, id: u16, house: i32) -> Option<usize> {
            self.calls.push(format!("infantry:{id}:{house}")); self.next += 1; Some(self.next)
        }
        fn create_building(&mut self, id: u16, house: i32) -> Option<usize> {
            self.calls.push(format!("building:{id}:{house}")); self.next += 1; Some(self.next)
        }
        fn create_vessel(&mut self, id: u16, house: i32) -> Option<usize> {
            self.calls.push(format!("vessel:{id}:{house}")); self.next += 1; Some(self.next)
        }
        fn set_strength(&mut self, object: usize, strength: i32) { self.calls.push(format!("strength:{object}:{strength}")); }
        fn unlimbo(&mut self, object: usize, cell: u16) -> bool { self.calls.push(format!("unlimbo:{object}:{cell}")); true }
        fn scenario_init(&self) -> bool { self.scenario }
        fn set_scenario_init(&mut self, enabled: bool) { self.scenario = enabled; self.calls.push(format!("scenario:{enabled}")); }
    }

    #[test]
    fn constructor_captures_only_supported_techno_types() {
        let record = CarryoverRecord::from_techno(Some(TechnoSnapshot {
            kind: CarryoverKind::Unit, type_id: 12, house: 3, strength: 87, cell: 0x1234,
        }));
        assert_eq!(record, CarryoverRecord {
            kind: CarryoverKind::Unit, type_id: 12, house: 3, strength: 87, cell: 0x1234,
        });
        assert_eq!(CarryoverRecord::from_techno(None), CarryoverRecord::default());
    }

    #[test]
    fn infantry_creation_temporarily_disables_scenario_init_and_restores_it() {
        let record = CarryoverRecord {
            kind: CarryoverKind::Infantry, type_id: 5, house: 2, strength: 40, cell: 99,
        };
        let mut world = FakeWorld { scenario: true, ..Default::default() };
        assert!(!record.create(&mut world));
        assert!(world.scenario);
        assert_eq!(world.calls, [
            "infantry:5:2", "strength:1:40", "scenario:false",
            "unlimbo:1:99", "scenario:true",
        ]);
    }

    #[test]
    fn non_infantry_creation_keeps_scenario_flag_and_still_returns_false() {
        let record = CarryoverRecord {
            kind: CarryoverKind::Vessel, type_id: 8, house: 4, strength: 100, cell: 77,
        };
        let mut world = FakeWorld { scenario: true, ..Default::default() };
        assert!(!record.create(&mut world));
        assert!(world.scenario);
        assert_eq!(world.calls, ["vessel:8:4", "strength:1:100", "unlimbo:1:77"]);
    }

    #[test]
    fn unsupported_type_does_not_create_an_object() {
        let record = CarryoverRecord { kind: CarryoverKind::Other, ..Default::default() };
        let mut world = FakeWorld::default();
        assert!(!record.create(&mut world));
        assert!(world.calls.is_empty());
    }
}
