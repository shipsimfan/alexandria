use crate::math::{
    Vector4,
    number::{Infinity, NaN, NegInfinity, One, Zero},
};
use std::{marker::Destruct, ops::Neg};

impl<T: Zero> Vector4<T> {
    /// A [`Vector4`] with all values set to 0
    pub const ZERO: Vector4<T> = Vector4::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO);
}

impl<T: Zero> Zero for Vector4<T> {
    const ZERO: Self = Self::ZERO;
}

impl<T: One> Vector4<T> {
    /// A [`Vector4`] with all values set to 1
    pub const ONE: Vector4<T> = Vector4::new(T::ONE, T::ONE, T::ONE, T::ONE);

    /// A [`Vector4`] with all values set to 1
    pub const NORMALIZED_ONE: Vector4<T> = Vector4::new(
        T::NORMALIZED_ONE,
        T::NORMALIZED_ONE,
        T::NORMALIZED_ONE,
        T::NORMALIZED_ONE,
    );
}

impl<T: One> One for Vector4<T> {
    const ONE: Self = Self::ONE;

    const NORMALIZED_ONE: Self = Self::NORMALIZED_ONE;
}

impl<T: Zero + One> Vector4<T> {
    /// A [`Vector4`] which has 1 in the x-axis and 0 in all others
    pub const X: Vector4<T> = Vector4::new(T::ONE, T::ZERO, T::ZERO, T::ZERO);

    /// A [`Vector4`] which has 1 in the y-axis and 0 in all others
    pub const Y: Vector4<T> = Vector4::new(T::ZERO, T::ONE, T::ZERO, T::ZERO);

    /// A [`Vector4`] which has 1 in the z-axis and 0 in all others
    pub const Z: Vector4<T> = Vector4::new(T::ZERO, T::ZERO, T::ONE, T::ZERO);

    /// A [`Vector4`] which has 1 in the w-axis and 0 in all others
    pub const W: Vector4<T> = Vector4::new(T::ZERO, T::ZERO, T::ZERO, T::ONE);

    /// A [`Vector4`] which points to the right
    pub const RIGHT: Vector4<T> = Self::X;

    /// A [`Vector4`] which points up
    pub const UP: Vector4<T> = Self::Y;

    /// A [`Vector4`] which points forwards
    pub const FORWARD: Vector4<T> = Self::Z;

    /// A [`Vector4`] which points ana
    pub const ANA: Vector4<T> = Self::W;
}

impl<T: const Neg<Output = T> + const Destruct + Zero + One> Vector4<T> {
    /// A [`Vector4`] which points to the left
    pub const LEFT: Vector4<T> = -Self::RIGHT;

    /// A [`Vector4`] which points down
    pub const DOWN: Vector4<T> = -Self::UP;

    /// A [`Vector4`] which points backwards
    pub const BACKWARD: Vector4<T> = -Self::FORWARD;

    /// A [`Vector4`] which points kata
    pub const KATA: Vector4<T> = -Self::ANA;
}

impl<T: NaN> Vector4<T> {
    /// A [`Vector4`] with all values being `NaN`
    pub const NAN: Vector4<T> = Vector4::new(T::NAN, T::NAN, T::NAN, T::NAN);
}

impl<T: NaN> NaN for Vector4<T> {
    const NAN: Self = Self::NAN;
}

impl<T: Infinity> Vector4<T> {
    /// A [`Vector4`] with all values being `NaN`
    pub const INFINITY: Vector4<T> =
        Vector4::new(T::INFINITY, T::INFINITY, T::INFINITY, T::INFINITY);
}

impl<T: Infinity> Infinity for Vector4<T> {
    const INFINITY: Self = Self::INFINITY;
}

impl<T: NegInfinity> Vector4<T> {
    /// A [`Vector4`] with all values being `NaN`
    pub const NEG_INFINITY: Vector4<T> = Vector4::new(
        T::NEG_INFINITY,
        T::NEG_INFINITY,
        T::NEG_INFINITY,
        T::NEG_INFINITY,
    );
}

impl<T: NegInfinity> NegInfinity for Vector4<T> {
    const NEG_INFINITY: Self = Self::NEG_INFINITY;
}
