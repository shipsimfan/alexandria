use crate::math::Rational;
use std::num::NonZeroU32;

impl Rational {
    /// Create a new [`Rational`] for an integer `i`
    pub const fn from_int(i: i32) -> Rational {
        Rational {
            numerator: i,
            denominator: unsafe { NonZeroU32::new_unchecked(1) },
        }
    }
}
