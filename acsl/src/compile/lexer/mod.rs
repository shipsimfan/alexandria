use lct_diagnostics::SliceByteCharStream;

mod expect;
mod new;
mod next;
mod span;

/// Parses tokens from a stream
pub(in crate::compile) struct Lexer<'a> {
    /// The source of bytes
    stream: SliceByteCharStream<'a>,
}
