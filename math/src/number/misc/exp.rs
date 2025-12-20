/// Computes `e^(self)`
pub trait Exp {
    /// Computes `e^(self)`
    fn exp(self) -> Self;
}

impl Exp for f32 {
    fn exp(self) -> Self {
        f32::exp(self)
    }
}

impl Exp for f64 {
    fn exp(self) -> Self {
        f64::exp(self)
    }
}
