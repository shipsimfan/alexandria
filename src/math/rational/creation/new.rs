use crate::math::Rational;
use std::num::NonZeroU32;

impl Rational {
    /// Create a new [`Rational`]
    pub const fn new(numerator: i32, denominator: NonZeroU32) -> Rational {
        Rational {
            numerator,
            denominator,
        }
    }
}
