use lct_diagnostics::SliceByteCharStream;

mod expect;
mod new;
mod next;
mod span;
mod type_id;

/// Parses tokens from a stream
pub(in crate::compile) struct Lexer<'a> {
    /// The source of bytes
    stream: SliceByteCharStream<'a>,

    /// The AST type id to assign to the next found type definition
    next_type_id: u32,
}
