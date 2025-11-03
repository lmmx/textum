use std::hash::Hash;

use super::Extent;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Controls boundary inclusion in the selected range.
pub enum BoundaryMode {
    /// Omits the boundary from the selection.
    Exclude,
    /// Includes the boundary in selection.
    Include,
    /// Expands selection beyond the boundary by the specified extent.
    Extend(Extent),
}
