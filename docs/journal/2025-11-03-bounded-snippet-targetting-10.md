# 2025-11-03: Target Matching

## Current State

- Target enum defined with five variants: Literal, Pattern, Line, Char, Position (src/snip/target.rs:4-14)
- Target derives Debug, Clone, PartialEq, Eq, Hash (src/snip/target.rs:3, src/snip/target.rs:18-60)
- Pattern variant feature-gated behind "regex" cargo feature (src/snip/target.rs:6-7)

## Missing

- Target resolution to rope char indices (no src/snip/target/matching.rs exists)
- Target resolution error type with variants for NotFound, OutOfBounds, InvalidPosition (no src/snip/target/error.rs exists)
- Target::resolve method returning Result<usize, TargetError> to find first char position in rope
- Function to search rope for first Literal string match
- Function to search rope for first Pattern regex match  
- Function to convert Line number to char index via rope.line_to_char with bounds validation
- Function to validate Char index against rope.len_chars() bounds
- Function to convert Position{line, col} to char index with one-indexing adjustment and bounds validation
- Unit tests for target resolution at src/tests/target_matching.rs with #[path] attribute in src/snip/target.rs

---

Plan:

1. Create `src/snip/target/error.rs` with `TargetError` enum (NotFound, OutOfBounds, InvalidPosition variants), all documented
2. Create `src/snip/target/matching.rs` with resolution functions, all documented
3. Add `impl Target { pub fn resolve(&self, rope: &Rope) -> Result<usize, TargetError> }` in matching.rs
4. Create `src/tests/target_matching.rs` with unit tests
5. Add `#[path = "tests/target_matching.rs"] mod target_matching;` at end of src/snip/target.rs
