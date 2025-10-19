use crate::compile::ast::{expressions::Expression, Path};

mod display;

/// A call to a static function
#[derive(Debug)]
pub(in crate::compile) struct FunctionCall<'a> {
    /// The path to the function being called
    path: Path<'a>,

    /// The parameters being passed into the function
    parameters: Vec<Expression<'a>>,
}
