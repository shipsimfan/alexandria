use lct_diagnostics::Span;

mod kind;

mod display;
mod eq;
mod from;
mod get;
mod parse;

pub(in crate::compile) use kind::KeywordKind;

/// An identifier with a special meaning
#[derive(Debug, Clone)]
pub(in crate::compile) struct Keyword {
    /// The type of keyword this is
    kind: KeywordKind,

    /// The span where the keyword is located
    span: Span,
}
