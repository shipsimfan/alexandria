use crate::compile::tokens::{Punctuation, PunctuationKind};
use lct_diagnostics::Span;

impl From<PunctuationKind> for Punctuation {
    fn from(kind: PunctuationKind) -> Self {
        Punctuation {
            kind,
            span: Span::default(),
        }
    }
}
