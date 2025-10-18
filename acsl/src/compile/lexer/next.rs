use crate::compile::{Lexer, Token};
use lct_diagnostics::{Diag, DiagCtxt};

impl<'a> Lexer<'a> {
    /// Parses the next [`Token`] from the stream
    pub fn next<'b, 'c>(
        &mut self,
        diag: &'c DiagCtxt<'b>,
    ) -> Result<Option<Token<'a>>, Diag<'b, 'c>> {
        todo!()
    }
}
