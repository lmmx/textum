use std::hash::Hash;

use crate::snip::Target;

/// `BoundaryError` enum type raised by boundary resolution.
pub mod error;
/// Extent configuration for boundary expansion.
pub mod extent;
/// Boundary treatment modes (whether to include/exclude/extend them).
pub mod mode;
/// Boundary resolution struct and `Boundary::resolve` implementation.
pub mod resolution;

pub use error::*;
pub use extent::*;
pub use mode::*;
pub use resolution::*;

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
