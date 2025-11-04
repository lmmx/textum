//! Target resolution to rope indices.

use super::error::TargetError;
use crate::snip::Target;
use ropey::Rope;

impl Target {
    /// Resolves this target to a character index in the given rope.
    ///
    /// Returns the first occurrence for `Literal` and `Pattern` targets.
    /// Returns the character index at the start of the line for `Line` targets.
    /// Returns the character index for `Char` targets if within bounds.
    /// Returns the character index for `Position` targets, converting from one-indexed line/col.
    ///
    /// # Errors
    ///
    /// Returns [`TargetError::NotFound`] if a `Literal` or `Pattern` target has no match.
    /// Returns [`TargetError::OutOfBounds`] if a `Char` target exceeds rope length.
    /// Returns [`TargetError::InvalidPosition`] if a `Line` or `Position` target refers to
    /// a line or column that does not exist in the rope.
    ///
    /// # Examples
    ///
    /// ```
    /// use textum::Target;
    /// use ropey::Rope;
    ///
    /// let rope = Rope::from_str("hello\nworld\n");
    ///
    /// // Line target (0-indexed)
    /// let line_target = Target::Line(1);
    /// assert_eq!(line_target.resolve(&rope).unwrap(), 6);
    ///
    /// // Char target
    /// let char_target = Target::Char(7);
    /// assert_eq!(char_target.resolve(&rope).unwrap(), 7);
    ///
    /// // Position target (1-indexed)
    /// let pos_target = Target::Position { line: 2, col: 1 };
    /// assert_eq!(pos_target.resolve(&rope).unwrap(), 6);
    /// ```
    pub fn resolve(&self, rope: &Rope) -> Result<usize, TargetError> {
        match self {
            Target::Literal(s) => resolve_literal(rope, s),
            #[cfg(feature = "regex")]
            Target::Pattern { regex, .. } => resolve_pattern(rope, regex),
            Target::Line(n) => resolve_line(rope, *n),
            Target::Char(n) => resolve_char(rope, *n),
            Target::Position { line, col } => resolve_position(rope, *line, *col),
        }
    }
}

/// Resolves a literal string target to its first occurrence in the rope.
fn resolve_literal(rope: &Rope, needle: &str) -> Result<usize, TargetError> {
    if needle.is_empty() {
        return Ok(0);
    }

    let needle_chars: Vec<char> = needle.chars().collect();
    let mut char_idx = 0;
    let mut chars_iter = rope.chars();

    while let Some(c) = chars_iter.next() {
        if c == needle_chars[0] {
            // Potential match found, check remaining characters
            let start_idx = char_idx;
            let mut match_idx = 1;
            let mut lookahead = chars_iter.clone();

            while match_idx < needle_chars.len() {
                match lookahead.next() {
                    Some(ch) if ch == needle_chars[match_idx] => {
                        match_idx += 1;
                    }
                    _ => break,
                }
            }

            if match_idx == needle_chars.len() {
                return Ok(start_idx);
            }
        }
        char_idx += 1;
    }

    Err(TargetError::NotFound)
}

/// Resolves a regex pattern target to its first match in the rope.
#[cfg(feature = "regex")]
fn resolve_pattern(
    rope: &Rope,
    regex: &regex_cursor::engines::meta::Regex,
) -> Result<usize, TargetError> {
    use regex_cursor::{Input as RegexInput, RopeyCursor};

    let cursor = RopeyCursor::new(rope.slice(..));
    let input = RegexInput::new(cursor);

    regex
        .find(input)
        .map(|m| m.start())
        .ok_or(TargetError::NotFound)
}

/// Resolves a line number target to the character index at the start of that line.
fn resolve_line(rope: &Rope, line: usize) -> Result<usize, TargetError> {
    if line >= rope.len_lines() {
        return Err(TargetError::InvalidPosition { line, col: None });
    }
    Ok(rope.line_to_char(line))
}

/// Resolves a character index target, validating it is within bounds.
fn resolve_char(rope: &Rope, char_idx: usize) -> Result<usize, TargetError> {
    if char_idx >= rope.len_chars() {
        return Err(TargetError::OutOfBounds);
    }
    Ok(char_idx)
}

/// Resolves a position target (one-indexed line and column) to a character index.
fn resolve_position(rope: &Rope, line: usize, col: usize) -> Result<usize, TargetError> {
    // Convert from one-indexed to zero-indexed
    let line_idx = line.saturating_sub(1);
    let col_idx = col.saturating_sub(1);

    // Validate line exists
    if line_idx >= rope.len_lines() {
        return Err(TargetError::InvalidPosition {
            line,
            col: Some(col),
        });
    }

    let line_start = rope.line_to_char(line_idx);
    let line_end = if line_idx + 1 < rope.len_lines() {
        rope.line_to_char(line_idx + 1)
    } else {
        rope.len_chars()
    };

    let line_len = line_end - line_start;

    // Validate column exists within line
    if col_idx >= line_len {
        return Err(TargetError::InvalidPosition {
            line,
            col: Some(col),
        });
    }

    Ok(line_start + col_idx)
}

#[cfg(test)]
#[path = "../../tests/target_matching.rs"]
mod target_matching;
