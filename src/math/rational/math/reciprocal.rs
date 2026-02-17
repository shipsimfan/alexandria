use crate::math::Rational;
use std::num::NonZeroU32;

impl Rational {
    /// Find the reciprocal of this [`Rational`]
    ///
    /// This function will panic if `self.numerator` is 0
    pub const fn reciprocal(self) -> Rational {
        let sign = self.numerator.signum();
        Rational::new(
            self.denominator.get() as i32 * sign,
            NonZeroU32::new(self.numerator.abs() as _).unwrap(),
        )
    }
}
