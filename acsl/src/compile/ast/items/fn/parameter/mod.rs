use lct_diagnostics::Span;

mod display;
mod parse;

/// A parameter passed into a function
#[derive(Debug)]
pub(in crate::compile) struct FnParameter<'a> {
    /// The name of the parameter
    name: &'a str,

    /// The type of the parameter
    r#type: &'a str,

    /// The location of this [`FnParameter`]
    span: Span,
}
