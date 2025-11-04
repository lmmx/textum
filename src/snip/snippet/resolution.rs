use super::{Extent, Snippet, SnippetError};
use ropey::Rope;

use super::boundary::{
    calculate_bytes_extent, calculate_chars_extent, calculate_lines_extent,
    calculate_matching_extent, BoundaryMode,
};

#[derive(Debug, Clone, PartialEq, Eq)]
/// The concrete start and end indices of a resolved snippet within a [`Rope`].
pub struct SnippetResolution {
    /// The starting character index of the resolved snippet.
    pub start: usize,
    /// The ending character index of the resolved snippet (exclusive).
    pub end: usize,
}

/// Validates that a range is valid and within rope bounds.
fn validate_range(start: usize, end: usize, rope: &Rope) -> Result<(), SnippetError> {
    let rope_len = rope.len_chars();

    // Allow start == end for zero-width ranges (insertions)
    if start > end {
        return Err(SnippetError::InvalidRange { start, end });
    }

    if end > rope_len {
        return Err(SnippetError::OutOfBounds {
            index: end,
            rope_len,
        });
    }

    Ok(())
}

impl Snippet {
    /// Resolves this snippet into absolute character indices.
    ///
    /// # Errors
    ///
    /// Returns [`SnippetError`] if boundaries cannot be resolved or the resulting range is invalid.
    pub fn resolve(&self, rope: &Rope) -> Result<SnippetResolution, SnippetError> {
        match self {
            Snippet::At(boundary) => {
                let res = boundary.resolve(rope)?;
                validate_range(res.start, res.end, rope)?;
                Ok(SnippetResolution {
                    start: res.start,
                    end: res.end,
                })
            }
            Snippet::From(boundary) => {
                let res = boundary.resolve(rope)?;
                let end = rope.len_chars();
                validate_range(res.end, end, rope)?;
                Ok(SnippetResolution {
                    start: res.end,
                    end,
                })
            }
            Snippet::To(boundary) => {
                let (target_start, target_end) = boundary.target.resolve_range(rope)?;

                let to_end = match &boundary.mode {
                    BoundaryMode::Exclude => target_start, // Before the target
                    BoundaryMode::Include => target_end,   // After the target
                    BoundaryMode::Extend(extent) => {
                        let extended = match extent {
                            Extent::Lines(n) => calculate_lines_extent(rope, target_end, *n)?,
                            Extent::Chars(n) => calculate_chars_extent(rope, target_end, *n)?,
                            Extent::Bytes(n) => calculate_bytes_extent(rope, target_end, *n)?,
                            Extent::Matching(n, t) => {
                                calculate_matching_extent(rope, target_end, *n, t)?
                            }
                        };
                        extended
                    }
                };

                validate_range(0, to_end, rope)?;
                Ok(SnippetResolution {
                    start: 0,
                    end: to_end,
                })
            }
            Snippet::Between { start, end } => {
                // For Between semantics:
                // - Start boundary in Exclude mode: start AFTER the target (use .end)
                // - Start boundary in Include mode: start AT the target (use .start)
                // - End boundary in Exclude mode: end BEFORE the target (need target.start)
                // - End boundary in Include mode: end AFTER the target (use .end)

                let start_res = start.resolve(rope)?;
                let (end_target_start, end_target_end) = end.target.resolve_range(rope)?;

                let between_start = start_res.start;
                let between_end = match &end.mode {
                    BoundaryMode::Exclude => end_target_start, // Before the target
                    BoundaryMode::Include => end_target_end,   // After the target
                    BoundaryMode::Extend(extent) => {
                        // Extend mode: start from end of target and extend
                        let extended = match extent {
                            Extent::Lines(n) => calculate_lines_extent(rope, end_target_end, *n)?,
                            Extent::Chars(n) => calculate_chars_extent(rope, end_target_end, *n)?,
                            Extent::Bytes(n) => calculate_bytes_extent(rope, end_target_end, *n)?,
                            Extent::Matching(n, t) => {
                                calculate_matching_extent(rope, end_target_end, *n, t)?
                            }
                        };
                        extended
                    }
                };

                validate_range(between_start, between_end, rope)?;
                Ok(SnippetResolution {
                    start: between_start,
                    end: between_end,
                })
            }
            Snippet::All => Ok(SnippetResolution {
                start: 0,
                end: rope.len_chars(),
            }),
        }
    }
}

#[cfg(test)]
#[path = "../../tests/snippet_resolution.rs"]
mod snippet_resolution;
