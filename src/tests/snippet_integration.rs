use crate::snip::snippet::{Boundary, BoundaryMode, Extent, Snippet};
use crate::snip::Target;
use ropey::Rope;

#[test]
fn test_workflow_comment_block_replacement() {
    // Complete workflow: Find HTML comment markers, replace inner content
    // Demonstrates Target::Literal → Boundary → Snippet::Between → replace
    let rope = Rope::from_str(
        "<!DOCTYPE html>\n<!-- content -->\n<p>Old paragraph</p>\n<!-- /content -->\n</html>"
    );

    // Step 1: Define targets for start and end markers
    let start_target = Target::Literal("<!-- content -->".to_string());
    let end_target = Target::Literal("<!-- /content -->".to_string());

    // Step 2: Create boundaries that exclude the markers themselves
    let start_boundary = Boundary::new(start_target, BoundaryMode::Exclude);
    let end_boundary = Boundary::new(end_target, BoundaryMode::Exclude);

    // Step 3: Create snippet selecting content between markers
    let snippet = Snippet::Between {
        start: start_boundary,
        end: end_boundary,
    };

    // Step 4: Replace with new content
    let new_content = "\n<p>New paragraph</p>\n<p>Second paragraph</p>\n";
    let result = snippet.replace(&rope, new_content).unwrap();

    assert_eq!(
        result.to_string(),
        "<!DOCTYPE html>\n<!-- content -->\n<p>New paragraph</p>\n<p>Second paragraph</p>\n<!-- /content -->\n</html>"
    );
}

#[test]
fn test_workflow_line_range_deletion() {
    // Complete workflow: Select line range by line numbers, delete
    // Demonstrates Target::Line → Boundary → Snippet::Between → empty replace
    let rope = Rope::from_str("line 1\nline 2\nline 3\nline 4\nline 5\n");

    // Step 1: Target lines 1 and 3 (0-indexed: lines 2, 3, 4)
    let start_target = Target::Line(1);
    let end_target = Target::Line(3);

    // Step 2: Include start line, exclude end line to delete lines 2-3
    let start_boundary = Boundary::new(start_target, BoundaryMode::Include);
    let end_boundary = Boundary::new(end_target, BoundaryMode::Exclude);

    // Step 3: Create snippet for the range
    let snippet = Snippet::Between {
        start: start_boundary,
        end: end_boundary,
    };

    // Step 4: Delete by replacing with empty string
    let result = snippet.replace(&rope, "").unwrap();

    assert_eq!(result.to_string(), "line 1\nline 4\nline 5\n");
}

#[test]
fn test_workflow_insert_at_end_of_line() {
    // Complete workflow: Find line, position at end, insert text
    // Demonstrates Target::Line → BoundaryMode::Include → Snippet::At → replace
    let rope = Rope::from_str("def function():\n    pass\n");

    // Step 1: Target the first line
    let target = Target::Line(0);

    // Step 2: Include the line to get its full span, then we'll insert after
    let boundary = Boundary::new(target, BoundaryMode::Include);

    // Step 3: Use At to target just this line
    let snippet = Snippet::At(boundary);

    // Step 4: Replace with line plus new docstring
    let result = snippet.replace(&rope, "def function():\n    \"\"\"Docstring here.\"\"\"\n").unwrap();

    assert_eq!(result.to_string(), "def function():\n    \"\"\"Docstring here.\"\"\"\n    pass\n");
}

#[test]
fn test_workflow_multiple_replacements() {
    // Tests applying multiple sequential replacements to same rope
    // Demonstrates composition: rope1 → replace → rope2 → replace → rope3
    let rope1 = Rope::from_str("Step 1: TODO\nStep 2: TODO\nStep 3: TODO\n");

    // First replacement: Update step 1
    let target1 = Target::Literal("Step 1: TODO".to_string());
    let boundary1 = Boundary::new(target1, BoundaryMode::Include);
    let snippet1 = Snippet::At(boundary1);
    let rope2 = snippet1.replace(&rope1, "Step 1: Complete").unwrap();

    assert_eq!(rope2.to_string(), "Step 1: Complete\nStep 2: TODO\nStep 3: TODO\n");

    // Second replacement: Update step 2
    let target2 = Target::Literal("Step 2: TODO".to_string());
    let boundary2 = Boundary::new(target2, BoundaryMode::Include);
    let snippet2 = Snippet::At(boundary2);
    let rope3 = snippet2.replace(&rope2, "Step 2: In Progress").unwrap();

    assert_eq!(rope3.to_string(), "Step 1: Complete\nStep 2: In Progress\nStep 3: TODO\n");

    // Third replacement: Update step 3
    let target3 = Target::Literal("Step 3: TODO".to_string());
    let boundary3 = Boundary::new(target3, BoundaryMode::Include);
    let snippet3 = Snippet::At(boundary3);
    let rope4 = snippet3.replace(&rope3, "Step 3: Complete").unwrap();

    assert_eq!(rope4.to_string(), "Step 1: Complete\nStep 2: In Progress\nStep 3: Complete\n");
}

#[test]
fn test_workflow_nested_boundaries() {
    // Tests snippet within snippet scenario (outer markers, inner markers)
    // First finds outer range, then operates within that range
    let rope = Rope::from_str(
        "<div>\n  <section>\n    <p>inner content</p>\n  </section>\n</div>"
    );

    // Step 1: Extract the section content
    let outer_start = Target::Literal("<section>".to_string());
    let outer_end = Target::Literal("</section>".to_string());
    let outer_start_boundary = Boundary::new(outer_start, BoundaryMode::Exclude);
    let outer_end_boundary = Boundary::new(outer_end, BoundaryMode::Exclude);
    let outer_snippet = Snippet::Between {
        start: outer_start_boundary,
        end: outer_end_boundary,
    };

    let section_content = rope
        .slice(outer_snippet.resolve(&rope).unwrap().start..outer_snippet.resolve(&rope).unwrap().end)
        .to_string();
    let inner_rope = Rope::from_str(&section_content);

    // Step 2: Replace inner paragraph within the extracted section
    let inner_target = Target::Literal("<p>inner content</p>".to_string());
    let inner_boundary = Boundary::new(inner_target, BoundaryMode::Include);
    let inner_snippet = Snippet::At(inner_boundary);

    let new_section_content = inner_snippet.replace(&inner_rope, "<p>updated content</p>").unwrap();

    // Step 3: Put the modified section back into the outer snippet
    let final_result = outer_snippet.replace(&rope, &new_section_content.to_string()).unwrap();

    assert_eq!(
        final_result.to_string(),
        "<div>\n  <section>\n    <p>updated content</p>\n  </section>\n</div>"
    );
}

#[test]
fn test_workflow_extend_matching_pattern() {
    // Complete workflow: Find marker, extend until N occurrences of pattern
    // Demonstrates Target::Literal → Extent::Matching → replace
    let rope = Rope::from_str("data:\nline 1\nline 2\nline 3\nline 4\nrest of file");

    // Step 1: Find "data:" marker
    let target = Target::Literal("data:".to_string());

    // Step 2: Extend until 3 newlines are found (capturing 3 data lines)
    let newline_target = Target::Literal("\n".to_string());
    let boundary = Boundary::new(
        target,
        BoundaryMode::Extend(Extent::Matching(3, newline_target))
    );

    // Step 3: Select from marker end to after 3rd newline
    let snippet = Snippet::At(boundary);

    // Step 4: Replace the data section
    let result = snippet.replace(&rope, "\nreplaced data\n").unwrap();

    assert_eq!(result.to_string(), "data:\nreplaced data\nline 4\nrest of file");
}

#[test]
fn test_workflow_position_based_editing() {
    // Complete workflow: Use Position target for precise char editing
    // Demonstrates Target::Position → precise replacement
    let rope = Rope::from_str("line 1\nline 2\nline 3\n");

    // Step 1: Target line 2, column 6 (the "2" in "line 2")
    let target = Target::Position { line: 2, col: 6 };

    // Step 2: Create zero-width boundary for insertion
    let boundary = Boundary::new(target, BoundaryMode::Exclude);

    // Step 3: Create snippet at that position
    let snippet = Snippet::At(boundary);

    // Step 4: Insert text at precise position
    let result = snippet.replace(&rope, "X").unwrap();

    // The Position (2, 6) is at char index 12, which is the '2' character
    // Exclude mode places us at index 12 (zero-width)
    // Inserting pushes the '2' forward
    assert_eq!(result.to_string(), "line 1\nline X2\nline 3\n");
}

#[cfg(feature = "regex")]
#[test]
fn test_workflow_regex_boundary_replacement() {
    // Complete workflow using regex patterns for boundaries
    // Demonstrates Target::Pattern → boundary resolution → replace
    let rope = Rope::from_str("Config: version=1.2.3, status=active, mode=production");

    // Step 1: Create regex pattern to match version number
    let target = Target::pattern(r"version=\d+\.\d+\.\d+").unwrap();

    // Step 2: Create boundary that includes the match
    let boundary = Boundary::new(target, BoundaryMode::Include);

    // Step 3: Create snippet at the matched pattern
    let snippet = Snippet::At(boundary);

    // Step 4: Replace with updated version
    let result = snippet.replace(&rope, "version=2.0.0").unwrap();

    assert_eq!(result.to_string(), "Config: version=2.0.0, status=active, mode=production");
}

#[test]
fn test_workflow_edge_case_single_char_rope() {
    // Tests all operations on single-character rope
    // Verifies edge case handling for minimal content
    let rope = Rope::from_str("x");

    // Test 1: Replace entire rope
    let snippet_all = Snippet::All;
    let result1 = snippet_all.replace(&rope, "y").unwrap();
    assert_eq!(result1.to_string(), "y");

    // Test 2: Insert before the character
    let target_char = Target::Char(0);
    let boundary_exclude = Boundary::new(target_char, BoundaryMode::Exclude);
    let snippet_insert = Snippet::At(boundary_exclude);
    let result2 = snippet_insert.replace(&rope, "a").unwrap();
    assert_eq!(result2.to_string(), "ax");

    // Test 3: Delete the character
    let boundary_include = Boundary::new(Target::Char(0), BoundaryMode::Include);
    let snippet_delete = Snippet::At(boundary_include);
    let result3 = snippet_delete.replace(&rope, "").unwrap();
    assert_eq!(result3.to_string(), "");
}

#[test]
fn test_workflow_edge_case_eof_operations() {
    // Tests operations at exact EOF position
    // Verifies boundary conditions at rope end
    let rope = Rope::from_str("line 1\nline 2\n");

    // Test 1: Insert at EOF
    let eof_pos = rope.len_chars();
    let target_eof = Target::Char(eof_pos - 1); // Last actual character
    let boundary_after = Boundary::new(target_eof, BoundaryMode::Include);
    let snippet_from_last = Snippet::From(boundary_after);
    let result1 = snippet_from_last.replace(&rope, "line 3\n").unwrap();
    assert_eq!(result1.to_string(), "line 1\nline 2\nline 3\n");

    // Test 2: Select from beginning to EOF
    let last_line = Target::Line(rope.len_lines() - 1);
    let boundary_last_line = Boundary::new(last_line, BoundaryMode::Exclude);
    let snippet_to_eof = Snippet::From(boundary_last_line);
    let result2 = snippet_to_eof.replace(&rope, "").unwrap();
    assert_eq!(result2.to_string(), "line 1\nline 2\n");

    // Test 3: Append to end using To snippet
    let target_end = Target::Char(rope.len_chars() - 1);
    let boundary_to_end = Boundary::new(target_end, BoundaryMode::Include);
    let snippet_append = Snippet::From(boundary_to_end);
    let result3 = snippet_append.replace(&rope, "appended").unwrap();
    assert_eq!(result3.to_string(), "line 1\nline 2\nappended");
}
