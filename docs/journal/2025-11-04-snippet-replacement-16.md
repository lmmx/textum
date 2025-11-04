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

---

## File Organization Plan

1. **src/snip/snippet/error.rs** - Create SnippetError enum with variants: BoundaryError(BoundaryError), InvalidRange, InvalidUtf8, OutOfBounds
2. **src/snip/snippet/resolution.rs** - Create SnippetResolution struct and Snippet::resolve implementation
3. **src/snip/snippet/replacement.rs** - Create Snippet::replace, validate_replacement_utf8, apply_replacement implementations
4. **src/snip/snippet.rs** - Add module declarations: `pub mod error;`, `pub mod resolution;`, `pub mod replacement;`
5. **src/snip/snippet.rs** - Add re-exports: `pub use error::*;`, `pub use resolution::*;`
6. **src/snip.rs** - Add re-export: `pub use snippet::{SnippetError, SnippetResolution};`
7. **src/lib.rs** - Add re-export: `pub use snip::snippet::{Snippet, SnippetError, SnippetResolution};`
8. **src/tests/snippet_resolution.rs** - Create comprehensive resolution tests for all Snippet variants
9. **src/tests/snippet_replacement.rs** - Create replacement operation tests (insert, delete, edit)
10. **src/tests/snippet_integration.rs** - Create end-to-end workflow tests

## Design Specifications

### SnippetError Enum

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnippetError {
    BoundaryError(BoundaryError),
    InvalidRange { start: usize, end: usize },
    InvalidUtf8(String),
    OutOfBounds { index: usize, rope_len: usize },
}
```

Variants represent:
- `BoundaryError` - wraps errors from boundary resolution
- `InvalidRange` - start >= end after boundary resolution
- `InvalidUtf8` - replacement string contains invalid UTF-8
- `OutOfBounds` - resolved range exceeds rope length

### SnippetResolution Struct

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnippetResolution {
    pub start: usize,
    pub end: usize,
}
```

Represents the final (start, end) char indices after resolving all boundaries. End is exclusive for consistency with Rust slice semantics.

### Snippet::resolve Method

Located in src/snip/snippet/resolution.rs:

```rust
impl Snippet {
    pub fn resolve(&self, rope: &Rope) -> Result<SnippetResolution, SnippetError> {
        match self {
            Snippet::At(boundary) => {
                let res = boundary.resolve(rope)?;
                validate_range(res.start, res.end, rope)?;
                Ok(SnippetResolution { start: res.start, end: res.end })
            }
            Snippet::From(boundary) => {
                let res = boundary.resolve(rope)?;
                let end = rope.len_chars();
                validate_range(res.end, end, rope)?;
                Ok(SnippetResolution { start: res.end, end })
            }
            Snippet::To(boundary) => {
                let res = boundary.resolve(rope)?;
                validate_range(0, res.start, rope)?;
                Ok(SnippetResolution { start: 0, end: res.start })
            }
            Snippet::Between { start, end } => {
                let start_res = start.resolve(rope)?;
                let end_res = end.resolve(rope)?;
                validate_range(start_res.end, end_res.start, rope)?;
                Ok(SnippetResolution { start: start_res.end, end: end_res.start })
            }
            Snippet::All => {
                Ok(SnippetResolution { start: 0, end: rope.len_chars() })
            }
        }
    }
}
```

Helper function `validate_range` checks start < end and both within rope bounds, returning SnippetError::InvalidRange or SnippetError::OutOfBounds as appropriate.

### Snippet::replace Method

Located in src/snip/snippet/replacement.rs:

```rust
impl Snippet {
    pub fn replace(&self, rope: &Rope, replacement: &str) -> Result<Rope, SnippetError> {
        validate_replacement_utf8(replacement)?;
        let resolution = self.resolve(rope)?;
        apply_replacement(rope, resolution.start, resolution.end, replacement)
    }
}
```

Validates UTF-8, resolves snippet to indices, applies replacement to rope.

### validate_replacement_utf8 Function

```rust
fn validate_replacement_utf8(s: &str) -> Result<(), SnippetError> {
    // str is already UTF-8 valid by Rust's type system, but check for any
    // additional validation requirements (e.g., no null bytes, specific encoding)
    if s.contains('\0') {
        return Err(SnippetError::InvalidUtf8("null bytes not allowed".into()));
    }
    Ok(())
}
```

Performs validation beyond Rust's built-in UTF-8 guarantees if needed.

### apply_replacement Function

```rust
fn apply_replacement(
    rope: &Rope,
    start: usize,
    end: usize,
    replacement: &str,
) -> Result<Rope, SnippetError> {
    let mut new_rope = rope.clone();
    new_rope.remove(start..end);
    new_rope.insert(start, replacement);
    Ok(new_rope)
}
```

Creates modified rope by removing range and inserting replacement. Zero-width ranges (start == end) perform pure insertion. Empty replacement performs pure deletion.

### Test Coverage

**src/tests/snippet_resolution.rs** tests:
- Each Snippet variant resolves correctly
- Boundary modes affect resolution as expected
- Invalid ranges detected and reported
- Out-of-bounds ranges rejected

**src/tests/snippet_replacement.rs** tests:
- Insert operation (zero-width range)
- Delete operation (empty replacement)
- Edit operation (replace existing text)
- UTF-8 validation catches invalid input
- Boundary inclusion/exclusion respected in final result

**src/tests/snippet_integration.rs** tests:
- Complete workflow: Target → Boundary → Snippet → Resolution → Replacement
- Multi-line replacements
- Regex-based snippet patterns (if regex feature enabled)
- Edge cases: empty rope, EOF positions, BOF positions
