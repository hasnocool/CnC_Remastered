//! Safe Rust representation of the legacy Shape.cpp global buffer state.
//!
//! Both game variants compile the same upstream Shape.cpp implementation.

/// Shape-compression workspace configuration.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ShapeBufferConfig {
    size: usize,
    configured: bool,
}

impl ShapeBufferConfig {
    /// Equivalent to Set_Shape_Buffer: remember the caller-provided capacity.
    /// The actual memory remains owned by the caller in Rust.
    pub const fn set_shape_buffer(&mut self, size: usize) {
        self.size = size;
        self.configured = true;
    }

    pub const fn size(&self) -> usize { self.size }
    pub const fn is_configured(&self) -> bool { self.configured }

    /// True when a candidate compressed shape fits the configured workspace.
    pub const fn can_fit(&self, encoded_size: usize) -> bool {
        self.configured && encoded_size <= self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tracks_shape_workspace_capacity() {
        let mut config = ShapeBufferConfig::default();
        assert!(!config.can_fit(1));
        config.set_shape_buffer(1024);
        assert_eq!(config.size(), 1024);
        assert!(config.can_fit(1024));
        assert!(!config.can_fit(1025));
    }
}
