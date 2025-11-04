# 2025-11-04: Boundary Resolution

## Current State

- Boundary struct pairs Target with BoundaryMode (src/snip/snippet/boundary.rs:11-14)
- Boundary fields are public (src/snip/snippet/boundary.rs:12-13)
- Boundary provides new() constructor (src/snip/snippet/boundary.rs:17-20)
- BoundaryMode enum defines Exclude, Include, Extend(Extent) (src/snip/snippet/boundary/mode.rs:6-10)
- Extent enum defines Lines, Chars, Bytes, Matching(usize, Target) (src/snip/snippet/boundary/extent.rs:6-11)
- Target::resolve method returns first char index of match (src/snip/target/matching.rs:41-50)
- TargetError enum with NotFound, OutOfBounds, InvalidPosition, InvalidPattern variants (src/snip/target/error.rs:6-21)

## Missing

- Target::resolve_range method returning (start, end) span in src/snip/target/matching.rs
- BoundaryResolution struct in src/snip/snippet/boundary.rs or new src/snip/snippet/boundary/resolution.rs
- BoundaryError enum in new src/snip/snippet/boundary/error.rs
- Boundary::resolve method in new src/snip/snippet/boundary/resolution.rs
- calculate_lines_extent function in src/snip/snippet/boundary/extent.rs
- calculate_chars_extent function in src/snip/snippet/boundary/extent.rs
- calculate_bytes_extent function in src/snip/snippet/boundary/extent.rs
- calculate_matching_extent function in src/snip/snippet/boundary/extent.rs
- Unit tests for Target::resolve_range in src/tests/target_matching.rs
- Unit tests for Boundary::resolve in new src/tests/boundary_resolution.rs
- Unit tests for extent calculations in src/tests/boundary_resolution.rs
