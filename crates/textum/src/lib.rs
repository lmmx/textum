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
//! use textum::{Patch, Rope};
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
//! use textum::{Patch, Rope};
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
//! use textum::{Boundary, BoundaryMode, Patch, Rope, Snippet, Target};
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
//!     file: Some("test.txt".to_string()),
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
//! ## String-Based API (No Rope Required)
//!
//! For convenience, patches can be applied directly to strings without needing to
//! import or work with `Rope` types. Use `Patch::in_memory()` for patches that
//! don't need a file path:
//!
//! ```
//! use textum::{Patch, Snippet, Boundary, BoundaryMode, Target};
//!
//! let content = "hello world";
//!
//! let snippet = Snippet::At(Boundary::new(
//!     Target::Literal("world".to_string()),
//!     BoundaryMode::Include,
//! ));
//!
//! let patch = Patch::in_memory(snippet, "rust");
//! let result = patch.apply_to_string(content).unwrap();
//! assert_eq!(result, "hello rust");
//! ```
//!
//! ## Apply Single Patch to File
//!
//! Apply a patch to a file from disk, with options to inspect or write results:
//!
//! ```no_run
//! use textum::{Patch, BoundaryMode};
//!
//! let patch = Patch::from_literal_target(
//!     "tests/fixtures/sample.txt".to_string(),
//!     "world",
//!     BoundaryMode::Include,
//!     "rust",
//! );
//!
//! // Get the result without writing
//! let result = patch.apply_to_file().unwrap();
//! println!("Would change to: {}", result);
//!
//! // Or write directly to disk
//! patch.write_to_file().unwrap();
//! ```
//!
//! ## Composing Multiple Patches
//!
//! For applying multiple patches to one or more files, use `PatchSet`.
//! Use `apply_to_files()` to get results in memory for inspection, or
//! `write_to_files()` to apply and write directly to disk:
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
//! // Get results in memory for inspection
//! let results = set.apply_to_files().unwrap();
//! assert_eq!(results.get("tests/fixtures/sample.txt").unwrap(), "goodbye rust\n");
//!
//! // Or write directly to disk
//! // set.write_to_files().unwrap();
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

/// Re-export of ropey's Rope for convenience.
///
/// Users who need fine-grained control over rope operations can use this type directly.
/// For simpler use cases, consider using `Patch::apply_to_string()` which works with
/// `&str` and `String` directly, or `Patch::apply_to_file()` / `Patch::write_to_file()`
/// for file operations.
pub use ropey::Rope;
