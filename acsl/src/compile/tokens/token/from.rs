use crate::compile::{
    tokens::{
        FloatingPoint, Identifier, Integer, Keyword, KeywordKind, Punctuation, PunctuationKind,
    },
    Token,
};

impl<'a> From<FloatingPoint> for Token<'a> {
    fn from(value: FloatingPoint) -> Self {
        Token::FloatingPoint(value)
    }
}

impl<'a> From<f64> for Token<'a> {
    fn from(value: f64) -> Self {
        Token::FloatingPoint(value.into())
    }
}

impl<'a> From<Identifier<'a>> for Token<'a> {
    fn from(value: Identifier<'a>) -> Self {
        Token::Identifier(value)
    }
}

impl<'a> From<&'a str> for Token<'a> {
    fn from(value: &'a str) -> Self {
        Token::Identifier(value.into())
    }
}

impl<'a> From<Integer> for Token<'a> {
    fn from(value: Integer) -> Self {
        Token::Integer(value)
    }
}

impl<'a> From<u64> for Token<'a> {
    fn from(value: u64) -> Self {
        Token::Integer(value.into())
    }
}

impl<'a> From<Keyword> for Token<'a> {
    fn from(value: Keyword) -> Self {
        Token::Keyword(value)
    }
}

impl<'a> From<KeywordKind> for Token<'a> {
    fn from(value: KeywordKind) -> Self {
        Token::Keyword(value.into())
    }
}

impl<'a> From<Punctuation> for Token<'a> {
    fn from(value: Punctuation) -> Self {
        Token::Punctuation(value)
    }
}

impl<'a> From<PunctuationKind> for Token<'a> {
    fn from(value: PunctuationKind) -> Self {
        Token::Punctuation(value.into())
    }
}
