//! Intrusive doubly linked-list behavior ported from REDALERT/LINK.H and LINK.CPP.
//!
//! The original embeds raw Next/Prev pointers into each element. This Rust
//! version stores values in a stable index arena and uses LinkId handles,
//! retaining insertion-after, add-head/tail, traversal, and unlink behavior.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LinkId(usize);

impl LinkId {
    pub const fn index(self) -> usize { self.0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkError {
    InvalidLink(LinkId),
    AlreadyLinked(LinkId),
    SelfInsertion(LinkId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LinkNode<T> {
    value: T,
    next: Option<LinkId>,
    previous: Option<LinkId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkArena<T> {
    nodes: Vec<LinkNode<T>>,
}

impl<T> Default for LinkArena<T> {
    fn default() -> Self { Self::new() }
}

impl<T> LinkArena<T> {
    pub const fn new() -> Self { Self { nodes: Vec::new() } }

    pub fn insert(&mut self, value: T) -> LinkId {
        let id = LinkId(self.nodes.len());
        self.nodes.push(LinkNode { value, next: None, previous: None });
        id
    }

    pub fn len(&self) -> usize { self.nodes.len() }
    pub fn is_empty(&self) -> bool { self.nodes.is_empty() }

    fn node(&self, id: LinkId) -> Result<&LinkNode<T>, LinkError> {
        self.nodes.get(id.0).ok_or(LinkError::InvalidLink(id))
    }

    fn node_mut(&mut self, id: LinkId) -> Result<&mut LinkNode<T>, LinkError> {
        self.nodes.get_mut(id.0).ok_or(LinkError::InvalidLink(id))
    }

    pub fn value(&self, id: LinkId) -> Option<&T> {
        self.nodes.get(id.0).map(|node| &node.value)
    }

    pub fn value_mut(&mut self, id: LinkId) -> Option<&mut T> {
        self.nodes.get_mut(id.0).map(|node| &mut node.value)
    }

    pub fn get_next(&self, id: LinkId) -> Option<LinkId> {
        self.nodes.get(id.0).and_then(|node| node.next)
    }

    pub fn get_previous(&self, id: LinkId) -> Option<LinkId> {
        self.nodes.get(id.0).and_then(|node| node.previous)
    }

    pub fn is_linked(&self, id: LinkId) -> bool {
        self.nodes.get(id.0).is_some_and(|node| node.next.is_some() || node.previous.is_some())
    }

    /// Walk backward to the head of a list.
    pub fn head_of_list(&self, id: LinkId) -> Result<LinkId, LinkError> {
        let mut current = id;
        let mut remaining = self.nodes.len().saturating_add(1);
        while let Some(previous) = self.node(current)?.previous {
            if previous == id || remaining == 0 { break; }
            current = previous;
            remaining -= 1;
        }
        Ok(current)
    }

    /// Walk forward to the tail of a list.
    pub fn tail_of_list(&self, id: LinkId) -> Result<LinkId, LinkError> {
        let mut current = id;
        let mut remaining = self.nodes.len().saturating_add(1);
        while let Some(next) = self.node(current)?.next {
            if next == id || remaining == 0 { break; }
            current = next;
            remaining -= 1;
        }
        Ok(current)
    }

    /// Add this node immediately after the supplied list node.
    pub fn add_after(&mut self, this: LinkId, list: LinkId) -> Result<LinkId, LinkError> {
        self.node(this)?;
        self.node(list)?;
        if this == list { return Err(LinkError::SelfInsertion(this)); }
        if self.is_linked(this) { return Err(LinkError::AlreadyLinked(this)); }

        let next = self.node(list)?.next;
        self.node_mut(list)?.next = Some(this);
        {
            let node = self.node_mut(this)?;
            node.previous = Some(list);
            node.next = next;
        }
        if let Some(next) = next {
            self.node_mut(next)?.previous = Some(this);
        }
        self.head_of_list(this)
    }

    /// Make this node the new head of the supplied list.
    pub fn add_head(&mut self, this: LinkId, list: LinkId) -> Result<LinkId, LinkError> {
        self.node(this)?;
        self.node(list)?;
        if this == list { return Err(LinkError::SelfInsertion(this)); }
        if self.is_linked(this) { return Err(LinkError::AlreadyLinked(this)); }

        let head = self.head_of_list(list)?;
        self.node_mut(head)?.previous = Some(this);
        {
            let node = self.node_mut(this)?;
            node.previous = None;
            node.next = Some(head);
        }
        Ok(this)
    }

    /// Append this node to the supplied list.
    pub fn add_tail(&mut self, this: LinkId, list: LinkId) -> Result<LinkId, LinkError> {
        self.node(this)?;
        self.node(list)?;
        if this == list { return Err(LinkError::SelfInsertion(this)); }
        if self.is_linked(this) { return Err(LinkError::AlreadyLinked(this)); }

        let tail = self.tail_of_list(list)?;
        self.node_mut(tail)?.next = Some(this);
        {
            let node = self.node_mut(this)?;
            node.previous = Some(tail);
            node.next = None;
        }
        self.head_of_list(this)
    }

    /// Remove this node and reconnect its neighbors. Returns the new head, or
    /// None when the removed node was the only member of its list.
    pub fn remove(&mut self, this: LinkId) -> Result<Option<LinkId>, LinkError> {
        self.node(this)?;
        let head = self.head_of_list(this)?;
        let tail = self.tail_of_list(this)?;
        let previous = self.node(this)?.previous;
        let next = self.node(this)?.next;

        if let Some(previous) = previous { self.node_mut(previous)?.next = next; }
        if let Some(next) = next { self.node_mut(next)?.previous = previous; }
        {
            let node = self.node_mut(this)?;
            node.previous = None;
            node.next = None;
        }

        if head == this {
            if tail == this { Ok(None) } else { Ok(Some(self.head_of_list(tail)?)) }
        } else {
            Ok(Some(head))
        }
    }

    /// Clear pointers without changing any neighboring node; like source Zap,
    /// this is a low-level operation intended only for known-detached storage.
    pub fn zap(&mut self, this: LinkId) -> Result<(), LinkError> {
        let node = self.node_mut(this)?;
        node.next = None;
        node.previous = None;
        Ok(())
    }

    /// Equivalent of copy/assignment list placement: remove this node from its
    /// old list and insert it immediately after the source node.
    pub fn assign_list_position(&mut self, this: LinkId, source: LinkId) -> Result<LinkId, LinkError> {
        self.node(this)?;
        self.node(source)?;
        if this == source { return self.head_of_list(this); }
        if self.is_linked(this) { self.remove(this)?; }
        self.add_after(this, source)
    }

    pub fn iter_from(&self, start: LinkId) -> Result<LinkIter<'_, T>, LinkError> {
        self.node(start)?;
        Ok(LinkIter { arena: self, next: Some(start), remaining: self.nodes.len().saturating_add(1) })
    }

    pub fn list_values(&self, start: LinkId) -> Result<Vec<&T>, LinkError> {
        let head = self.head_of_list(start)?;
        Ok(self.iter_from(head)?.map(|id| &self.nodes[id.0].value).collect())
    }
}

pub struct LinkIter<'a, T> {
    arena: &'a LinkArena<T>,
    next: Option<LinkId>,
    remaining: usize,
}

impl<T> Iterator for LinkIter<'_, T> {
    type Item = LinkId;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;
        if self.remaining == 0 {
            self.next = None;
            return None;
        }
        self.remaining -= 1;
        self.next = self.arena.nodes.get(current.0).and_then(|node| node.next);
        Some(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_after_preserves_doubly_linked_order() {
        let mut list = LinkArena::new();
        let a = list.insert("a");
        let b = list.insert("b");
        let c = list.insert("c");
        list.add_after(b, a).unwrap();
        list.add_after(c, b).unwrap();
        assert_eq!(list.head_of_list(c).unwrap(), a);
        assert_eq!(list.tail_of_list(a).unwrap(), c);
        assert_eq!(list.get_next(a), Some(b));
        assert_eq!(list.get_previous(c), Some(b));
        assert_eq!(list.list_values(b).unwrap(), vec![&"a", &"b", &"c"]);
    }

    #[test]
    fn add_head_and_tail_match_source_insertion_rules() {
        let mut list = LinkArena::new();
        let a = list.insert("a");
        let b = list.insert("b");
        let c = list.insert("c");
        let d = list.insert("d");
        list.add_after(b, a).unwrap();
        assert_eq!(list.add_head(c, b).unwrap(), c);
        assert_eq!(list.add_tail(d, a).unwrap(), c);
        assert_eq!(list.list_values(a).unwrap(), vec![&"c", &"a", &"d", &"b"]);
    }

    #[test]
    fn remove_reconnects_neighbors_and_returns_new_head() {
        let mut list = LinkArena::new();
        let a = list.insert(1);
        let b = list.insert(2);
        let c = list.insert(3);
        list.add_after(b, a).unwrap();
        list.add_after(c, b).unwrap();
        assert_eq!(list.remove(b).unwrap(), Some(a));
        assert_eq!(list.list_values(a).unwrap(), vec![&1, &3]);
        assert_eq!(list.get_next(a), Some(c));
        assert_eq!(list.get_previous(c), Some(a));
        assert!(!list.is_linked(b));
    }

    #[test]
    fn removing_head_and_singleton_returns_source_compatible_results() {
        let mut list = LinkArena::new();
        let a = list.insert(1);
        let b = list.insert(2);
        list.add_after(b, a).unwrap();
        assert_eq!(list.remove(a).unwrap(), Some(b));
        assert_eq!(list.head_of_list(b).unwrap(), b);
        assert_eq!(list.remove(b).unwrap(), None);
    }

    #[test]
    fn duplicate_insertion_and_invalid_indices_are_rejected() {
        let mut list = LinkArena::new();
        let a = list.insert(1);
        let b = list.insert(2);
        list.add_after(b, a).unwrap();
        assert_eq!(list.add_after(b, a), Err(LinkError::AlreadyLinked(b)));
        assert_eq!(list.add_after(a, a), Err(LinkError::SelfInsertion(a)));
        assert_eq!(list.get_next(LinkId(99)), None);
        assert_eq!(list.remove(LinkId(99)), Err(LinkError::InvalidLink(LinkId(99))));
    }

    #[test]
    fn assignment_moves_a_node_to_the_source_list_position() {
        let mut list = LinkArena::new();
        let a = list.insert("a");
        let b = list.insert("b");
        let c = list.insert("c");
        list.add_after(b, a).unwrap();
        list.assign_list_position(c, b).unwrap();
        assert_eq!(list.list_values(a).unwrap(), vec![&"a", &"b", &"c"]);
    }
}
