use std::hash::Hash;

use crate::snip::target::Target;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Extent {
    Lines(usize),
    Chars(usize),
    Bytes(usize),
    Matching(usize, Target),
}
