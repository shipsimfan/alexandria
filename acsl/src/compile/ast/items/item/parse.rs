use crate::compile::{
    ast::{
        items::{Fn, Struct},
        Item,
    },
    tokens::{KeywordKind, PunctuationKind},
    Lexer, Token,
};
use lct_diagnostics::{Diag, DiagCtxt};

impl<'a> Item<'a> {
    /// Parse an [`Item`] from `source`
    pub fn parse<'b, 'c>(
        lexer: &mut Lexer<'a>,
        diag: &'c DiagCtxt<'b>,
    ) -> Result<Option<Self>, Diag<'b, 'c>> {
        match lexer.next(diag)? {
            Some(Token::Keyword(keyword)) => match keyword.kind() {
                KeywordKind::Fn => Fn::parse(keyword.span().start(), Vec::new(), lexer, diag)
                    .map(|r#fn| Some(Item::Fn(r#fn))),
                KeywordKind::Struct => Struct::parse(keyword.span().start(), lexer, diag)
                    .map(|r#struct| Some(Item::Struct(r#struct))),
            },
            Some(Token::Punctuation(punctuation)) => match punctuation.kind() {
                PunctuationKind::Hash => {
                    Fn::parse_attributes(punctuation.span().start(), lexer, diag)
                        .map(|r#fn| Some(Item::Fn(r#fn)))
                }
                _ => Err(diag.err_span(format!("unexpected {}", punctuation), punctuation.span())),
            },
            Some(token) => Err(diag.err_span(format!("unexpected {}", token), token.span())),
            None => Ok(None),
        }
    }
}
