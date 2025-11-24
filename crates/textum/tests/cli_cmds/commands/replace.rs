//! Tests for individual CLI commands (replace, delete, apply).

use assert_cmd::cargo_bin_cmd;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn replace_simple_literal() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("test.txt");
    fs::write(&file, "hello world\n").unwrap();

    cargo_bin_cmd!("textum")
        .args(&["replace", "world", "rust", file.to_str().unwrap()])
        .assert()
        .success();

    assert_eq!(fs::read_to_string(&file).unwrap(), "hello rust\n");
}

#[test]
fn replace_between_markers_exclude_boundaries() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("readme.md");
    fs::write(
        &file,
        "# Title\n<!-- start -->old content<!-- end -->\n# Footer\n",
    )
    .unwrap();

    cargo_bin_cmd!("textum")
        .args(&[
            "replace",
            "<!-- start -->",
            "new content",
            "--until",
            "<!-- end -->",
            file.to_str().unwrap(),
        ])
        .assert()
        .success();

    assert_eq!(
        fs::read_to_string(&file).unwrap(),
        "# Title\n<!-- start -->new content<!-- end -->\n# Footer\n"
    );
}

#[test]
fn replace_between_markers_include_boundaries() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("test.txt");
    fs::write(&file, "before\n[START]content[END]\nafter\n").unwrap();

    cargo_bin_cmd!("textum")
        .args(&[
            "replace",
            "[START]",
            "REPLACED",
            "--until",
            "[END]",
            "--include-markers",
            file.to_str().unwrap(),
        ])
        .assert()
        .success();

    assert_eq!(
        fs::read_to_string(&file).unwrap(),
        "before\nREPLACED\nafter\n"
    );
}

#[test]
fn replace_line_range() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("test.txt");
    fs::write(&file, "line0\nline1\nline2\nline3\nline4\n").unwrap();

    cargo_bin_cmd!("textum")
        .args(&[
            "replace",
            "dummy", // Not used when --lines is specified
            "REPLACED",
            "--lines",
            "1:3",
            file.to_str().unwrap(),
        ])
        .assert()
        .success();

    assert_eq!(
        fs::read_to_string(&file).unwrap(),
        "line0\nREPLACED\nline3\nline4\n"
    );
}

#[test]
fn replace_multiple_files() {
    let temp = TempDir::new().unwrap();
    let file1 = temp.path().join("file1.txt");
    let file2 = temp.path().join("file2.txt");
    fs::write(&file1, "hello world\n").unwrap();
    fs::write(&file2, "hello world\n").unwrap();

    cargo_bin_cmd!("textum")
        .args(&[
            "replace",
            "world",
            "rust",
            file1.to_str().unwrap(),
            file2.to_str().unwrap(),
        ])
        .assert()
        .success();

    assert_eq!(fs::read_to_string(&file1).unwrap(), "hello rust\n");
    assert_eq!(fs::read_to_string(&file2).unwrap(), "hello rust\n");
}

#[test]
fn replace_dry_run_does_not_modify() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("test.txt");
    let original = "hello world\n";
    fs::write(&file, original).unwrap();

    cargo_bin_cmd!("textum")
        .args(&[
            "replace",
            "world",
            "rust",
            "--dry-run",
            file.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Would patch"));

    assert_eq!(fs::read_to_string(&file).unwrap(), original);
}

#[test]
fn replace_diff_shows_changes() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("test.txt");
    fs::write(&file, "hello world\n").unwrap();

    cargo_bin_cmd!("textum")
        .args(&["replace", "world", "rust", "--diff", file.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("-hello world"))
        .stdout(predicate::str::contains("+hello rust"));

    // Verify file was NOT modified (--diff implies --dry-run)
    assert_eq!(fs::read_to_string(&file).unwrap(), "hello world\n");
}

#[cfg(feature = "regex")]
#[test]
fn replace_with_regex_pattern() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("test.txt");
    fs::write(&file, "fn foo() {}\nfn bar() {}\n").unwrap();

    cargo_bin_cmd!("textum")
        .args(&[
            "replace",
            r"fn \w+\(",
            "pub fn main(",
            "--pattern",
            file.to_str().unwrap(),
        ])
        .assert()
        .success();

    let content = fs::read_to_string(&file).unwrap();
    assert!(content.starts_with("pub fn main("));
}
