use crate::compile::{
    ast::{items::StructField, Attribute},
    tokens::PunctuationKind,
    Lexer, Token,
};
use lct_diagnostics::{Diag, DiagCtxt, Span};

impl<'a> StructField<'a> {
    /// Parse a [`StructField`] from `source` with attributes
    pub fn parse_attributes<'b, 'c>(
        start: usize,
        lexer: &mut Lexer<'a>,
        diag: &'c DiagCtxt<'b>,
    ) -> Result<Self, Diag<'b, 'c>> {
        let mut attributes = Vec::new();
        let mut first = true;
        let name = loop {
            let attr_start = if first {
                first = false;
                start
            } else {
                match lexer.next(diag)? {
                    Some(Token::Identifier(identifier)) => break identifier.content(),
                    Some(Token::Punctuation(punctuation)) => match punctuation.kind() {
                        PunctuationKind::Hash => punctuation.span().start(),
                        _ => return Err(diag.err_span("expected field name", punctuation.span())),
                    },
                    Some(token) => return Err(diag.err_span("expected field name", token.span())),
                    None => return Err(diag.err_span("expected field name", lexer.span())),
                }
            };

            attributes.push(Attribute::parse(attr_start, lexer, diag)?);
        };

        StructField::parse(name, start, attributes, lexer, diag)
    }

    /// Parse a [`StructField`] from `source`
    pub fn parse<'b, 'c>(
        name: &'a str,
        start: usize,
        attributes: Vec<Attribute<'a>>,
        lexer: &mut Lexer<'a>,
        diag: &'c DiagCtxt<'b>,
    ) -> Result<Self, Diag<'b, 'c>> {
        lexer.expect(PunctuationKind::Colon, diag)?;
        let r#type = lexer.expect_identifier(diag)?;

        let end = r#type.span().end();
        Ok(StructField {
            attributes,
            name,
            r#type,
            span: Span::new(start, end),
        })
    }
}
