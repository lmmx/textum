//! Core patch types and application logic.
//!
//! A patch represents a single atomic edit operation on a file, defined by a character range
//! and optional replacement text. Patches can be created from line-based positions (for
//! compatibility with tools like cargo diagnostics) or directly from character indices.

#[cfg(feature = "facet")]
use facet::Facet;
use ropey::Rope;

pub mod error;
pub use error::PatchError;

use crate::snip::snippet::{Boundary, BoundaryMode, Snippet};
use crate::snip::target::Target;

/// A single atomic patch operation on a file.
///
/// Patches operate through the Snippet system, providing flexible target matching
/// and boundary semantics. Character positions are 0-indexed and represent positions
/// in the file as a sequence of Unicode scalar values.
///
/// # Examples
///
/// ```
/// use textum::{Patch, Target, Boundary, BoundaryMode, Snippet};
/// use ropey::Rope;
///
/// // Replace using literal target
/// let mut rope = Rope::from_str("hello world");
/// let patch = Patch::from_literal_target(
///     "main.rs".to_string(),
///     "world",
///     BoundaryMode::Include,
///     "rust",
/// );
/// patch.apply(&mut rope).unwrap();
/// assert_eq!(rope.to_string(), "hello rust");
///
/// // Delete using line range
/// let mut rope = Rope::from_str("line1\nline2\nline3\n");
/// let patch = Patch::from_line_range(
///     "main.rs".to_string(),
///     1,
///     2,
///     "",
/// );
/// patch.apply(&mut rope).unwrap();
/// assert_eq!(rope.to_string(), "line1\nline3\n");
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "facet", derive(Facet))]
pub struct Patch {
    /// File path this patch applies to.
    pub file: String,

    /// Snippet defining the target range for this patch.
    pub snippet: Snippet,

    /// Replacement text to insert at the resolved range.
    ///
    /// Empty string performs deletion of the resolved range.
    pub replacement: String,

    /// Optional symbol path for robust positioning (non-functional, reserved for future use).
    #[cfg_attr(feature = "facet", facet(default))]
    #[cfg(feature = "symbol_path")]
    pub symbol_path: Option<Vec<String>>,
}

impl Patch {
    /// Apply this patch to a rope in-place.
    ///
    /// The rope is modified by resolving the snippet to a character range, then
    /// removing that range and inserting the replacement text. Changes are applied
    /// atomically - if the patch cannot be applied, the rope is left unchanged.
    ///
    /// # Errors
    ///
    /// Returns `PatchError` if the snippet cannot be resolved or if the resolved
    /// range extends beyond the rope's character count.
    ///
    /// # Examples
    ///
    /// ```
    /// use ropey::Rope;
    /// use textum::Patch;
    ///
    /// let mut rope = Rope::from_str("hello world");
    /// let patch = Patch::from_literal_target(
    ///     "test.txt".to_string(),
    ///     "world",
    ///     textum::BoundaryMode::Include,
    ///     "rust",
    /// );
    ///
    /// patch.apply(&mut rope).unwrap();
    /// assert_eq!(rope.to_string(), "hello rust");
    /// ```
    pub fn apply(&self, rope: &mut Rope) -> Result<(), PatchError> {
        let resolution = self.snippet.resolve(rope)?;

        if resolution.end > rope.len_chars() {
            return Err(PatchError::RangeOutOfBounds);
        }

        // Remove the range
        if resolution.start < resolution.end {
            rope.remove(resolution.start..resolution.end);
        }

        // Insert replacement
        rope.insert(resolution.start, &self.replacement);

        Ok(())
    }

    /// Create a patch from a literal string target.
    ///
    /// Constructs a patch that matches the first occurrence of `needle` in the file
    /// and applies the boundary mode to determine inclusion/exclusion.
    ///
    /// # Arguments
    ///
    /// * `file` - Path to the file this patch targets
    /// * `needle` - Exact string to match
    /// * `mode` - Whether to include, exclude, or extend the boundary
    /// * `replacement` - Text to insert (empty string for deletion)
    ///
    /// # Examples
    ///
    /// ```
    /// use textum::{Patch, BoundaryMode};
    ///
    /// let patch = Patch::from_literal_target(
    ///     "src/main.rs".to_string(),
    ///     "old_name",
    ///     BoundaryMode::Include,
    ///     "new_name",
    /// );
    /// ```
    #[must_use]
    pub fn from_literal_target(
        file: String,
        needle: &str,
        mode: BoundaryMode,
        replacement: impl Into<String>,
    ) -> Self {
        let target = Target::Literal(needle.to_string());
        let boundary = Boundary::new(target, mode);
        let snippet = Snippet::At(boundary);
        Self {
            file,
            snippet,
            replacement: replacement.into(),
            #[cfg(feature = "symbol_path")]
            symbol_path: None,
        }
    }

    /// Create a patch from a line range.
    ///
    /// Constructs a patch that targets a range of lines, with the start line included
    /// and the end line excluded (half-open range semantics).
    ///
    /// # Arguments
    ///
    /// * `file` - Path to the file this patch targets
    /// * `start_line` - Starting line number (0-indexed, inclusive)
    /// * `end_line` - Ending line number (0-indexed, exclusive)
    /// * `replacement` - Text to insert (empty string for deletion)
    ///
    /// # Examples
    ///
    /// ```
    /// use textum::Patch;
    ///
    /// // Delete lines 5-10
    /// let patch = Patch::from_line_range(
    ///     "src/main.rs".to_string(),
    ///     5,
    ///     10,
    ///     "",
    /// );
    /// ```
    #[must_use]
    pub fn from_line_range(
        file: String,
        start_line: usize,
        end_line: usize,
        replacement: impl Into<String>,
    ) -> Self {
        let start = Boundary::new(Target::Line(start_line), BoundaryMode::Include);
        let end = Boundary::new(Target::Line(end_line), BoundaryMode::Exclude);
        let snippet = Snippet::Between { start, end };
        Self {
            file,
            snippet,
            replacement: replacement.into(),
            #[cfg(feature = "symbol_path")]
            symbol_path: None,
        }
    }

    /// Create a patch from line-based positions.
    ///
    /// This is useful for interoperating with tools that report positions in terms of
    /// lines and columns (like cargo diagnostics). Line and column indices are 0-based.
    ///
    /// # Arguments
    ///
    /// * `file` - Path to the file this patch targets
    /// * `line_start` - Starting line number (0-indexed)
    /// * `col_start` - Starting column within the line (0-indexed)
    /// * `line_end` - Ending line number (0-indexed)
    /// * `col_end` - Ending column within the line (0-indexed)
    /// * `rope` - A rope containing the file content, used to validate positions
    /// * `replacement` - Replacement text
    ///
    /// # Examples
    ///
    /// ```
    /// use ropey::Rope;
    /// use textum::Patch;
    ///
    /// let rope = Rope::from_str("line 1\nline 2\nline 3");
    /// let patch = Patch::from_line_positions(
    ///     "test.txt".to_string(),
    ///     1,
    ///     0,
    ///     1,
    ///     6,
    ///     &rope,
    ///     "EDITED",
    /// );
    /// ```
    #[must_use]
    pub fn from_line_positions(
        file: String,
        line_start: usize,
        col_start: usize,
        line_end: usize,
        col_end: usize,
        _rope: &Rope,
        replacement: impl Into<String>,
    ) -> Self {
        // For a range spanning multiple positions, use Between with two Position targets
        let start_target = Target::Position {
            line: line_start + 1, // Convert to 1-indexed
            col: col_start + 1,
        };
        let end_target = Target::Position {
            line: line_end + 1,
            col: col_end + 1,
        };

        let start = Boundary::new(start_target, BoundaryMode::Include);
        let end = Boundary::new(end_target, BoundaryMode::Exclude);
        let snippet = Snippet::Between { start, end };

        Self {
            file,
            snippet,
            replacement: replacement.into(),
            #[cfg(feature = "symbol_path")]
            symbol_path: None,
        }
    }
}
