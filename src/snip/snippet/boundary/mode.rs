use std::hash::Hash;

#[cfg(feature = "facet")]
use facet::Facet;

use super::Extent;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "facet", derive(Facet))]
#[repr(u8)]
/// Controls boundary inclusion in the selected range.
pub enum BoundaryMode {
    /// Omits the boundary from the selection.
    Exclude,
    /// Includes the boundary in selection.
    Include,
    /// Expands selection beyond the boundary by the specified extent.
    Extend(Extent),
}
