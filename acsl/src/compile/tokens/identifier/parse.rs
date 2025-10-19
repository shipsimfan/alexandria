use crate::compile::{
    tokens::{Identifier, Keyword},
    Token,
};
use lct_diagnostics::{Diag, DiagCtxt, SliceByteCharStream, Span};

impl<'a> Identifier<'a> {
    /// Parses an [`Identifier`] or [`Keyword`] from `stream`
    pub(in crate::compile::tokens) fn parse<'b, 'c>(
        start: usize,
        stream: &mut SliceByteCharStream<'a>,
        diag: &'c DiagCtxt<'b>,
    ) -> Result<Token<'a>, Diag<'b, 'c>> {
        // Parse remanining characters
        while let Some(c) = stream
            .peek()
            .map_err(|error| diag.err_span(error.to_string(), stream.span()))?
        {
            if !c.is_ascii_alphanumeric() && c != '_' {
                break;
            }

            stream.next().unwrap();
        }

        let end = stream.offset();
        let span = Span::new(start, end);
        let content = unsafe { std::str::from_utf8_unchecked(&stream.bytes()[start..end]) };

        // Check if it is a keyword
        if let Some(keyword) = Keyword::parse(content, span) {
            return Ok(Token::Keyword(keyword));
        }

        Ok(Token::Identifier(Identifier { content, span }))
    }
}
