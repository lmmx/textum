#![allow(missing_docs)]
use predicates::prelude::*;
use std::fs;
use tempfile::NamedTempFile;
use textum::{BoundaryMode, Patch, PatchSet};

#[test]
fn patch_replaces_text_in_tempfile() {
    // Create temp file with known content
    let temp = NamedTempFile::new().unwrap();
    fs::write(temp.path(), "Hello Louis!").unwrap();

    // Define a patch that replaces "Louis" with "World"
    let patch = Patch::from_literal_target(
        temp.path().to_string_lossy().into(),
        "Louis",
        BoundaryMode::Include,
        "World",
    );

    let mut set = PatchSet::new();
    set.add(patch);

    // Apply and capture results
    let results = set
        .apply_to_files()
        .expect("patch application should succeed");

    // Assert new content in map
    let new_content = results
        .get(temp.path().to_str().unwrap())
        .expect("file should be in results");
    assert_eq!(new_content, "Hello World!");

    // Write result and verify on disk
    fs::write(temp.path(), new_content).unwrap();
    let on_disk = fs::read_to_string(temp.path()).unwrap();

    // Use predicates for clean assertion
    assert!(predicate::str::contains("World").eval(&on_disk));
    assert!(predicate::str::is_match(r"^Hello\sWorld!$")
        .unwrap()
        .eval(&on_disk));
}
