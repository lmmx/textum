//! Real-world usage scenarios and integration tests.

use assert_cmd::cargo_bin_cmd;
use std::fs;
use tempfile::TempDir;

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
