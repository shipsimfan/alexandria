use crate::math::Rational;
use std::hash::Hash;

impl Hash for Rational {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.numerator.hash(state);
        self.denominator.hash(state);
    }
}
