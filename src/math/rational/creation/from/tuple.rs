use crate::math::Rational;
use std::num::NonZeroU32;

impl Rational {
    /// Create a new [`Rational`] from a tuple
    pub const fn from_tuple((numerator, denominator): (i32, NonZeroU32)) -> Rational {
        Rational::new(numerator, denominator)
    }
}

impl const From<(i32, NonZeroU32)> for Rational {
    fn from(tuple: (i32, NonZeroU32)) -> Self {
        Rational::from_tuple(tuple)
    }
}
