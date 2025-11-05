//! A syntactic patching library with character-level granularity.
//!
//! `textum` provides a robust way to apply patches to source files using rope data structures
//! for efficient editing and a powerful snippet system for flexible target specification.
//! Unlike traditional line-based patch formats, textum operates with character, byte, and
//! line granularity through the Snippet API, supporting literal matching, regex patterns,
//! and boundary semantics.
//!
//! # Core Concepts
//!
//! ## Patches
//!
//! A `Patch` specifies a file, a `Snippet` defining the target range, and replacement text.
//! Patches compose through `PatchSet`, which handles resolution, validation, and application.
//!
//! ## Snippets
//!
//! Snippets define text ranges through:
//! - **Targets**: What to match (Literal, Pattern, Line, Char, Position)
//! - **Boundaries**: How to treat matches (Include, Exclude, Extend)
//! - **Modes**: Range selection (At, From, To, Between, All)
//!
//! ## Hunks
//!
//! textum works with hunks - contiguous change blocks that may include context through
//! boundary extension. Multiple patches with overlapping non-empty replacements are
//! rejected to maintain unambiguous application order.
//!
//! # Examples
//!
//! ## Simple Literal Replacement
//!
//! ```
//! use textum::Patch;
//! use ropey::Rope;
//!
//! let mut rope = Rope::from_str("hello world");
//! let patch = Patch::from_literal_target(
//!     "test.txt".to_string(),
//!     "world",
//!     textum::BoundaryMode::Include,
//!     "rust",
//! );
//!
//! patch.apply(&mut rope).unwrap();
//! assert_eq!(rope.to_string(), "hello rust");
//! ```
//!
//! ## Line Range Deletion
//!
//! ```
//! use textum::Patch;
//! use ropey::Rope;
//!
//! let mut rope = Rope::from_str("line1\nline2\nline3\nline4\n");
//! let patch = Patch::from_line_range(
//!     "test.txt".to_string(),
//!     1,  // Start at line 1 (inclusive)
//!     3,  // End before line 3 (exclusive)
//!     "",
//! );
//!
//! patch.apply(&mut rope).unwrap();
//! assert_eq!(rope.to_string(), "line1\nline4\n");
//! ```
//!
//! ## Between Markers
//!
//! ```
//! use textum::{Patch, Boundary, BoundaryMode, Snippet, Target};
//! use ropey::Rope;
//!
//! let mut rope = Rope::from_str("<!-- start -->old<!-- end -->");
//!
//! let start = Boundary::new(
//!     Target::Literal("<!-- start -->".to_string()),
//!     BoundaryMode::Exclude,
//! );
//! let end = Boundary::new(
//!     Target::Literal("<!-- end -->".to_string()),
//!     BoundaryMode::Exclude,
//! );
//! let snippet = Snippet::Between { start, end };
//!
//! let patch = Patch {
//!     file: "test.txt".to_string(),
//!     snippet,
//!     replacement: "new".to_string(),
//!     #[cfg(feature = "symbol_path")]
//!     symbol_path: None,
//! };
//!
//! patch.apply(&mut rope).unwrap();
//! assert_eq!(rope.to_string(), "<!-- start -->new<!-- end -->");
//! ```
//!
//! ## Composing Multiple Patches
//!
//! ```
//! use textum::{Patch, PatchSet, BoundaryMode};
//!
//! let mut set = PatchSet::new();
//!
//! set.add(Patch::from_literal_target(
//!     "tests/fixtures/sample.txt".to_string(),
//!     "hello",
//!     BoundaryMode::Include,
//!     "goodbye",
//! ));
//!
//! set.add(Patch::from_literal_target(
//!     "tests/fixtures/sample.txt".to_string(),
//!     "world",
//!     BoundaryMode::Include,
//!     "rust",
//! ));
//!
//! let results = set.apply_to_files().unwrap();
//! assert_eq!(results.get("tests/fixtures/sample.txt").unwrap(), "goodbye rust\n");
//! ```
//!
//! ## JSON API with Facet
//!
//! Enable the `json` feature to deserialize patches from JSON:
//!
//! ```
//! #[cfg(feature = "json")]
//! fn example() -> Result<(), textum::PatchError> {
//!     use textum::{Patch, PatchSet};
//!
//!     let input = r#"[
//!       {
//!         "file": "tests/fixtures/sample.txt",
//!         "snippet": {
//!           "At": {
//!             "target": {"Literal": "hello"},
//!             "mode": "Include"
//!           }
//!         },
//!         "replacement": "goodbye"
//!       }
//!     ]"#;
//!
//!     let patches: Vec<Patch> = facet_json::from_str(&input)?;
//!
//!     let mut set = PatchSet::new();
//!     for patch in patches {
//!         set.add(patch);
//!     }
//!
//!     let results = set.apply_to_files()?;
//!     for (file, content) in results {
//!         std::fs::write(&file, content)?;
//!     }
//!
//!     Ok(())
//! }
//! ```
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::multiple_crate_versions)]

pub mod composer;
pub mod patch;
pub mod snip;

pub use composer::PatchSet;
pub use patch::{Patch, PatchError};
pub use snip::snippet::boundary::{Boundary, BoundaryMode};
pub use snip::snippet::{Snippet, SnippetError, SnippetResolution};
pub use snip::target::Target;
