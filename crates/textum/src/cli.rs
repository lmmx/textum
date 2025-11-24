//! textum: A syntactic patching library with char-level granularity.
//!
//! Command-line interface for applying patches from JSON.
//!
//! `textum` provides a robust way to apply patches to source files using rope data structures
//! for efficient editing and tree-sitter for syntactic awareness. Unlike traditional line-based
//! patch formats, textum operates at character granularity and can compose multiple patches
//! with automatic offset tracking.
//!
//! Reads a JSON array of patches from a file or stdin and applies them to their target files.
//! Modified files are written back to disk unless `--dry-run` is specified.
#![allow(clippy::multiple_crate_versions)]

/// Command-line interface for applying patches from JSON.
#[cfg(feature = "cli")]
#[path = "cli/core.rs"]
pub mod inner;

/// Hint replacement CLI for when the cli module is used without building the cli feature.
#[cfg(not(feature = "cli"))]
#[path = "cli/stub.rs"]
pub mod inner;

pub use inner::main;

// Re-export the real inner modules to crate-level for convenience behind feature gate
#[cfg(feature = "cli")]
pub use inner::{args, diff, handlers, utils};
