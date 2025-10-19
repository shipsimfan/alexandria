use lct_diagnostics::Span;

mod display;
mod eq;
mod get;
mod parse;

/// An floating point number token in a stream
#[derive(Debug, Clone)]
pub(in crate::compile) struct FloatingPoint {
    /// The value of the floating pointer number
    value: f64,

    /// The span where the floating point number is located
    span: Span,
}
