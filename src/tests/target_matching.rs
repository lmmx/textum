use crate::snip::target::error::TargetError;
use crate::snip::Target;
use ropey::Rope;

#[test]
fn test_resolve_char_valid() {
    let rope = Rope::from_str("hello world");
    let target = Target::Char(6);
    assert_eq!(target.resolve(&rope).unwrap(), 6);
}

#[test]
fn test_resolve_char_at_end() {
    let rope = Rope::from_str("hello");
    let target = Target::Char(5);
    assert!(matches!(
        target.resolve(&rope),
        Err(TargetError::OutOfBounds)
    ));
}

#[test]
fn test_resolve_char_out_of_bounds() {
    let rope = Rope::from_str("hello");
    let target = Target::Char(10);
    assert!(matches!(
        target.resolve(&rope),
        Err(TargetError::OutOfBounds)
    ));
}

#[test]
fn test_resolve_line_first() {
    let rope = Rope::from_str("hello\nworld\n");
    let target = Target::Line(0);
    assert_eq!(target.resolve(&rope).unwrap(), 0);
}

#[test]
fn test_resolve_line_second() {
    let rope = Rope::from_str("hello\nworld\n");
    let target = Target::Line(1);
    assert_eq!(target.resolve(&rope).unwrap(), 6);
}

#[test]
fn test_resolve_line_out_of_bounds() {
    let rope = Rope::from_str("hello\nworld\n");
    let target = Target::Line(5);
    assert!(matches!(
        target.resolve(&rope),
        Err(TargetError::InvalidPosition { line: 5, col: None })
    ));
}

#[test]
fn test_resolve_position_valid() {
    let rope = Rope::from_str("hello\nworld\n");
    // Line 2 (one-indexed), column 1 (one-indexed) = 'w' at char index 6
    let target = Target::Position { line: 2, col: 1 };
    assert_eq!(target.resolve(&rope).unwrap(), 6);
}

#[test]
fn test_resolve_position_mid_line() {
    let rope = Rope::from_str("hello\nworld\n");
    // Line 1, column 3 = 'l' at char index 2
    let target = Target::Position { line: 1, col: 3 };
    assert_eq!(target.resolve(&rope).unwrap(), 2);
}

#[test]
fn test_resolve_position_invalid_line() {
    let rope = Rope::from_str("hello\nworld\n");
    let target = Target::Position { line: 10, col: 1 };
    assert!(matches!(
        target.resolve(&rope),
        Err(TargetError::InvalidPosition {
            line: 10,
            col: Some(1)
        })
    ));
}

#[test]
fn test_resolve_position_invalid_column() {
    let rope = Rope::from_str("hello\nworld\n");
    let target = Target::Position { line: 1, col: 20 };
    assert!(matches!(
        target.resolve(&rope),
        Err(TargetError::InvalidPosition {
            line: 1,
            col: Some(20)
        })
    ));
}

#[test]
fn test_resolve_literal_found() {
    let rope = Rope::from_str("hello world");
    let target = Target::Literal("world".to_string());
    assert_eq!(target.resolve(&rope).unwrap(), 6);
}

#[test]
fn test_resolve_literal_not_found() {
    let rope = Rope::from_str("hello world");
    let target = Target::Literal("rust".to_string());
    assert!(matches!(target.resolve(&rope), Err(TargetError::NotFound)));
}

#[test]
fn test_resolve_literal_empty() {
    let rope = Rope::from_str("hello");
    let target = Target::Literal("".to_string());
    assert_eq!(target.resolve(&rope).unwrap(), 0);
}

#[test]
fn test_resolve_literal_at_start() {
    let rope = Rope::from_str("hello world");
    let target = Target::Literal("hello".to_string());
    assert_eq!(target.resolve(&rope).unwrap(), 0);
}

#[cfg(feature = "regex")]
#[test]
fn test_resolve_pattern_found() {
    let rope = Rope::from_str("hello world 123");
    let target = Target::pattern(r"\d+").unwrap();
    assert_eq!(target.resolve(&rope).unwrap(), 12);
}

#[cfg(feature = "regex")]
#[test]
fn test_resolve_pattern_not_found() {
    let rope = Rope::from_str("hello world");
    let target = Target::pattern(r"\d+").unwrap();
    assert!(matches!(target.resolve(&rope), Err(TargetError::NotFound)));
}
