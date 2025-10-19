use crate::compile::tokens::{Keyword, KeywordKind};
use lct_diagnostics::Span;

impl From<KeywordKind> for Keyword {
    fn from(kind: KeywordKind) -> Self {
        Keyword {
            kind,
            span: Span::default(),
        }
    }
}
