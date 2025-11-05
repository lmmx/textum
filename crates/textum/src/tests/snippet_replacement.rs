use crate::snip::snippet::{Boundary, BoundaryMode, Snippet, SnippetError};
use crate::snip::Target;
use ropey::Rope;

#[test]
fn test_replace_insert_at_position_exclude() {
    let rope = Rope::from_str("hello world");
    let target = Target::Char(4); // The o in "hello"
    let boundary = Boundary::new(target, BoundaryMode::Exclude); // Exclude the o
    let snippet = Snippet::At(boundary);

    let result = snippet.replace(&rope, ", beautiful").unwrap();

    assert_eq!(result.to_string(), "hello, beautiful world");
}

#[test]
fn test_replace_insert_at_position_include() {
    let rope = Rope::from_str("hello world");
    let target = Target::Char(5); // The space after "hello"
    let boundary = Boundary::new(target, BoundaryMode::Include); // Include the space
    let snippet = Snippet::At(boundary);

    let result = snippet.replace(&rope, ", beautiful ").unwrap();

    assert_eq!(result.to_string(), "hello, beautiful world");
}

#[test]
fn test_replace_delete_range() {
    // Tests empty replacement string performs deletion
    // Uses Snippet::At with Include mode and empty replacement
    let rope = Rope::from_str("line1\nline2\nline3\n");
    let target = Target::Line(1); // "line2\n"
    let boundary = Boundary::new(target, BoundaryMode::Include);
    let snippet = Snippet::At(boundary);

    let result = snippet.replace(&rope, "").unwrap();

    assert_eq!(result.to_string(), "line1\nline3\n");
}

#[test]
fn test_replace_edit_existing_text() {
    // Tests non-empty replacement on non-zero range performs edit
    // Uses Snippet::At with Include mode to replace a line
    let rope = Rope::from_str("line1\nline2\nline3\n");
    let target = Target::Line(1);
    let boundary = Boundary::new(target, BoundaryMode::Include);
    let snippet = Snippet::At(boundary);

    let result = snippet.replace(&rope, "MODIFIED\n").unwrap();

    assert_eq!(result.to_string(), "line1\nMODIFIED\nline3\n");
}

#[test]
fn test_replace_between_boundaries() {
    // Tests replacing content between two markers
    // Uses Snippet::Between with HTML comment markers, replaces inner content
    let rope = Rope::from_str("<!-- start -->old content<!-- end -->");
    let start_target = Target::Literal("<!-- start -->".to_string());
    let end_target = Target::Literal("<!-- end -->".to_string());
    let start_boundary = Boundary::new(start_target, BoundaryMode::Exclude);
    let end_boundary = Boundary::new(end_target, BoundaryMode::Exclude);
    let snippet = Snippet::Between {
        start: start_boundary,
        end: end_boundary,
    };

    let result = snippet.replace(&rope, "new content").unwrap();

    assert_eq!(result.to_string(), "<!-- start -->new content<!-- end -->");
}

#[test]
fn test_replace_from_boundary_to_eof_exclude() {
    // Tests Snippet::From replaces from boundary to end of file
    // Verifies EOF handling in replacement
    let rope = Rope::from_str("keep this\nreplace from here\nand this too\n");
    let target = Target::Literal("\nreplace from here".to_string());
    let boundary = Boundary::new(target, BoundaryMode::Exclude);
    let snippet = Snippet::From(boundary);

    let result = snippet.replace(&rope, "\nnew ending").unwrap();

    // Exclude mode: starts AFTER the matched text, so "replace from here\n" remains
    assert_eq!(
        result.to_string(),
        "keep this\nreplace from here\nnew ending"
    );
}

#[test]
fn test_replace_from_boundary_to_eof_include() {
    // Tests Snippet::From replaces from boundary to end of file
    // Verifies EOF handling in replacement
    let rope = Rope::from_str("keep this\nreplace from here\nand this too\n");
    let target = Target::Literal("replace from here".to_string());
    let boundary = Boundary::new(target, BoundaryMode::Include);
    let snippet = Snippet::From(boundary);

    let result = snippet.replace(&rope, "\nnew ending").unwrap();

    assert_eq!(
        result.to_string(),
        "keep this\nreplace from here\nnew ending"
    );
}

#[test]
fn test_replace_to_boundary_from_bof() {
    // Tests Snippet::To replaces from start of file to boundary
    // Verifies BOF handling in replacement
    let rope = Rope::from_str("replace this\nand this\nkeep from here");
    let target = Target::Literal("keep from here".to_string());
    let boundary = Boundary::new(target, BoundaryMode::Exclude);
    let snippet = Snippet::To(boundary);

    let result = snippet.replace(&rope, "new beginning\n").unwrap();

    assert_eq!(result.to_string(), "new beginning\nkeep from here");
}

#[test]
fn test_replace_entire_rope() {
    // Tests Snippet::All replaces entire rope content
    // Verifies complete replacement operation
    let rope = Rope::from_str("old content\nline 2\nline 3");
    let snippet = Snippet::All;

    let result = snippet.replace(&rope, "completely new").unwrap();

    assert_eq!(result.to_string(), "completely new");
}

#[test]
fn test_replace_multiline_content() {
    // Tests replacing a multi-line selection with multi-line replacement
    // Uses Snippet::Between spanning multiple lines
    let rope =
        Rope::from_str("header\n<!-- start -->\nold line 1\nold line 2\n<!-- end -->\nfooter");
    let start_target = Target::Literal("<!-- start -->".to_string());
    let end_target = Target::Literal("<!-- end -->".to_string());
    let start_boundary = Boundary::new(start_target, BoundaryMode::Exclude);
    let end_boundary = Boundary::new(end_target, BoundaryMode::Exclude);
    let snippet = Snippet::Between {
        start: start_boundary,
        end: end_boundary,
    };

    let replacement = "\nnew line 1\nnew line 2\n";
    let result = snippet.replace(&rope, replacement).unwrap();

    assert_eq!(
        result.to_string(),
        "header\n<!-- start -->\nnew line 1\nnew line 2\n<!-- end -->\nfooter"
    );
}

#[test]
fn test_replace_with_unicode() {
    // Tests replacement containing Unicode characters (emoji, accents)
    // Verifies UTF-8 handling and char boundary correctness
    let rope = Rope::from_str("hello world");
    let target = Target::Literal("world".to_string());
    let boundary = Boundary::new(target, BoundaryMode::Include);
    let snippet = Snippet::At(boundary);

    let result = snippet.replace(&rope, "🌍 café").unwrap();

    assert_eq!(result.to_string(), "hello 🌍 café");

    // Verify char count is correct with multi-byte characters
    let expected_chars = "hello 🌍 café".chars().count();
    assert_eq!(result.len_chars(), expected_chars);
}

#[test]
fn test_replace_null_byte_rejection() {
    // Tests that replacement containing null bytes is rejected
    // Verifies InvalidUtf8 error is returned
    let rope = Rope::from_str("hello world");
    let target = Target::Literal("world".to_string());
    let boundary = Boundary::new(target, BoundaryMode::Include);
    let snippet = Snippet::At(boundary);

    let result = snippet.replace(&rope, "bad\0string");

    assert!(matches!(result, Err(SnippetError::InvalidUtf8(_))));
}

#[test]
fn test_replace_empty_rope() {
    // Tests replacement operations on empty rope
    // Uses Snippet::All with empty source and non-empty replacement
    let rope = Rope::from_str("");
    let snippet = Snippet::All;

    let result = snippet.replace(&rope, "new content").unwrap();

    assert_eq!(result.to_string(), "new content");
}

#[test]
fn test_replace_preserves_surrounding_text() {
    // Tests that replacement only affects target range
    // Verifies text before and after target is unchanged
    let rope = Rope::from_str("prefix [REPLACE ME] suffix");
    let target = Target::Literal("[REPLACE ME]".to_string());
    let boundary = Boundary::new(target, BoundaryMode::Include);
    let snippet = Snippet::At(boundary);

    let result = snippet.replace(&rope, "[DONE]").unwrap();

    assert_eq!(result.to_string(), "prefix [DONE] suffix");

    // Verify prefix is unchanged
    let prefix = result.slice(0..7).to_string();
    assert_eq!(prefix, "prefix ");

    // Verify suffix is unchanged
    let suffix_start = result.len_chars() - 7;
    let suffix = result.slice(suffix_start..result.len_chars()).to_string();
    assert_eq!(suffix, " suffix");
}

#[cfg(feature = "regex")]
#[test]
fn test_replace_regex_pattern() {
    // Tests Snippet with regex Pattern target performs replacement
    // Uses Pattern to match and replace a numeric sequence
    let rope = Rope::from_str("version 1.2.3 is old");
    let target = Target::pattern(r"\d+\.\d+\.\d+").unwrap();
    let boundary = Boundary::new(target, BoundaryMode::Include);
    let snippet = Snippet::At(boundary);

    let result = snippet.replace(&rope, "2.0.0").unwrap();

    assert_eq!(result.to_string(), "version 2.0.0 is old");
}
