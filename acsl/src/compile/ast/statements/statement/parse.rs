use crate::compile::{
    ast::{Expression, Statement},
    Lexer, Token,
};
use lct_diagnostics::{Diag, DiagCtxt};

impl<'a> Statement<'a> {
    /// Parse an [`Statement`] from `source`
    pub fn parse<'b, 'c>(
        token: Token<'a>,
        lexer: &mut Lexer<'a>,
        diag: &'c DiagCtxt<'b>,
    ) -> Result<Self, Diag<'b, 'c>> {
        Expression::parse(token, lexer, diag).map(Statement::Expression)
    }
}
