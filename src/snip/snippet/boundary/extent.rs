use std::hash::Hash;

use crate::snip::target::Target;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Measures distance for boundary extension.
///
/// May be given in absolute terms or subject to some target(s).
pub enum Extent {
    /// Extends by a line count.
    Lines(usize),
    /// Extends by a character count.
    Chars(usize),
    /// Extends by a byte count.
    Bytes(usize),
    /// Extends by a particular count of pattern matches.
    Matching(usize, Target),
}
