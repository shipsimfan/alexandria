use crate::compile::{ast::Path, tokens::Identifier};
use lct_diagnostics::Span;

impl<'a> Path<'a> {
    /// Create a new [`Path`] with one value
    pub fn new_one(first: Identifier<'a>) -> Self {
        Path {
            first: first.content(),
            second: None,
            span: first.span(),
        }
    }

    /// Create a new [`Path`] with one value
    pub fn new_two(first: Identifier<'a>, second: Identifier<'a>) -> Self {
        Path {
            first: first.content(),
            second: Some(second.content()),
            span: Span::new(first.span().start(), second.span().end()),
        }
    }
}
