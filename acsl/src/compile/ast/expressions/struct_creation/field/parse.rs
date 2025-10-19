use crate::compile::{
    ast::{expressions::StructCreationField, Expression},
    tokens::{Identifier, PunctuationKind},
    Lexer,
};
use lct_diagnostics::{Diag, DiagCtxt, Span};

impl<'a> StructCreationField<'a> {
    /// Parse an [`StructCreationField`] from `source`
    pub fn parse<'b, 'c>(
        name: Identifier<'a>,
        lexer: &mut Lexer<'a>,
        diag: &'c DiagCtxt<'b>,
    ) -> Result<Self, Diag<'b, 'c>> {
        lexer.expect(PunctuationKind::Colon, diag)?;
        let value = Expression::parse(
            lexer
                .next(diag)?
                .ok_or_else(|| diag.err_span("unexpected end of file", lexer.span()))?,
            lexer,
            diag,
        )?;

        let end = value.span().end();
        Ok(StructCreationField {
            name: name.content(),
            value,
            span: Span::new(name.span().start(), end),
        })
    }
}
