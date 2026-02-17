use crate::math::Rational;

impl const Clone for Rational {
    fn clone(&self) -> Self {
        Rational {
            numerator: self.numerator,
            denominator: self.denominator,
        }
    }
}

impl Copy for Rational {}
