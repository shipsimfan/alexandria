use crate::compile::{
    ast::{
        expressions::{FunctionCall, StructCreation},
        path::Path,
        Expression,
    },
    tokens::PunctuationKind,
    Lexer, Token,
};
use lct_diagnostics::{Diag, DiagCtxt};

impl<'a> Expression<'a> {
    /// Parse an [`Expression`] from `source`
    pub fn parse<'b, 'c>(
        token: Token<'a>,
        lexer: &mut Lexer<'a>,
        diag: &'c DiagCtxt<'b>,
    ) -> Result<Self, Diag<'b, 'c>> {
        let identifier = match token {
            Token::FloatingPoint(floating_point) => {
                return Ok(Expression::Literal(floating_point.into()))
            }
            Token::Integer(integer) => return Ok(Expression::Literal(integer.into())),
            Token::Identifier(identifier) => identifier,
            _ => return Err(diag.err_span(format!("unexpected {}", token), token.span())),
        };

        match lexer.next(diag)? {
            Some(Token::Punctuation(punctuation)) => match punctuation.kind() {
                PunctuationKind::DoubleColon => {
                    FunctionCall::parse_path(identifier, lexer, diag).map(Expression::FunctionCall)
                }
                PunctuationKind::OpenParentheses => {
                    FunctionCall::parse(Path::new_one(identifier), lexer, diag)
                        .map(Expression::FunctionCall)
                }
                PunctuationKind::OpenBrace => {
                    StructCreation::parse(identifier, lexer, diag).map(Expression::StructCreation)
                }
                _ => Err(diag.err_span(format!("unexpected {}", punctuation), punctuation.span())),
            },
            Some(token) => Err(diag.err_span(format!("unexpected {}", token), token.span())),
            None => Err(diag.err_span(format!("unexpected end of file"), lexer.span())),
        }
    }
}
