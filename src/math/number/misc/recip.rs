/// Computes the reciprocal of a value
pub const trait Recip: Sized {
    /// Computes the reciprocal of a value
    fn recip(self) -> Self;
}

impl const Recip for f32 {
    fn recip(self) -> Self {
        f32::recip(self)
    }
}

impl const Recip for f64 {
    fn recip(self) -> Self {
        f64::recip(self)
    }
}
