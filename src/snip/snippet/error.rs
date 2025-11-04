use crate::snip::BoundaryError;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Errors that can occur during snippet operations.
pub enum SnippetError {
    /// An error occurred in boundary resolution.
    BoundaryError(BoundaryError),
    /// The resolved range is invalid (start >= end).
    InvalidRange {
        /// Starting index of the invalid range.
        start: usize,
        /// Ending index of the invalid range.
        end: usize,
    },
    /// The replacement string contains invalid UTF-8.
    InvalidUtf8(String),
    /// The resolved index is out of bounds.
    OutOfBounds {
        /// The index that exceeded bounds.
        index: usize,
        /// The length of the rope.
        rope_len: usize,
    },
}

impl From<BoundaryError> for SnippetError {
    fn from(err: BoundaryError) -> Self {
        SnippetError::BoundaryError(err)
    }
}
