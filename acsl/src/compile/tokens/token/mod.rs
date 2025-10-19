use crate::compile::tokens::{FloatingPoint, Identifier, Integer, Keyword, Punctuation};

mod display;
mod from;
mod parse;
mod span;

/// A token that can appear in an ACSL stream
#[derive(Debug, Clone, PartialEq)]
pub(in crate::compile) enum Token<'a> {
    /// A name of a code element
    Identifier(Identifier<'a>),

    /// An identifier with a special meaning
    Keyword(Keyword),

    /// A punctuating token
    Punctuation(Punctuation),

    /// An integer
    Integer(Integer),

    /// A floating point number
    FloatingPoint(FloatingPoint),
}
