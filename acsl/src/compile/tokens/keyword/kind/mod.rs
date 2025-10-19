mod display;
mod parse;

/// The kind of keyword a token is
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::compile) enum KeywordKind {
    /// The keyword introdcues a function
    Fn,

    /// The keyword introduces a structure
    Struct,
}
