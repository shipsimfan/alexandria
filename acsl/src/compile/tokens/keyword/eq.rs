use crate::compile::tokens::Keyword;

impl PartialEq for Keyword {
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
    }
}
