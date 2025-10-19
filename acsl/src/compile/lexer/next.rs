use crate::compile::{Lexer, Token};
use lct_diagnostics::{Diag, DiagCtxt};

impl<'a> Lexer<'a> {
    /// Parses the next [`Token`] from the stream
    pub fn next<'b, 'c>(
        &mut self,
        diag: &'c DiagCtxt<'b>,
    ) -> Result<Option<Token<'a>>, Diag<'b, 'c>> {
        if self.stream.is_empty() {
            return Ok(None);
        }

        // Skip whitespace
        while let Some(c) = self
            .stream
            .peek()
            .map_err(|error| diag.err_span(error.to_string(), self.stream.span()))?
        {
            if !c.is_ascii_whitespace() {
                break;
            }

            self.stream.next().unwrap();
        }

        Token::parse(&mut self.stream, diag).map(Some)
    }
}
