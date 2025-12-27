use crate::{
    Matrix3x3, Vector3,
    number::{One, Zero},
};

impl<T: Zero> Matrix3x3<T> {
    /// A [`Matrix3x3`] with all values set to 0
    pub const ZERO: Matrix3x3<T> = Matrix3x3::new_rows(Vector3::ZERO, Vector3::ZERO, Vector3::ZERO);
}

impl<T: Zero> Zero for Matrix3x3<T> {
    const ZERO: Matrix3x3<T> = Matrix3x3::ZERO;
}

impl<T: One> Matrix3x3<T> {
    /// A [`Matrix3x3`] with all values set to 1
    pub const ONE: Matrix3x3<T> = Matrix3x3::new_rows(Vector3::ONE, Vector3::ONE, Vector3::ONE);
}

impl<T: One> One for Matrix3x3<T> {
    const ONE: Matrix3x3<T> = Matrix3x3::ONE;

    const NORMALIZED_ONE: Self = Matrix3x3::new_rows(
        Vector3::NORMALIZED_ONE,
        Vector3::NORMALIZED_ONE,
        Vector3::NORMALIZED_ONE,
    );
}

impl<T: Zero + One> Matrix3x3<T> {
    /// A [`Matrix3x3`] with only the diagonal set to 1 and the rest set to 0
    pub const IDENTITY: Matrix3x3<T> = Matrix3x3::diagonal(T::ONE, T::ONE, T::ONE);
}
