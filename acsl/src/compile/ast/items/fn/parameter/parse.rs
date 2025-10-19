use crate::compile::{
    ast::items::FnParameter,
    tokens::{Identifier, PunctuationKind},
    Lexer,
};
use lct_diagnostics::{Diag, DiagCtxt, Span};

impl<'a> FnParameter<'a> {
    /// Parse a [`Fn`] from `source` with attributes
    pub fn parse<'b, 'c>(
        name: Identifier<'a>,
        lexer: &mut Lexer<'a>,
        diag: &'c DiagCtxt<'b>,
    ) -> Result<Self, Diag<'b, 'c>> {
        lexer.expect(PunctuationKind::Colon, diag)?;
        let r#type = lexer.expect_identifier(diag)?;

        Ok(FnParameter {
            name: name.content(),
            r#type: r#type.content(),
            span: Span::new(name.span().start(), name.span().end()),
        })
    }
}
