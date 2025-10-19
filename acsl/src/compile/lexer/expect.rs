use crate::compile::{tokens::Identifier, Lexer, Token};
use lct_diagnostics::{Diag, DiagCtxt};

impl<'a> Lexer<'a> {
    /// Expect a specific `target` token next in the stream
    pub fn expect<'b, 'c, 'd, T: Into<Token<'d>>>(
        &mut self,
        target: T,
        diag: &'c DiagCtxt<'b>,
    ) -> Result<Token<'a>, Diag<'b, 'c>> {
        let token = match self.next(diag)? {
            Some(token) => token,
            None => return Err(diag.err_span("unexpected end of file", self.stream.span())),
        };

        let target = target.into();
        if token != target {
            return Err(diag.err_span(format!("expected {}", target), token.span()));
        }

        Ok(token)
    }

    /// Expect an [`Identifier`] as the next token or produce an error
    pub fn expect_identifier<'b, 'c>(
        &mut self,
        diag: &'c DiagCtxt<'b>,
    ) -> Result<Identifier<'a>, Diag<'b, 'c>> {
        match self.next(diag)? {
            Some(Token::Identifier(identifier)) => Ok(identifier),
            Some(token) => Err(diag.err_span("expected identifier", token.span())),
            None => Err(diag.err_span("unexpected end of file", self.stream.span())),
        }
    }
}
