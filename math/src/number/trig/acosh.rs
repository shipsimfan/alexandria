/// Computes the inverse hyperbolic cosine of a value in radians
pub trait Acosh {
    /// Computes the inverse hyperbolic cosine of a value in radians
    fn acosh(self) -> Self;
}

impl Acosh for f32 {
    fn acosh(self) -> Self {
        f32::acosh(self)
    }
}

impl Acosh for f64 {
    fn acosh(self) -> Self {
        f64::acosh(self)
    }
}
