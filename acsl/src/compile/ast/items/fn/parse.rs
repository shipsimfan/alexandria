use crate::compile::{
    ast::{items::Fn, Attribute, Statement},
    tokens::{KeywordKind, PunctuationKind},
    Lexer, Token,
};
use lct_diagnostics::{Diag, DiagCtxt, Span};

impl<'a> Fn<'a> {
    /// Parse a [`Fn`] from `source` with attributes
    pub fn parse_attributes<'b, 'c>(
        start: usize,
        lexer: &mut Lexer<'a>,
        diag: &'c DiagCtxt<'b>,
    ) -> Result<Self, Diag<'b, 'c>> {
        let mut attributes = Vec::new();
        let mut first = true;
        loop {
            let attr_start = if first {
                first = false;
                start
            } else {
                match lexer.next(diag)? {
                    Some(Token::Keyword(keyword)) => match keyword.kind() {
                        KeywordKind::Fn => break,
                        _ => return Err(diag.err_span("expected \"fn\"", keyword.span())),
                    },
                    Some(Token::Punctuation(punctuation)) => match punctuation.kind() {
                        PunctuationKind::Hash => punctuation.span().start(),
                        _ => return Err(diag.err_span("expected \"fn\"", punctuation.span())),
                    },
                    Some(token) => return Err(diag.err_span("expected \"fn\"", token.span())),
                    None => return Err(diag.err_span("expected \"fn\"", lexer.span())),
                }
            };

            attributes.push(Attribute::parse(attr_start, lexer, diag)?);
        }

        Fn::parse(start, attributes, lexer, diag)
    }

    /// Parse a [`Fn`] from `source`
    pub fn parse<'b, 'c>(
        start: usize,
        attributes: Vec<Attribute<'a>>,
        lexer: &mut Lexer<'a>,
        diag: &'c DiagCtxt<'b>,
    ) -> Result<Self, Diag<'b, 'c>> {
        let name = lexer.expect_identifier(diag)?.content();

        // Parse parameters
        lexer.expect(PunctuationKind::OpenParentheses, diag)?;
        let parameters = Vec::new();
        loop {
            match lexer.next(diag)? {
                Some(Token::Identifier(parameter_name)) => {
                    todo!("parse parameter name: {}", parameter_name)
                }
                Some(Token::Punctuation(punctuation)) => match punctuation.kind() {
                    PunctuationKind::CloseParentheses => break,
                    _ => return Err(diag.err_span("expected parameter or ')'", punctuation.span())),
                },
                Some(token) => return Err(diag.err_span("expected parameter or ')'", token.span())),
                None => return Err(diag.err_span("expected parameter or ')'", lexer.span())),
            }
        }

        // Parse return type
        let return_type = match lexer.next(diag)? {
            Some(Token::Punctuation(punctuation)) => match punctuation.kind() {
                PunctuationKind::OpenBrace => None,
                PunctuationKind::FatArrow => Some(lexer.expect_identifier(diag)?.content()),
                _ => return Err(diag.err_span("expected '->' or '{'", punctuation.span())),
            },
            Some(token) => return Err(diag.err_span("expected '->' or '{'", token.span())),
            None => return Err(diag.err_span("expected '->' or '{'", lexer.span())),
        };

        // Parse body
        if return_type.is_some() {
            lexer.expect(PunctuationKind::OpenBrace, diag)?;
        }

        let mut body = Vec::new();
        let (end, return_statement) = loop {
            let statement = match lexer.next(diag)? {
                Some(Token::Punctuation(punctuation)) => match punctuation.kind() {
                    PunctuationKind::CloseBrace => break (punctuation.span().end(), None),
                    _ => Statement::parse(Token::Punctuation(punctuation), lexer, diag)?,
                },
                Some(token) => Statement::parse(token, lexer, diag)?,
                None => return Err(diag.err_span("unexpected end of file", lexer.span())),
            };

            match lexer.next(diag)? {
                Some(Token::Punctuation(punctuation)) => match punctuation.kind() {
                    PunctuationKind::CloseBrace => {
                        break (punctuation.span().end(), Some(statement))
                    }
                    PunctuationKind::Semicolon => body.push(statement),
                    _ => return Err(diag.err_span("expected ';' or '}'", punctuation.span())),
                },
                Some(token) => return Err(diag.err_span("expected ';' or '}'", token.span())),
                None => return Err(diag.err_span("expected ';' or '}'", lexer.span())),
            }
        };

        Ok(Fn {
            attributes,
            name,
            parameters,
            return_type,
            body,
            return_statement,
            span: Span::new(start, end),
        })
    }
}
