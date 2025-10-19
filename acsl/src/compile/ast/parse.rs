use crate::compile::{ast::Item, Ast, Lexer};
use lct_diagnostics::{Diag, DiagCtxt, Source};

impl<'a> Ast<'a> {
    /// Parse an [`Ast`] from `source`
    pub fn parse<'b, 'c>(source: &'a Source, diag: &'c DiagCtxt<'b>) -> Result<Self, Diag<'b, 'c>> {
        let mut lexer = Lexer::new(source);

        let mut items = Vec::new();
        while let Some(item) = Item::parse(&mut lexer, diag)? {
            items.push(item);
        }

        Ok(Ast { items })
    }
}
