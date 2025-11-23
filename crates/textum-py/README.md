# ¶ ⠶ textum

[![PyPI](https://img.shields.io/pypi/v/textum?color=%2300dc00)](https://pypi.org/project/textum)
[![crates.io](https://img.shields.io/crates/v/textum.svg)](https://crates.io/crates/textum)
[![documentation](https://docs.rs/textum/badge.svg)](https://docs.rs/textum)
[![MIT/Apache-2.0 licensed](https://img.shields.io/crates/l/textum.svg)](./LICENSE)
[![pre-commit.ci status](https://results.pre-commit.ci/badge/github/lmmx/textum/master.svg)](https://results.pre-commit.ci/latest/github/lmmx/textum/master)

A syntactic patching library with character-level granularity.

## Installation
```bash
pip install textum
```

## Quick Start
```python
import textum

# Create a simple patch
patch = textum.Patch.from_literal_target(
    file="example.txt",
    needle="old text",
    mode="include",
    replacement="new text"
)

# Apply to string content
content = "This is old text in a file"
result = patch.apply_to_string(content)
print(result)  # "This is new text in a file"

# Apply to a file and get result without writing
result = patch.apply_to_file()
print(result)  # File content with patch applied

# Or write directly to disk
patch.write_to_file()
```

## Working with Multiple Patches
```python
import textum

# Create multiple patches
patchset = textum.PatchSet()

patch1 = textum.Patch.from_literal_target(
    file="example.txt",
    needle="foo",
    mode="include",
    replacement="FOO"
)

patch2 = textum.Patch.from_literal_target(
    file="example.txt",
    needle="bar",
    mode="include",
    replacement="BAR"
)

patchset.add(patch1)
patchset.add(patch2)

# Get results for inspection
results = patchset.apply_to_files()
for file, content in results.items():
    print(f"{file}: {content}")

# Or write all changes to disk
patchset.write_to_files()
```

## Advanced Usage

### Using Snippets and Boundaries
```python
# Create a target
target = textum.Target.literal("hello")

# Create a boundary with mode
boundary = textum.Boundary(target, "include")

# Create a snippet
snippet = textum.Snippet.at(boundary)

# Create a patch with the snippet
patch = textum.Patch(
    file="test.txt",
    snippet=snippet,
    replacement="goodbye"
)
```

### Line-based Patching
```python
# Replace lines 5-10 (start inclusive, end exclusive)
patch = textum.Patch.from_line_range(
    file="large_file.txt",
    start_line=5,
    end_line=10,
    replacement="new content\n"
)

# Delete lines 5-10
patch = textum.Patch.from_line_range(
    file="large_file.txt",
    start_line=5,
    end_line=10,
    replacement=""
)
```

### Between Markers
```python
# Replace content between HTML comments
start = textum.Boundary(
    textum.Target.literal("<!-- start -->"),
    "exclude"
)
end = textum.Boundary(
    textum.Target.literal("<!-- end -->"),
    "exclude"
)

snippet = textum.Snippet.between(start, end)

patch = textum.Patch(
    file="template.html",
    snippet=snippet,
    replacement="new content"
)

# Preview the change
result = patch.apply_to_file()
print(result)

# Apply if satisfied
patch.write_to_file()
```

### Pattern Matching with Regex
```python
# Use regex patterns to match targets
target = textum.Target.pattern(r"version = \"\d+\.\d+\.\d+\"")
boundary = textum.Boundary(target, "include")
snippet = textum.Snippet.at(boundary)

patch = textum.Patch(
    file="pyproject.toml",
    snippet=snippet,
    replacement='version = "2.0.0"'
)
```

### JSON Import/Export
```python
# Load patches from JSON
json_data = '''[{
    "file": "test.txt",
    "snippet": {
        "At": {
            "target": {"Literal": "old"},
            "mode": "Include"
        }
    },
    "replacement": "new"
}]'''
patches = textum.load_patches_from_json(json_data)

# Apply loaded patches
patchset = textum.PatchSet()
for patch in patches:
    patchset.add(patch)
patchset.write_to_files()

# Save patches to JSON
json_str = textum.save_patches_to_json(patches)
```

## API Overview

### Patch Methods

- `apply_to_string(content: str) -> str` - Apply patch to string content
- `apply_to_file() -> str` - Read file, apply patch, return result (doesn't write)
- `write_to_file() -> None` - Read file, apply patch, write back to disk

### PatchSet Methods

- `add(patch: Patch) -> None` - Add a patch to the set
- `apply_to_files() -> dict[str, str]` - Apply all patches, return results (doesn't write)
- `write_to_files() -> None` - Apply all patches and write to disk

### Target Types

- `Target.literal(text: str)` - Match exact text
- `Target.pattern(regex: str)` - Match using regex
- `Target.line(line_number: int)` - Match specific line
- `Target.char(char_index: int)` - Match character position
- `Target.position(line: int, col: int)` - Match line:column position

### Boundary Modes

- `"include"` - Include the matched target in the replacement range
- `"exclude"` - Exclude the matched target from the replacement range

## Licensing

Textum is [MIT licensed](https://github.com/lmmx/textum/blob/master/LICENSE), a permissive open source license.
