//! CargoClass linked-object hold from both variants' CARGO.CPP.

use std::collections::HashSet;

pub type CargoObjectId = u64;

pub trait CargoWorld {
    fn limbo(&mut self, object: CargoObjectId);
    fn next(&self, object: CargoObjectId) -> Option<CargoObjectId>;
    fn set_next(&mut self, object: CargoObjectId, next: Option<CargoObjectId>);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CargoError { CycleDetected }

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CargoClass {
    quantity: u8,
    cargo_hold: Option<CargoObjectId>,
}

impl CargoClass {
    pub const fn new() -> Self { Self { quantity: 0, cargo_hold: None } }
    pub const fn how_many(&self) -> u8 { self.quantity }
    pub const fn is_something_attached(&self) -> bool { self.cargo_hold.is_some() }
    pub const fn attached_object(&self) -> Option<CargoObjectId> { self.cargo_hold }

    /// Attach an object chain in LIFO order. The existing cargo chain is
    /// appended to the tail of the incoming object's existing Next chain.
    pub fn attach(
        &mut self, world: &mut impl CargoWorld, object: Option<CargoObjectId>,
    ) -> Result<(), CargoError> {
        let Some(object) = object else { return Ok(()) };
        world.limbo(object);

        let mut tail = object;
        let mut visited = HashSet::new();
        visited.insert(object);
        while let Some(next) = world.next(tail) {
            if !visited.insert(next) { return Err(CargoError::CycleDetected); }
            tail = next;
        }
        world.set_next(tail, self.cargo_hold);
        self.cargo_hold = Some(object);

        self.quantity = 0;
        let mut current = self.cargo_hold;
        let mut counted = HashSet::new();
        while let Some(id) = current {
            if !counted.insert(id) { return Err(CargoError::CycleDetected); }
            self.quantity = self.quantity.wrapping_add(1);
            current = world.next(id);
        }
        Ok(())
    }

    /// Detach the most recently attached object and sever its Next link.
    pub fn detach_object(&mut self, world: &mut impl CargoWorld) -> Option<CargoObjectId> {
        let unit = self.attached_object()?;
        self.cargo_hold = world.next(unit);
        world.set_next(unit, None);
        self.quantity = self.quantity.wrapping_sub(1);
        Some(unit)
    }

    /// CHEAT_KEYS Debug_Dump payload, matching (%d)%04X.
    pub fn debug_dump_text(&self) -> Option<String> {
        self.cargo_hold.map(|object| format!("({}){:04X}", self.quantity, object as u16))
    }

    /// CargoClass::~CargoClass only clears its own head pointer; it does not
    /// traverse or mutate the linked objects.
    pub fn destroy(&mut self) {
        self.cargo_hold = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[derive(Default)]
    struct FakeWorld {
        next: HashMap<CargoObjectId, Option<CargoObjectId>>,
        limboed: Vec<CargoObjectId>,
    }
    impl CargoWorld for FakeWorld {
        fn limbo(&mut self, object: CargoObjectId) { self.limboed.push(object); }
        fn next(&self, object: CargoObjectId) -> Option<CargoObjectId> {
            self.next.get(&object).copied().flatten()
        }
        fn set_next(&mut self, object: CargoObjectId, next: Option<CargoObjectId>) {
            self.next.insert(object, next);
        }
    }

    #[test]
    fn attach_preserves_incoming_chain_and_appends_existing_hold() {
        let mut world = FakeWorld::default();
        world.set_next(1, Some(2));
        world.set_next(2, None);
        world.set_next(3, None);
        let mut cargo = CargoClass::new();
        cargo.attach(&mut world, Some(3)).unwrap();
        cargo.attach(&mut world, Some(1)).unwrap();
        assert_eq!(cargo.attached_object(), Some(1));
        assert_eq!(world.next(1), Some(2));
        assert_eq!(world.next(2), Some(3));
        assert_eq!(cargo.how_many(), 3);
        assert_eq!(world.limboed, [3, 1]);
    }

    #[test]
    fn detach_returns_head_and_severs_next_pointer() {
        let mut world = FakeWorld::default();
        let mut cargo = CargoClass::new();
        cargo.attach(&mut world, Some(1)).unwrap();
        cargo.attach(&mut world, Some(2)).unwrap();
        assert_eq!(cargo.detach_object(&mut world), Some(2));
        assert_eq!(world.next(2), None);
        assert_eq!(cargo.attached_object(), Some(1));
        assert_eq!(cargo.how_many(), 1);
    }

    #[test]
    fn null_attach_and_empty_detach_are_noops() {
        let mut world = FakeWorld::default();
        let mut cargo = CargoClass::new();
        cargo.attach(&mut world, None).unwrap();
        assert_eq!(cargo.detach_object(&mut world), None);
        assert_eq!(cargo.how_many(), 0);
        assert!(!cargo.is_something_attached());
    }

    #[test]
    fn quantity_uses_unsigned_byte_wrapping() {
        let mut world = FakeWorld::default();
        let mut cargo = CargoClass::new();
        for id in 0..256 { cargo.attach(&mut world, Some(id)).unwrap(); }
        assert_eq!(cargo.how_many(), 0);
    }

    #[test]
    fn debug_payload_and_destructor_match_source() {
        let mut world = FakeWorld::default();
        let mut cargo = CargoClass::new();
        cargo.attach(&mut world, Some(0x12345)).unwrap();
        assert_eq!(cargo.debug_dump_text().as_deref(), Some("(1)2345"));
        cargo.destroy();
        assert_eq!(cargo.how_many(), 1);
        assert!(!cargo.is_something_attached());
        assert_eq!(world.next(0x12345), None);
    }

    #[test]
    fn detects_preexisting_link_cycle_instead_of_looping_forever() {
        let mut world = FakeWorld::default();
        world.set_next(1, Some(2));
        world.set_next(2, Some(1));
        let mut cargo = CargoClass::new();
        assert_eq!(cargo.attach(&mut world, Some(1)), Err(CargoError::CycleDetected));
    }
}
