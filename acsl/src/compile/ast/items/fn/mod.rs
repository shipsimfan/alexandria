#![allow(unused)]

use crate::compile::ast::{Attribute, Statement};
use lct_diagnostics::Span;

mod parameter;

mod parse;
mod pretty_print;

pub(in crate::compile) use parameter::FnParameter;

/// A function definition
#[derive(Debug)]
pub(in crate::compile) struct Fn<'a> {
    /// The attributes affecting this function
    attributes: Vec<Attribute<'a>>,

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

    /// The location of this [`Fn`]
    span: Span,
}
