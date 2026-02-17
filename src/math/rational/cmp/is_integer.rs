use crate::math::Rational;

impl Rational {
    /// Is this [`Rational`] an integer?
    pub const fn is_integer(&self) -> bool {
        self.numerator as i64 % self.denominator.get() as i64 == 0
    }
}
