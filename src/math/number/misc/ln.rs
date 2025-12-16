/// Computes the natural logarithm of a value
pub trait Ln {
    /// Computes the natural logarithm of a value
    fn ln(self) -> Self;
}

impl Ln for f32 {
    fn ln(self) -> Self {
        f32::ln(self)
    }
}

impl Ln for f64 {
    fn ln(self) -> Self {
        f64::ln(self)
    }
}
