import os
import tempfile

import textum


def test_literal_patch():
    """Test basic literal string replacement."""
    patch = textum.Patch.from_literal_target(
        file="test.txt",
        needle="world",
        mode="include",
        replacement="rust",
    )

    result = patch.apply_to_string("hello world")
    assert result == "hello rust"


def test_patchset():
    """Test applying multiple patches."""
    patchset = textum.PatchSet()

    patch1 = textum.Patch.from_literal_target(
        file="test.txt", needle="foo", mode="include", replacement="FOO"
    )
    patch2 = textum.Patch.from_literal_target(
        file="test.txt", needle="bar", mode="include", replacement="BAR"
    )

    patchset.add(patch1)
    patchset.add(patch2)


def test_line_range():
    """Test line-based patching."""
    content = "line1\nline2\nline3\nline4\n"

    patch = textum.Patch.from_line_range(
        file="test.txt", start_line=1, end_line=3, replacement="replaced\n"
    )

    result = patch.apply_to_string(content)
    assert result == "line1\nreplaced\nline4\n"


def test_apply_to_file():
    """Test applying a patch to a file from disk."""
    # Create a temporary file
    with tempfile.NamedTemporaryFile(mode="w", delete=False, suffix=".txt") as f:
        f.write("hello world\n")
        temp_path = f.name

    try:
        # Create patch
        patch = textum.Patch.from_literal_target(
            file=temp_path,
            needle="world",
            mode="include",
            replacement="rust",
        )

        # Apply and get result without writing
        result = patch.apply_to_file()
        assert result == "hello rust\n"

        # Verify file wasn't modified yet
        with open(temp_path, "r") as f:
            content = f.read()
        assert content == "hello world\n"

        # Now write to file
        patch.write_to_file()

        # Verify the file was modified
        with open(temp_path, "r") as f:
            content = f.read()
        assert content == "hello rust\n"
    finally:
        # Clean up
        if os.path.exists(temp_path):
            os.unlink(temp_path)


def test_write_to_file():
    """Test writing a patch directly to a file."""
    # Create a temporary file
    with tempfile.NamedTemporaryFile(mode="w", delete=False, suffix=".txt") as f:
        f.write("hello world\n")
        temp_path = f.name

    try:
        # Create and apply patch
        patch = textum.Patch.from_literal_target(
            file=temp_path,
            needle="world",
            mode="include",
            replacement="rust",
        )

        patch.write_to_file()  # ← Changed from apply_to_file()

        # Verify the file was modified
        with open(temp_path, "r") as f:
            content = f.read()

        assert content == "hello rust\n"
    finally:
        # Clean up
        if os.path.exists(temp_path):
            os.unlink(temp_path)
