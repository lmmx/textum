//! Core patch types and application logic.
//!
//! A patch represents a single atomic edit operation on a file, defined by a character range
//! and optional replacement text. Patches can be created from line-based positions (for
//! compatibility with tools like cargo diagnostics) or directly from character indices.

use facet::Facet;
use ropey::Rope;

/// A single atomic patch operation on a file.
///
/// Patches operate at character-level granularity and can represent insertions, deletions,
/// or replacements. Character positions are 0-indexed and represent positions in the file
/// as a sequence of Unicode scalar values.
///
/// # Examples
///
/// ```
/// use textum::Patch;
/// use ropey::Rope;
///
/// // Delete characters 5-10
/// let mut rope = Rope::from_str("hello world friend");
/// let delete = Patch {
///     file: "main.rs".to_string(),
///     range: (5, 11),
///     replacement: None,
///     symbol_path: None,
///     max_line_drift: None,
/// };
/// delete.apply(&mut rope).unwrap();
/// assert_eq!(rope.to_string(), "hello friend");
///
/// // Insert text at position 5
/// let mut rope = Rope::from_str("helloworld");
/// let insert = Patch {
///     file: "main.rs".to_string(),
///     range: (5, 5),
///     replacement: Some(" ".into()),
///     symbol_path: None,
///     max_line_drift: None,
/// };
/// insert.apply(&mut rope).unwrap();
/// assert_eq!(rope.to_string(), "hello world");
///
/// // Replace characters 6-11 with new text
/// let mut rope = Rope::from_str("hello world");
/// let replace = Patch {
///     file: "main.rs".to_string(),
///     range: (6, 11),
///     replacement: Some("rust".into()),
///     symbol_path: None,
///     max_line_drift: None,
/// };
/// replace.apply(&mut rope).unwrap();
/// assert_eq!(rope.to_string(), "hello rust");
/// ```
#[derive(Debug, Clone, Facet)]
pub struct Patch {
    /// File path this patch applies to.
    pub file: String,

    /// Character index range to replace (start, end).
    ///
    /// - For insertions: start == end
    /// - For deletions: replacement is None
    /// - For replacements: start < end and replacement is Some
    ///
    /// Indices are 0-based and count Unicode scalar values, not bytes.
    pub range: (usize, usize),

    /// Replacement text to insert at the start of the range.
    ///
    /// If None, this is a pure deletion. If Some, the range is removed and this text
    /// is inserted in its place.
    // pub replacement: Option<Text>,
    pub replacement: Option<String>,

    /// Optional symbol path for robust positioning.
    ///
    /// When provided, allows the patch to be relocated if the exact character positions
    /// have shifted but the syntactic context remains identifiable. For example,
    /// `vec!["mod foo", "fn bar"]` identifies a function bar inside module foo.
    pub symbol_path: Option<Vec<String>>,

    /// Optional maximum line drift for fuzzy matching.
    ///
    /// When provided, the patch will search within this many lines of the target position
    /// to find a matching context. This allows patches to remain valid even when unrelated
    /// code changes have shifted line numbers.
    pub max_line_drift: Option<usize>,
}

/// Errors that can occur when applying patches.
#[derive(Debug)]
pub enum PatchError {
    /// The patch range exceeds the file's character count.
    RangeOutOfBounds,

    /// The target file could not be found or read.
    FileNotFound,

    /// An I/O error occurred while reading or writing files.
    IoError(std::io::Error),
}

impl std::fmt::Display for PatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RangeOutOfBounds => write!(f, "Patch range exceeds file bounds"),
            Self::FileNotFound => write!(f, "Target file not found"),
            Self::IoError(e) => write!(f, "I/O error: {e}"),
        }
    }
}

impl std::error::Error for PatchError {}

impl Patch {
    /// Apply this patch to a rope in-place.
    ///
    /// The rope is modified directly by removing the specified range and inserting the
    /// replacement text (if any). Changes are applied atomically - if the patch cannot
    /// be applied (e.g. due to out of bounds range), the rope is left unchanged.
    ///
    /// # Errors
    ///
    /// Returns `PatchError::RangeOutOfBounds` if the patch range extends beyond the
    /// rope's character count.
    ///
    /// # Examples
    ///
    /// ```
    /// use ropey::Rope;
    /// use textum::Patch;
    ///
    /// let mut rope = Rope::from_str("hello world");
    /// let patch = Patch {
    ///     file: "test.txt".to_string(),
    ///     range: (6, 11),
    ///     replacement: Some("rust".into()),
    ///     symbol_path: None,
    ///     max_line_drift: None,
    /// };
    ///
    /// patch.apply(&mut rope).unwrap();
    /// assert_eq!(rope.to_string(), "hello rust");
    /// ```
    pub fn apply(&self, rope: &mut Rope) -> Result<(), PatchError> {
        let (start, end) = self.range;

        if end > rope.len_chars() {
            return Err(PatchError::RangeOutOfBounds);
        }

        // Remove the range
        if start < end {
            rope.remove(start..end);
        }

        // Insert replacement if provided
        if let Some(ref text) = self.replacement {
            rope.insert(start, text.as_str());
        }

        Ok(())
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
    /// * `rope` - A rope containing the file content, used to convert line positions to char indices
    /// * `replacement` - Optional replacement text
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
    ///     1,  // line_start
    ///     0,  // col_start
    ///     1,  // line_end
    ///     6,  // col_end
    ///     &rope,
    ///     Some("EDITED".into()),
    /// );
    ///
    /// assert_eq!(patch.range, (7, 13));
    /// ```
    #[must_use]
    pub fn from_line_positions(
        file: String,
        line_start: usize,
        col_start: usize,
        line_end: usize,
        col_end: usize,
        rope: &Rope,
        replacement: Option<String>,
        // replacement: Option<Text>,
    ) -> Self {
        let start_char = rope.line_to_char(line_start) + col_start;
        let end_char = rope.line_to_char(line_end) + col_end;

        Self {
            file,
            range: (start_char, end_char),
            replacement,
            symbol_path: None,
            max_line_drift: None,
        }
    }
}
