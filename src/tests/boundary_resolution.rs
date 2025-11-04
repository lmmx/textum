use crate::snip::boundary::{Boundary, BoundaryMode, Extent};
use crate::snip::Target;
use ropey::Rope;

#[test]
fn test_resolve_exclude_mode() {
    let rope = Rope::from_str("alpha\nbeta\ngamma\n");
    let target = Target::Line(1);
    let boundary = Boundary::new(target, BoundaryMode::Exclude);

    let resolved = boundary.resolve(&rope).unwrap();
    let line_start = rope.line_to_char(1);
    let line_end = rope.line_to_char(2);

    assert!(
        line_start < line_end,
        "line start should be before line end"
    );

    // Exclude sets start = end
    assert_eq!(resolved.start, line_end);
    assert_eq!(resolved.end, line_end);
}

#[test]
fn test_resolve_include_mode() {
    let rope = Rope::from_str("alpha\nbeta\ngamma\n");
    let target = Target::Line(1);
    let boundary = Boundary::new(target, BoundaryMode::Include);

    let resolved = boundary.resolve(&rope).unwrap();
    let line_start = rope.line_to_char(1);
    let line_end = rope.line_to_char(2);

    assert_eq!(resolved.start, line_start);
    assert_eq!(resolved.end, line_end);
}

#[test]
fn test_resolve_extend_lines() {
    let rope = Rope::from_str("one\ntwo\nthree\nfour\n");
    let target = Target::Line(1);
    let boundary = Boundary::new(target.clone(), BoundaryMode::Extend(Extent::Lines(2)));

    let resolved = boundary.resolve(&rope).unwrap();

    let start = rope.line_to_char(target.resolve(&rope).unwrap() + 1); // after target line
    let expected_end = rope.line_to_char(1 + 2); // extend 2 lines from line 1
    let segment = &rope.slice(resolved.start..resolved.end);

    println!(
        "resolve_extend_lines: start={} end={} expected_end={} segment='{}'",
        resolved.start, resolved.end, expected_end, segment
    );

    assert_eq!(resolved.start, start);
    assert_eq!(resolved.end, expected_end);
}

#[test]
fn test_resolve_extend_chars() {
    let rope = Rope::from_str("abcdefg");
    let target = Target::Char(2);
    let boundary = Boundary::new(target, BoundaryMode::Extend(Extent::Chars(3)));

    let resolved = boundary.resolve(&rope).unwrap();
    assert_eq!(resolved.start, 3); // Char at index after target
    assert_eq!(resolved.end, 6); // 3 chars forward
}

#[test]
fn test_resolve_extend_bytes() {
    let rope = Rope::from_str("hello 🎉");
    let target = Target::Char(6); // the space before emoji
    let boundary = Boundary::new(target, BoundaryMode::Extend(Extent::Bytes(4)));

    let resolved = boundary.resolve(&rope);
    println!("resolve_extend_bytes: result={:?}", resolved);

    let resolved = resolved.unwrap();
    let segment = &rope.slice(resolved.start..resolved.end);
    println!(
        "start={} end={} segment='{}'",
        resolved.start, resolved.end, segment
    );

    assert_eq!(resolved.start, 7);
    assert_eq!(resolved.end, 8);
}

#[test]
fn test_resolve_extend_matching_literal() {
    let rope = Rope::from_str("a\nb\nc\nd\n");
    let target = Target::Line(0);
    let needle = Target::Literal("\n".to_string());
    let boundary = Boundary::new(target, BoundaryMode::Extend(Extent::Matching(2, needle)));

    let resolved = boundary.resolve(&rope).unwrap();
    let start = rope.line_to_char(1); // after target line
    let end = rope.line_to_char(3); // after 2 matches
    assert_eq!(resolved.start, start);
    assert_eq!(resolved.end, end);
}

#[test]
fn test_extend_matching_invalid() {
    let rope = Rope::from_str("abc");
    let target = Target::Line(0);
    let empty_literal = Target::Literal(String::new());
    let boundary = Boundary::new(
        target,
        BoundaryMode::Extend(Extent::Matching(1, empty_literal)),
    );

    let result = boundary.resolve(&rope);

    if let Err(e) = &result {
        println!("extend_matching_invalid: error={:?}", e);
    }

    assert!(matches!(
        result,
        Err(crate::snip::boundary::BoundaryError::InvalidExtent)
    ));
}

#[test]
fn test_extend_out_of_bounds() {
    let rope = Rope::from_str("abc");
    let target = Target::Char(1);
    let boundary = Boundary::new(target, BoundaryMode::Extend(Extent::Chars(10)));

    let result = boundary.resolve(&rope);
    assert!(matches!(
        result,
        Err(crate::snip::boundary::BoundaryError::ExtentOutOfBounds)
    ));
}
