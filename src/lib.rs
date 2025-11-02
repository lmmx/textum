//! A syntactic patching library with character-level granularity.
//!
//! `textum` provides a robust way to apply patches to source files using rope data structures
//! for efficient editing and tree-sitter for syntactic awareness. Unlike traditional line-based
//! patch formats, textum operates at character granularity and can compose multiple patches
//! with automatic offset tracking.
//!
//! # Examples
//!
//! ```
//! use textum::{Patch, PatchSet};
//! use ropey::Rope;
//!
//! let mut rope = Rope::from_str("hello world");
//! let patch = Patch {
//!     file: "test.txt".to_string(),
//!     range: (6, 11),
//!     replacement: Some("rust".into()),
//!     symbol_path: None,
//!     max_line_drift: None,
//! };
//!
//! patch.apply(&mut rope).unwrap();
//! assert_eq!(rope.to_string(), "hello rust");
//! ```
#![allow(clippy::multiple_crate_versions)]

pub mod composer;
pub mod patch;

pub use composer::PatchSet;
pub use patch::{Patch, PatchError};
