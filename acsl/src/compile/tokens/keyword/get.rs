use crate::compile::tokens::{Keyword, KeywordKind};
use lct_diagnostics::Span;

impl Keyword {
    /// Gets the kind of [`Keyword`] this is
    pub fn kind(&self) -> KeywordKind {
        self.kind
    }

    /// Gets the location of this [`Keyword`]
    pub fn span(&self) -> Span {
        self.span
    }
}
