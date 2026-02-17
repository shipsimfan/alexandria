use crate::math::Rational;

impl std::fmt::Debug for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(Rational))
            .field("numerator", &self.numerator)
            .field("denominator", &self.denominator)
            .finish()
    }
}
