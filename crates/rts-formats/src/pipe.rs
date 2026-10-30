//! Push-oriented data-chain primitives ported from REDALERT/PIPE.H and PIPE.CPP.
//!
//! The original Pipe is a linked chain: Put forwards to ChainTo, Flush forwards
//! to ChainTo, and End defaults to Flush. Replacing a link detaches the new
//! destination from its former predecessor and flushes the old downstream chain.
//! This Rust implementation models the same operations with safe shared handles.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// Error returned when a Pipe link would introduce a cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipeLinkError {
    Cycle,
}

/// One node in a push stream.
///
/// Derived stages override put/flush/end and call the supplied downstream handle
/// to preserve the original base-class forwarding behavior.
pub trait PipeStage {
    fn put(&mut self, source: &[u8], downstream: Option<&PipeHandle>) -> usize {
        match downstream {
            Some(next) => next.put(source),
            None => source.len(),
        }
    }

    fn flush(&mut self, downstream: Option<&PipeHandle>) -> usize {
        downstream.map_or(0, PipeHandle::flush)
    }

    fn end(&mut self, downstream: Option<&PipeHandle>) -> usize {
        self.flush(downstream)
    }
}

struct PipeNode {
    stage: RefCell<Box<dyn PipeStage>>,
    chain_to: RefCell<Option<PipeHandle>>,
    chain_from: RefCell<Weak<PipeNode>>,
}

/// Cloneable handle to a Pipe node.
///
/// Like the C++ object, a node may be addressed independently from the chain
/// that owns its downstream link. Handles keep linked nodes alive safely.
#[derive(Clone)]
pub struct PipeHandle {
    node: Rc<PipeNode>,
}

impl std::fmt::Debug for PipeHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipeHandle")
            .field("has_downstream", &self.node.chain_to.borrow().is_some())
            .field("has_upstream", &(self.node.chain_from.borrow().strong_count() > 0))
            .finish()
    }
}

impl PipeHandle {
    pub fn new(stage: impl PipeStage + 'static) -> Self {
        Self {
            node: Rc::new(PipeNode {
                stage: RefCell::new(Box::new(stage)),
                chain_to: RefCell::new(None),
                chain_from: RefCell::new(Weak::new()),
            }),
        }
    }

    /// Forward a byte slice through this node and the rest of its chain.
    pub fn put(&self, source: &[u8]) -> usize {
        let downstream = self.node.chain_to.borrow().clone();
        self.node.stage.borrow_mut().put(source, downstream.as_ref())
    }

    /// Flush this node and all downstream stages.
    pub fn flush(&self) -> usize {
        let downstream = self.node.chain_to.borrow().clone();
        self.node.stage.borrow_mut().flush(downstream.as_ref())
    }

    /// End the stream; the base implementation delegates to flush.
    pub fn end(&self) -> usize {
        let downstream = self.node.chain_to.borrow().clone();
        self.node.stage.borrow_mut().end(downstream.as_ref())
    }

    pub fn downstream(&self) -> Option<Self> {
        self.node.chain_to.borrow().clone()
    }

    pub fn upstream(&self) -> Option<Self> {
        self.node.chain_from.borrow().upgrade().map(|node| Self { node })
    }

    /// Port of Pipe::Put_To.
    ///
    /// A replacement destination is detached from its previous predecessor.
    /// The current downstream chain is detached and flushed before the new
    /// destination is installed. Cyclic chains are rejected explicitly.
    pub fn put_to(&self, destination: Option<Self>) -> Result<(), PipeLinkError> {
        if let Some(next) = destination.as_ref() {
            if Rc::ptr_eq(&self.node, &next.node) {
                return Err(PipeLinkError::Cycle);
            }

            let mut cursor = Some(next.clone());
            while let Some(current) = cursor {
                if Rc::ptr_eq(&self.node, &current.node) {
                    return Err(PipeLinkError::Cycle);
                }
                cursor = current.node.chain_to.borrow().clone();
            }
        }

        let current = self.node.chain_to.borrow().clone();
        match (&current, &destination) {
            (Some(old), Some(new)) if Rc::ptr_eq(&old.node, &new.node) => return Ok(()),
            (None, None) => return Ok(()),
            _ => {}
        }

        // Match Pipe::Put_To: the new destination must first be detached from
        // its former predecessor, if it has one.
        if let Some(next) = destination.as_ref() {
            let previous = { next.node.chain_from.borrow().upgrade() };
            if let Some(previous) = previous {
                PipeHandle { node: previous }.put_to(None)?;
            }
        }

        // Match the old link teardown: clear the downstream back-reference,
        // then flush the entire old downstream chain before replacing it.
        if let Some(old) = self.node.chain_to.borrow_mut().take() {
            *old.node.chain_from.borrow_mut() = Weak::new();
            old.flush();
        }

        if let Some(next) = destination {
            *self.node.chain_to.borrow_mut() = Some(next.clone());
            *next.node.chain_from.borrow_mut() = Rc::downgrade(&self.node);
        }

        Ok(())
    }

    /// Explicitly remove this node from a chain and reconnect its neighbors.
    /// This models the C++ destructor's chain-preserving unlink operation.
    pub fn unlink(&self) -> Result<(), PipeLinkError> {
        let previous = self.upstream();
        let next = self.downstream();

        if let Some(previous) = previous {
            // Let the previous node detach this one and flush through its still-
            // connected downstream link, matching Pipe::~Pipe ordering.
            previous.put_to(next)?;
        } else if let Some(next) = next {
            *next.node.chain_from.borrow_mut() = Weak::new();
        }
        *self.node.chain_to.borrow_mut() = None;
        *self.node.chain_from.borrow_mut() = Weak::new();

        Ok(())
    }

    pub fn is_connected_to(&self, destination: &Self) -> bool {
        let mut cursor = self.downstream();
        while let Some(current) = cursor {
            if Rc::ptr_eq(&current.node, &destination.node) {
                return true;
            }
            cursor = current.downstream();
        }
        false
    }
}

/// Base pass-through endpoint. With no downstream node, all bytes are accepted.
#[derive(Debug, Default, Clone, Copy)]
pub struct PassthroughPipe;

impl PipeStage for PassthroughPipe {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[derive(Clone)]
    struct Recorder {
        bytes: Rc<RefCell<Vec<u8>>>,
        flushes: Rc<RefCell<usize>>,
    }

    impl PipeStage for Recorder {
        fn put(&mut self, source: &[u8], _downstream: Option<&PipeHandle>) -> usize {
            self.bytes.borrow_mut().extend_from_slice(source);
            source.len()
        }

        fn flush(&mut self, _downstream: Option<&PipeHandle>) -> usize {
            *self.flushes.borrow_mut() += 1;
            0
        }
    }

    #[test]
    fn base_pipe_forwards_and_terminal_accepts_bytes() {
        let terminal = PipeHandle::new(PassthroughPipe);
        let first = PipeHandle::new(PassthroughPipe);
        first.put_to(Some(terminal.clone())).unwrap();
        assert_eq!(first.put(b"data"), 4);
        assert!(first.is_connected_to(&terminal));
    }

    #[test]
    fn replacing_downstream_detaches_old_and_flushes_it() {
        let flushes = Rc::new(RefCell::new(0));
        let old = PipeHandle::new(Recorder {
            bytes: Rc::new(RefCell::new(Vec::new())),
            flushes: flushes.clone(),
        });
        let new = PipeHandle::new(PassthroughPipe);
        let first = PipeHandle::new(PassthroughPipe);

        first.put_to(Some(old.clone())).unwrap();
        first.put_to(Some(new.clone())).unwrap();

        assert_eq!(*flushes.borrow(), 1);
        assert!(old.upstream().is_none());
        assert_eq!(first.downstream().unwrap().put(b"x"), 1);
    }

    #[test]
    fn destination_is_detached_from_its_previous_parent() {
        let destination = PipeHandle::new(PassthroughPipe);
        let first_parent = PipeHandle::new(PassthroughPipe);
        let second_parent = PipeHandle::new(PassthroughPipe);

        first_parent.put_to(Some(destination.clone())).unwrap();
        second_parent.put_to(Some(destination.clone())).unwrap();

        assert!(first_parent.downstream().is_none());
        assert!(destination.upstream().is_some());
        assert!(destination.upstream().unwrap().is_connected_to(&destination));
    }

    #[test]
    fn cycles_are_rejected_without_mutating_links() {
        let first = PipeHandle::new(PassthroughPipe);
        let second = PipeHandle::new(PassthroughPipe);
        first.put_to(Some(second.clone())).unwrap();
        assert_eq!(second.put_to(Some(first.clone())), Err(PipeLinkError::Cycle));
        assert!(first.is_connected_to(&second));
        assert!(second.downstream().is_none());
    }

    #[test]
    fn unlink_reconnects_neighbors_and_flushes_old_downstream() {
        let flushes = Rc::new(RefCell::new(0));
        let first = PipeHandle::new(PassthroughPipe);
        let middle = PipeHandle::new(PassthroughPipe);
        let last = PipeHandle::new(Recorder {
            bytes: Rc::new(RefCell::new(Vec::new())),
            flushes: flushes.clone(),
        });

        first.put_to(Some(middle.clone())).unwrap();
        middle.put_to(Some(last.clone())).unwrap();
        middle.unlink().unwrap();

        assert!(first.is_connected_to(&last));
        assert!(middle.upstream().is_none());
        assert_eq!(*flushes.borrow(), 1);
    }
}
