use crate::compile::tokens::Punctuation;

impl PartialEq for Punctuation {
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
    }
}
