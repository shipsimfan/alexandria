use lct_diagnostics::Span;

mod display;
mod eq;
mod get;
mod new;
mod parse;

/// An integer token in a stream
#[derive(Debug, Clone)]
pub(in crate::compile) struct Integer {
    /// The value of the integer
    value: u64,

    /// The span where the integer is located
    span: Span,
}
