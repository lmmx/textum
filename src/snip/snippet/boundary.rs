use std::hash::Hash;

use crate::snip::Target;

/// Extent configuration for boundary expansion.
pub mod extent;
/// Boundary treatment modes (whether to include/exclude/extend them).
pub mod mode;
pub use extent::*;
pub use mode::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Pairs a target with the mode of inclusion/exclusion/extension of its boundaries.
pub struct Boundary {
    /// The pattern or position defining this boundary.
    pub target: Target,
    /// Whether to include, exclude, or extend beyond this boundary.
    pub mode: BoundaryMode,
}

impl Boundary {
    #[must_use]
    /// Constructs a boundary from a target and mode.
    pub fn new(target: Target, mode: BoundaryMode) -> Self {
        Self { target, mode }
    }
}
