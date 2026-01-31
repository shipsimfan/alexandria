use crate::math::{
    Vector2,
    number::{Infinity, NaN, NegInfinity, One, Zero},
};
use std::{marker::Destruct, ops::Neg};

impl<T: Zero> Vector2<T> {
    /// A [`Vector2`] with all values set to 0
    pub const ZERO: Vector2<T> = Vector2::new(T::ZERO, T::ZERO);
}

impl<T: Zero> Zero for Vector2<T> {
    const ZERO: Vector2<T> = Vector2::ZERO;
}

impl<T: One> Vector2<T> {
    /// A [`Vector2`] with all values set to 1
    pub const ONE: Vector2<T> = Vector2::new(T::ONE, T::ONE);

    /// A [`Vector2`] with all values set to 1
    pub const NORMALIZED_ONE: Vector2<T> = Vector2::new(T::NORMALIZED_ONE, T::NORMALIZED_ONE);
}

impl<T: One> One for Vector2<T> {
    const ONE: Vector2<T> = Vector2::ONE;

    const NORMALIZED_ONE: Vector2<T> = Vector2::NORMALIZED_ONE;
}

impl<T: Zero + One> Vector2<T> {
    /// A [`Vector2`] which has 1 in the x-axis and 0 in all others
    pub const X: Vector2<T> = Vector2::new(T::ONE, T::ZERO);

    /// A [`Vector2`] which has 1 in the y-axis and 0 in all others
    pub const Y: Vector2<T> = Vector2::new(T::ZERO, T::ONE);

    /// A [`Vector2`] which points to the right
    pub const RIGHT: Vector2<T> = Vector2::X;

    /// A [`Vector2`] which points up
    pub const UP: Vector2<T> = Vector2::Y;
}

impl<T: const Neg<Output = T> + const Destruct + Zero + One> Vector2<T> {
    /// A [`Vector2`] which points to the left
    pub const LEFT: Vector2<T> = -Vector2::RIGHT;

    /// A [`Vector2`] which points down
    pub const DOWN: Vector2<T> = -Vector2::UP;
}

impl<T: NaN> Vector2<T> {
    /// A [`Vector2`] with all values being `NaN`
    pub const NAN: Vector2<T> = Vector2::new(T::NAN, T::NAN);
}

impl<T: NaN> NaN for Vector2<T> {
    const NAN: Vector2<T> = Vector2::NAN;
}

impl<T: Infinity> Vector2<T> {
    /// A [`Vector2`] with all values being `NaN`
    pub const INFINITY: Vector2<T> = Vector2::new(T::INFINITY, T::INFINITY);
}

impl<T: Infinity> Infinity for Vector2<T> {
    const INFINITY: Vector2<T> = Vector2::INFINITY;
}

impl<T: NegInfinity> Vector2<T> {
    /// A [`Vector2`] with all values being `NaN`
    pub const NEG_INFINITY: Vector2<T> = Vector2::new(T::NEG_INFINITY, T::NEG_INFINITY);
}

impl<T: NegInfinity> NegInfinity for Vector2<T> {
    const NEG_INFINITY: Vector2<T> = Vector2::NEG_INFINITY;
}
