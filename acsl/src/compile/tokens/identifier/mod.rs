use lct_diagnostics::Span;

mod display;
mod eq;
mod get;

/// A name of a code element
#[derive(Debug, Clone)]
pub(in crate::compile) struct Identifier<'a> {
    /// The content of the identifier itself
    content: &'a str,

    /// The span where the identifier is located
    span: Span,
}
