impl Snippet {
    pub fn replace(&self, rope: &Rope, replacement: &str) -> Result<Rope, SnippetError> {
        validate_replacement_utf8(replacement)?;
        let resolution = self.resolve(rope)?;
        apply_replacement(rope, resolution.start, resolution.end, replacement)
    }
}

fn validate_replacement_utf8(s: &str) -> Result<(), SnippetError> {
    // str is already UTF-8 valid by Rust's type system, but check for any
    // additional validation requirements (e.g., no null bytes, specific encoding)
    if s.contains('\0') {
        return Err(SnippetError::InvalidUtf8("null bytes not allowed".into()));
    }
    Ok(())
}

fn apply_replacement(
    rope: &Rope,
    start: usize,
    end: usize,
    replacement: &str,
) -> Result<Rope, SnippetError> {
    let mut new_rope = rope.clone();
    new_rope.remove(start..end);
    new_rope.insert(start, replacement);
    Ok(new_rope)
}
