use crate::compile::{
    ast::{
        expressions::{Expression, FunctionCall},
        Path,
    },
    tokens::{Identifier, PunctuationKind},
    Lexer, Token,
};
use lct_diagnostics::{Diag, DiagCtxt, Span};

impl<'a> FunctionCall<'a> {
    /// Parse a [`FunctionCall`] with a prefixed path from `source`
    pub fn parse_path<'b, 'c>(
        first: Identifier<'a>,
        lexer: &mut Lexer<'a>,
        diag: &'c DiagCtxt<'b>,
    ) -> Result<Self, Diag<'b, 'c>> {
        let second = lexer.expect_identifier(diag)?;
        let path = Path::new_two(first, second);

        lexer.expect(PunctuationKind::OpenParentheses, diag)?;

        FunctionCall::parse(path, lexer, diag)
    }

    /// Parse a [`FunctionCall`] from `source`
    pub fn parse<'b, 'c>(
        path: Path<'a>,
        lexer: &mut Lexer<'a>,
        diag: &'c DiagCtxt<'b>,
    ) -> Result<Self, Diag<'b, 'c>> {
        let mut parameters = Vec::new();
        let end = loop {
            match lexer.next(diag)? {
                Some(Token::Punctuation(punctuation)) => match punctuation.kind() {
                    PunctuationKind::CloseParentheses => break punctuation.span().end(),
                    _ => parameters.push(Expression::parse(
                        Token::Punctuation(punctuation),
                        lexer,
                        diag,
                    )?),
                },
                Some(token) => parameters.push(Expression::parse(token, lexer, diag)?),
                None => return Err(diag.err_span("unexpected end of file", lexer.span())),
            }

            match lexer.next(diag)? {
                Some(Token::Punctuation(punctuation)) => match punctuation.kind() {
                    PunctuationKind::Comma => {}
                    PunctuationKind::CloseParentheses => break punctuation.span().end(),
                    _ => return Err(diag.err_span("expected ',' or ')'", punctuation.span())),
                },
                Some(token) => return Err(diag.err_span("expected ',' or ')'", token.span())),
                None => return Err(diag.err_span("expected ',' or ')'", lexer.span())),
            }
        };

        let start = path.span().start();
        Ok(FunctionCall {
            path,
            parameters,
            span: Span::new(start, end),
        })
    }
}
