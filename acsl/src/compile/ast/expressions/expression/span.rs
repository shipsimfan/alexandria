use crate::compile::ast::Expression;
use lct_diagnostics::Span;

impl<'a> Expression<'a> {
    /// Gets the location of this [`Expression`]
    pub fn span(&self) -> Span {
        match self {
            Expression::FunctionCall(function_call) => function_call.span(),
            Expression::Literal(literal) => literal.span(),
            Expression::StructCreation(struct_creation) => struct_creation.span(),
        }
    }
}
