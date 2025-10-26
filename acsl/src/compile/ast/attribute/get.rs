use crate::compile::ast::attribute::Attribute;
use lct_diagnostics::Span;

impl<'a> Attribute<'a> {
    /// Get the name of this attribute
    pub const fn name(&self) -> &'a str {
        self.name
    }

    /// Get the location of this attribute
    pub const fn span(&self) -> Span {
        self.span
    }
}
