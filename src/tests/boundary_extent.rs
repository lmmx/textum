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
fn test_calculate_bytes_extent_success() {
    let rope = Rope::from_str("aé😊");
    let start_char = 1;
    let byte_count = 3;
    let idx = calculate_bytes_extent(&rope, start_char, byte_count).unwrap();

    let segment = &rope.slice(start_char..idx);
    println!(
        "bytes_extent: start_char={} byte_count={} end_char={} segment='{}'",
        start_char, byte_count, idx, segment
    );

    assert_eq!(idx, 2); // UTF-8 boundary respected
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
fn test_calculate_matching_extent_success() {
    let rope = Rope::from_str("a\nb\nc\nd\n");
    let target = Target::Literal("\n".to_string());
    let idx = calculate_matching_extent(&rope, 0, 3, &target).unwrap();
    assert_eq!(idx, 6); // after 3rd newline
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
