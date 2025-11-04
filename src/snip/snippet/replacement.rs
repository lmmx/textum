//! Snippet replacement operations on rope structures.

use crate::Snippet;
use crate::SnippetError;
use ropey::Rope;

impl Snippet {
    /// Replaces the text selected by this snippet with the given replacement string.
    ///
    /// This method resolves the snippet's boundaries to determine the target range, validates
    /// the replacement text, and returns a new [`Rope`] with the replacement applied.
    ///
    /// # Behavior by Snippet Type
    ///
    /// - **Zero-width range** (start == end): Performs insertion at the position
    /// - **Empty replacement**: Performs deletion of the selected range
    /// - **Non-empty replacement on non-zero range**: Performs edit (replace existing text)
    ///
    /// # Arguments
    ///
    /// * `rope` - The rope containing the text to modify
    /// * `replacement` - The string to insert at the resolved position
    ///
    /// # Returns
    ///
    /// Returns a new [`Rope`] with the replacement applied.
    ///
    /// # Errors
    ///
    /// Returns [`SnippetError::BoundaryError`] if the snippet's boundaries cannot be resolved.
    /// Returns [`SnippetError::InvalidRange`] if the resolved range is invalid (start >= end).
    /// Returns [`SnippetError::InvalidUtf8`] if the replacement string contains null bytes.
    /// Returns [`SnippetError::OutOfBounds`] if the resolved range exceeds rope length.
    ///
    /// # Examples
    ///
    /// Insert text at a position:
    ///
    /// ```rust
    /// use textum::{Snippet, Target, Boundary, BoundaryMode};
    /// use ropey::Rope;
    ///
    /// let rope = Rope::from_str("hello world");
    /// let target = Target::Char(5); // After "hello"
    /// let boundary = Boundary::new(target, BoundaryMode::Exclude);
    /// let snippet = Snippet::At(boundary);
    ///
    /// let result = snippet.replace(&rope, ", beautiful").unwrap();
    /// assert_eq!(result.to_string(), "hello, beautiful world");
    /// ```
    ///
    /// Delete a line:
    ///
    /// ```rust
    /// use textum::{Snippet, Target, Boundary, BoundaryMode};
    /// use ropey::Rope;
    ///
    /// let rope = Rope::from_str("line1\nline2\nline3\n");
    /// let target = Target::Line(1); // Second line
    /// let boundary = Boundary::new(target, BoundaryMode::Include);
    /// let snippet = Snippet::At(boundary);
    ///
    /// let result = snippet.replace(&rope, "").unwrap();
    /// assert_eq!(result.to_string(), "line1\nline3\n");
    /// ```
    ///
    /// Replace text between boundaries:
    ///
    /// ```rust
    /// use textum::{Snippet, Target, Boundary, BoundaryMode};
    /// use ropey::Rope;
    ///
    /// let rope = Rope::from_str("<!-- comment -->text<!-- /comment -->");
    /// let start_target = Target::Literal("<!-- comment -->".to_string());
    /// let end_target = Target::Literal("<!-- /comment -->".to_string());
    /// let start_boundary = Boundary::new(start_target, BoundaryMode::Exclude);
    /// let end_boundary = Boundary::new(end_target, BoundaryMode::Exclude);
    /// let snippet = Snippet::Between { start: start_boundary, end: end_boundary };
    ///
    /// let result = snippet.replace(&rope, "new content").unwrap();
    /// assert_eq!(result.to_string(), "<!-- comment -->new content<!-- /comment -->");
    /// ```
    pub fn replace(&self, rope: &Rope, replacement: &str) -> Result<Rope, SnippetError> {
        validate_replacement_utf8(replacement)?;
        let resolution = self.resolve(rope)?;
        Ok(apply_replacement(
            rope,
            resolution.start,
            resolution.end,
            replacement,
        ))
    }
}

/// Validates that a replacement string meets UTF-8 requirements.
///
/// While Rust's `&str` type guarantees valid UTF-8, this function performs additional
/// validation to ensure the replacement text is suitable for rope operations.
///
/// # Arguments
///
/// * `s` - The replacement string to validate
///
/// # Returns
///
/// Returns `Ok(())` if the string is valid.
///
/// # Errors
///
/// Returns [`SnippetError::InvalidUtf8`] if the string contains null bytes, which are
/// not permitted in text replacements.
///
/// # Examples
///
/// ```rust
/// # use textum::snip::snippet::replacement::validate_replacement_utf8;
/// assert!(validate_replacement_utf8("hello").is_ok());
/// assert!(validate_replacement_utf8("hello\0world").is_err());
/// ```
pub fn validate_replacement_utf8(s: &str) -> Result<(), SnippetError> {
    // str is already UTF-8 valid by Rust's type system, but check for any
    // additional validation requirements (e.g., no null bytes, specific encoding)
    if s.contains('\0') {
        return Err(SnippetError::InvalidUtf8("null bytes not allowed".into()));
    }
    Ok(())
}

/// Applies a replacement operation to a rope at the specified character range.
///
/// This function creates a new rope with the specified range replaced by the given text.
/// The operation is performed by removing the range and inserting the replacement.
///
/// # Arguments
///
/// * `rope` - The source rope to modify
/// * `start` - Starting character index (inclusive)
/// * `end` - Ending character index (exclusive)
/// * `replacement` - The text to insert at the position
///
/// # Returns
///
/// Returns a new [`Rope`] with the replacement applied.
///
/// # Behavior
///
/// - If `start == end`: Performs pure insertion (no text removed)
/// - If `replacement` is empty: Performs pure deletion (no text inserted)
/// - Otherwise: Removes `[start..end)` and inserts `replacement`
///
/// # Examples
///
/// ```rust
/// # use textum::snip::snippet::replacement::apply_replacement;
/// # use ropey::Rope;
/// let rope = Rope::from_str("hello world");
///
/// // Replace "world" with "rust"
/// let result = apply_replacement(&rope, 6, 11, "rust");
/// assert_eq!(result.to_string(), "hello rust");
///
/// // Insert at position (zero-width range)
/// let result = apply_replacement(&rope, 5, 5, ",");
/// assert_eq!(result.to_string(), "hello, world");
///
/// // Delete range (empty replacement)
/// let result = apply_replacement(&rope, 5, 11, "");
/// assert_eq!(result.to_string(), "hello");
/// ```
#[must_use]
pub fn apply_replacement(rope: &Rope, start: usize, end: usize, replacement: &str) -> Rope {
    let mut new_rope = rope.clone();
    new_rope.remove(start..end);
    new_rope.insert(start, replacement);
    new_rope
}

#[cfg(test)]
#[path = "../../tests/snippet_replacement.rs"]
mod snippet_replacement;
