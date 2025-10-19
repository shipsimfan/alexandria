use crate::compile::ast::expressions::{FunctionCall, Literal, StructCreation};

mod display;
mod parse;
mod span;

/// An expression which may result in a value
#[derive(Debug)]
pub(in crate::compile) enum Expression<'a> {
    /// The expression is the calling of a function
    FunctionCall(FunctionCall<'a>),

    /// The expression is a literal value
    Literal(Literal),

    /// The expression creates a new structure
    StructCreation(StructCreation<'a>),
}
