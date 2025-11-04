# 2025-11-04: Snippet Replacement Operation

## Current State

- Snippet enum defines five range selection variants: At, From, To, Between, All (src/snip/snippet.rs:15-34)
- Boundary struct pairs Target with BoundaryMode (src/snip/snippet/boundary.rs:20-33)
- BoundaryResolution struct provides concrete (start, end) char indices (src/snip/snippet/boundary/resolution.rs:7-16)
- Boundary::resolve method converts boundaries to BoundaryResolution (src/snip/snippet/boundary/resolution.rs:31-65)
- Target::resolve_range returns (start, end) spans for all Target variants (src/snip/target/matching.rs:62-152)
- BoundaryMode::Exclude, Include, Extend control boundary treatment (src/snip/snippet/boundary/mode.rs:6-14)
- Extent calculations handle Lines, Chars, Bytes, Matching extensions (src/snip/snippet/boundary/extent.rs:36-253)
- All boundary resolution functions return Result with BoundaryError (src/snip/snippet/boundary/error.rs:5-27)
- Target resolution functions return Result with TargetError (src/snip/target/error.rs:6-45)

## Missing

- SnippetResolution struct in new src/snip/snippet/resolution.rs to hold resolved (start, end) range
- SnippetError enum in new src/snip/snippet/error.rs for replacement operation errors
- Snippet::resolve method in src/snip/snippet/resolution.rs returning SnippetResolution from resolved boundaries
- Snippet::replace method in src/snip/snippet/replacement.rs accepting rope and replacement string, returning Result<Rope, SnippetError>
- Helper function validate_replacement_utf8 in src/snip/snippet/replacement.rs checking replacement string validity
- Helper function apply_replacement in src/snip/snippet/replacement.rs performing actual rope modification
- Module declarations in src/snip/snippet.rs for error, resolution, replacement modules
- Re-exports in src/snip.rs and src/lib.rs for public API (Snippet, SnippetError, SnippetResolution)
- Unit tests in src/tests/snippet_resolution.rs covering all Snippet variants
- Unit tests in src/tests/snippet_replacement.rs covering insert, delete, edit operations
- Integration tests in src/tests/snippet_integration.rs demonstrating end-to-end snippet operations
