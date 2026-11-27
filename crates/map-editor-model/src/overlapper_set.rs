//! Source-derived rectangle overlap index from the pinned C# map editor.
//! Unlike occupancy, overlapping rectangles are permitted. Queries compute transitive closure.
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OverlapperId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OverlapRect { pub x: i32, pub y: i32, pub width: i32, pub height: i32 }

impl OverlapRect {
    pub const fn new(x: i32, y: i32, width: i32, height: i32) -> Self { Self { x, y, width, height } }
    pub const fn is_valid(self) -> bool { self.width > 0 && self.height > 0 }
    fn right(self) -> i64 { i64::from(self.x) + i64::from(self.width) }
    fn bottom(self) -> i64 { i64::from(self.y) + i64::from(self.height) }
    /// Half-open rectangle intersection, as used by System.Drawing.Rectangle.
    pub fn intersects(self, other: Self) -> bool {
        self.is_valid() && other.is_valid() && i64::from(self.x) < other.right()
            && i64::from(other.x) < self.right() && i64::from(self.y) < other.bottom()
            && i64::from(other.y) < self.bottom()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OverlapperError { DuplicateObject(OverlapperId), InvalidBounds(OverlapperId) }

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OverlapperSet { rectangles: BTreeMap<OverlapperId, OverlapRect> }

impl OverlapperSet {
    pub fn new() -> Self { Self::default() }
    pub fn len(&self) -> usize { self.rectangles.len() }
    pub fn is_empty(&self) -> bool { self.rectangles.is_empty() }
    pub fn contains(&self, id: OverlapperId) -> bool { self.rectangles.contains_key(&id) }
    pub fn bounds(&self, id: OverlapperId) -> Option<OverlapRect> { self.rectangles.get(&id).copied() }
    pub fn add(&mut self, id: OverlapperId, bounds: OverlapRect) -> Result<(), OverlapperError> {
        if self.rectangles.contains_key(&id) { return Err(OverlapperError::DuplicateObject(id)); }
        if !bounds.is_valid() { return Err(OverlapperError::InvalidBounds(id)); }
        self.rectangles.insert(id, bounds);
        Ok(())
    }
    pub fn remove(&mut self, id: OverlapperId) -> bool { self.rectangles.remove(&id).is_some() }
    pub fn clear(&mut self) { self.rectangles.clear(); }

    /// Returns every object connected to query through one or more rectangle intersections.
    /// Sorted IDs make results deterministic and independent of insertion order.
    pub fn overlapping_closure(&self, query: OverlapRect) -> Vec<OverlapperId> {
        if !query.is_valid() { return Vec::new(); }
        let mut included = BTreeSet::new();
        let mut frontier = vec![query];
        while let Some(current) = frontier.pop() {
            for (&id, &bounds) in &self.rectangles {
                if !included.contains(&id) && current.intersects(bounds) {
                    included.insert(id);
                    frontier.push(bounds);
                }
            }
        }
        included.into_iter().collect()
    }
    pub fn objects(&self) -> impl ExactSizeIterator<Item = OverlapperId> + '_ {
        self.rectangles.keys().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn intersection_uses_half_open_edges() {
        let a = OverlapRect::new(0, 0, 2, 2);
        assert!(a.intersects(OverlapRect::new(1, 1, 2, 2)));
        assert!(!a.intersects(OverlapRect::new(2, 0, 1, 1)));
        assert!(!a.intersects(OverlapRect::new(0, 0, 0, 2)));
    }
    #[test]
    fn query_expands_transitively() {
        let mut set = OverlapperSet::new();
        set.add(OverlapperId(3), OverlapRect::new(0, 0, 2, 2)).unwrap();
        set.add(OverlapperId(1), OverlapRect::new(1, 0, 3, 2)).unwrap();
        set.add(OverlapperId(2), OverlapRect::new(3, 0, 2, 2)).unwrap();
        set.add(OverlapperId(4), OverlapRect::new(20, 20, 1, 1)).unwrap();
        assert_eq!(set.overlapping_closure(OverlapRect::new(0, 0, 1, 1)),
            vec![OverlapperId(1), OverlapperId(2), OverlapperId(3)]);
    }
    #[test]
    fn permits_overlap_and_rejects_bad_bounds_or_duplicate_ids() {
        let mut set = OverlapperSet::new();
        set.add(OverlapperId(1), OverlapRect::new(0, 0, 3, 3)).unwrap();
        set.add(OverlapperId(2), OverlapRect::new(1, 1, 3, 3)).unwrap();
        assert_eq!(set.add(OverlapperId(3), OverlapRect::new(0, 0, 0, 1)), Err(OverlapperError::InvalidBounds(OverlapperId(3))));
        assert_eq!(set.add(OverlapperId(1), OverlapRect::new(5, 5, 1, 1)), Err(OverlapperError::DuplicateObject(OverlapperId(1))));
    }
    #[test]
    fn remove_and_clear_update_index() {
        let mut set = OverlapperSet::new();
        set.add(OverlapperId(9), OverlapRect::new(-5, -5, 2, 2)).unwrap();
        assert!(set.remove(OverlapperId(9)));
        assert!(set.overlapping_closure(OverlapRect::new(-5, -5, 2, 2)).is_empty());
        set.add(OverlapperId(2), OverlapRect::new(0, 0, 1, 1)).unwrap();
        set.clear();
        assert!(set.is_empty());
    }
}
