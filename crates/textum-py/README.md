# ¶ ⠶ textum

[![PyPI](https://img.shields.io/pypi/v/textum?color=%2300dc00)](https://pypi.org/project/textum)
[![crates.io](https://img.shields.io/crates/v/textum.svg)](https://crates.io/crates/textum)
[![documentation](https://docs.rs/textum/badge.svg)](https://docs.rs/textum)
[![MIT/Apache-2.0 licensed](https://img.shields.io/crates/l/textum.svg)](./LICENSE)
[![pre-commit.ci status](https://results.pre-commit.ci/badge/github/lmmx/textum/master.svg)](https://results.pre-commit.ci/latest/github/lmmx/textum/master)

A syntactic patching library with char-level granularity.

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

# Work with multiple patches
patchset = textum.PatchSet()
patchset.add(patch)

# Apply to actual files
results = patchset.apply_to_files()
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
```

### JSON Import/Export

```python
# Load patches from JSON
json_data = '[{"file": "test.txt", ...}]'
patches = textum.load_patches_from_json(json_data)

# Save patches to JSON
json_str = textum.save_patches_to_json(patches)
```

## Licensing

Textum is [MIT licensed](https://github.com/lmmx/textum/blob/master/LICENSE), a permissive open source license.
