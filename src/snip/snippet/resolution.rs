use super::{Snippet, SnippetError};
use ropey::Rope;

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

    if start >= end {
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
                let res = boundary.resolve(rope)?;
                validate_range(0, res.start, rope)?;
                Ok(SnippetResolution {
                    start: 0,
                    end: res.start,
                })
            }
            Snippet::Between { start, end } => {
                let start_res = start.resolve(rope)?;
                let end_res = end.resolve(rope)?;
                validate_range(start_res.end, end_res.start, rope)?;
                Ok(SnippetResolution {
                    start: start_res.end,
                    end: end_res.start,
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
