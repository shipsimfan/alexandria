/// Computes the inverse hyperbolic tangent of a value in radians
pub trait Atanh {
    /// Computes the inverse hyperbolic tangent of a value in radians
    fn atanh(self) -> Self;
}

impl Atanh for f32 {
    fn atanh(self) -> Self {
        f32::atanh(self)
    }
}

impl Atanh for f64 {
    fn atanh(self) -> Self {
        f64::atanh(self)
    }
}
