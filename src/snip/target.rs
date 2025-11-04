//! Target specifications for boundary matching.
use std::hash::{Hash, Hasher};

pub mod error;
pub mod matching;

#[cfg(feature = "regex")]
use error::TargetError;

#[derive(Debug, Clone)]
/// Defines what text position or pattern a boundary matches.
pub enum Target {
    /// An exact string to match.
    Literal(String),
    #[cfg(feature = "regex")]
    /// Matches a regular expression pattern.
    Pattern {
        /// The original pattern string.
        pattern: String,
        /// The compiled regex.
        regex: regex_cursor::engines::meta::Regex,
    },
    /// Matches an absolute line number.
    Line(usize),
    /// Matches an absolute character index.
    Char(usize),
    /// Matches a line and column coordinate.
    Position {
        /// One-indexed line number.
        line: usize,
        /// One-indexed column number.
        col: usize,
    },
}

impl Target {
    /// Creates a new Pattern target from a regex pattern string.
    ///
    /// # Errors
    ///
    /// Returns [`TargetError::InvalidPattern`] if the pattern cannot be compiled into a valid regex.
    #[cfg(feature = "regex")]
    pub fn pattern(pattern: impl Into<String>) -> Result<Self, TargetError> {
        let pattern = pattern.into();
        let regex = regex_cursor::engines::meta::Regex::new(&pattern)
            .map_err(|e| TargetError::InvalidPattern(e.to_string()))?;
        Ok(Target::Pattern { pattern, regex })
    }
}

impl Eq for Target {}

impl PartialEq for Target {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Target::Literal(a), Target::Literal(b)) => a == b,
            #[cfg(feature = "regex")]
            (Target::Pattern { pattern: a, .. }, Target::Pattern { pattern: b, .. }) => a == b,
            (Target::Line(a), Target::Line(b)) => a == b,
            (Target::Char(a), Target::Char(b)) => a == b,
            (Target::Position { line: l1, col: c1 }, Target::Position { line: l2, col: c2 }) => {
                l1 == l2 && c1 == c2
            }
            _ => false,
        }
    }
}

impl Hash for Target {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Target::Literal(s) => {
                0u8.hash(state);
                s.hash(state);
            }
            #[cfg(feature = "regex")]
            Target::Pattern { pattern, .. } => {
                1u8.hash(state);
                pattern.hash(state);
            }
            Target::Line(n) => {
                2u8.hash(state);
                n.hash(state);
            }
            Target::Char(n) => {
                3u8.hash(state);
                n.hash(state);
            }
            Target::Position { line, col } => {
                4u8.hash(state);
                line.hash(state);
                col.hash(state);
            }
        }
    }
}
