use crate::compile::ast::expressions::StructCreation;
use lct_diagnostics::Span;

impl<'a> StructCreation<'a> {
    /// Gets the location of this [`StructCreation`]
    pub fn span(&self) -> Span {
        self.span
    }
}
