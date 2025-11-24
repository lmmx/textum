//! Tests for individual CLI commands (replace, delete, apply).

use assert_cmd::cargo_bin_cmd;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn apply_json_from_file() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("test.txt");
    let patch_file = temp.path().join("patches.json");

    fs::write(&file, "hello world\n").unwrap();

    let json = format!(
        r#"[{{"file": "{}", "snippet": {{"At": {{"target": {{"Literal": "world"}}, "mode": "Include"}}}}, "replacement": "rust"}}]"#,
        file.to_str().unwrap()
    );
    fs::write(&patch_file, json).unwrap();

    cargo_bin_cmd!("textum")
        .args(["apply", patch_file.to_str().unwrap()])
        .assert()
        .success();

    assert_eq!(fs::read_to_string(&file).unwrap(), "hello rust\n");
}

#[test]
fn apply_json_from_stdin() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("test.txt");
    fs::write(&file, "hello world\n").unwrap();

    let json = format!(
        r#"[{{"file": "{}", "snippet": {{"At": {{"target": {{"Literal": "world"}}, "mode": "Include"}}}}, "replacement": "rust"}}]"#,
        file.to_str().unwrap()
    );

    cargo_bin_cmd!("textum")
        .arg("apply")
        .write_stdin(json)
        .assert()
        .success();

    assert_eq!(fs::read_to_string(&file).unwrap(), "hello rust\n");
}

#[test]
fn apply_with_verbose_flag() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("test.txt");
    fs::write(&file, "hello world\n").unwrap();

    let json = format!(
        r#"[{{"file": "{}", "snippet": {{"At": {{"target": {{"Literal": "world"}}, "mode": "Include"}}}}, "replacement": "rust"}}]"#,
        file.to_str().unwrap()
    );

    cargo_bin_cmd!("textum")
        .arg("apply")
        .arg("--verbose")
        .write_stdin(json)
        .assert()
        .success()
        .stderr(predicate::str::contains("Loaded 1 patch(es)"))
        .stderr(predicate::str::contains("Successfully patched 1 file(s)"));
}
