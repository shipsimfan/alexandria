use crate::compile::ast::Path;
use lct_diagnostics::Span;

impl<'a> Path<'a> {
    /// Get the location of this [`Path`]
    pub fn span(&self) -> Span {
        self.span
    }
}
