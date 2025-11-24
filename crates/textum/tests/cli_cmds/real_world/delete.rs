//! Real-world usage scenarios and integration tests.

use assert_cmd::cargo_bin_cmd;
use std::fs;
use tempfile::TempDir;

#[test]
fn delete_commented_code_blocks() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("code.rs");

    let content = r#"fn main() {
    println!("active");
    // BEGIN DEBUG
    // println!("debug info");
    // println!("more debug");
    // END DEBUG
    println!("more active");
}
"#;

    fs::write(&file, content).unwrap();

    cargo_bin_cmd!("textum")
        .args([
            "delete",
            "    // BEGIN DEBUG\n",
            "--until",
            "// END DEBUG\n",
            "--include-markers",
            file.to_str().unwrap(),
        ])
        .assert()
        .success();

    let expected = r#"fn main() {
    println!("active");
    println!("more active");
}
"#;

    assert_eq!(fs::read_to_string(&file).unwrap(), expected);
}

#[test]
fn preserve_file_structure_line_operations() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("structured.txt");

    let content = "header\nsection1\nsection2\nsection3\nfooter\n";
    fs::write(&file, content).unwrap();

    // Delete middle sections
    cargo_bin_cmd!("textum")
        .args(["delete", "dummy", "--lines", "1:4", file.to_str().unwrap()])
        .assert()
        .success();

    assert_eq!(fs::read_to_string(&file).unwrap(), "header\nfooter\n");
}
