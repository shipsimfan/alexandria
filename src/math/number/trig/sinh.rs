/// Computes the hyperbolic sine of a value in radians
pub trait Sinh {
    /// Computes the hyperbolic sine of a value in radians
    fn sinh(self) -> Self;
}

impl Sinh for f32 {
    fn sinh(self) -> Self {
        f32::sinh(self)
    }
}

impl Sinh for f64 {
    fn sinh(self) -> Self {
        f64::sinh(self)
    }
}
