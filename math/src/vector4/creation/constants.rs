use crate::{
    Vector4,
    number::{One, Zero},
};

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
    pub const RIGHT: Vector4<T> = Vector4::X;

    /// A [`Vector4`] which points up
    pub const UP: Vector4<T> = Vector4::Y;

    /// A [`Vector4`] which points forward
    pub const FORWARD: Vector4<T> = Vector4::Z;

    /// A [`Vector4`] which points ana
    pub const ANA: Vector4<T> = Vector4::W;
}
