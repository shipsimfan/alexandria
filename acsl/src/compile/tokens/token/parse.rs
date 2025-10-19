use crate::compile::{
    tokens::{FloatingPoint, Identifier, Punctuation},
    Token,
};
use lct_diagnostics::{Diag, DiagCtxt, SliceByteCharStream, Span};

impl<'a> Token<'a> {
    /// Parse a [`Token`] from `stream`
    pub fn parse<'b, 'c>(
        stream: &mut SliceByteCharStream<'a>,
        diag: &'c DiagCtxt<'b>,
    ) -> Result<Self, Diag<'b, 'c>> {
        let (c, offset) = stream
            .next_off()
            .map_err(|error| diag.err_span(error.to_string(), stream.span()))?
            .ok_or_else(|| diag.err_span("unexpected end of file", stream.span()))?;

        if c.is_ascii_digit() {
            return FloatingPoint::parse(c, offset, stream, diag);
        }

        if c.is_ascii_alphabetic() || c == '_' {
            return Identifier::parse(offset, stream, diag);
        }

        Punctuation::parse(c, Span::new_one(offset), stream, diag).map(Token::Punctuation)
    }
}
