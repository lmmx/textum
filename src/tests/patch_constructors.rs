//! Tests for Patch constructor methods.

use crate::patch::Patch;
use crate::snip::snippet::BoundaryMode;
use ropey::Rope;

#[test]
fn test_from_literal_target_basic() {
    let patch = Patch::from_literal_target(
        "test.txt".to_string(),
        "needle",
        BoundaryMode::Include,
        "replacement",
    );

    assert_eq!(patch.file, "test.txt");
    assert_eq!(patch.replacement, "replacement");

    let rope = Rope::from_str("find the needle here");
    let resolution = patch.snippet.resolve(&rope).unwrap();
    assert_eq!(resolution.start, 9);
    assert_eq!(resolution.end, 15);
}

#[test]
fn test_from_literal_target_exclude_mode() {
    let patch = Patch::from_literal_target(
        "test.txt".to_string(),
        "marker",
        BoundaryMode::Exclude,
        "inserted",
    );

    let rope = Rope::from_str("before marker after");
    let resolution = patch.snippet.resolve(&rope).unwrap();
    // Exclude mode: starts after "marker"
    assert_eq!(resolution.start, 13);
    assert_eq!(resolution.end, 13);
}

#[test]
fn test_from_line_range_basic() {
    let patch = Patch::from_line_range(
        "test.txt".to_string(),
        1,
        3,
        "new content\n",
    );

    let rope = Rope::from_str("line 0\nline 1\nline 2\nline 3\n");
    let resolution = patch.snippet.resolve(&rope).unwrap();

    // Should span from start of line 1 to start of line 3
    let line1_start = rope.line_to_char(1);
    let line3_start = rope.line_to_char(3);
    assert_eq!(resolution.start, line1_start);
    assert_eq!(resolution.end, line3_start);
}

#[test]
fn test_from_line_range_deletion() {
    let mut rope = Rope::from_str("keep\ndelete1\ndelete2\nkeep\n");
    let patch = Patch::from_line_range(
        "test.txt".to_string(),
        1,
        3,
        "",
    );

    patch.apply(&mut rope).unwrap();
    assert_eq!(rope.to_string(), "keep\nkeep\n");
}

#[test]
fn test_from_line_positions_single_line() {
    let rope = Rope::from_str("hello world");
    let patch = Patch::from_line_positions(
        "test.txt".to_string(),
        0,
        6,
        0,
        11,
        &rope,
        "rust",
    );

    let resolution = patch.snippet.resolve(&rope).unwrap();
    assert_eq!(resolution.start, 6);
    assert_eq!(resolution.end, 11);
}

#[test]
fn test_from_line_positions_multi_line() {
    let rope = Rope::from_str("line 1\nline 2\nline 3");
    let patch = Patch::from_line_positions(
        "test.txt".to_string(),
        0,
        3,
        2,
        3,
        &rope,
        "X",
    );

    let resolution = patch.snippet.resolve(&rope).unwrap();
    // From char 3 of line 0 to char 3 of line 2
    assert_eq!(resolution.start, 3);
    assert_eq!(resolution.end, 17);
}

#[cfg(feature = "regex")]
#[test]
fn test_pattern_based_patch() {
    use crate::snip::snippet::{Boundary, Snippet};
    use crate::snip::Target;

    let mut rope = Rope::from_str("version 1.2.3 is old");
    let target = Target::pattern(r"\d+\.\d+\.\d+").unwrap();
    let boundary = Boundary::new(target, BoundaryMode::Include);
    let snippet = Snippet::At(boundary);

    let patch = Patch {
        file: "test.txt".to_string(),
        snippet,
        replacement: "2.0.0".to_string(),
        #[cfg(feature = "symbol_path")]
        symbol_path: None,
    };

    patch.apply(&mut rope).unwrap();
    assert_eq!(rope.to_string(), "version 2.0.0 is old");
}
