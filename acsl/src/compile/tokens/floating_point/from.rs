use crate::compile::tokens::FloatingPoint;
use lct_diagnostics::Span;

impl From<f64> for FloatingPoint {
    fn from(value: f64) -> Self {
        FloatingPoint {
            value,
            span: Span::default(),
        }
    }
}
