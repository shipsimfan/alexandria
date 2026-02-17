use crate::math::{
    Rational,
    number::{One, Zero},
};

impl Rational {
    /// A [`Rational`] with a value of 0
    pub const ZERO: Rational = Rational::from_int(0);

    /// A [`Rational`] with a value of 1
    pub const ONE: Rational = Rational::from_int(1);
}

impl Zero for Rational {
    const ZERO: Self = Self::ZERO;
}

impl One for Rational {
    const ONE: Self = Self::ONE;
    const NORMALIZED_ONE: Self = Rational::from_int(i32::NORMALIZED_ONE);
}
