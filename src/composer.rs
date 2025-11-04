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
/// use textum::{Patch, PatchSet, BoundaryMode};
///
/// let mut set = PatchSet::new();
///
/// set.add(Patch::from_literal_target(
///     "tests/fixtures/sample.txt".to_string(),
///     "hello",
///     BoundaryMode::Include,
///     "goodbye",
/// ));
///
/// set.add(Patch::from_literal_target(
///     "tests/fixtures/sample.txt".to_string(),
///     "world",
///     BoundaryMode::Include,
///     "rust",
/// ));
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
    /// use textum::{Patch, PatchSet, BoundaryMode};
    ///
    /// let mut set = PatchSet::new();
    /// set.add(Patch::from_literal_target(
    ///     "main.rs".to_string(),
    ///     "old",
    ///     BoundaryMode::Include,
    ///     "new",
    /// ));
    /// ```
    pub fn add(&mut self, patch: Patch) {
        self.patches.push(patch);
    }

    /// Apply all patches in this set to their target files.
    ///
    /// Patches are grouped by file and all snippets are resolved before sorting.
    /// Resolved ranges are validated for overlaps - if two patches with non-empty
    /// replacements have overlapping resolved ranges, an error is returned.
    ///
    /// Patches are then sorted by reverse character index (highest first) and applied
    /// sequentially to maintain stable positions. The resulting file contents are
    /// returned as a map from file path to content.
    ///
    /// This method reads files from disk, applies all patches for that file, and
    /// returns the modified content. It does not write to disk - use the returned
    /// map to write files as needed.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - any file cannot be read,
    /// - any snippet cannot be resolved,
    /// - resolved ranges overlap with non-empty replacements,
    /// - or any patch has an invalid range.
    ///
    /// If an error occurs, no files are modified.
    ///
    /// # Examples
    ///
    /// ```
    /// use textum::{Patch, PatchSet};
    ///
    /// let mut set = PatchSet::new();
    /// set.add(Patch::from_literal_target(
    ///     "tests/fixtures/sample.txt".to_string(),
    ///     "world",
    ///     textum::BoundaryMode::Include,
    ///     "rust",
    /// ));
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
            let rope = Rope::from_str(&content);

            // Resolve all snippets to concrete ranges
            let mut resolved: Vec<(&Patch, (usize, usize))> = Vec::new();
            for patch in &patches {
                let resolution = patch.snippet.resolve(&rope)?;
                let range = (resolution.start, resolution.end);
                resolved.push((patch, range));
            }

            // Check for overlapping ranges with non-empty replacements
            for i in 0..resolved.len() {
                for j in (i + 1)..resolved.len() {
                    let (patch1, range1) = resolved[i];
                    let (patch2, range2) = resolved[j];

                    // Check if ranges overlap
                    let overlaps = range1.0 < range2.1 && range2.0 < range1.1;

                    if overlaps && !patch1.replacement.is_empty() && !patch2.replacement.is_empty()
                    {
                        return Err(PatchError::OverlappingRanges { range1, range2 });
                    }
                }
            }

            // Sort by reverse position for stable application
            resolved.sort_by_key(|(_, range)| std::cmp::Reverse(range.0));

            // Apply patches in reverse order
            let mut rope = rope;
            for (patch, _) in resolved {
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
