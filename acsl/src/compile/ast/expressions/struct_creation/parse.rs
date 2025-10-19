use crate::compile::{
    ast::expressions::{StructCreation, StructCreationField},
    tokens::{Identifier, PunctuationKind},
    Lexer, Token,
};
use lct_diagnostics::{Diag, DiagCtxt, Span};

impl<'a> StructCreation<'a> {
    /// Parse n [`StructCreation`] from `source`
    pub fn parse<'b, 'c>(
        name: Identifier<'a>,
        lexer: &mut Lexer<'a>,
        diag: &'c DiagCtxt<'b>,
    ) -> Result<Self, Diag<'b, 'c>> {
        let mut fields = Vec::new();
        let end = loop {
            let field_name = match lexer.next(diag)? {
                Some(Token::Identifier(identifier)) => identifier,
                Some(Token::Punctuation(punctuation)) => match punctuation.kind() {
                    PunctuationKind::CloseBrace => break punctuation.span().end(),
                    _ => {
                        return Err(
                            diag.err_span("expected a field name or a '}'", punctuation.span())
                        )
                    }
                },
                Some(token) => {
                    return Err(diag.err_span("expected a field name or a '}'", token.span()))
                }
                None => return Err(diag.err_span("expected a field name or a '}'", lexer.span())),
            };

            fields.push(StructCreationField::parse(field_name, lexer, diag)?);

            match lexer.next(diag)? {
                Some(Token::Punctuation(punctuation)) => match punctuation.kind() {
                    PunctuationKind::Comma => {}
                    PunctuationKind::CloseBrace => break punctuation.span().end(),
                    _ => {
                        return Err(
                            diag.err_span("expected a field name or a '}'", punctuation.span())
                        )
                    }
                },
                Some(token) => return Err(diag.err_span("expected a ',' or a '}'", token.span())),
                None => return Err(diag.err_span("expected a ',' or a '}'", lexer.span())),
            }
        };

        Ok(StructCreation {
            name: name.content(),
            fields,
            span: Span::new(name.span().start(), end),
        })
    }
}
