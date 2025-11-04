use crate::snip::boundary::{
    calculate_bytes_extent, calculate_chars_extent, calculate_lines_extent,
    calculate_matching_extent, BoundaryError,
};
use crate::snip::Target;
use ropey::Rope;

#[test]
fn test_calculate_lines_extent_success() {
    let rope = Rope::from_str("1\n2\n3\n4\n5");
    let idx = calculate_lines_extent(&rope, 2, 2).unwrap();
    assert_eq!(idx, 8); // start of line 4
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
    let idx = calculate_bytes_extent(&rope, 1, 3).unwrap();
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
