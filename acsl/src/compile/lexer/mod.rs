use lct_diagnostics::SliceByteCharStream;

mod new;
mod next;

/// Parses tokens from a stream
pub(in crate::compile) struct Lexer<'a> {
    /// The source of bytes
    stream: SliceByteCharStream<'a>,
}
