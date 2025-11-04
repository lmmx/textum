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

    // Target line 1 ("two\n")
    let target_line = 1;
    let extent_lines = 2;

    let target = Target::Line(target_line);
    let boundary = Boundary::new(target, BoundaryMode::Extend(Extent::Lines(extent_lines)));

    // First, understand what the target resolves to
    let _target_line_start = rope.line_to_char(target_line); // char 4
    let target_line_end = rope.line_to_char(target_line + 1); // char 8

    // With Extend mode, we start from target_line_end (char 8)
    // Char 8 is the start of line 2 ("three\n")
    let extend_from_line = rope.char_to_line(target_line_end); // line 2

    // Extending 2 lines forward from line 2
    let extended_to_line = extend_from_line + extent_lines; // line 4
    let extended_to_char = rope.line_to_char(extended_to_line); // char 19

    let resolved = boundary.resolve(&rope).unwrap();

    assert_eq!(
        resolved.start, target_line_end,
        "Start should be at end of target line"
    );
    assert_eq!(
        resolved.end, extended_to_char,
        "End should be 2 lines forward from where we started extending"
    );
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
    // input: "hello 🎉"
    //
    //         h      e      l      l      o      space    emoji
    //
    // width:  1      1      1      1      1      1        4
    //
    // chars:  0      1      2      3      4      5        6
    //
    // bytes:  0-0    1-1    2-2    3-3    4-4    5-5      6-9
    //
    let rope = Rope::from_str("hello 🎉");
    // Rope structure:
    // chars 0-4: "hello"
    // char 5: " " (space)
    // char 6: "🎉" (emoji, 4 bytes)

    // Target the SPACE (char 5), not the emoji
    let space_char_idx = 5; // Target the space which is at char 5
    let ext_size = 4; // Extend by 4 bytes as the emoji is 4 bytes long
    let target = Target::Char(space_char_idx);
    let boundary = Boundary::new(target, BoundaryMode::Extend(Extent::Bytes(ext_size)));
    let resolved = boundary.resolve(&rope).unwrap();

    // resolve_range for Char(5) gives (5, 6)
    // So we extend from char 6 (start of emoji)
    let target_end = space_char_idx + 1; // char 6
                                         //
    assert_eq!(resolved.start, target_end, "Should start at end of space");

    // Extending 4 bytes from char 6
    let start_byte = rope.char_to_byte(target_end); // byte 6 (start of emoji)
    let target_byte = start_byte + ext_size; // byte 10

    // Byte 10 is exactly at rope.len_bytes() (EOF)
    // This should map to char 7 (which equals rope.len_chars())
    let result_char = rope.byte_to_char(target_byte);

    assert_eq!(
        resolved.end, result_char,
        "Should extend through the entire emoji"
    );
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
