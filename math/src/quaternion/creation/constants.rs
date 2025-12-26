use crate::{
    Quaternion,
    number::{One, Zero},
};

impl<T: Zero> Quaternion<T> {
    /// A [`Quaternion`] with all values set to 0
    pub const ZERO: Quaternion<T> = Quaternion::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO);
}

impl<T: Zero> Zero for Quaternion<T> {
    const ZERO: Quaternion<T> = Quaternion::ZERO;
}

impl<T: One> Quaternion<T> {
    /// A [`Quaternion`] with all values set to 0
    pub const ONE: Quaternion<T> = Quaternion::new(T::ONE, T::ONE, T::ONE, T::ONE);
}

impl<T: One> One for Quaternion<T> {
    const ONE: Quaternion<T> = Quaternion::ONE;

    const NORMALIZED_ONE: Self = Quaternion::new(
        T::NORMALIZED_ONE,
        T::NORMALIZED_ONE,
        T::NORMALIZED_ONE,
        T::NORMALIZED_ONE,
    );
}

impl<T: Zero + One> Quaternion<T> {
    /// The identity [`Quaternion`] that represents no rotation
    pub const IDENTITY: Quaternion<T> = Quaternion::new(T::ZERO, T::ZERO, T::ZERO, T::ONE);
}
