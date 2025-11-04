use crate::snip::boundary::{
    calculate_bytes_extent, calculate_chars_extent, calculate_lines_extent,
    calculate_matching_extent, BoundaryError,
};
use crate::snip::Target;
use ropey::Rope;

#[test]
fn test_calculate_lines_extent_success() {
    let rope = Rope::from_str("1\n2\n3\n4\n5");
    let start_line = 2;
    let extent_lines = 2;
    let from_char = rope.line_to_char(start_line); // start of line 2
    let idx = calculate_lines_extent(&rope, from_char, extent_lines).unwrap();

    let end_line = start_line.saturating_add(extent_lines);
    let extended_to_char_idx = rope.line_to_char(end_line); // start of line 4

    assert_eq!(idx, extended_to_char_idx); // start of line 4 (2 lines after line 2)
}

#[test]
fn test_calculate_lines_extent_out_of_bounds() {
    let rope = Rope::from_str("1\n2\n3");
    assert!(matches!(
        calculate_lines_extent(&rope, 0, 5),
        Err(BoundaryError::ExtentOutOfBounds)
    ));
}

#[test]
fn test_calculate_chars_extent_success() {
    let rope = Rope::from_str("abcdef");
    let idx = calculate_chars_extent(&rope, 2, 3).unwrap();
    assert_eq!(idx, 5);
}

#[test]
fn test_calculate_chars_extent_out_of_bounds() {
    let rope = Rope::from_str("abc");
    assert!(matches!(
        calculate_chars_extent(&rope, 1, 5),
        Err(BoundaryError::ExtentOutOfBounds)
    ));
}

#[test]
fn test_calculate_chars_extent_to_eof() {
    let rope = Rope::from_str("hello");
    // From char 3, extend 2 chars to reach EOF (char 5)
    let result = calculate_chars_extent(&rope, 3, 2).unwrap();
    assert_eq!(result, rope.len_chars(), "Should allow extending to EOF");
    assert_eq!(result, 5);
}

#[test]
fn test_calculate_bytes_extent_success() {
    // input: "aé😊"
    //
    //         a      é      emoji
    //
    // width:  1      2      4
    //
    // chars:  0      1      2
    //
    // bytes:  0-0    1-2    3-6
    //
    let rope = Rope::from_str("aé😊");

    // Start from 'é' (char 1), extend by 2 bytes
    let start_char = 1; // 'é' at char index 1
    let byte_count = 2; // Extend by 2 bytes

    // Verify what we're starting from
    let start_byte = rope.char_to_byte(start_char); // byte 1 (start of 'é')
    let target_byte = start_byte + byte_count; // byte 1 + 2 = byte 3

    // Byte 3 is exactly the start of the emoji char (bytes 3-6)
    let emoji_char_idx = rope.byte_to_char(target_byte); // char 2

    let result = calculate_bytes_extent(&rope, start_char, byte_count).unwrap();

    assert_eq!(
        result, emoji_char_idx,
        "Should land exactly at start of emoji (char 2)"
    );

    // Verify the segment we'd extract
    let segment = rope.slice(start_char..result);
    assert_eq!(
        segment.to_string(),
        "é",
        "Should cover just the é character"
    );
}

#[test]
fn test_calculate_bytes_extent_rounds_past_eof() {
    // input: "aé😊"
    //
    //         a      é      emoji
    //
    // width:  1      2      4
    //
    // chars:  0      1      2
    //
    // bytes:  0-0    1-2    3-6
    //
    let rope = Rope::from_str("aé😊");

    // Start from 'é' (char 1), try to extend by 3 bytes
    let start_char = 1; // 'é' at char index 1
    let byte_count = 3; // Extend by 3 bytes

    // Verify what happens:
    let start_byte = rope.char_to_byte(start_char); // byte 1 (start of 'é')
    let target_byte = start_byte + byte_count; // byte 1 + 3 = byte 4

    // Byte 4 is INSIDE the emoji (which spans bytes 3-6)
    // The emoji char starts at byte 3, so byte 4 is mid-character
    let emoji_char_idx = rope.byte_to_char(target_byte); // char 2 (the emoji containing byte 4)
    let emoji_start_byte = rope.char_to_byte(emoji_char_idx); // byte 3

    assert!(
        emoji_start_byte < target_byte,
        "Target byte 4 is inside the emoji that starts at byte 3"
    );

    // When we land mid-character, spec says "round forward to next char boundary"
    // Next char after char 2 would be char 3
    // But rope only has 3 chars (0, 1, 2), so char 3 doesn't exist!
    let next_char = emoji_char_idx + 1; // char 3
    assert_eq!(
        next_char,
        rope.len_chars(),
        "Rounding forward would need char 3, which is past EOF"
    );

    let result = calculate_bytes_extent(&rope, start_char, byte_count);

    assert!(
        matches!(result, Err(BoundaryError::ExtentOutOfBounds)),
        "Should fail because rounding forward would exceed rope length"
    );
}

#[test]
fn test_calculate_bytes_extent_out_of_bounds() {
    let rope = Rope::from_str("abc");
    assert!(matches!(
        calculate_bytes_extent(&rope, 1, 10),
        Err(BoundaryError::ExtentOutOfBounds)
    ));
}

#[test]
fn test_calculate_bytes_extent_to_eof() {
    let rope = Rope::from_str("hello");
    // From char 3 ('l'), extend 2 bytes to reach EOF
    let result = calculate_bytes_extent(&rope, 3, 2).unwrap();
    assert_eq!(result, rope.len_chars(), "Should allow extending to EOF");
    assert_eq!(result, 5);
}

#[test]
fn test_calculate_matching_extent_success() {
    let rope = Rope::from_str("a\nb\nc\nd\n");
    let target = Target::Literal("\n".to_string());
    let idx = calculate_matching_extent(&rope, 0, 3, &target).unwrap();
    assert_eq!(idx, 6); // after 3rd newline
}

#[test]
fn test_calculate_matching_extent_from_eof() {
    let rope = Rope::from_str("a\nb\n");
    let target = Target::Literal("\n".to_string());

    // Starting from EOF, we can't find any matches forward
    let result = calculate_matching_extent(&rope, rope.len_chars(), 1, &target);
    assert!(
        matches!(result, Err(BoundaryError::ExtentOutOfBounds)),
        "Should fail when starting from EOF"
    );
}

#[test]
fn test_calculate_matching_extent_insufficient_matches() {
    let rope = Rope::from_str("a\nb\n");
    let target = Target::Literal("\n".to_string());
    assert!(matches!(
        calculate_matching_extent(&rope, 0, 5, &target),
        Err(BoundaryError::ExtentOutOfBounds)
    ));
}

#[test]
fn test_calculate_matching_extent_invalid_target() {
    let rope = Rope::from_str("abc");
    let target = Target::Literal(String::new());
    assert!(matches!(
        calculate_matching_extent(&rope, 0, 1, &target),
        Err(BoundaryError::InvalidExtent)
    ));
}
