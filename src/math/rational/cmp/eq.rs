use crate::math::Rational;

impl const PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        let a = self.numerator as i64 * other.denominator.get() as i64;
        let b = other.numerator as i64 * self.denominator.get() as i64;
        a == b
    }
}

impl const Eq for Rational {}
