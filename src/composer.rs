//! Composition and application of multiple patches.
//!
//! The `PatchSet` type allows you to group multiple patches and apply them together,
//! with automatic handling of offset adjustments. Patches are grouped by file and
//! applied in reverse order to maintain stable positions.

use crate::patch::{Patch, PatchError};
use ropey::Rope;
use std::collections::HashMap;

/// A collection of patches that can be applied together.
///
/// `PatchSet` handles the complexity of applying multiple patches to the same file
/// by sorting them appropriately and tracking offset changes. Patches are applied
/// in reverse order (highest position first) to avoid invalidating subsequent patches.
///
/// # Examples
///
/// ```
/// use textum::{Patch, PatchSet};
///
/// let mut set = PatchSet::new();
///
/// set.add(Patch {
///     file: "tests/fixtures/sample.txt".to_string(),
///     range: (0, 5),
///     replacement: Some("goodbye".into()),
///     #[cfg(feature = "symbol_path")]
///     symbol_path: None,
///     #[cfg(feature = "line_tol")]
///     max_line_drift: None,
/// });
///
/// set.add(Patch {
///     file: "tests/fixtures/sample.txt".to_string(),
///     range: (6, 11),
///     replacement: Some("rust".into()),
///     #[cfg(feature = "symbol_path")]
///     symbol_path: None,
///     #[cfg(feature = "line_tol")]
///     max_line_drift: None,
/// });
///
/// let results = set.apply_to_files().unwrap();
/// assert_eq!(results.get("tests/fixtures/sample.txt").unwrap(), "goodbye rust\n");
/// ```
pub struct PatchSet {
    /// The patches in this set.
    patches: Vec<Patch>,
}

impl PatchSet {
    /// Create a new empty patch set.
    ///
    /// # Examples
    ///
    /// ```
    /// use textum::PatchSet;
    ///
    /// let set = PatchSet::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            patches: Vec::new(),
        }
    }

    /// Add a patch to this set.
    ///
    /// Patches are not applied until `apply_to_files` is called. Multiple patches
    /// can target the same file.
    ///
    /// # Examples
    ///
    /// ```
    /// use textum::{Patch, PatchSet};
    ///
    /// let mut set = PatchSet::new();
    /// set.add(Patch {
    ///     file: "main.rs".to_string(),
    ///     range: (10, 15),
    ///     replacement: Some("hello".into()),
    ///     #[cfg(feature = "symbol_path")]
    ///     symbol_path: None,
    ///     #[cfg(feature = "line_tol")]
    ///     max_line_drift: None,
    /// });
    /// ```
    pub fn add(&mut self, patch: Patch) {
        self.patches.push(patch);
    }

    /// Apply all patches in this set to their target files.
    ///
    /// Patches are grouped by file and applied in reverse position order (highest
    /// character index first) to avoid invalidating subsequent patch positions.
    /// The resulting file contents are returned as a map from file path to content.
    ///
    /// This method reads files from disk, applies all patches for that file, and
    /// returns the modified content. It does not write to disk - use the returned
    /// map to write files as needed.
    ///
    /// # Errors
    ///
    /// Returns an error if any file cannot be read or if any patch has an invalid range.
    /// If an error occurs, no files are modified.
    ///
    /// # Examples
    ///
    /// ```
    /// use textum::{Patch, PatchSet};
    ///
    /// let mut set = PatchSet::new();
    /// set.add(Patch {
    ///     file: "tests/fixtures/sample.txt".to_string(),
    ///     range: (6, 11),
    ///     replacement: Some("rust".into()),
    ///     #[cfg(feature = "symbol_path")]
    ///     symbol_path: None,
    ///     #[cfg(feature = "line_tol")]
    ///     max_line_drift: None,
    /// });
    ///
    /// let results = set.apply_to_files().unwrap();
    /// assert_eq!(results.get("tests/fixtures/sample.txt").unwrap(), "hello rust\n");
    /// ```
    pub fn apply_to_files(&self) -> Result<HashMap<String, String>, PatchError> {
        let mut results = HashMap::new();

        // Group patches by file
        let mut by_file: HashMap<String, Vec<&Patch>> = HashMap::new();
        for patch in &self.patches {
            by_file.entry(patch.file.clone()).or_default().push(patch);
        }

        for (file, patches) in by_file {
            let content = std::fs::read_to_string(&file).map_err(PatchError::IoError)?;
            let mut rope = Rope::from_str(&content);

            // Sort patches by start position in reverse order for stable application
            let mut sorted = patches.clone();
            sorted.sort_by_key(|p| std::cmp::Reverse(p.range.0));

            for patch in sorted {
                patch.apply(&mut rope)?;
            }

            results.insert(file, rope.to_string());
        }

        Ok(results)
    }
}

impl Default for PatchSet {
    fn default() -> Self {
        Self::new()
    }
}
