use crate::compile::tokens::Punctuation;

impl std::fmt::Display for Punctuation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "punctuation '{}'", self.kind)
    }
}
