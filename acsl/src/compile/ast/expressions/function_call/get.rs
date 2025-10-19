use crate::compile::ast::expressions::FunctionCall;
use lct_diagnostics::Span;

impl<'a> FunctionCall<'a> {
    /// Gets the location of this [`FunctionCall`]
    pub fn span(&self) -> Span {
        self.span
    }
}
