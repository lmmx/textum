use super::*;

#[test]
fn test_groups_by_file() {
    let mut set = PatchSet::new();

    set.add(Patch::from_literal_target(
        "file1.txt".to_string(),
        "old",
        BoundaryMode::Include,
        "new",
    ));

    set.add(Patch::from_literal_target(
        "file2.txt".to_string(),
        "old",
        BoundaryMode::Include,
        "new",
    ));

    assert_eq!(set.patches.len(), 2);
    assert_eq!(set.patches[0].file, "file1.txt");
    assert_eq!(set.patches[1].file, "file2.txt");
}

#[test]
fn test_reverse_sort_intra_line() {
    use std::fs;
    let test_file = "tests/fixtures/reverse_sort_test.txt";
    fs::write(test_file, "abcdefghij").unwrap();

    let mut set = PatchSet::new();

    // Add patches in forward order at different char positions
    set.add(Patch::from_literal_target(
        test_file.to_string(),
        "a",
        BoundaryMode::Include,
        "X",
    ));

    set.add(Patch::from_literal_target(
        test_file.to_string(),
        "f",
        BoundaryMode::Include,
        "Y",
    ));

    let results = set.apply_to_files().unwrap();
    assert_eq!(results.get(test_file).unwrap(), "XbcdeYghij");

    fs::remove_file(test_file).ok();
}

#[test]
fn test_multiple_patches_same_file() {
    let results = {
        let mut set = PatchSet::new();

        set.add(Patch::from_literal_target(
            "tests/fixtures/sample.txt".to_string(),
            "hello",
            BoundaryMode::Include,
            "goodbye",
        ));

        set.add(Patch::from_literal_target(
            "tests/fixtures/sample.txt".to_string(),
            "world",
            BoundaryMode::Include,
            "rust",
        ));

        set.apply_to_files().unwrap()
    };

    assert_eq!(
        results.get("tests/fixtures/sample.txt").unwrap(),
        "goodbye rust\n"
    );
}

#[test]
fn test_overlapping_ranges_rejected() {
    use std::fs;
    let test_file = "tests/fixtures/overlap_test.txt";
    fs::write(test_file, "abcdef").unwrap();

    let mut set = PatchSet::new();

    // "bcd" at chars 1-4
    set.add(Patch::from_literal_target(
        test_file.to_string(),
        "bcd",
        BoundaryMode::Include,
        "XXX",
    ));

    // "def" at chars 3-6 (overlaps with previous)
    set.add(Patch::from_literal_target(
        test_file.to_string(),
        "def",
        BoundaryMode::Include,
        "YYY",
    ));

    let result = set.apply_to_files();
    assert!(matches!(result, Err(PatchError::OverlappingRanges { .. })));

    fs::remove_file(test_file).ok();
}

#[test]
fn test_overlapping_deletions_allowed() {
    use std::fs;
    let test_file = "tests/fixtures/overlap_delete_test.txt";
    fs::write(test_file, "abcdef").unwrap();

    let mut set = PatchSet::new();

    // Both patches delete overlapping ranges - should be allowed
    set.add(Patch::from_literal_target(
        test_file.to_string(),
        "bcd",
        BoundaryMode::Include,
        "",
    ));

    set.add(Patch::from_literal_target(
        test_file.to_string(),
        "def",
        BoundaryMode::Include,
        "",
    ));

    // Should succeed since both are deletions
    let result = set.apply_to_files();
    assert!(result.is_ok());

    fs::remove_file(test_file).ok();
}
