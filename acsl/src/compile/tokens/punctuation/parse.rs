use crate::compile::tokens::{Punctuation, PunctuationKind};
use lct_diagnostics::{Diag, DiagCtxt, SliceByteCharStream, Span};

impl Punctuation {
    /// Parse a [`Punctuation`] from `c1` and `stream`
    pub(in crate::compile::tokens) fn parse<'a, 'b>(
        c1: char,
        span: Span,
        stream: &mut SliceByteCharStream,
        diag: &'b DiagCtxt<'a>,
    ) -> Result<Self, Diag<'a, 'b>> {
        match c1 {
            ':' => match stream.peek() {
                Ok(Some(':')) => {
                    let end_span = stream.span();
                    stream.next().unwrap();
                    Ok(Punctuation {
                        kind: PunctuationKind::DoubleColon,
                        span: Span::new(span.start(), end_span.end()),
                    })
                }
                _ => Ok(Punctuation {
                    kind: PunctuationKind::Colon,
                    span,
                }),
            },
            ',' => Ok(Punctuation {
                kind: PunctuationKind::Comma,
                span,
            }),
            '-' => match stream.peek() {
                Ok(Some('>')) => {
                    let end_span = stream.span();
                    stream.next().unwrap();
                    Ok(Punctuation {
                        kind: PunctuationKind::FatArrow,
                        span: Span::new(span.start(), end_span.end()),
                    })
                }
                _ => Ok(Punctuation {
                    kind: PunctuationKind::Dash,
                    span,
                }),
            },
            '.' => Ok(Punctuation {
                kind: PunctuationKind::Dot,
                span,
            }),
            '#' => Ok(Punctuation {
                kind: PunctuationKind::Hash,
                span,
            }),
            ';' => Ok(Punctuation {
                kind: PunctuationKind::Semicolon,
                span,
            }),
            '{' => Ok(Punctuation {
                kind: PunctuationKind::OpenBrace,
                span,
            }),
            '}' => Ok(Punctuation {
                kind: PunctuationKind::CloseBrace,
                span,
            }),
            '[' => Ok(Punctuation {
                kind: PunctuationKind::OpenBracket,
                span,
            }),
            ']' => Ok(Punctuation {
                kind: PunctuationKind::CloseBracket,
                span,
            }),
            '(' => Ok(Punctuation {
                kind: PunctuationKind::OpenParentheses,
                span,
            }),
            ')' => Ok(Punctuation {
                kind: PunctuationKind::CloseParentheses,
                span,
            }),
            _ => Err(diag.err_span(
                format!("unexpected character '{}'", c1.escape_debug()),
                span,
            )),
        }
    }
}
