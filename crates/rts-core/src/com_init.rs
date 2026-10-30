//! Portable lifecycle equivalent for REDALERT/COMINIT.CPP.
//!
//! Upstream creates a process-global object whose constructor calls
//! OleInitialize(NULL) and whose destructor calls OleUninitialize(). This
//! module isolates those platform calls behind a host-provided implementation.

/// Host implementation of the Windows OLE apartment lifecycle.
pub trait OleRuntime {
    /// Initialize the calling thread's OLE apartment.
    fn ole_initialize(&mut self);
    /// Balance the initialization when the guard is dropped.
    fn ole_uninitialize(&mut self);
}

/// RAII guard corresponding to the upstream global ComInit object.
pub struct ComInitGuard<R: OleRuntime> {
    runtime: R,
}

impl<R: OleRuntime> ComInitGuard<R> {
    /// Calls OLE initialization once and retains the runtime for cleanup.
    pub fn new(mut runtime: R) -> Self {
        runtime.ole_initialize();
        Self { runtime }
    }

    /// Borrow the platform runtime while the apartment is initialized.
    pub fn runtime(&self) -> &R { &self.runtime }

    /// Mutably borrow the platform runtime while the apartment is initialized.
    pub fn runtime_mut(&mut self) -> &mut R { &mut self.runtime }
}

impl<R: OleRuntime> Drop for ComInitGuard<R> {
    fn drop(&mut self) {
        self.runtime.ole_uninitialize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    struct FakeOle(Rc<Cell<i32>>);
    impl OleRuntime for FakeOle {
        fn ole_initialize(&mut self) { self.0.set(self.0.get() + 1); }
        fn ole_uninitialize(&mut self) { self.0.set(self.0.get() - 1); }
    }

    #[test]
    fn balances_initialize_and_uninitialize() {
        let count = Rc::new(Cell::new(0));
        {
            let _guard = ComInitGuard::new(FakeOle(count.clone()));
            assert_eq!(count.get(), 1);
        }
        assert_eq!(count.get(), 0);
    }
}
