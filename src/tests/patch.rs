use super::*;
use ropey::Rope;

#[test]
fn test_apply_replace() {
    let mut rope = Rope::from_str("hello world");
    let patch = Patch {
        file: "test.txt".to_string(),
        range: (6, 11),
        replacement: Some("rust".to_string()),
        symbol_path: None,
        max_line_drift: None,
    };

    patch.apply(&mut rope).unwrap();
    assert_eq!(rope.to_string(), "hello rust");
}

#[test]
fn test_apply_insert() {
    let mut rope = Rope::from_str("helloworld");
    let patch = Patch {
        file: "test.txt".to_string(),
        range: (5, 5),
        replacement: Some(" ".to_string()),
        symbol_path: None,
        max_line_drift: None,
    };

    patch.apply(&mut rope).unwrap();
    assert_eq!(rope.to_string(), "hello world");
}

#[test]
fn test_apply_delete() {
    let mut rope = Rope::from_str("hello world");
    let patch = Patch {
        file: "test.txt".to_string(),
        range: (5, 11),
        replacement: None,
        symbol_path: None,
        max_line_drift: None,
    };

    patch.apply(&mut rope).unwrap();
    assert_eq!(rope.to_string(), "hello");
}

#[test]
fn test_bounds_check() {
    let mut rope = Rope::from_str("hello");
    let patch = Patch {
        file: "test.txt".to_string(),
        range: (0, 100),
        replacement: None,
        symbol_path: None,
        max_line_drift: None,
    };

    assert!(matches!(patch.apply(&mut rope), Err(PatchError::RangeOutOfBounds)));
}

#[test]
fn test_from_line_positions_char_calculation() {
    let rope = Rope::from_str("line 1\nline 2\nline 3");
    let patch = Patch::from_line_positions(
        "test.txt".to_string(),
        1,
        0,
        1,
        6,
        &rope,
        Some("EDITED".to_string()),
    );

    assert_eq!(patch.range, (7, 13));
}
