//! Cargo hold linked-list behavior from REDALERT/CARGO.CPP and TIBERIANDAWN/CARGO.CPP.

use std::collections::{HashMap, HashSet};

pub type CargoObjectId = u64;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CargoObject {
    pub id: CargoObjectId,
    pub next: Option<CargoObjectId>,
    pub in_limbo: bool,
}

impl CargoObject {
    pub const fn new(id: CargoObjectId) -> Self {
        Self { id, next: None, in_limbo: false }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CargoError {
    UnknownObject(CargoObjectId),
    ExistingCycle(CargoObjectId),
    ChainCycle(CargoObjectId),
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CargoWorld {
    objects: HashMap<CargoObjectId, CargoObject>,
    limbo_calls: Vec<CargoObjectId>,
}

impl CargoWorld {
    pub fn insert_object(&mut self, id: CargoObjectId) -> bool {
        self.objects.insert(id, CargoObject::new(id)).is_none()
    }

    pub fn insert(&mut self, object: CargoObject) -> Option<CargoObject> {
        self.objects.insert(object.id, object)
    }

    pub fn get(&self, id: CargoObjectId) -> Option<&CargoObject> { self.objects.get(&id) }
    pub fn limbo_calls(&self) -> &[CargoObjectId] { &self.limbo_calls }

    pub fn set_next(&mut self, id: CargoObjectId, next: Option<CargoObjectId>) -> Result<(), CargoError> {
        if !self.objects.contains_key(&id) { return Err(CargoError::UnknownObject(id)); }
        if let Some(next_id) = next {
            if !self.objects.contains_key(&next_id) { return Err(CargoError::UnknownObject(next_id)); }
        }
        self.objects.get_mut(&id).expect("validated object").next = next;
        Ok(())
    }

    fn limbo(&mut self, id: CargoObjectId) -> Result<(), CargoError> {
        let object = self.objects.get_mut(&id).ok_or(CargoError::UnknownObject(id))?;
        object.in_limbo = true;
        self.limbo_calls.push(id);
        Ok(())
    }

    fn chain_tail(&self, head: CargoObjectId) -> Result<CargoObjectId, CargoError> {
        let mut current = head;
        let mut seen = HashSet::new();
        loop {
            if !seen.insert(current) { return Err(CargoError::ExistingCycle(current)); }
            let object = self.objects.get(&current).ok_or(CargoError::UnknownObject(current))?;
            match object.next {
                Some(next) => current = next,
                None => return Ok(current),
            }
        }
    }

    fn chain_quantity(&self, head: Option<CargoObjectId>) -> Result<u8, CargoError> {
        let mut current = head;
        let mut seen = HashSet::new();
        let mut quantity = 0u8;
        while let Some(id) = current {
            if !seen.insert(id) { return Err(CargoError::ChainCycle(id)); }
            let object = self.objects.get(&id).ok_or(CargoError::UnknownObject(id))?;
            quantity = quantity.wrapping_add(1);
            current = object.next;
        }
        Ok(quantity)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CargoClass {
    quantity: u8,
    cargo_hold: Option<CargoObjectId>,
}

impl CargoClass {
    pub const fn new() -> Self { Self { quantity: 0, cargo_hold: None } }
    pub const fn how_many(&self) -> u8 { self.quantity }
    pub const fn is_something_attached(&self) -> bool { self.cargo_hold.is_some() }
    pub const fn attached_object(&self) -> Option<CargoObjectId> { self.cargo_hold }

    /// Attach calls Limbo, preserves the incoming chain, appends the previous
    /// hold at its tail, installs the new head, and recounts the byte Quantity.
    pub fn attach(&mut self, world: &mut CargoWorld, object: Option<CargoObjectId>) -> Result<(), CargoError> {
        let Some(id) = object else { return Ok(()); };
        world.limbo(id)?;
        let tail = world.chain_tail(id)?;
        if let Some(previous) = self.cargo_hold { world.set_next(tail, Some(previous))?; }
        self.cargo_hold = Some(id);
        self.quantity = world.chain_quantity(self.cargo_hold)?;
        Ok(())
    }

    /// Detach_Object removes the head, clears Next, and decrements Quantity.
    pub fn detach_object(&mut self, world: &mut CargoWorld) -> Result<Option<CargoObjectId>, CargoError> {
        let Some(head) = self.attached_object() else { return Ok(None); };
        let next = world.get(head).ok_or(CargoError::UnknownObject(head))?.next;
        self.cargo_hold = next;
        world.set_next(head, None)?;
        self.quantity = self.quantity.wrapping_sub(1);
        Ok(Some(head))
    }

    /// The C++ destructor only nulls CargoHold; cargo objects are not deleted.
    pub fn clear_hold(&mut self) { self.cargo_hold = None; }

    pub fn validate(&self, world: &CargoWorld) -> Result<bool, CargoError> {
        Ok(world.chain_quantity(self.cargo_hold)? == self.quantity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attach_is_lifo_and_calls_limbo() {
        let mut world = CargoWorld::default();
        for id in 1..=3 { world.insert_object(id); }
        let mut cargo = CargoClass::new();
        for id in 1..=3 { cargo.attach(&mut world, Some(id)).unwrap(); }
        assert_eq!(cargo.attached_object(), Some(3));
        assert_eq!(cargo.how_many(), 3);
        assert_eq!(world.get(3).unwrap().next, Some(2));
        assert_eq!(world.get(2).unwrap().next, Some(1));
        assert_eq!(world.limbo_calls(), &[1, 2, 3]);
        assert!(cargo.validate(&world).unwrap());
    }

    #[test]
    fn attach_preserves_an_existing_incoming_chain() {
        let mut world = CargoWorld::default();
        for id in 1..=3 { world.insert_object(id); }
        world.set_next(2, Some(3)).unwrap();
        let mut cargo = CargoClass::new();
        cargo.attach(&mut world, Some(1)).unwrap();
        cargo.attach(&mut world, Some(2)).unwrap();
        assert_eq!(world.get(2).unwrap().next, Some(3));
        assert_eq!(world.get(3).unwrap().next, Some(1));
        assert_eq!(cargo.how_many(), 3);
        assert!(cargo.validate(&world).unwrap());
    }

    #[test]
    fn detach_returns_head_and_separates_its_next_link() {
        let mut world = CargoWorld::default();
        for id in 1..=3 { world.insert_object(id); }
        let mut cargo = CargoClass::new();
        for id in 1..=3 { cargo.attach(&mut world, Some(id)).unwrap(); }
        assert_eq!(cargo.detach_object(&mut world).unwrap(), Some(3));
        assert_eq!(world.get(3).unwrap().next, None);
        assert_eq!(cargo.attached_object(), Some(2));
        assert_eq!(cargo.how_many(), 2);
    }

    #[test]
    fn null_and_empty_operations_are_no_ops() {
        let mut world = CargoWorld::default();
        let mut cargo = CargoClass::new();
        cargo.attach(&mut world, None).unwrap();
        assert_eq!(cargo.detach_object(&mut world).unwrap(), None);
        assert_eq!(cargo.how_many(), 0);
    }

    #[test]
    fn invalid_and_cyclic_chains_are_rejected() {
        let mut world = CargoWorld::default();
        world.insert_object(1);
        world.insert_object(2);
        world.set_next(2, Some(2)).unwrap();
        let mut cargo = CargoClass::new();
        cargo.attach(&mut world, Some(1)).unwrap();
        assert_eq!(cargo.attach(&mut world, Some(2)), Err(CargoError::ExistingCycle(2)));
        assert_eq!(cargo.attached_object(), Some(1));
        assert!(matches!(cargo.attach(&mut world, Some(99)), Err(CargoError::UnknownObject(99))));
    }

    #[test]
    fn quantity_retains_unsigned_byte_wrapping() {
        let mut world = CargoWorld::default();
        for id in 0..=255 { world.insert_object(id); }
        let mut cargo = CargoClass::new();
        for id in 0..=255 { cargo.attach(&mut world, Some(id)).unwrap(); }
        assert_eq!(cargo.how_many(), 0);
        assert!(cargo.is_something_attached());
        assert_eq!(cargo.validate(&world), Ok(true));
    }
}
