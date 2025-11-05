use super::{
    calculate_bytes_extent, calculate_chars_extent, calculate_lines_extent,
    calculate_matching_extent, Boundary, BoundaryError, BoundaryMode, Extent,
};
use ropey::Rope;

#[derive(Debug, Clone, PartialEq, Eq)]
/// The concrete start and end indices of a resolved boundary within a [`Rope`].
///
/// Represents the absolute character range of a snippet boundary once all targets and extents have
/// been resolved to numeric indices.
pub struct BoundaryResolution {
    /// The starting character index of the resolved boundary.
    pub start: usize,
    /// The ending character index of the resolved boundary (exclusive).
    pub end: usize,
}

impl Boundary {
    /// Resolves this boundary into a pair of absolute character indices.
    ///
    /// The resolution process combines the boundary’s base target with its
    /// configured extent, producing a concrete `(start, end)` range suitable
    /// for slicing a [`Rope`].
    ///
    /// # Errors
    ///
    /// Returns a [`BoundaryError::TargetError`] if the base target could not
    /// be resolved, or a [`BoundaryError::ExtentOutOfBounds`] /
    /// [`BoundaryError::InvalidExtent`] if the extent specification was
    /// invalid or extended past the end of the rope.
    ///
    /// # Examples
    ///
    /// ```
    /// use ropey::Rope;
    /// use textum::snip::snippet::boundary::BoundaryResolution;
    /// use textum::Target;
    ///
    /// let rope = Rope::from_str("alpha\nbeta\ngamma\n");
    /// let target = Target::Line(1);
    /// let start = target.resolve(&rope).unwrap();
    /// let end = start + 5;
    /// let boundary = BoundaryResolution { start, end };
    ///
    /// assert_eq!(boundary.start, 6);
    /// assert_eq!(boundary.end, 11);
    /// ```
    pub fn resolve(&self, rope: &Rope) -> Result<BoundaryResolution, BoundaryError> {
        let (start, end) = self
            .target
            .resolve_range(rope)
            .map_err(BoundaryError::from)?;
        match &self.mode {
            BoundaryMode::Exclude => Ok(BoundaryResolution { start: end, end }),
            BoundaryMode::Include => Ok(BoundaryResolution { start, end }),
            BoundaryMode::Extend(extent) => {
                let new_end = match extent {
                    Extent::Lines(n) => calculate_lines_extent(rope, end, *n)?,
                    Extent::Chars(n) => calculate_chars_extent(rope, end, *n)?,
                    Extent::Bytes(n) => calculate_bytes_extent(rope, end, *n)?,
                    Extent::Matching(n, t) => calculate_matching_extent(rope, end, *n, t)?,
                };
                Ok(BoundaryResolution {
                    start: end,
                    end: new_end,
                })
            }
        }
    }
}

#[cfg(test)]
#[path = "../../../tests/boundary_resolution.rs"]
mod boundary_resolution;
