use lct_diagnostics::Span;

mod kind;

mod display;
mod eq;
mod from;
mod get;
mod parse;

pub(in crate::compile) use kind::PunctuationKind;

/// A punctuating token
#[derive(Debug, Clone)]
pub(in crate::compile) struct Punctuation {
    /// The kind of punctuation this is
    kind: PunctuationKind,

    /// The span where this punctuation is located
    span: Span,
}
