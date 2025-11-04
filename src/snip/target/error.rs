//! Error types for target resolution.

use std::fmt;

/// Errors that can occur when resolving a target to a rope index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetError {
    /// The target was not found in the rope.
    NotFound,
    /// The target index is out of bounds.
    OutOfBounds,
    /// The target position is invalid (e.g., line or column exceeds rope bounds).
    InvalidPosition {
        /// The line number that was invalid.
        line: usize,
        /// The column number that was invalid (if applicable).
        col: Option<usize>,
    },
    /// The regex pattern failed to compile.
    #[cfg(feature = "regex")]
    InvalidPattern(String),
}

impl fmt::Display for TargetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "Target not found in rope"),
            Self::OutOfBounds => write!(f, "Target index out of bounds"),
            Self::InvalidPosition {
                line,
                col: Some(col),
            } => {
                write!(f, "Invalid position: line {line}, column {col}")
            }
            Self::InvalidPosition { line, col: None } => {
                write!(f, "Invalid position: line {line}")
            }
            #[cfg(feature = "regex")]
            Self::InvalidPattern(msg) => write!(f, "Invalid regex pattern: {msg}"),
        }
    }
}

impl std::error::Error for TargetError {}
