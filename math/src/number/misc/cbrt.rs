/// Computes the cube root of a value
pub trait Cbrt {
    /// Computes the cube root of a value
    fn cbrt(self) -> Self;
}

impl Cbrt for f32 {
    fn cbrt(self) -> Self {
        f32::cbrt(self)
    }
}

impl Cbrt for f64 {
    fn cbrt(self) -> Self {
        f64::cbrt(self)
    }
}
