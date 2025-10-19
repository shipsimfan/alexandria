use lct_diagnostics::Span;

mod display;
mod parse;

/// An attribute affecting a node in the AST
#[derive(Debug)]
pub(in crate::compile) struct Attribute<'a> {
    /// The name of the attribute
    name: &'a str,

    /// The location of this [`Attribute`]
    span: Span,
}
