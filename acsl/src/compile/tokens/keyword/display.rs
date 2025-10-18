use crate::compile::tokens::Keyword;

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "keyword \"{}\"", self.kind)
    }
}
