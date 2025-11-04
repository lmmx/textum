#![allow(missing_docs)]
#[cfg(feature = "cli")]
mod cli_integration {
    use assert_cmd::cargo_bin_cmd;
    use predicates::prelude::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn cli_applies_simple_patch_from_file() {
        let temp = TempDir::new().unwrap();

        // Create a source file to patch
        let source_file = temp.path().join("hello.txt");
        fs::write(&source_file, "Hello Louis!").unwrap();

        // Create a patch file with all required fields
        let patch_file = temp.path().join("patches.json");
        let patch_json = format!(
            r#"[{{
                "file": "{}",
                "range": [6, 11],
                "replacement": "World"
            }}]"#,
            source_file.display()
        );
        fs::write(&patch_file, patch_json).unwrap();

        // Apply the patch
        cargo_bin_cmd!("textum")
            .arg(patch_file.to_str().unwrap())
            .assert()
            .success()
            .stderr(predicate::str::contains("Patched:"));

        // Verify the result
        let result = fs::read_to_string(&source_file).unwrap();
        assert_eq!(result, "Hello World!");
    }

    #[test]
    fn cli_applies_patch_from_stdin() {
        let temp = TempDir::new().unwrap();

        let source_file = temp.path().join("greet.txt");
        fs::write(&source_file, "Hi there").unwrap();

        let patch_json = format!(
            r#"[{{
                "file": "{}",
                "range": [0, 2],
                "replacement": "Hello"
            }}]"#,
            source_file.display()
        );

        cargo_bin_cmd!("textum")
            .write_stdin(patch_json)
            .assert()
            .success();

        let result = fs::read_to_string(&source_file).unwrap();
        assert_eq!(result, "Hello there");
    }

    #[test]
    fn cli_dry_run_does_not_modify_file() {
        let temp = TempDir::new().unwrap();

        let source_file = temp.path().join("original.txt");
        let original_content = "Don't change me!";
        fs::write(&source_file, original_content).unwrap();

        let patch_json = format!(
            r#"[{{
                "file": "{}",
                "range": [0, 5],
                "replacement": "Please"
            }}]"#,
            source_file.display()
        );

        cargo_bin_cmd!("textum")
            .arg("--dry-run")
            .write_stdin(patch_json)
            .assert()
            .success()
            .stderr(predicate::str::contains("Would patch:"));

        // File should remain unchanged
        let result = fs::read_to_string(&source_file).unwrap();
        assert_eq!(result, original_content);
    }

    #[test]
    fn cli_verbose_shows_patch_count() {
        let temp = TempDir::new().unwrap();

        let source_file = temp.path().join("test.txt");
        fs::write(&source_file, "test").unwrap();

        let patch_json = format!(
            r#"[{{
                "file": "{}",
                "range": [0, 4],
                "replacement": "best"
            }}]"#,
            source_file.display()
        );

        cargo_bin_cmd!("textum")
            .arg("--verbose")
            .write_stdin(patch_json)
            .assert()
            .success()
            .stderr(predicate::str::contains("Loaded 1 patch(es)"));
    }

    #[test]
    fn cli_applies_multiple_patches() {
        let temp = TempDir::new().unwrap();

        let file1 = temp.path().join("file1.txt");
        let file2 = temp.path().join("file2.txt");
        fs::write(&file1, "foo").unwrap();
        fs::write(&file2, "bar").unwrap();

        let patch_json = format!(
            r#"[
                {{
                    "file": "{}",
                    "range": [0, 3],
                    "replacement": "baz"
                }},
                {{
                    "file": "{}",
                    "range": [0, 3],
                    "replacement": "qux"
                }}
            ]"#,
            file1.display(),
            file2.display()
        );

        cargo_bin_cmd!("textum")
            .write_stdin(patch_json)
            .assert()
            .success();

        assert_eq!(fs::read_to_string(&file1).unwrap(), "baz");
        assert_eq!(fs::read_to_string(&file2).unwrap(), "qux");
    }

    #[test]
    fn cli_fails_on_invalid_json() {
        cargo_bin_cmd!("textum")
            .write_stdin("not valid json")
            .assert()
            .failure()
            .stderr(predicate::str::contains("Error:"));
    }

    #[test]
    fn cli_fails_on_missing_file() {
        cargo_bin_cmd!("textum")
            .arg("nonexistent-patches.json")
            .assert()
            .failure();
    }

    #[test]
    fn cli_deletion_patch() {
        let temp = TempDir::new().unwrap();

        let source_file = temp.path().join("delete.txt");
        fs::write(&source_file, "Hello World!").unwrap();

        // Delete "World" by replacing with empty string
        let patch_json = format!(
            r#"[{{
                "file": "{}",
                "range": [6, 11],
                "replacement": ""
            }}]"#,
            source_file.display()
        );

        cargo_bin_cmd!("textum")
            .write_stdin(patch_json)
            .assert()
            .success();

        let result = fs::read_to_string(&source_file).unwrap();
        assert_eq!(result, "Hello !");
    }

    #[test]
    fn cli_insertion_patch() {
        let temp = TempDir::new().unwrap();

        let source_file = temp.path().join("insert.txt");
        fs::write(&source_file, "HelloWorld").unwrap();

        // Insert a space at position 5 (zero-length range)
        let patch_json = format!(
            r#"[{{
                "file": "{}",
                "range": [5, 5],
                "replacement": " "
            }}]"#,
            source_file.display()
        );

        cargo_bin_cmd!("textum")
            .write_stdin(patch_json)
            .assert()
            .success();

        let result = fs::read_to_string(&source_file).unwrap();
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn cli_dry_run_verbose_shows_content() {
        let temp = TempDir::new().unwrap();

        let source_file = temp.path().join("preview.txt");
        fs::write(&source_file, "before").unwrap();

        let patch_json = format!(
            r#"[{{
                "file": "{}",
                "range": [0, 6],
                "replacement": "after"
            }}]"#,
            source_file.display()
        );

        cargo_bin_cmd!("textum")
            .arg("--dry-run")
            .arg("--verbose")
            .write_stdin(patch_json)
            .assert()
            .success()
            .stdout(predicate::str::contains("after"));

        // Original file unchanged
        assert_eq!(fs::read_to_string(&source_file).unwrap(), "before");
    }
}
