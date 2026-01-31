use crate::math::{
    Vector3,
    number::{Infinity, NaN, NegInfinity, One, Zero},
};
use std::{marker::Destruct, ops::Neg};

impl<T: Zero> Vector3<T> {
    /// A [`Vector3`] with all values set to 0
    pub const ZERO: Vector3<T> = Vector3::new(T::ZERO, T::ZERO, T::ZERO);
}

impl<T: Zero> Zero for Vector3<T> {
    const ZERO: Self = Self::ZERO;
}

impl<T: One> Vector3<T> {
    /// A [`Vector3`] with all values set to 1
    pub const ONE: Vector3<T> = Vector3::new(T::ONE, T::ONE, T::ONE);

    /// A [`Vector3`] with all values set to 1
    pub const NORMALIZED_ONE: Vector3<T> =
        Vector3::new(T::NORMALIZED_ONE, T::NORMALIZED_ONE, T::NORMALIZED_ONE);
}

impl<T: One> One for Vector3<T> {
    const ONE: Self = Self::ONE;

    const NORMALIZED_ONE: Self = Self::NORMALIZED_ONE;
}

impl<T: Zero + One> Vector3<T> {
    /// A [`Vector3`] which has 1 in the x-axis and 0 in all others
    pub const X: Vector3<T> = Vector3::new(T::ONE, T::ZERO, T::ZERO);

    /// A [`Vector3`] which has 1 in the y-axis and 0 in all others
    pub const Y: Vector3<T> = Vector3::new(T::ZERO, T::ONE, T::ZERO);

    /// A [`Vector3`] which has 1 in the z-axis and 0 in all others
    pub const Z: Vector3<T> = Vector3::new(T::ZERO, T::ZERO, T::ONE);

    /// A [`Vector3`] which points to the right
    pub const RIGHT: Vector3<T> = Self::X;

    /// A [`Vector3`] which points up
    pub const UP: Vector3<T> = Self::Y;

    /// A [`Vector3`] which points forwards
    pub const FORWARD: Vector3<T> = Self::Z;
}

impl<T: const Neg<Output = T> + const Destruct + Zero + One> Vector3<T> {
    /// A [`Vector3`] which points to the left
    pub const LEFT: Vector3<T> = -Self::RIGHT;

    /// A [`Vector3`] which points down
    pub const DOWN: Vector3<T> = -Self::UP;

    /// A [`Vector3`] which points backwards
    pub const BACKWARD: Vector3<T> = -Self::FORWARD;
}

impl<T: NaN> Vector3<T> {
    /// A [`Vector3`] with all values being `NaN`
    pub const NAN: Vector3<T> = Vector3::new(T::NAN, T::NAN, T::NAN);
}

impl<T: NaN> NaN for Vector3<T> {
    const NAN: Self = Self::NAN;
}

impl<T: Infinity> Vector3<T> {
    /// A [`Vector3`] with all values being `NaN`
    pub const INFINITY: Vector3<T> = Vector3::new(T::INFINITY, T::INFINITY, T::INFINITY);
}

impl<T: Infinity> Infinity for Vector3<T> {
    const INFINITY: Self = Self::INFINITY;
}

impl<T: NegInfinity> Vector3<T> {
    /// A [`Vector3`] with all values being `NaN`
    pub const NEG_INFINITY: Vector3<T> =
        Vector3::new(T::NEG_INFINITY, T::NEG_INFINITY, T::NEG_INFINITY);
}

impl<T: NegInfinity> NegInfinity for Vector3<T> {
    const NEG_INFINITY: Self = Self::NEG_INFINITY;
}
