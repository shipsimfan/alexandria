use crate::compile::ast::Statement;

mod parameter;

mod display;

pub(in crate::compile) use parameter::FnParameter;

/// A function definition
#[derive(Debug)]
pub(in crate::compile) struct Fn<'a> {
    /// The name of the function
    name: &'a str,

    /// The parameters passed to this function
    parameters: Vec<FnParameter<'a>>,

    /// The name of the return type
    return_type: Option<&'a str>,

    /// The statements making up the body of this function
    body: Vec<Statement<'a>>,

    /// The statement which contains a return value
    return_statement: Option<Statement<'a>>,
}
