use crate::math::{
    number::{Infinity, NaN, NegInfinity, One, Zero},
    Vector2,
};

impl<T: Zero> Zero for Vector2<T> {
    const ZERO: Self = Vector2::zero();
}

impl<T: One> One for Vector2<T> {
    const ONE: Self = Vector2::one();
}

impl<T: Zero + One> Vector2<T> {
    /// Unit [`Vector2`] along the positive x-axis
    pub const UNIT_X: Self = Vector2::unit_x();

    /// Unit [`Vector2`] along the positive y-axis
    pub const UNIT_Y: Self = Vector2::unit_y();
}

impl<T: Infinity> Infinity for Vector2<T> {
    const INFINITY: Self = Vector2::infinity();
}

impl<T: NegInfinity> NegInfinity for Vector2<T> {
    const NEG_INFINITY: Self = Vector2::neg_infinity();
}

impl<T: NaN> NaN for Vector2<T> {
    const NAN: Self = Vector2::nan();
}
