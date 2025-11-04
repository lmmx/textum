# 2025-11-03: Target Matching

## Current State

- Target enum defined with five variants: Literal, Pattern, Line, Char, Position (src/snip/target.rs:6-20)
- Pattern variant stores both pattern string and compiled regex (src/snip/target.rs:11-15)
- Target::pattern constructor creates Pattern from string, returns TargetError::InvalidPattern on compilation failure (src/snip/target.rs:40-47)
- Target derives Debug, Clone, PartialEq, Eq, Hash comparing pattern strings for Pattern variant (src/snip/target.rs:24-48, src/snip/target.rs:50-75)
- Pattern variant feature-gated behind "regex" cargo feature using regex-cursor crate (src/snip/target.rs:11, Cargo.toml)
- TargetError enum with NotFound, OutOfBounds, InvalidPosition, InvalidPattern variants (src/snip/target/error.rs:6-18)
- Target::resolve method returns Result<usize, TargetError> with char index of first match (src/snip/target/matching.rs:15-50)
- resolve_literal searches rope via chars() iterator (src/snip/target/matching.rs:53-81)
- resolve_pattern uses regex-cursor RopeyCursor for chunk-aware regex matching (src/snip/target/matching.rs:84-98)
- resolve_line converts line number to char index with bounds validation (src/snip/target/matching.rs:101-108)
- resolve_char validates char index against rope.len_chars() with strict bounds (src/snip/target/matching.rs:111-118)
- resolve_position converts one-indexed line/col to char index with line length validation (src/snip/target/matching.rs:121-145)
- Unit tests cover all Target variants and error cases (src/tests/target_matching.rs)
