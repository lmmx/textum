# textum-py

Python bindings for [textum](https://github.com/lmmx/textum) - a syntactic patching library with char-level granularity.

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

## License

MIT
