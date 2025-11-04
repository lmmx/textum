#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnippetError {
    BoundaryError(BoundaryError),
    InvalidRange { start: usize, end: usize },
    InvalidUtf8(String),
    OutOfBounds { index: usize, rope_len: usize },
}
