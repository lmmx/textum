//! Real-world usage scenarios and integration tests.

use assert_cmd::cargo_bin_cmd;
use std::fs;
use tempfile::TempDir;

#[test]
fn readme_update_between_comment_markers() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("README.md");

    let original = r#"# Project

<!-- stats -->
Old statistics here
<!-- /stats -->

## Details
"#;

    fs::write(&file, original).unwrap();

    let new_stats = "**Lines:** 1,234\n**Files:** 56";

    cargo_bin_cmd!("textum")
        .args(&[
            "replace",
            "<!-- stats -->",
            new_stats,
            "--until",
            "<!-- /stats -->",
            file.to_str().unwrap(),
        ])
        .assert()
        .success();

    let expected = format!("# Project\n\n<!-- stats -->{new_stats}<!-- /stats -->\n\n## Details\n");

    assert_eq!(fs::read_to_string(&file).unwrap(), expected);
}

#[test]
fn batch_rename_across_multiple_files() {
    let temp = TempDir::new().unwrap();
    let file1 = temp.path().join("module1.rs");
    let file2 = temp.path().join("module2.rs");

    fs::write(&file1, "fn old_function() {}\n").unwrap();
    fs::write(&file2, "pub fn old_function() -> u32 { 0 }\n").unwrap();

    cargo_bin_cmd!("textum")
        .args(&[
            "replace",
            "old_function",
            "new_function",
            file1.to_str().unwrap(),
            file2.to_str().unwrap(),
        ])
        .assert()
        .success();

    assert_eq!(
        fs::read_to_string(&file1).unwrap(),
        "fn new_function() {}\n"
    );
    assert_eq!(
        fs::read_to_string(&file2).unwrap(),
        "pub fn new_function() -> u32 { 0 }\n"
    );
}

#[test]
fn multiple_patches_json_different_operations() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("code.rs");

    fs::write(&file, "fn old_name() {}\nconst OLD: u32 = 42;\n").unwrap();

    let json = format!(
        r#"[
            {{
                "file": "{}",
                "snippet": {{"At": {{"target": {{"Literal": "old_name"}}, "mode": "Include"}}}},
                "replacement": "new_name"
            }},
            {{
                "file": "{}",
                "snippet": {{"At": {{"target": {{"Literal": "OLD"}}, "mode": "Include"}}}},
                "replacement": "NEW"
            }}
        ]"#,
        file.to_str().unwrap(),
        file.to_str().unwrap()
    );

    cargo_bin_cmd!("textum")
        .arg("apply")
        .write_stdin(json)
        .assert()
        .success();

    assert_eq!(
        fs::read_to_string(&file).unwrap(),
        "fn new_name() {}\nconst NEW: u32 = 42;\n"
    );
}

#[cfg(feature = "regex")]
#[test]
fn update_version_numbers_with_regex() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("Cargo.toml");

    let content = r#"[package]
name = "example"
version = "0.1.0"
"#;

    fs::write(&file, content).unwrap();

    cargo_bin_cmd!("textum")
        .args(&[
            "replace",
            r#"version = "\d+\.\d+\.\d+""#,
            r#"version = "1.0.0""#,
            "--pattern",
            file.to_str().unwrap(),
        ])
        .assert()
        .success();

    let expected = r#"[package]
name = "example"
version = "1.0.0"
"#;

    assert_eq!(fs::read_to_string(&file).unwrap(), expected);
}
