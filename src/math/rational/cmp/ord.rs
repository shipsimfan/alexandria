use crate::math::Rational;
use std::cmp::Ordering;

const impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const impl Ord for Rational {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.numerator as i64 * other.denominator.get() as i64;
        let b = other.numerator as i64 * self.denominator.get() as i64;
        a.cmp(&b)
    }
}
