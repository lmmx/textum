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
//!     #[cfg(feature = "symbol_path")]
//!     symbol_path: None,
//!     #[cfg(feature = "line_tol")]
//!     max_line_drift: None,
//! };
//!
//! patch.apply(&mut rope).unwrap();
//! assert_eq!(rope.to_string(), "hello rust");
//! ```
//!
//! ## `Patch` feature gating
//!
//! The `Patch` struct has three required fields: `file`, `range`, and `replacement`, i.e. it is a
//! triple of (from, to, replacement) specifying a patch for a particular file as input.
//!
//! - The file may be STDIN.
//! - The range may be empty (i.e. of zero length), such as `[5, 5]`, making its replacement
//!   effectively an insertion.
//!
//! ## Use the `facet` feature for a `Facet` derive macro
//!
//! A `Patch` compiled with the `facet` feature is decorated with the `facet::Facet` derive macro
//! which means you can deserialise from JSON objects, and because they are given default values,
//! the optional fields `symbol_path` and `max_line_drift` are not required.
//!
//! That is, you can write a patch concisely like this:
//!
//! ```
//! #[cfg(feature = "json")]
//! fn example() -> Result<(), textum::PatchError> {
//!     use textum::{Patch, PatchSet};
//!
//!     let dry_run = true; // Change to false to actually write files
//!
//!     let input = r#"[
//!       {"file": "tests/fixtures/sample.txt", "range": [0, 5], "replacement": "goodbye"}
//!     ]"#;
//!
//!     let patches: Vec<Patch> = facet_json::from_str(&input)?;
//!
//!     let mut set = PatchSet::new();
//!     for patch in patches {
//!         set.add(patch);
//!     }
//!
//!     match set.apply_to_files() {
//!         Ok(results) => {
//!             for (file, content) in results {
//!                 if dry_run {
//!                     println!("Would write {} to {}", content, file);
//!                 } else {
//!                     std::fs::write(&file, content)?;
//!                     println!("Wrote to {}", file);
//!                 }
//!             }
//!         }
//!         Err(e) => return Err(e),
//!     }
//!
//!     Ok(())
//! }
//! ```
#![allow(clippy::multiple_crate_versions)]

pub mod composer;
pub mod patch;

pub use composer::PatchSet;
pub use patch::{Patch, PatchError};
