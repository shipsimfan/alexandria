use crate::math::Rational;

impl Rational {
    /// Get this [`Rational`] as an [`f64`]
    pub const fn as_f64(self) -> f64 {
        self.numerator as f64 / self.denominator.get() as f64
    }
}

impl const Into<f64> for Rational {
    fn into(self) -> f64 {
        self.as_f64()
    }
}
