use crate::compile::{
    tokens::{FloatingPoint, Integer},
    Token,
};
use lct_diagnostics::{Diag, DiagCtxt, SliceByteCharStream, Span};

impl FloatingPoint {
    /// Parse a [`FloatingPoint`] number or an [`Integer`] from `c1` and `stream`
    pub(in crate::compile::tokens) fn parse<'a, 'b, 'c>(
        c1: char,
        start: usize,
        stream: &mut SliceByteCharStream<'a>,
        diag: &'c DiagCtxt<'b>,
    ) -> Result<Token<'a>, Diag<'b, 'c>> {
        // Check for an integer prefix
        if c1 == '0' {
            let base = match stream.peek() {
                Ok(Some('x')) => Some(16),
                Ok(Some('o')) => Some(8),
                Ok(Some('b')) => Some(2),
                _ => None,
            };

            if let Some(base) = base {
                stream.next().unwrap();
                return Integer::parse(start, base, stream, diag).map(Token::Integer);
            }
        }

        // Start tracking the ending offset
        let mut end = start + 1;

        // Prep the parts of the number
        let mut f = (c1 as u8 - b'0') as u64;
        let mut e = 0;

        // Parse the integer portion
        let mut decimal = false;
        while let Some(c) = stream
            .next()
            .map_err(|error| diag.err_span(error.to_string(), stream.span()))?
        {
            match c {
                '.' => {
                    decimal = true;
                    end = stream.offset();
                    break;
                }
                c if c.is_ascii_digit() => {
                    end = stream.offset();

                    f = match f.checked_mul(10) {
                        Some(f) => match f.checked_add((c as u8 - b'0') as u64) {
                            Some(f) => f,
                            None => f,
                        },
                        None => {
                            e += 1;
                            f
                        }
                    };
                }
                _ => break,
            }
        }

        // Parse integer if no decimal
        if !decimal {
            let span = Span::new(start, end);

            if e > 0 {
                return Err(diag.err_span("integer outside of range", span));
            }

            return Ok(Token::Integer(Integer::new(f, span)));
        }

        // Parse the frational portion
        let mut potential_f = f;
        let mut potential_e = e;
        while let Some(c) = stream
            .next()
            .map_err(|error| diag.err_span(error.to_string(), stream.span()))?
        {
            if !c.is_ascii_digit() {
                break;
            }

            end = stream.offset();

            if c == '0' {
                potential_f = match potential_f.checked_mul(10) {
                    Some(f) => {
                        potential_e -= 1;
                        f
                    }
                    None => potential_f,
                };

                continue;
            }

            if let Some(pot_f) = potential_f.checked_mul(10) {
                potential_e -= 1;
                potential_f = match pot_f.checked_add((c as u8 - b'0') as u64) {
                    Some(f) => f,
                    None => pot_f,
                };

                e = potential_e;
                f = potential_f;
            }
        }

        // Convert fractional and exponent into float
        let mut value = if f == 0 {
            0.0
        } else {
            (f as f64) * 10f64.powi(e)
        };

        Ok(Token::FloatingPoint(FloatingPoint {
            value,
            span: Span::new(start, end),
        }))
    }
}
