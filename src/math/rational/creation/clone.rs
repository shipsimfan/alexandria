use crate::math::Rational;

const impl Clone for Rational {
    fn clone(&self) -> Self {
        Rational {
            numerator: self.numerator,
            denominator: self.denominator,
        }
    }
}

impl Copy for Rational {}
