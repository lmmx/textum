use std::hash::Hash;

/// Boundary specification and treatment modes.
pub mod boundary;
pub use boundary::{Boundary, BoundaryMode, Extent};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Specifies a text range through boundary markers or positions.
///
/// The exceptions are the `Between` variant (which takes two `Boundary` arguments for the start and
/// end; and the `All` variant (which takes no `Boundary` argument, because it implies the entire
/// file. The `From` variant implies an end position of End Of File, and the `To` variant implies a
/// start position of the Beginning Of File.
pub enum Snippet {
    At(Boundary),
    From(Boundary),
    To(Boundary),
    Between { start: Boundary, end: Boundary },
    All,
}
