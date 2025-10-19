use crate::compile::ast::Expression;

mod display;
mod parse;

/// A statement which makes up part of the body of a function
#[derive(Debug)]
pub(in crate::compile) enum Statement<'a> {
    /// The statement consists of only an expression
    Expression(Expression<'a>),
}
