/// Computes the hyperbolic tangent of a value in radians
pub trait Tanh {
    /// Computes the hyperbolic tangent of a value in radians
    fn tanh(self) -> Self;
}

impl Tanh for f32 {
    fn tanh(self) -> Self {
        f32::tanh(self)
    }
}

impl Tanh for f64 {
    fn tanh(self) -> Self {
        f64::tanh(self)
    }
}
