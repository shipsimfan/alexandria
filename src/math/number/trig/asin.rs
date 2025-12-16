/// Computes the inverse sine of a value in radians
pub trait Asin {
    /// Computes the inverse sine of a value in radians
    fn asin(self) -> Self;
}

impl Asin for f32 {
    fn asin(self) -> Self {
        f32::asin(self)
    }
}

impl Asin for f64 {
    fn asin(self) -> Self {
        f64::asin(self)
    }
}
