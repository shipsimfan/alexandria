use crate::compile::tokens::Identifier;
use lct_diagnostics::Span;

impl<'a> Identifier<'a> {
    /// Gets the content of the [`Identifier`]
    pub fn content(&self) -> &'a str {
        self.content
    }

    /// Gets the location of this [`Identifier`]
    pub fn span(&self) -> Span {
        self.span
    }
}
