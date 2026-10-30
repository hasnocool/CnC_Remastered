//! LayerClass from both REDALERT/LAYER.CPP and TIBERIANDAWN/LAYER.CPP.
//!
//! Ordering uses ObjectClass::operator< and operator>, both based on Sort_Y().
//! The legacy Sort() performs exactly one adjacent-swap pass.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LayerObject { pub id: u64, pub sort_y: i32 }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LayerVariant { RedAlert, TiberianDawn }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LayerInsertError { GrowthDisabled, CapacityExhausted, CapacityOverflow }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LayerClass {
    variant: LayerVariant,
    objects: Vec<LayerObject>,
    length: usize,
    allocated: bool,
    max_capacity: Option<usize>,
    growth_step: usize,
}

impl LayerClass {
    /// None represents the legacy zero VectorMax (unbounded maximum).
    pub fn new(
        variant: LayerVariant,
        initial_capacity: usize,
        allocated: bool,
        max_capacity: Option<usize>,
        growth_step: usize,
    ) -> Self {
        let length = max_capacity.map_or(initial_capacity, |limit| initial_capacity.min(limit));
        Self {
            variant,
            objects: Vec::with_capacity(length),
            length,
            allocated,
            max_capacity,
            growth_step,
        }
    }

    pub fn variant(&self) -> LayerVariant { self.variant }
    pub fn count(&self) -> usize { self.objects.len() }
    pub fn length(&self) -> usize { self.length }
    pub fn is_empty(&self) -> bool { self.objects.is_empty() }
    pub fn as_slice(&self) -> &[LayerObject] { &self.objects }
    pub fn get(&self, index: usize) -> Option<&LayerObject> { self.objects.get(index) }

    pub fn init(&mut self) { self.objects.clear(); }
    pub fn one_time(&mut self) {}

    fn grow_for_one(&mut self) -> Result<(), LayerInsertError> {
        if self.objects.len() < self.length { return Ok(()); }
        // Exact DynamicVectorClass condition:
        // (IsAllocated || !VectorMax) && GrowthStep > 0.
        if !(self.allocated || self.max_capacity.is_none()) || self.growth_step == 0 {
            return Err(LayerInsertError::GrowthDisabled);
        }
        let new_length = self.length.checked_add(self.growth_step)
            .ok_or(LayerInsertError::CapacityOverflow)?;
        if self.max_capacity.is_some_and(|maximum| new_length > maximum) {
            return Err(LayerInsertError::CapacityExhausted);
        }
        self.objects.reserve(new_length.saturating_sub(self.objects.capacity()));
        self.length = new_length;
        Ok(())
    }

    /// Submit(object, false) delegates to DynamicVectorClass::Add.
    pub fn submit(&mut self, object: LayerObject, sort: bool) -> Result<(), LayerInsertError> {
        if sort { self.sorted_add(object) } else { self.add(object) }
    }

    pub fn add(&mut self, object: LayerObject) -> Result<(), LayerInsertError> {
        self.grow_for_one()?;
        self.objects.push(object);
        Ok(())
    }

    /// Sorted_Add inserts before the first object whose Sort_Y is greater.
    /// Equal keys therefore retain insertion order.
    pub fn sorted_add(&mut self, object: LayerObject) -> Result<(), LayerInsertError> {
        self.grow_for_one()?;
        let index = self.objects.iter()
            .position(|existing| existing.sort_y > object.sort_y)
            .unwrap_or(self.objects.len());
        self.objects.insert(index, object);
        Ok(())
    }

    /// One legacy adjacent-swap pass, not a full sort.
    pub fn sort(&mut self) {
        if self.objects.len() < 2 { return; }
        for index in 0..self.objects.len() - 1 {
            if self.objects[index + 1].sort_y < self.objects[index].sort_y {
                self.objects.swap(index, index + 1);
            }
        }
    }
}

impl Default for LayerClass {
    fn default() -> Self {
        Self::new(LayerVariant::TiberianDawn, 0, true, None, 10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn object(id: u64, sort_y: i32) -> LayerObject { LayerObject { id, sort_y } }

    #[test]
    fn unsorted_submit_appends_and_sorted_submit_inserts() {
        let mut layer = LayerClass::new(LayerVariant::RedAlert, 4, true, Some(8), 4);
        layer.submit(object(1, 30), false).unwrap();
        layer.submit(object(2, 10), false).unwrap();
        layer.submit(object(3, 20), true).unwrap();
        assert_eq!(layer.as_slice(), &[object(2, 10), object(3, 20), object(1, 30)]);
    }

    #[test]
    fn sorted_add_is_stable_for_equal_sort_keys() {
        let mut layer = LayerClass::new(LayerVariant::TiberianDawn, 0, true, None, 2);
        layer.sorted_add(object(1, 10)).unwrap();
        layer.sorted_add(object(2, 10)).unwrap();
        layer.sorted_add(object(3, 10)).unwrap();
        assert_eq!(layer.as_slice(), &[object(1, 10), object(2, 10), object(3, 10)]);
    }

    #[test]
    fn sort_performs_one_adjacent_swap_pass() {
        let mut layer = LayerClass::new(LayerVariant::RedAlert, 5, true, Some(5), 1);
        for value in [5, 4, 3, 2, 1] { layer.add(object(value as u64, value)).unwrap(); }
        layer.sort();
        assert_eq!(layer.as_slice(), &[object(4, 4), object(3, 3), object(2, 2), object(1, 1), object(5, 5)]);
        layer.sort();
        assert_eq!(layer.as_slice(), &[object(3, 3), object(2, 2), object(1, 1), object(4, 4), object(5, 5)]);
    }

    #[test]
    fn non_allocated_fixed_maximum_rejects_growth() {
        let mut layer = LayerClass::new(LayerVariant::RedAlert, 1, false, Some(1), 1);
        layer.add(object(1, 10)).unwrap();
        assert_eq!(layer.add(object(2, 20)), Err(LayerInsertError::GrowthDisabled));
        assert_eq!(layer.count(), 1);
    }

    #[test]
    fn allocated_vectors_grow_by_step_but_respect_maximum() {
        let mut layer = LayerClass::new(LayerVariant::TiberianDawn, 2, true, Some(4), 2);
        layer.add(object(1, 1)).unwrap();
        layer.add(object(2, 2)).unwrap();
        layer.add(object(3, 3)).unwrap();
        layer.add(object(4, 4)).unwrap();
        assert_eq!(layer.length(), 4);
        assert_eq!(layer.add(object(5, 5)), Err(LayerInsertError::CapacityExhausted));
    }

    #[test]
    fn init_clears_active_objects_and_one_time_is_no_op() {
        let mut layer = LayerClass::default();
        layer.add(object(1, 2)).unwrap();
        layer.one_time();
        layer.init();
        assert!(layer.is_empty());
        assert_eq!(layer.length(), 10);
    }
}
