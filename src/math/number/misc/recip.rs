/// Computes the reciprocal of a value
pub const trait Recip: Sized {
    /// Computes the reciprocal of a value
    fn recip(self) -> Self;
}

const impl Recip for f32 {
    fn recip(self) -> Self {
        f32::recip(self)
    }
}

const impl Recip for f64 {
    fn recip(self) -> Self {
        f64::recip(self)
    }
}
