use crate::snip::target::error::TargetError;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Errors that can occur while resolving snippet boundaries.
///
/// These represent failures when converting a logical boundary definition into concrete character
/// indices within a [`Rope`].
pub enum BoundaryError {
    /// An error occurred in the underlying [`Target`] resolution logic.
    TargetError(TargetError),
    /// The extent moved beyond the bounds of the [`Rope`].
    ///
    /// Returned when attempting to extend past the end of the buffer.
    ExtentOutOfBounds,
    /// The extent definition was invalid or not applicable.
    ///
    /// Returned when a boundary’s extent type cannot be resolved in the given context, such as
    /// attempting to use a non-pattern [`Target`] for a matching-based extent.
    InvalidExtent,
}

impl From<TargetError> for BoundaryError {
    fn from(err: TargetError) -> Self {
        BoundaryError::TargetError(err)
    }
}
