use crate::compile::tokens::FloatingPoint;
use lct_diagnostics::Span;

impl FloatingPoint {
    /// Gets the value of this [`FloatingPoint`] number
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Gets the location of this [`FloatingPoint`] number
    pub fn span(&self) -> Span {
        self.span
    }
}
