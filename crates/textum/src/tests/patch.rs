use super::*;
use crate::snip::snippet::{Boundary, BoundaryMode};
use crate::snip::Target;
use ropey::Rope;

#[test]
fn test_apply_replace() {
    let mut rope = Rope::from_str("hello world");
    let patch = Patch::from_literal_target(
        "test.txt".to_string(),
        "world",
        BoundaryMode::Include,
        "rust",
    );

    patch.apply(&mut rope).unwrap();
    assert_eq!(rope.to_string(), "hello rust");
}

#[test]
fn test_apply_insert() {
    let mut rope = Rope::from_str("helloworld");
    let target = Target::Char(5);
    let boundary = Boundary::new(target, BoundaryMode::Exclude);
    let snippet = Snippet::At(boundary);
    let patch = Patch {
        file: "test.txt".to_string(),
        snippet,
        replacement: " ".to_string(),
        #[cfg(feature = "symbol_path")]
        symbol_path: None,
    };

    patch.apply(&mut rope).unwrap();
    assert_eq!(rope.to_string(), "hello world");
}

#[test]
fn test_apply_delete() {
    let mut rope = Rope::from_str("hello world");
    let patch = Patch::from_line_range(
        "test.txt".to_string(),
        0,
        1,
        "",
    );

    patch.apply(&mut rope).unwrap();
    assert_eq!(rope.to_string(), "");
}

#[test]
fn test_bounds_check() {
    let mut rope = Rope::from_str("hello");
    let target = Target::Char(100);
    let boundary = Boundary::new(target, BoundaryMode::Include);
    let snippet = Snippet::At(boundary);
    let patch = Patch {
        file: "test.txt".to_string(),
        snippet,
        replacement: "".to_string(),
        #[cfg(feature = "symbol_path")]
        symbol_path: None,
    };

    assert!(patch.apply(&mut rope).is_err());
}

#[test]
fn test_from_line_positions_snippet_construction() {
    let rope = Rope::from_str("line 1\nline 2\nline 3");
    let patch = Patch::from_line_positions(
        "test.txt".to_string(),
        1,
        0,
        1,
        6,
        &rope,
        "EDITED",
    );

    // Verify snippet resolves to correct range
    let resolution = patch.snippet.resolve(&rope).unwrap();
    assert_eq!(resolution.start, 7);
    assert_eq!(resolution.end, 13);
}

#[test]
fn test_from_literal_target_constructor() {
    let mut rope = Rope::from_str("foo bar baz");
    let patch = Patch::from_literal_target(
        "test.txt".to_string(),
        "bar",
        BoundaryMode::Include,
        "qux",
    );

    patch.apply(&mut rope).unwrap();
    assert_eq!(rope.to_string(), "foo qux baz");
}

#[test]
fn test_from_line_range_constructor() {
    let mut rope = Rope::from_str("line1\nline2\nline3\nline4\n");
    let patch = Patch::from_line_range(
        "test.txt".to_string(),
        1,
        3,
        "replaced\n",
    );

    patch.apply(&mut rope).unwrap();
    assert_eq!(rope.to_string(), "line1\nreplaced\nline4\n");
}
