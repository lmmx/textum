use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub enum Target {
    Literal(String),
    #[cfg(feature = "regex")]
    Pattern(regex::Regex),
    Line(usize),
    Char(usize),
    Position {
        line: usize,
        col: usize,
    },
}

impl Eq for Target {}

impl PartialEq for Target {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Target::Literal(a), Target::Literal(b)) => a == b,
            #[cfg(feature = "regex")]
            (Target::Pattern(a), Target::Pattern(b)) => a.as_str() == b.as_str(),
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
            Target::Pattern(r) => {
                1u8.hash(state);
                r.as_str().hash(state);
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
