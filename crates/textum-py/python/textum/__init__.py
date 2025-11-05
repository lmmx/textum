"""Python bindings for textum - syntactic patching with char-level granularity."""

from __future__ import annotations

from ._textum import (
    PyBoundary as Boundary,
)
from ._textum import (
    PyPatch as Patch,
)
from ._textum import (
    PyPatchSet as PatchSet,
)
from ._textum import (
    PySnippet as Snippet,
)
from ._textum import (
    PyTarget as Target,
)
from ._textum import (
    load_patches_from_json,
    save_patches_to_json,
)

__version__ = "0.1.0"

__all__ = [
    "Patch",
    "PatchSet",
    "Snippet",
    "Boundary",
    "Target",
    "load_patches_from_json",
    "save_patches_to_json",
]
