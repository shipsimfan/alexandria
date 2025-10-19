use crate::compile::Lexer;
use lct_diagnostics::Span;

impl<'a> Lexer<'a> {
    /// Get the span of the next character in the stream
    pub fn span(&self) -> Span {
        self.stream.span()
    }
}
