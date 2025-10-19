use crate::compile::tokens::{Keyword, KeywordKind};
use lct_diagnostics::Span;

impl Keyword {
    /// Tries to parse `content` into a [`Keyword`]
    pub(in crate::compile::tokens) fn parse(content: &str, span: Span) -> Option<Self> {
        KeywordKind::parse(content).map(|kind| Keyword { kind, span })
    }
}
