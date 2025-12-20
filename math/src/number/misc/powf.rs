/// Raises a value to another value
pub const trait PowF {
    /// Raises a value to another value
    fn powf(self, n: Self) -> Self;
}

impl PowF for f32 {
    fn powf(self, n: Self) -> Self {
        f32::powf(self, n)
    }
}

impl PowF for f64 {
    fn powf(self, n: Self) -> Self {
        f64::powf(self, n)
    }
}
