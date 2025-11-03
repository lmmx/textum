use std::hash::Hash;

use super::Extent;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BoundaryMode {
    Exclude,
    Include,
    Extend(Extent),
}
