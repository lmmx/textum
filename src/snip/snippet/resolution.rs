#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnippetResolution {
    pub start: usize,
    pub end: usize,
}

impl Snippet {
    pub fn resolve(&self, rope: &Rope) -> Result<SnippetResolution, SnippetError> {
        match self {
            Snippet::At(boundary) => {
                let res = boundary.resolve(rope)?;
                validate_range(res.start, res.end, rope)?;
                Ok(SnippetResolution {
                    start: res.start,
                    end: res.end,
                })
            }
            Snippet::From(boundary) => {
                let res = boundary.resolve(rope)?;
                let end = rope.len_chars();
                validate_range(res.end, end, rope)?;
                Ok(SnippetResolution {
                    start: res.end,
                    end,
                })
            }
            Snippet::To(boundary) => {
                let res = boundary.resolve(rope)?;
                validate_range(0, res.start, rope)?;
                Ok(SnippetResolution {
                    start: 0,
                    end: res.start,
                })
            }
            Snippet::Between { start, end } => {
                let start_res = start.resolve(rope)?;
                let end_res = end.resolve(rope)?;
                validate_range(start_res.end, end_res.start, rope)?;
                Ok(SnippetResolution {
                    start: start_res.end,
                    end: end_res.start,
                })
            }
            Snippet::All => Ok(SnippetResolution {
                start: 0,
                end: rope.len_chars(),
            }),
        }
    }
}
