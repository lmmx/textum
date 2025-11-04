# 2025-11-04: Patch-Snippet Integration

## Current State

- Patch struct has file, range, replacement fields (src/patch.rs:65-92)
- Patch.apply modifies rope in-place using char ranges (src/patch.rs:175-193)
- PatchSet groups patches by file, sorts by reverse position, applies sequentially (src/composer.rs:133-163)
- Snippet system resolves boundaries to char ranges via Target matching (src/snip/snippet/resolution.rs:37-125)
- Snippet.replace creates new rope with replacement applied (src/snip/snippet/replacement.rs:60-91)
- Target supports Literal, Pattern, Line, Char, Position matching (src/snip/target.rs:11-35)
- Boundary combines Target with BoundaryMode (Exclude, Include, Extend) (src/snip/snippet/boundary.rs:20-30)
- Extent supports Lines, Chars, Bytes, Matching(count, Target) (src/snip/snippet/boundary/extent.rs:11-21)
- replacement field is Option<String> to distinguish deletion (None) from empty replacement (Some("")) (src/patch.rs:71-77)

## Missing

- Patch.snippet field to replace range field
- Patch.apply implementation using Snippet.resolve
- PatchSet overlap detection for resolved ranges with non-empty replacements
- PatchSet intra-line reverse-order sorting using char indices
- Patch::from_literal_target constructor
- Patch::from_line_range constructor
- Updated Patch::from_line_positions using Snippet::At with Position target
- PatchError variants for SnippetError, BoundaryError, TargetError
- Tests for Snippet-based Patch construction
- Tests for PatchSet overlap detection and rejection
- CLI JSON deserialization for Snippet-based patch format
- Documentation examples showing Snippet-based Patch API

## Divergence

- Patch.replacement is Option<String> semantically distinguishing deletion from empty replacement (src/patch.rs:71-77) but Snippet.replace accepts &str without null distinction (src/snip/snippet/replacement.rs:60)
- max_line_drift field exists for fuzzy matching (src/patch.rs:89-92) but Target::Pattern provides actual pattern matching via regex
- README shows range-based construction but implementation will use Snippet-based construction
