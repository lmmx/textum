use std::hash::Hash;

pub mod boundary;
pub use boundary::{Boundary, BoundaryMode, Extent};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Snippet {
    At(Boundary),
    From(Boundary),
    To(Boundary),
    Between { start: Boundary, end: Boundary },
    All,
}
