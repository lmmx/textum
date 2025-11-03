use std::hash::Hash;

use crate::snip::Target;

pub mod extent;
pub mod mode;
pub use extent::*;
pub use mode::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Boundary {
    pub target: Target,
    pub mode: BoundaryMode,
}

impl Boundary {
    #[must_use]
    pub fn new(target: Target, mode: BoundaryMode) -> Self {
        Self { target, mode }
    }
}
