/// Computes the inverse tangent of a value in radians
pub trait Atan {
    /// Computes the inverse tangent of a value in radians
    fn atan(self) -> Self;
}

impl Atan for f32 {
    fn atan(self) -> Self {
        f32::atan(self)
    }
}

impl Atan for f64 {
    fn atan(self) -> Self {
        f64::atan(self)
    }
}
