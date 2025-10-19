use crate::compile::{ast::Expression, Lexer, Token};
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

        todo!("check for struct creation or function call: {}", identifier)
    }
}
