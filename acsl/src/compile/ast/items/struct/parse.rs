use crate::compile::{
    ast::items::{Struct, StructField},
    tokens::PunctuationKind,
    Lexer, Token,
};
use lct_diagnostics::{Diag, DiagCtxt, Span};

impl<'a> Struct<'a> {
    /// Parse a [`Struct`] from `source`
    pub fn parse<'b, 'c>(
        start: usize,
        lexer: &mut Lexer<'a>,
        diag: &'c DiagCtxt<'b>,
    ) -> Result<Self, Diag<'b, 'c>> {
        let name = lexer.expect_identifier(diag)?.content();
        lexer.expect(PunctuationKind::OpenBrace, diag)?;

        let mut fields = Vec::new();
        let end = loop {
            match lexer.next(diag)? {
                Some(Token::Identifier(field_name)) => fields.push(StructField::parse(
                    field_name.content(),
                    field_name.span().start(),
                    Vec::new(),
                    lexer,
                    diag,
                )?),
                Some(Token::Punctuation(punctuation)) => match punctuation.kind() {
                    PunctuationKind::CloseBrace => break punctuation.span().end(),
                    PunctuationKind::Hash => fields.push(StructField::parse_attributes(
                        punctuation.span().start(),
                        lexer,
                        diag,
                    )?),
                    _ => {
                        return Err(
                            diag.err_span("expected a field name or a '}'", punctuation.span())
                        );
                    }
                },
                Some(token) => {
                    return Err(diag.err_span("expected a field name or a '}'", token.span()))
                }
                None => return Err(diag.err_span("expected a field name or a '}'", lexer.span())),
            }

            match lexer.next(diag)? {
                Some(Token::Punctuation(punctuation)) => match punctuation.kind() {
                    PunctuationKind::Comma => {}
                    PunctuationKind::CloseBrace => break punctuation.span().end(),
                    _ => return Err(diag.err_span("expected a ',' or a '}'", punctuation.span())),
                },
                Some(token) => return Err(diag.err_span("expected a ',' or a '}'", token.span())),
                None => return Err(diag.err_span("expected a ',' or a '}'", lexer.span())),
            }
        };

        Ok(Struct {
            name,
            fields,
            span: Span::new(start, end),
        })
    }
}
