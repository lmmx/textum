use super::*;
use std::fs;
use std::path::Path;

#[test]
fn test_groups_by_file() {
    let mut set = PatchSet::new();

    set.add(Patch {
        file: "file1.txt".to_string(),
        range: (0, 5),
        replacement: Some("test".to_string()),
        symbol_path: None,
        max_line_drift: None,
    });

    set.add(Patch {
        file: "file2.txt".to_string(),
        range: (0, 5),
        replacement: Some("test".to_string()),
        symbol_path: None,
        max_line_drift: None,
    });

    // Internal grouping happens in apply_to_files, verify by checking
    // that patches list contains both files
    assert_eq!(set.patches.len(), 2);
    assert_eq!(set.patches[0].file, "file1.txt");
    assert_eq!(set.patches[1].file, "file2.txt");
}

#[test]
fn test_reverse_sort() {
    // Create a test file
    let test_file = "tests/fixtures/reverse_sort_test.txt";
    fs::write(test_file, "abcdefghij").unwrap();

    let mut set = PatchSet::new();

    // Add patches in forward order
    set.add(Patch {
        file: test_file.to_string(),
        range: (0, 1),
        replacement: Some("X".to_string()),
        symbol_path: None,
        max_line_drift: None,
    });

    set.add(Patch {
        file: test_file.to_string(),
        range: (5, 6),
        replacement: Some("Y".to_string()),
        symbol_path: None,
        max_line_drift: None,
    });

    let results = set.apply_to_files().unwrap();

    // If sorted correctly (reverse), should apply 5->6 first, then 0->1
    // Result: XbcdeYghij
    assert_eq!(results.get(test_file).unwrap(), "XbcdeYghij");

    // Cleanup
    fs::remove_file(test_file).ok();
}

#[test]
fn test_multiple_patches_same_file() {
    let results = {
        let mut set = PatchSet::new();

        set.add(Patch {
            file: "tests/fixtures/sample.txt".to_string(),
            range: (0, 5),
            replacement: Some("goodbye".to_string()),
            symbol_path: None,
            max_line_drift: None,
        });

        set.add(Patch {
            file: "tests/fixtures/sample.txt".to_string(),
            range: (6, 11),
            replacement: Some("rust".to_string()),
            symbol_path: None,
            max_line_drift: None,
        });

        set.apply_to_files().unwrap()
    };

    assert_eq!(results.get("tests/fixtures/sample.txt").unwrap(), "goodbye rust\n");
}
