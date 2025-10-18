use crate::compile::Token;
use lct_diagnostics::Span;

impl<'a> Token<'a> {
    pub fn span(&self) -> Span {
        match self {
            Token::Identifier(identifier) => identifier.span(),
            Token::Keyword(keyword) => keyword.span(),
            Token::Punctuation(punctuation) => punctuation.span(),
            Token::Integer(integer) => integer.span(),
            Token::FloatingPoint(floating_point) => floating_point.span(),
        }
    }
}
