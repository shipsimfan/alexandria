use crate::compile::Token;

impl<'a> std::fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Identifier(identifier) => identifier.fmt(f),
            Token::Keyword(keyword) => keyword.fmt(f),
            Token::Punctuation(punctuation) => punctuation.fmt(f),
            Token::Integer(integer) => integer.fmt(f),
            Token::FloatingPoint(floating_point) => floating_point.fmt(f),
        }
    }
}
