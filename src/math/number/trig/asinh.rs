/// Computes the inverse hyperbolic sine of a value in radians
pub trait Asinh {
    /// Computes the inverse hyperbolic sine of a value in radians
    fn asinh(self) -> Self;
}

impl Asinh for f32 {
    fn asinh(self) -> Self {
        f32::asinh(self)
    }
}

impl Asinh for f64 {
    fn asinh(self) -> Self {
        f64::asinh(self)
    }
}
