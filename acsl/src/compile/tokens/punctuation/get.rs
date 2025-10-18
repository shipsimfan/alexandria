use crate::compile::tokens::{Punctuation, PunctuationKind};
use lct_diagnostics::Span;

impl Punctuation {
    /// Gets the kind of [`Punctuation`] this is
    pub fn kind(&self) -> PunctuationKind {
        self.kind
    }

    /// Gets the location of this [`Punctuation`]
    pub fn span(&self) -> Span {
        self.span
    }
}
