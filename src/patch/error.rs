//! Error types for patch operations.

use crate::snip::target::error::TargetError;
use crate::snip::{BoundaryError, SnippetError};
use std::fmt;

/// Errors that can occur when applying patches.
#[derive(Debug)]
pub enum PatchError {
    /// The patch range exceeds the file's character count.
    RangeOutOfBounds,

    /// The target file could not be found or read.
    FileNotFound,

    /// An I/O error occurred while reading or writing files.
    IoError(std::io::Error),

    /// An error occurred during snippet resolution.
    SnippetError(SnippetError),

    /// An error occurred during boundary resolution.
    BoundaryError(BoundaryError),

    /// An error occurred during target resolution.
    TargetError(TargetError),

    /// Resolved patch ranges overlap with non-empty replacements.
    OverlappingRanges {
        /// First overlapping range.
        range1: (usize, usize),
        /// Second overlapping range.
        range2: (usize, usize),
    },
}

impl fmt::Display for PatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RangeOutOfBounds => write!(f, "Patch range exceeds file bounds"),
            Self::FileNotFound => write!(f, "Target file not found"),
            Self::IoError(e) => write!(f, "I/O error: {e}"),
            Self::SnippetError(e) => write!(f, "Snippet error: {e:?}"),
            Self::BoundaryError(e) => write!(f, "Boundary error: {e:?}"),
            Self::TargetError(e) => write!(f, "Target error: {e}"),
            Self::OverlappingRanges { range1, range2 } => {
                write!(f, "Overlapping ranges: {range1:?} and {range2:?}")
            }
        }
    }
}

impl std::error::Error for PatchError {}

impl From<std::io::Error> for PatchError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<SnippetError> for PatchError {
    fn from(e: SnippetError) -> Self {
        Self::SnippetError(e)
    }
}

impl From<BoundaryError> for PatchError {
    fn from(e: BoundaryError) -> Self {
        Self::BoundaryError(e)
    }
}

impl From<TargetError> for PatchError {
    fn from(e: TargetError) -> Self {
        Self::TargetError(e)
    }
}

#[cfg(feature = "json")]
impl From<facet_json::DeserError<'_>> for PatchError {
    fn from(e: facet_json::DeserError<'_>) -> Self {
        Self::IoError(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            e.to_string(),
        ))
    }
}
