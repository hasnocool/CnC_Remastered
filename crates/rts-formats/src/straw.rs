//! Pull-oriented data-chain primitives ported from REDALERT/STRAW.H and STRAW.CPP.
//!
//! A Straw is a demand-driven chain: a consumer asks the last stage for bytes;
//! each base stage forwards that request to its upstream ChainTo node. Unlike
//! Pipe, the terminal base Straw returns zero bytes when it has no upstream.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// Error returned when a Straw link would introduce a cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrawLinkError {
    Cycle,
}

/// One stage in a demand-driven input chain.
pub trait StrawStage {
    fn get(&mut self, destination: &mut [u8], upstream: Option<&StrawHandle>) -> usize {
        match upstream {
            Some(source) => source.get(destination),
            None => 0,
        }
    }
}

struct StrawNode {
    stage: RefCell<Box<dyn StrawStage>>,
    chain_to: RefCell<Option<StrawHandle>>,
    chain_from: RefCell<Weak<StrawNode>>,
}

/// Cloneable handle to a pull-stream stage.
#[derive(Clone)]
pub struct StrawHandle {
    node: Rc<StrawNode>,
}

impl std::fmt::Debug for StrawHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StrawHandle")
            .field("has_upstream", &self.node.chain_to.borrow().is_some())
            .field("has_downstream", &self.node.chain_from.borrow().strong_count() > 0)
            .finish()
    }
}

impl StrawHandle {
    pub fn new(stage: impl StrawStage + 'static) -> Self {
        Self {
            node: Rc::new(StrawNode {
                stage: RefCell::new(Box::new(stage)),
                chain_to: RefCell::new(None),
                chain_from: RefCell::new(Weak::new()),
            }),
        }
    }

    /// Fetch bytes from this stage, which may request data from its upstream.
    pub fn get(&self, destination: &mut [u8]) -> usize {
        if destination.is_empty() {
            return 0;
        }
        let upstream = self.node.chain_to.borrow().clone();
        self.node.stage.borrow_mut().get(destination, upstream.as_ref()).min(destination.len())
    }

    pub fn upstream(&self) -> Option<Self> {
        self.node.chain_to.borrow().clone()
    }

    pub fn downstream(&self) -> Option<Self> {
        self.node.chain_from.borrow().upgrade().map(|node| Self { node })
    }

    /// Port of Straw::Get_From.
    ///
    /// If a new source is already attached to a different consumer, that old
    /// consumer is detached first. Replacing the current source does not flush.
    pub fn get_from(&self, source: Option<Self>) -> Result<(), StrawLinkError> {
        if let Some(next) = source.as_ref() {
            if Rc::ptr_eq(&self.node, &next.node) {
                return Err(StrawLinkError::Cycle);
            }
            let mut cursor = Some(next.clone());
            while let Some(current) = cursor {
                if Rc::ptr_eq(&self.node, &current.node) {
                    return Err(StrawLinkError::Cycle);
                }
                cursor = current.node.chain_to.borrow().clone();
            }
        }

        let current = self.node.chain_to.borrow().clone();
        match (&current, &source) {
            (Some(old), Some(new)) if Rc::ptr_eq(&old.node, &new.node) => return Ok(()),
            (None, None) => return Ok(()),
            _ => {}
        }

        if let Some(next) = source.as_ref() {
            if let Some(previous_consumer) = next.node.chain_from.borrow().upgrade() {
                StrawHandle { node: previous_consumer }.get_from(None)?;
            }
        }

        if let Some(old) = self.node.chain_to.borrow_mut().take() {
            *old.node.chain_from.borrow_mut() = Weak::new();
        }

        if let Some(next) = source {
            *self.node.chain_to.borrow_mut() = Some(next.clone());
            *next.node.chain_from.borrow_mut() = Rc::downgrade(&self.node);
        }

        Ok(())
    }

    /// Remove this stage while preserving the chain on either side.
    pub fn unlink(&self) -> Result<(), StrawLinkError> {
        let previous_consumer = self.downstream();
        let upstream = self.upstream();

        if let Some(upstream) = upstream.as_ref() {
            *upstream.node.chain_from.borrow_mut() = Weak::new();
        }
        *self.node.chain_to.borrow_mut() = None;
        *self.node.chain_from.borrow_mut() = Weak::new();

        if let Some(consumer) = previous_consumer {
            consumer.get_from(upstream)?;
        }

        Ok(())
    }
}

/// Empty base endpoint: a base Straw without ChainTo returns zero bytes.
#[derive(Debug, Default, Clone, Copy)]
pub struct EmptyStraw;

impl StrawStage for EmptyStraw {}

/// A finite byte source useful for loaders, tests, and asset decoders.
#[derive(Debug, Clone)]
pub struct SliceStraw {
    bytes: Vec<u8>,
    offset: usize,
}

impl SliceStraw {
    pub fn new(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            bytes: bytes.into(),
            offset: 0,
        }
    }

    pub const fn position(&self) -> usize {
        self.offset
    }

    pub fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.offset)
    }
}

impl StrawStage for SliceStraw {
    fn get(&mut self, destination: &mut [u8], _upstream: Option<&StrawHandle>) -> usize {
        let count = destination.len().min(self.remaining());
        destination[..count].copy_from_slice(&self.bytes[self.offset..self.offset + count]);
        self.offset += count;
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_straw_without_source_returns_eof() {
        let straw = StrawHandle::new(EmptyStraw);
        let mut destination = [0; 8];
        assert_eq!(straw.get(&mut destination), 0);
    }

    #[test]
    fn consumer_fetches_from_linked_upstream_source() {
        let source = StrawHandle::new(SliceStraw::new(b"westwood".to_vec()));
        let consumer = StrawHandle::new(EmptyStraw);
        consumer.get_from(Some(source.clone())).unwrap();

        let mut first = [0; 3];
        assert_eq!(consumer.get(&mut first), 3);
        assert_eq!(&first, b"wes");

        let mut second = [0; 8];
        assert_eq!(consumer.get(&mut second), 5);
        assert_eq!(&second[..5], b"twood");
        assert_eq!(consumer.get(&mut second), 0);
    }

    #[test]
    fn source_can_only_have_one_downstream_consumer() {
        let source = StrawHandle::new(SliceStraw::new(b"data".to_vec()));
        let first = StrawHandle::new(EmptyStraw);
        let second = StrawHandle::new(EmptyStraw);

        first.get_from(Some(source.clone())).unwrap();
        second.get_from(Some(source.clone())).unwrap();

        assert!(first.upstream().is_none());
        assert_eq!(source.downstream().unwrap().get(&mut [0; 4]), 4);
    }

    #[test]
    fn replacing_upstream_does_not_flush_or_consume_it() {
        let old = StrawHandle::new(SliceStraw::new(b"old".to_vec()));
        let new = StrawHandle::new(SliceStraw::new(b"new".to_vec()));
        let consumer = StrawHandle::new(EmptyStraw);

        consumer.get_from(Some(old.clone())).unwrap();
        consumer.get_from(Some(new.clone())).unwrap();

        assert!(old.downstream().is_none());
        let mut destination = [0; 8];
        assert_eq!(consumer.get(&mut destination), 3);
        assert_eq!(&destination[..3], b"new");
    }

    #[test]
    fn cycles_are_rejected() {
        let first = StrawHandle::new(EmptyStraw);
        let second = StrawHandle::new(EmptyStraw);
        first.get_from(Some(second.clone())).unwrap();
        assert_eq!(second.get_from(Some(first.clone())), Err(StrawLinkError::Cycle));
    }

    #[test]
    fn unlink_reconnects_consumer_to_upstream_source() {
        let source = StrawHandle::new(SliceStraw::new(b"stream".to_vec()));
        let middle = StrawHandle::new(EmptyStraw);
        let consumer = StrawHandle::new(EmptyStraw);
        middle.get_from(Some(source.clone())).unwrap();
        consumer.get_from(Some(middle.clone())).unwrap();

        middle.unlink().unwrap();

        assert!(consumer.upstream().is_some());
        let mut destination = [0; 8];
        assert_eq!(consumer.get(&mut destination), 6);
        assert_eq!(&destination[..6], b"stream");
    }
}
