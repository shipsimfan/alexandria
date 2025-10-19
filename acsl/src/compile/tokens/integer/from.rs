use crate::compile::tokens::Integer;
use lct_diagnostics::Span;

impl From<u64> for Integer {
    fn from(value: u64) -> Self {
        Integer {
            value,
            span: Span::default(),
        }
    }
}
