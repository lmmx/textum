use std::hash::Hash;

use ropey::Rope;

use super::BoundaryError;
use crate::snip::Target;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Measures distance for boundary extension.
///
/// May be given in absolute terms or subject to some target(s).
pub enum Extent {
    /// Extends by a line count.
    Lines(usize),
    /// Extends by a character count.
    Chars(usize),
    /// Extends by a byte count.
    Bytes(usize),
    /// Extends by a particular count of pattern matches.
    Matching(usize, Target),
}

/// Extends `end` by `count` lines (or fewer if hitting EOF).
///
/// Moves `count` lines forward from `from` and returns the char index at the start of that line.
///
/// # Arguments
///
/// * `rope` - The rope to navigate.
/// * `from` - Starting character index (inclusive).
/// * `count` - Number of lines to extend forward.
///
/// # Returns
///
/// Returns `Ok(char_index)` at the start of the target line.
///
/// # Errors
///
/// Returns [`BoundaryError::ExtentOutOfBounds`] if the target line
/// does not exist.
///
/// # Examples
///
/// Here we extend 3 lines from `from = 20`. The example shows which line start is returned.
///
/// ```rust
/// # use ropey::Rope;
/// # use textum::snip::snippet::boundary::calculate_lines_extent;
/// let rope = Rope::from("Line 1\nLine 2\nLine 3\nLine 4\nLine 5");
/// assert_eq!(calculate_lines_extent(&rope, 20, 3).unwrap(), 42);
/// ```
pub fn calculate_lines_extent(
    rope: &Rope,
    from: usize,
    count: usize,
) -> Result<usize, BoundaryError> {
    // Determine the line that `from` is inside
    let start_line = rope.char_to_line(from);
    // Target line index (zero-based)
    let target_line = start_line.saturating_add(count);

    // Must exist strictly within available lines
    if target_line >= rope.len_lines() {
        return Err(BoundaryError::ExtentOutOfBounds);
    }

    Ok(rope.line_to_char(target_line))
}

/// Extends `end` by `count` characters.
///
/// Returns `from + count` with bounds checking against the rope length.
///
/// # Arguments
///
/// * `rope` - The rope to navigate.
/// * `from` - Starting character index.
/// * `count` - Number of characters to extend forward.
///
/// # Returns
///
/// Returns `Ok(char_index)` equal to `from + count`.
///
/// # Errors
///
/// Returns [`BoundaryError::ExtentOutOfBounds`] if the target char index
/// would exceed the rope length.
///
/// # Examples
///
/// Here we extend 4 characters from `from = 3` and also show a case where extending past the rope length fails.
///
/// ```rust
/// # use ropey::Rope;
/// # use textum::snip::snippet::boundary::calculate_chars_extent;
/// let rope = Rope::from("Hello, world");
/// assert_eq!(calculate_chars_extent(&rope, 3, 4).unwrap(), 7);
/// assert!(calculate_chars_extent(&rope, 10, 5).is_err());
/// ```
pub fn calculate_chars_extent(
    rope: &Rope,
    from: usize,
    count: usize,
) -> Result<usize, BoundaryError> {
    let new_end = from.saturating_add(count);
    if new_end > rope.len_chars() {
        return Err(BoundaryError::ExtentOutOfBounds);
    }
    Ok(new_end)
}

/// Extends `end` by `count` bytes (UTF-8 safe).
///
/// Converts `from` to bytes, adds `count` bytes, and returns the corresponding char index,
/// rounding forward if the byte position falls inside a multi-byte character.
///
/// # Arguments
///
/// * `rope` - The rope to navigate.
/// * `from` - Starting character index.
/// * `count` - Number of bytes to extend forward.
///
/// # Returns
///
/// Returns `Ok(char_index)` corresponding to the position after advancing `count` bytes.
///
/// # Errors
///
/// Returns [`BoundaryError::ExtentOutOfBounds`] if the resulting index
/// would exceed the rope length.
///
/// # Examples
///
/// Here we extend 2 bytes from `from = 6` in "hello 🎉". This demonstrates that extending into a multi-byte
/// emoji rounds to the next character boundary, returning the full emoji rather than a mid-character slice.
///
/// ```rust
/// # use ropey::Rope;
/// # use textum::snip::snippet::boundary::calculate_bytes_extent;
/// let rope = Rope::from("hello 🎉");
/// assert_eq!(calculate_bytes_extent(&rope, 6, 2).unwrap(), 7);
/// ```
pub fn calculate_bytes_extent(
    rope: &Rope,
    from: usize,
    count: usize,
) -> Result<usize, BoundaryError> {
    let from_byte = rope.char_to_byte(from);
    let new_byte = from_byte.saturating_add(count);

    if new_byte > rope.len_bytes() {
        return Err(BoundaryError::ExtentOutOfBounds);
    }

    // rope.byte_to_char returns the char index containing the byte.
    // If `new_byte` is in the middle of a char, byte_to_char gives the char that byte belongs to;
    // spec requires rounding *forward* to the next char boundary, so detect that case.
    let char_idx = rope.byte_to_char(new_byte);
    let char_start_byte = rope.char_to_byte(char_idx);
    if char_start_byte < new_byte {
        // new_byte is inside the character that starts at char_idx -> move to next char
        let next = char_idx.saturating_add(1);
        if next > rope.len_chars() {
            return Err(BoundaryError::ExtentOutOfBounds);
        }
        Ok(next)
    } else {
        Ok(char_idx)
    }
}

/// Extends `end` by `count` occurrences of `target`.
///
/// Finds `count` occurrences of `target` forward from `from`, returning the char index immediately
/// after the last match.
///
/// # Arguments
///
/// * `rope` - The rope to navigate.
/// * `from` - Starting character index.
/// * `count` - Number of occurrences to extend forward.
/// * `target` - The pattern or literal to match.
///
/// # Returns
///
/// Returns `Ok(char_index)` immediately after the last match found.
///
/// # Errors
///
/// Returns [`BoundaryError::ExtentOutOfBounds`] if fewer than `count` matches
/// are found before the end of the rope.
/// Returns [`BoundaryError::InvalidExtent`] if the target type is not supported
/// for "Matching" extents (e.g., empty literals).
///
/// # Examples
///
/// Here we search for 3 occurrences of "\n" starting at `from = 20` to illustrate finding the Nth match.
///
/// ```rust
/// # use ropey::Rope;
/// # use textum::snip::snippet::boundary::calculate_matching_extent;
/// # use textum::snip::Target;
/// let rope = Rope::from("Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6");
/// let target = Target::Literal("\n".to_string());
/// assert_eq!(calculate_matching_extent(&rope, 20, 3, &target).unwrap(), 42);
/// ```
pub fn calculate_matching_extent(
    rope: &Rope,
    from: usize,
    count: usize,
    target: &Target,
) -> Result<usize, BoundaryError> {
    if count == 0 {
        return Ok(from);
    }

    // Check for invalid target types FIRST
    match target {
        Target::Literal(needle) if needle.is_empty() => {
            // Ambiguous: empty needle would match everywhere; treat as invalid for extent.
            return Err(BoundaryError::InvalidExtent);
        }
        Target::Line(_) | Target::Char(_) | Target::Position { .. } => {
            // Other Target kinds not meaningful for "Matching" (treat as invalid)
            return Err(BoundaryError::InvalidExtent);
        }
        Target::Literal(_) => {} // Valid case: Literal with content
        #[cfg(feature = "regex")]
        Target::Pattern { .. } => {} // Valid case: Pattern
    }

    if from > rope.len_chars() {
        return Err(BoundaryError::ExtentOutOfBounds);
    }

    let total_chars = rope.len_chars();
    let mut remaining = count;
    let mut cursor = from;

    while remaining > 0 {
        if cursor >= total_chars {
            return Err(BoundaryError::ExtentOutOfBounds);
        }

        match target {
            Target::Literal(needle) => {
                // Iterate chunks starting from the chunk that contains `cursor`.
                let (chunks_iter, mut chunk_byte_idx, mut chunk_char_idx, _) =
                    rope.chunks_at_char(cursor);

                let mut found = false;
                for chunk in chunks_iter {
                    // compute char offset inside this chunk where we begin searching
                    let local_char_offset = cursor.saturating_sub(chunk_char_idx);

                    // Convert local_char_offset (chars) to a byte offset inside `chunk`.
                    // Use char_indices to find the byte offset of that char.
                    let mut byte_offset_in_chunk = 0usize;
                    if local_char_offset > 0 {
                        let mut set = false;
                        for (reached, (b_idx, _ch)) in chunk.char_indices().enumerate() {
                            if reached == local_char_offset {
                                byte_offset_in_chunk = b_idx;
                                set = true;
                                break;
                            }
                        }
                        if !set {
                            // local_char_offset is at or past the end of this chunk's chars:
                            byte_offset_in_chunk = chunk.len();
                        }
                    }

                    // Search for needle starting at byte_offset_in_chunk within this chunk.
                    if byte_offset_in_chunk <= chunk.len() {
                        if let Some(rel_byte_pos) = chunk[byte_offset_in_chunk..].find(needle) {
                            // Compute absolute byte index of end-of-match (byte index immediately after match)
                            let match_end_byte =
                                chunk_byte_idx + byte_offset_in_chunk + rel_byte_pos + needle.len();
                            // Convert to char index (this should land on a char boundary)
                            let match_end_char = rope.byte_to_char(match_end_byte);
                            cursor = match_end_char;
                            found = true;
                            break;
                        }
                    }

                    // Advance to next chunk: update chunk_byte_idx and chunk_char_idx
                    let chunk_char_count = chunk.chars().count();
                    chunk_char_idx = chunk_char_idx.saturating_add(chunk_char_count);
                    chunk_byte_idx = chunk_byte_idx.saturating_add(chunk.len());
                }

                if !found {
                    return Err(BoundaryError::ExtentOutOfBounds);
                }

                remaining = remaining.saturating_sub(1);
            }

            #[cfg(feature = "regex")]
            Target::Pattern { regex, .. } => {
                use regex_cursor::{Input as RegexInput, RopeyCursor};
                // Create a RopeSlice from cursor to the end (zero-copy)
                let slice = rope.slice(cursor..);
                let cursor_struct = RopeyCursor::new(slice);
                let input = RegexInput::new(cursor_struct);
                if let Some(m) = regex.find(input) {
                    // m.end() is a char index relative to the slice; add cursor to get global
                    cursor = cursor.saturating_add(m.end());
                    remaining = remaining.saturating_sub(1);
                } else {
                    return Err(BoundaryError::ExtentOutOfBounds);
                }
            }

            _ => unreachable!(), // {Line|Char|Position} can never reach here due to the early return
        }
    }

    Ok(cursor)
}

#[cfg(test)]
#[path = "../../../tests/boundary_extent.rs"]
mod boundary_extent;
