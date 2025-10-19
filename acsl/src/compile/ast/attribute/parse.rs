use crate::compile::{ast::Attribute, tokens::PunctuationKind, Lexer};
use lct_diagnostics::{Diag, DiagCtxt, Span};

impl<'a> Attribute<'a> {
    /// Parse an [`Attribute`] from `source`
    pub fn parse<'b, 'c>(
        start: usize,
        lexer: &mut Lexer<'a>,
        diag: &'c DiagCtxt<'b>,
    ) -> Result<Self, Diag<'b, 'c>> {
        lexer.expect(PunctuationKind::OpenBracket, diag)?;
        let name = lexer.expect_identifier(diag)?.content();
        let end = lexer
            .expect(PunctuationKind::CloseBracket, diag)?
            .span()
            .end();

        Ok(Attribute {
            name: name,
            span: Span::new(start, end),
        })
    }
}
