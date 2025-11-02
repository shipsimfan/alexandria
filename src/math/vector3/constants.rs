use crate::math::{
    number::{Infinity, NaN, NegInfinity, One, Zero},
    Vector3,
};

impl<T: Zero> Zero for Vector3<T> {
    const ZERO: Self = Vector3::zero();
}

impl<T: One> One for Vector3<T> {
    const ONE: Self = Vector3::one();
}

impl<T: Zero + One> Vector3<T> {
    /// Unit [`Vector3`] along the positive x-axis
    pub const UNIT_X: Self = Vector3::unit_x();

    /// Unit [`Vector3`] along the positive y-axis
    pub const UNIT_Y: Self = Vector3::unit_y();

    /// Unit [`Vector3`] along the positive z-axis
    pub const UNIT_Z: Self = Vector3::unit_z();
}

impl<T: Infinity> Infinity for Vector3<T> {
    const INFINITY: Self = Vector3::infinity();
}

impl<T: NegInfinity> NegInfinity for Vector3<T> {
    const NEG_INFINITY: Self = Vector3::neg_infinity();
}

impl<T: NaN> NaN for Vector3<T> {
    const NAN: Self = Vector3::nan();
}
