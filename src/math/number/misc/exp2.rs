/// Computes `2^(self)`
pub trait Exp2 {
    /// Computes `2^(self)`
    fn exp2(self) -> Self;
}

impl Exp2 for f32 {
    fn exp2(self) -> Self {
        f32::exp2(self)
    }
}

impl Exp2 for f64 {
    fn exp2(self) -> Self {
        f64::exp2(self)
    }
}
