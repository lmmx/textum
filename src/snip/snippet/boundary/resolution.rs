use crate::snip::{Boundary, boundary::BoundaryError, BoundaryMode, Extent};
use ropey::Rope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundaryResolution {
    pub start: usize,
    pub end: usize,
}

impl Boundary {
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
