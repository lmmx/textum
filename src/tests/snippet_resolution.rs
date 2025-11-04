use crate::snip::snippet::{Boundary, BoundaryMode, Extent, Snippet, SnippetError};
use crate::snip::Target;
use ropey::Rope;

#[test]
fn test_resolve_at_single_line() {
    // Tests Snippet::At with a Line target resolves to correct char indices
    // Verifies that Include mode spans the entire line
    let rope = Rope::from_str("line1\nline2\nline3\n");
    let target = Target::Line(1); // Second line (0-indexed)
    let boundary = Boundary::new(target, BoundaryMode::Include);
    let snippet = Snippet::At(boundary);

    let resolution = snippet.resolve(&rope).unwrap();

    let line_start = rope.line_to_char(1); // Start of "line2\n"
    let line_end = rope.line_to_char(2); // Start of "line3\n"

    assert_eq!(resolution.start, line_start);
    assert_eq!(resolution.end, line_end);
    assert_eq!(
        rope.slice(resolution.start..resolution.end).to_string(),
        "line2\n"
    );
}

#[test]
fn test_resolve_at_exclude_mode() {
    // Tests Snippet::At with Exclude mode returns zero-width range after target
    // Verifies start == end after the boundary
    let rope = Rope::from_str("line1\nline2\nline3\n");
    let target = Target::Line(1);
    let boundary = Boundary::new(target, BoundaryMode::Exclude);
    let snippet = Snippet::At(boundary);

    let resolution = snippet.resolve(&rope).unwrap();

    let line_end = rope.line_to_char(2); // After "line2\n"

    assert_eq!(resolution.start, line_end);
    assert_eq!(resolution.end, line_end);
    assert_eq!(resolution.start, resolution.end, "Should be zero-width");
}

#[test]
fn test_resolve_from_boundary_to_eof() {
    // Tests Snippet::From resolves from boundary end to rope.len_chars()
    // Verifies that From variant correctly extends to EOF
    let rope = Rope::from_str("line1\nline2\nline3\n");
    let target = Target::Line(1);
    let boundary = Boundary::new(target, BoundaryMode::Include);
    let snippet = Snippet::From(boundary);

    let resolution = snippet.resolve(&rope).unwrap();

    let line_end = rope.line_to_char(2); // After "line2\n"
    let eof = rope.len_chars();

    assert_eq!(resolution.start, line_end);
    assert_eq!(resolution.end, eof);
    assert_eq!(
        rope.slice(resolution.start..resolution.end).to_string(),
        "line3\n"
    );
}

#[test]
fn test_resolve_to_boundary_from_bof() {
    // Tests Snippet::To resolves from 0 to boundary start
    // Verifies that To variant correctly starts at BOF
    let rope = Rope::from_str("line1\nline2\nline3\n");
    let target = Target::Line(2);
    let boundary = Boundary::new(target, BoundaryMode::Include);
    let snippet = Snippet::To(boundary);

    let resolution = snippet.resolve(&rope).unwrap();

    let bof = 0;
    let line_start = rope.line_to_char(2); // Start of "line3\n"

    assert_eq!(resolution.start, bof);
    assert_eq!(resolution.end, line_start);
    assert_eq!(
        rope.slice(resolution.start..resolution.end).to_string(),
        "line1\nline2\n"
    );
}

#[test]
fn test_resolve_between_boundaries() {
    // Tests Snippet::Between with two boundaries resolves to inner range
    // Verifies start.end to end.start span (content between markers)
    let rope = Rope::from_str("<!-- start -->content here<!-- end -->");
    let start_target = Target::Literal("<!-- start -->".to_string());
    let end_target = Target::Literal("<!-- end -->".to_string());
    let start_boundary = Boundary::new(start_target, BoundaryMode::Exclude);
    let end_boundary = Boundary::new(end_target, BoundaryMode::Exclude);
    let snippet = Snippet::Between {
        start: start_boundary,
        end: end_boundary,
    };

    let resolution = snippet.resolve(&rope).unwrap();

    let start_marker_end = 14; // After "<!-- start -->"
    let end_marker_start = 26; // Before "<!-- end -->"

    assert_eq!(resolution.start, start_marker_end);
    assert_eq!(resolution.end, end_marker_start);
    assert_eq!(
        rope.slice(resolution.start..resolution.end).to_string(),
        "content here"
    );
}

#[test]
fn test_resolve_between_asymmetric_modes() {
    // Tests Between with different boundary modes (Include vs Exclude)
    // Verifies each boundary's mode is respected independently
    let rope = Rope::from_str("<!-- start -->content here<!-- end -->");
    let start_target = Target::Literal("<!-- start -->".to_string());
    let end_target = Target::Literal("<!-- end -->".to_string());
    let start_boundary = Boundary::new(start_target, BoundaryMode::Include);
    let end_boundary = Boundary::new(end_target, BoundaryMode::Exclude);
    let snippet = Snippet::Between {
        start: start_boundary,
        end: end_boundary,
    };

    let resolution = snippet.resolve(&rope).unwrap();

    let start_marker_start = 0; // Include "<!-- start -->"
    let end_marker_start = 26; // Before "<!-- end -->"

    assert_eq!(resolution.start, start_marker_start);
    assert_eq!(resolution.end, end_marker_start);
    assert_eq!(
        rope.slice(resolution.start..resolution.end).to_string(),
        "<!-- start -->content here"
    );
}

#[test]
fn test_resolve_all_entire_rope() {
    // Tests Snippet::All resolves to (0, rope.len_chars())
    // Verifies complete rope selection
    let rope = Rope::from_str("line1\nline2\nline3\n");
    let snippet = Snippet::All;

    let resolution = snippet.resolve(&rope).unwrap();

    let bof = 0;
    let eof = rope.len_chars();

    assert_eq!(resolution.start, bof);
    assert_eq!(resolution.end, eof);
    assert_eq!(
        rope.slice(resolution.start..resolution.end).to_string(),
        "line1\nline2\nline3\n"
    );
}

#[test]
fn test_resolve_invalid_range_error() {
    // Tests that start >= end after resolution produces InvalidRange error
    // Uses Between with reversed boundaries to trigger error
    let rope = Rope::from_str("abc<!-- end --><!-- start -->xyz");
    let start_target = Target::Literal("<!-- start -->".to_string());
    let end_target = Target::Literal("<!-- end -->".to_string());
    let start_boundary = Boundary::new(start_target, BoundaryMode::Exclude);
    let end_boundary = Boundary::new(end_target, BoundaryMode::Exclude);
    let snippet = Snippet::Between {
        start: start_boundary,
        end: end_boundary,
    };

    let result = snippet.resolve(&rope);

    assert!(matches!(result, Err(SnippetError::InvalidRange { .. })));
}

#[test]
fn test_resolve_out_of_bounds_error() {
    // Tests that resolved range exceeding rope length produces OutOfBounds error
    // Uses Extend mode that goes past EOF
    let rope = Rope::from_str("short");
    let target = Target::Char(3);
    let boundary = Boundary::new(target, BoundaryMode::Extend(Extent::Chars(100)));
    let snippet = Snippet::At(boundary);

    let result = snippet.resolve(&rope);

    assert!(matches!(result, Err(SnippetError::BoundaryError(_))));
}

#[test]
fn test_resolve_boundary_error_propagation() {
    // Tests that BoundaryError from target resolution propagates correctly
    // Uses non-existent Literal target to trigger NotFound error
    let rope = Rope::from_str("hello world");
    let target = Target::Literal("nonexistent".to_string());
    let boundary = Boundary::new(target, BoundaryMode::Include);
    let snippet = Snippet::At(boundary);

    let result = snippet.resolve(&rope);

    assert!(matches!(result, Err(SnippetError::BoundaryError(_))));
}

#[test]
fn test_resolve_with_extend_mode() {
    // Tests Snippet::At with Extend(Lines(2)) resolves correctly
    // Verifies extent calculation is applied properly
    let rope = Rope::from_str("line1\nline2\nline3\nline4\nline5\n");
    let target = Target::Line(1); // "line2\n"
    let extent_lines = 2;
    let boundary = Boundary::new(target, BoundaryMode::Extend(Extent::Lines(extent_lines)));
    let snippet = Snippet::At(boundary);

    let resolution = snippet.resolve(&rope).unwrap();

    let target_line_end = rope.line_to_char(2); // After "line2\n"
    let extended_line = rope.char_to_line(target_line_end) + extent_lines; // 2 lines forward
    let extended_char = rope.line_to_char(extended_line);

    assert_eq!(resolution.start, target_line_end);
    assert_eq!(resolution.end, extended_char);
    assert_eq!(
        rope.slice(resolution.start..resolution.end).to_string(),
        "line3\nline4\n"
    );
}
