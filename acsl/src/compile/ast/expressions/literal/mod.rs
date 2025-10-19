use lct_diagnostics::Span;

mod kind;

mod display;
mod from;
mod get;

pub(in crate::compile) use kind::LiteralKind;

/// A literal value
#[derive(Debug)]
pub(in crate::compile) struct Literal {
    /// The kind of value contained
    kind: LiteralKind,

    /// The location of this [`Literal`]
    span: Span,
}
