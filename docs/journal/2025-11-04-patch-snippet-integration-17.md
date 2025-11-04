# 2025-11-04: Patch-Snippet Integration Plan

## Current State

- Patch struct has file, range, replacement fields (src/patch.rs:65-92)
- Patch.apply modifies rope in-place using char ranges (src/patch.rs:175-193)
- PatchSet groups patches by file, sorts by reverse position, applies sequentially (src/composer.rs:133-163)
- Snippet system resolves boundaries to char ranges via Target matching (src/snip/snippet/resolution.rs:37-125)
- Snippet.replace creates new rope with replacement applied (src/snip/snippet/replacement.rs:60-91)
- Target supports Literal, Pattern, Line, Char, Position matching (src/snip/target.rs:11-35)
- Boundary combines Target with BoundaryMode (Exclude, Include, Extend) (src/snip/snippet/boundary.rs:20-30)
- Extent supports Lines, Chars, Bytes, Matching(count, Target) (src/snip/snippet/boundary/extent.rs:11-21)
- max_line_drift field exists but unused in Patch (src/patch.rs:89-92, feature-gated)
- symbol_path field exists but unused in Patch (src/patch.rs:82-85, feature-gated)

## Missing

- Patch constructor accepting Snippet instead of raw range
- Patch.apply implementation using Snippet.replace instead of direct rope manipulation
- Target field in Patch struct to replace range field
- Boundary field(s) in Patch struct to specify selection semantics
- Migration path for existing range-based Patch construction (backwards compatibility or breaking change decision)
- Updated Patch.from_line_positions to construct Snippet from line/col coordinates
- PatchSet logic to handle Snippet-based patches (may need resolution before sorting)
- Removal of max_line_drift field and associated feature gate
- Tests for Patch using Snippet::At with various BoundaryMode options
- Tests for Patch using Snippet::Between with marker-based boundaries
- Tests for PatchSet applying multiple Snippet-based patches to same file
- Integration tests demonstrating fuzzy matching via Target::Pattern
- CLI JSON deserialization support for Snippet-based patch format
- Documentation updates in lib.rs examples showing Snippet-based Patch construction
- facet derive compatibility for new Patch struct shape (if using Snippet fields)

## Divergence

- README and lib.rs examples show range-based Patch construction (src/lib.rs:9-23, src/patch.rs:41-62) but planned implementation will use Snippet-based construction
- max_line_drift field documented as "fuzzy matching" feature (src/patch.rs:89-92) but Target::Pattern provides actual fuzzy matching capability through regex
- Patch described as "character-level granularity" (src/lib.rs:3-6) but Snippet system supports line-level, byte-level, and pattern-based granularity through Extent
