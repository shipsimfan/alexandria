mod floating_point;
mod identifier;
mod integer;
mod keyword;
mod punctuation;
mod token;

pub(in crate::compile) use floating_point::FloatingPoint;
pub(in crate::compile) use identifier::Identifier;
pub(in crate::compile) use integer::Integer;
pub(in crate::compile) use keyword::{Keyword, KeywordKind};
pub(in crate::compile) use punctuation::{Punctuation, PunctuationKind};
pub(in crate::compile) use token::Token;
