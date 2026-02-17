use crate::math::Rational;

impl Rational {
    /// Get this [`Rational`] as an [`f32`]
    pub const fn as_f32(self) -> f32 {
        self.numerator as f32 / self.denominator.get() as f32
    }
}

impl const Into<f32> for Rational {
    fn into(self) -> f32 {
        self.as_f32()
    }
}
