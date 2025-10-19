use crate::compile::ast::expressions::Literal;
use lct_diagnostics::Span;

impl Literal {
    /// Gets the location of this [`Literal`]
    pub fn span(&self) -> Span {
        self.span
    }
}
