use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;
use textum::{Boundary, BoundaryMode, Patch, PatchSet, Snippet, Target};

/// A Python wrapper for the Patch struct
#[pyclass]
struct PyPatch {
    inner: Patch,
}

#[pymethods]
impl PyPatch {
    #[new]
    #[pyo3(signature = (file, snippet, replacement, symbol_path=None))]
    fn new(
        file: String,
        snippet: PySnippet,
        replacement: String,
        symbol_path: Option<Vec<String>>,
    ) -> Self {
        PyPatch {
            inner: Patch {
                file,
                snippet: snippet.inner,
                replacement,
                #[cfg(feature = "symbol_path")]
                symbol_path,
            },
        }
    }

    /// Create a patch from a literal string target
    #[staticmethod]
    #[pyo3(signature = (file, needle, mode, replacement))]
    fn from_literal_target(
        file: String,
        needle: String,
        mode: String,
        replacement: String,
    ) -> PyResult<Self> {
        let boundary_mode = parse_boundary_mode(&mode)?;
        Ok(PyPatch {
            inner: Patch::from_literal_target(file, &needle, boundary_mode, replacement),
        })
    }

    /// Create a patch from a line range
    #[staticmethod]
    fn from_line_range(
        file: String,
        start_line: usize,
        end_line: usize,
        replacement: String,
    ) -> Self {
        PyPatch {
            inner: Patch::from_line_range(file, start_line, end_line, replacement),
        }
    }

    /// Apply this patch to a file's content (as string)
    fn apply_to_string(&self, content: String) -> PyResult<String> {
        let mut rope = ropey::Rope::from_str(&content);
        self.inner
            .apply(&mut rope)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        Ok(rope.to_string())
    }

    fn __repr__(&self) -> String {
        format!(
            "Patch(file='{}', replacement='{}')",
            self.inner.file, self.inner.replacement
        )
    }
}

/// A Python wrapper for PatchSet
#[pyclass]
struct PyPatchSet {
    inner: PatchSet,
}

#[pymethods]
impl PyPatchSet {
    #[new]
    fn new() -> Self {
        PyPatchSet {
            inner: PatchSet::new(),
        }
    }

    /// Add a patch to this set
    fn add(&mut self, patch: PyPatch) {
        self.inner.add(patch.inner.clone());
    }

    /// Apply all patches to their target files
    fn apply_to_files(&self) -> PyResult<HashMap<String, String>> {
        self.inner
            .apply_to_files()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))
    }

    fn __repr__(&self) -> String {
        "PatchSet()".to_string()
    }
}

/// A Python wrapper for Snippet
#[pyclass]
#[derive(Clone)]
struct PySnippet {
    inner: Snippet,
}

#[pymethods]
impl PySnippet {
    /// Create an At snippet
    #[staticmethod]
    fn at(boundary: PyBoundary) -> Self {
        PySnippet {
            inner: Snippet::At(boundary.inner),
        }
    }

    /// Create a From snippet
    #[staticmethod]
    fn from_boundary(boundary: PyBoundary) -> Self {
        PySnippet {
            inner: Snippet::From(boundary.inner),
        }
    }

    /// Create a To snippet
    #[staticmethod]
    fn to(boundary: PyBoundary) -> Self {
        PySnippet {
            inner: Snippet::To(boundary.inner),
        }
    }

    /// Create a Between snippet
    #[staticmethod]
    fn between(start: PyBoundary, end: PyBoundary) -> Self {
        PySnippet {
            inner: Snippet::Between {
                start: start.inner,
                end: end.inner,
            },
        }
    }

    /// Create an All snippet (selects entire file)
    #[staticmethod]
    fn all() -> Self {
        PySnippet {
            inner: Snippet::All,
        }
    }
}

/// A Python wrapper for Boundary
#[pyclass]
#[derive(Clone)]
struct PyBoundary {
    inner: Boundary,
}

#[pymethods]
impl PyBoundary {
    #[new]
    fn new(target: PyTarget, mode: String) -> PyResult<Self> {
        let boundary_mode = parse_boundary_mode(&mode)?;
        Ok(PyBoundary {
            inner: Boundary::new(target.inner, boundary_mode),
        })
    }
}

/// A Python wrapper for Target
#[pyclass]
#[derive(Clone)]
struct PyTarget {
    inner: Target,
}

#[pymethods]
impl PyTarget {
    /// Create a Literal target
    #[staticmethod]
    fn literal(needle: String) -> Self {
        PyTarget {
            inner: Target::Literal(needle),
        }
    }

    /// Create a Line target
    #[staticmethod]
    fn line(line_number: usize) -> Self {
        PyTarget {
            inner: Target::Line(line_number),
        }
    }

    /// Create a Char target
    #[staticmethod]
    fn char(char_index: usize) -> Self {
        PyTarget {
            inner: Target::Char(char_index),
        }
    }

    /// Create a Position target
    #[staticmethod]
    fn position(line: usize, col: usize) -> Self {
        PyTarget {
            inner: Target::Position { line, col },
        }
    }

    #[cfg(feature = "regex")]
    /// Create a Pattern (regex) target
    #[staticmethod]
    fn pattern(pattern: String) -> PyResult<Self> {
        Target::pattern(pattern)
            .map(|inner| PyTarget { inner })
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
    }
}

/// Load patches from JSON
#[pyfunction]
fn load_patches_from_json(json_str: String) -> PyResult<Vec<PyPatch>> {
    let patches: Vec<Patch> = facet_json::from_str(&json_str)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;

    Ok(patches.into_iter().map(|inner| PyPatch { inner }).collect())
}

/// Save patches to JSON
#[pyfunction]
fn save_patches_to_json(patches: Vec<PyPatch>) -> PyResult<String> {
    let inner_patches: Vec<Patch> = patches.into_iter().map(|p| p.inner).collect();
    facet_json::to_string(&inner_patches)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
}

// Helper function to parse boundary mode strings
fn parse_boundary_mode(mode: &str) -> PyResult<BoundaryMode> {
    match mode.to_lowercase().as_str() {
        "include" => Ok(BoundaryMode::Include),
        "exclude" => Ok(BoundaryMode::Exclude),
        _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Invalid boundary mode: '{}'. Must be 'include' or 'exclude'",
            mode
        ))),
    }
}

#[pymodule]
fn _textum(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyPatch>()?;
    m.add_class::<PyPatchSet>()?;
    m.add_class::<PySnippet>()?;
    m.add_class::<PyBoundary>()?;
    m.add_class::<PyTarget>()?;
    m.add_function(wrap_pyfunction!(load_patches_from_json, m)?)?;
    m.add_function(wrap_pyfunction!(save_patches_to_json, m)?)?;
    Ok(())
}
