use crate::compile::tokens::Integer;
use lct_diagnostics::{Diag, DiagCtxt, SliceByteCharStream, Span};

impl Integer {
    /// Parse and [`Integer`] from `stream` using `base`
    pub(in crate::compile::tokens) fn parse<'a, 'b>(
        start: usize,
        base: u32,
        stream: &mut SliceByteCharStream,
        diag: &'b DiagCtxt<'a>,
    ) -> Result<Self, Diag<'a, 'b>> {
        let mut value = 0u64;
        let mut digit = false;
        while let Some(c) = stream
            .peek()
            .map_err(|error| diag.err_span(error.to_string(), stream.span()))?
        {
            let d = match c.to_digit(base) {
                Some(d) => d as u64,
                None => break,
            };

            stream.next().unwrap();
            digit = true;

            value = match value.checked_mul(base as _) {
                Some(value) => value,
                None => {
                    return Err(
                        diag.err_span("integer out of range", Span::new(start, stream.offset()))
                    );
                }
            };

            value = match value.checked_add(d) {
                Some(value) => value,
                None => {
                    return Err(
                        diag.err_span("integer out of range", Span::new(start, stream.offset()))
                    );
                }
            };
        }

        if !digit {
            return Err(diag.err_span("no digits in number", stream.span()));
        }

        Ok(Integer {
            value,
            span: Span::new(start, stream.offset()),
        })
    }
}
