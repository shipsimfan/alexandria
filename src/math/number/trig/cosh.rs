/// Computes the hyperbolic cosine of a value in radians
pub trait Cosh {
    /// Computes the hyperbolic cosine of a value in radians
    fn cosh(self) -> Self;
}

impl Cosh for f32 {
    fn cosh(self) -> Self {
        f32::cosh(self)
    }
}

impl Cosh for f64 {
    fn cosh(self) -> Self {
        f64::cosh(self)
    }
}
