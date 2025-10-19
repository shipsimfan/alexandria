use crate::compile::tokens::Identifier;
use lct_diagnostics::Span;

impl<'a> From<&'a str> for Identifier<'a> {
    fn from(content: &'a str) -> Self {
        Identifier {
            content,
            span: Span::default(),
        }
    }
}
