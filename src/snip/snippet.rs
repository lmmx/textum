//! Snippet-based text selection and boundary specification.
use std::hash::Hash;

#[cfg(feature = "facet")]
use facet::Facet;

/// Boundary specification and treatment modes.
pub mod boundary;
/// Error types for snippet operations.
pub mod error;
/// Snippet replacement operations.
pub mod replacement;
/// Snippet resolution to rope indices.
pub mod resolution;

pub use boundary::{Boundary, BoundaryError, BoundaryMode, Extent};
pub use error::*;
pub use resolution::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "facet", derive(Facet))]
#[repr(u8)]
/// Specifies a text range through boundary markers or positions.
///
/// The exceptions are the `Between` variant (which takes two `Boundary` arguments for the start and
/// end; and the `All` variant (which takes no `Boundary` argument, because it implies the entire
/// file. The `From` variant implies an end position of End Of File, and the `To` variant implies a
/// start position of the Beginning Of File.
pub enum Snippet {
    /// Targets a single boundary location.
    At(Boundary),
    /// Selects from a boundary to end of file.
    From(Boundary),
    /// Selects from beginning of file to a boundary.
    To(Boundary),
    /// Selects the range between two boundaries.
    Between {
        /// Starting boundary of the range.
        start: Boundary,
        /// Ending boundary of the range.
        end: Boundary,
    },
    /// Selects the entire file.
    All,
}
