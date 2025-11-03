//! Snippet-based text selection and boundary specification.
//!
//! This module provides types for defining text ranges through boundary markers, supporting both
//! pattern-based and position-based targetting with configurable inclusion/exclusion behaviour.
//! There are no defaults, so the user must always specify if they want their replacements to be
//! inclusive of the boundary or not (this will depend on context and making the wrong assumption
//! could be a major form of error).

pub mod snippet;
pub mod target;

pub use snippet::*;
pub use target::*;
