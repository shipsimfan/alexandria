use crate::math::Rational;
use std::num::NonZeroU32;

impl Rational {
    /// Create a new [`Rational`] from the ration `numerator:denominator`
    pub const fn from_ratio(numerator: i32, denominator: NonZeroU32) -> Rational {
        Rational::new(numerator, denominator)
    }
}
