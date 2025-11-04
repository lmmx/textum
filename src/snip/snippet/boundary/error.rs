use crate::snip::target::error::TargetError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoundaryError {
    TargetError(TargetError),
    ExtentOutOfBounds,
    InvalidExtent,
}

impl From<TargetError> for BoundaryError {
    fn from(err: TargetError) -> Self {
        BoundaryError::TargetError(err)
    }
}
