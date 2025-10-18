use crate::compile::tokens::Integer;
use lct_diagnostics::Span;

impl Integer {
    /// Gets the value of this [`Integer`]
    pub fn value(&self) -> u64 {
        self.value
    }

    /// Gets the location of this [`Integer`]
    pub fn span(&self) -> Span {
        self.span
    }
}
