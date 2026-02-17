use crate::math::Rational;
use std::num::NonZeroU32;

impl Rational {
    /// Convert this [`Rational`] into a tuple
    pub const fn into_tuple(self) -> (i32, NonZeroU32) {
        (self.numerator, self.denominator)
    }
}

impl const Into<(i32, NonZeroU32)> for Rational {
    fn into(self) -> (i32, NonZeroU32) {
        self.into_tuple()
    }
}
