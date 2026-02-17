use crate::math::Rational;

impl Rational {
    /// Is this [`Rational`] zero?
    pub const fn is_zero(&self) -> bool {
        self.numerator == 0
    }
}
