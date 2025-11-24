//! Edge case and error handling tests.

use assert_cmd::cargo_bin_cmd;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn error_file_not_found() {
    cargo_bin_cmd!("textum")
        .args(["replace", "old", "new", "/nonexistent/file.txt"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}

#[test]
fn error_target_not_found() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("test.txt");
    fs::write(&file, "hello world\n").unwrap();

    cargo_bin_cmd!("textum")
        .args(["replace", "nonexistent", "new", file.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}

#[test]
fn error_invalid_line_range_format() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("test.txt");
    fs::write(&file, "hello\n").unwrap();

    cargo_bin_cmd!("textum")
        .args([
            "replace",
            "dummy",
            "new",
            "--lines",
            "invalid",
            file.to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid line range"));
}

#[test]
fn error_line_range_out_of_bounds() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("test.txt");
    fs::write(&file, "line1\nline2\n").unwrap();

    cargo_bin_cmd!("textum")
        .args([
            "replace",
            "dummy",
            "new",
            "--lines",
            "5:10",
            file.to_str().unwrap(),
        ])
        .assert()
        .failure();
}

#[test]
fn apply_invalid_json_fails() {
    cargo_bin_cmd!("textum")
        .arg("apply")
        .write_stdin("not valid json")
        .assert()
        .failure();
}

#[test]
fn apply_empty_json_array_succeeds() {
    cargo_bin_cmd!("textum")
        .arg("apply")
        .write_stdin("[]")
        .assert()
        .success();
}

#[test]
fn overlapping_patches_with_non_empty_replacements_error() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("overlap.txt");
    fs::write(&file, "abcdef").unwrap();

    let patch_json = format!(
        r#"[
            {{
                "file": "{}",
                "snippet": {{
                    "At": {{
                        "target": {{"Literal": "bcd"}},
                        "mode": "Include"
                    }}
                }},
                "replacement": "XXX"
            }},
            {{
                "file": "{}",
                "snippet": {{
                    "At": {{
                        "target": {{"Literal": "cde"}},
                        "mode": "Include"
                    }}
                }},
                "replacement": "YYY"
            }}
        ]"#,
        file.to_str().unwrap(),
        file.to_str().unwrap()
    );

    cargo_bin_cmd!("textum")
        .arg("apply")
        .write_stdin(patch_json)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Overlapping"));
}

#[test]
fn empty_file_literal_not_found() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("empty.txt");
    fs::write(&file, "").unwrap();

    cargo_bin_cmd!("textum")
        .args(["replace", "something", "else", file.to_str().unwrap()])
        .assert()
        .failure();
}

#[test]
fn file_without_trailing_newline() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("no_newline.txt");
    fs::write(&file, "hello world").unwrap();

    cargo_bin_cmd!("textum")
        .args(["replace", "world", "rust", file.to_str().unwrap()])
        .assert()
        .success();

    assert_eq!(fs::read_to_string(&file).unwrap(), "hello rust");
}

#[cfg(feature = "regex")]
#[test]
fn invalid_regex_pattern_error() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("test.txt");
    fs::write(&file, "test").unwrap();

    cargo_bin_cmd!("textum")
        .args([
            "replace",
            "[invalid",
            "replacement",
            "--pattern",
            file.to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid regex pattern"));
}
