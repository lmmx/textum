//! Tests for individual CLI commands (replace, delete, apply).

use assert_cmd::cargo_bin_cmd;
use std::fs;
use tempfile::TempDir;

#[test]
fn delete_simple_literal() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("test.txt");
    fs::write(&file, "hello world\n").unwrap();

    cargo_bin_cmd!("textum")
        .args(&["delete", " world", file.to_str().unwrap()])
        .assert()
        .success();

    assert_eq!(fs::read_to_string(&file).unwrap(), "hello\n");
}

#[test]
fn delete_line_range() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("test.txt");
    fs::write(&file, "line0\nline1\nline2\nline3\n").unwrap();

    cargo_bin_cmd!("textum")
        .args(&["delete", "dummy", "--lines", "1:3", file.to_str().unwrap()])
        .assert()
        .success();

    assert_eq!(fs::read_to_string(&file).unwrap(), "line0\nline3\n");
}

#[test]
fn delete_between_markers() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("test.txt");
    fs::write(&file, "keep\n<!-- start -->remove<!-- end -->\nkeep\n").unwrap();

    cargo_bin_cmd!("textum")
        .args(&[
            "delete",
            "<!-- start -->",
            "--until",
            "<!-- end -->",
            file.to_str().unwrap(),
        ])
        .assert()
        .success();

    assert_eq!(
        fs::read_to_string(&file).unwrap(),
        "keep\n<!-- start --><!-- end -->\nkeep\n"
    );
}
