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
