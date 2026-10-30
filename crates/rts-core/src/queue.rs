//! Fixed-capacity circular queue ported from REDALERT/QUEUE.H.
//!
//! The source QueueClass requires a power-of-two capacity and uses masked
//! indices. Add fails when full, Next drops one head item, First addresses the
//! oldest item, and logical indexing starts from Head regardless of wrap.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueError {
    ZeroCapacity,
    CapacityIsNotPowerOfTwo(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueueClass<T, const SIZE: usize> {
    array: [T; SIZE],
    count: usize,
    head: usize,
    tail: usize,
}

impl<T: Clone + Default, const SIZE: usize> QueueClass<T, SIZE> {
    pub fn try_new() -> Result<Self, QueueError> {
        if SIZE == 0 {
            return Err(QueueError::ZeroCapacity);
        }
        if !SIZE.is_power_of_two() {
            return Err(QueueError::CapacityIsNotPowerOfTwo(SIZE));
        }
        Ok(Self {
            array: core::array::from_fn(|_| T::default()),
            count: 0,
            head: 0,
            tail: 0,
        })
    }

    /// The original template requires a power-of-two capacity.
    pub fn new() -> Self {
        Self::try_new().expect("QueueClass capacity must be a nonzero power of two")
    }

    pub const fn count(&self) -> usize { self.count }
    pub const fn capacity(&self) -> usize { SIZE }
    pub const fn is_empty(&self) -> bool { self.count == 0 }
    pub const fn is_full(&self) -> bool { self.count == SIZE }
    pub const fn get_head(&self) -> usize { self.head }
    pub const fn get_tail(&self) -> usize { self.tail }

    /// Port of Init: reset counters without clearing the backing array.
    pub fn init(&mut self) {
        self.count = 0;
        self.head = 0;
        self.tail = 0;
    }

    /// Port of Add. Full queues reject the new value without mutation.
    pub fn add(&mut self, value: T) -> bool {
        if self.count >= SIZE {
            return false;
        }
        self.array[self.tail] = value;
        self.tail = (self.tail + 1) & (SIZE - 1);
        self.count += 1;
        true
    }

    /// Port of Next. Returns the number of entries remaining.
    pub fn next(&mut self) -> usize {
        if self.count != 0 {
            self.head = (self.head + 1) & (SIZE - 1);
            self.count -= 1;
        }
        self.count
    }

    /// Safe counterpart of First, which is undefined on an empty C++ queue.
    pub fn first(&self) -> Option<&T> {
        if self.count == 0 { None } else { Some(&self.array[self.head]) }
    }

    pub fn first_mut(&mut self) -> Option<&mut T> {
        if self.count == 0 { None } else { Some(&mut self.array[self.head]) }
    }

    /// Logical queue indexing: zero is oldest and count-1 is newest.
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.count {
            return None;
        }
        Some(&self.array[(self.head + index) & (SIZE - 1)])
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.count {
            return None;
        }
        let physical = (self.head + index) & (SIZE - 1);
        Some(&mut self.array[physical])
    }

    /// Access the physical array in the same order as the upstream Get_Array.
    /// Logical order is not guaranteed to begin at physical slot zero.
    pub fn physical_array(&self) -> &[T; SIZE] {
        &self.array
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = &T> {
        (0..self.count).map(|index| {
            &self.array[(self.head + index) & (SIZE - 1)]
        })
    }
}

impl<T: Clone + Default, const SIZE: usize> Default for QueueClass<T, SIZE> {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_rejects_non_power_of_two_capacity() {
        assert_eq!(QueueClass::<u8, 0>::try_new(), Err(QueueError::ZeroCapacity));
        assert_eq!(QueueClass::<u8, 3>::try_new(), Err(QueueError::CapacityIsNotPowerOfTwo(3)));
        assert!(QueueClass::<u8, 8>::try_new().is_ok());
    }

    #[test]
    fn add_and_next_preserve_fifo_order() {
        let mut queue = QueueClass::<i32, 4>::new();
        assert!(queue.add(10));
        assert!(queue.add(20));
        assert!(queue.add(30));
        assert_eq!(queue.count(), 3);
        assert_eq!(queue.first(), Some(&10));
        assert_eq!(queue.get(1), Some(&20));
        assert_eq!(queue.get(2), Some(&30));
        assert_eq!(queue.next(), 2);
        assert_eq!(queue.first(), Some(&20));
    }

    #[test]
    fn full_queue_rejects_new_items_without_overwriting_oldest() {
        let mut queue = QueueClass::<i32, 2>::new();
        assert!(queue.add(1));
        assert!(queue.add(2));
        assert!(!queue.add(3));
        assert_eq!(queue.iter().copied().collect::<Vec<_>>(), vec![1, 2]);
    }

    #[test]
    fn masked_indices_wrap_across_the_end_of_the_backing_array() {
        let mut queue = QueueClass::<i32, 4>::new();
        for value in 1..=4 { assert!(queue.add(value)); }
        assert_eq!(queue.next(), 3);
        assert_eq!(queue.next(), 2);
        assert!(queue.add(5));
        assert!(queue.add(6));
        assert_eq!(queue.get_head(), 2);
        assert_eq!(queue.get_tail(), 2);
        assert_eq!(queue.iter().copied().collect::<Vec<_>>(), vec![3, 4, 5, 6]);
    }

    #[test]
    fn init_resets_count_and_indices() {
        let mut queue = QueueClass::<i32, 8>::new();
        queue.add(100);
        queue.add(200);
        queue.init();
        assert_eq!(queue.count(), 0);
        assert_eq!(queue.get_head(), 0);
        assert_eq!(queue.get_tail(), 0);
        assert_eq!(queue.first(), None);
    }

    #[test]
    fn logical_mutation_and_out_of_range_access() {
        let mut queue = QueueClass::<i32, 4>::new();
        queue.add(1);
        queue.add(2);
        *queue.get_mut(1).unwrap() = 22;
        assert_eq!(queue.get(1), Some(&22));
        assert_eq!(queue.get(2), None);
        assert_eq!(queue.next(), 1);
        assert_eq!(queue.first(), Some(&22));
        assert_eq!(queue.next(), 0);
        assert_eq!(queue.next(), 0);
    }
}
