use crate::compile::tokens::Integer;
use lct_diagnostics::Span;

impl Integer {
    /// Creates a new [`Integer`]
    pub(in crate::compile::tokens) fn new(value: u64, span: Span) -> Self {
        Integer { value, span }
    }
}
