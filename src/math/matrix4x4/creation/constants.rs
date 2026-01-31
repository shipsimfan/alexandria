use crate::math::{
    Matrix4x4, Vector4,
    number::{One, Zero},
};

impl<T: Zero> Matrix4x4<T> {
    /// A [`Matrix4x4`] with all values set to 0
    pub const ZERO: Matrix4x4<T> =
        Matrix4x4::new_rows(Vector4::ZERO, Vector4::ZERO, Vector4::ZERO, Vector4::ZERO);
}

impl<T: Zero> Zero for Matrix4x4<T> {
    const ZERO: Matrix4x4<T> = Matrix4x4::ZERO;
}

impl<T: One> Matrix4x4<T> {
    /// A [`Matrix4x4`] with all values set to 1
    pub const ONE: Matrix4x4<T> =
        Matrix4x4::new_rows(Vector4::ONE, Vector4::ONE, Vector4::ONE, Vector4::ONE);
}

impl<T: One> One for Matrix4x4<T> {
    const ONE: Matrix4x4<T> = Matrix4x4::ONE;

    const NORMALIZED_ONE: Self = Matrix4x4::new_rows(
        Vector4::NORMALIZED_ONE,
        Vector4::NORMALIZED_ONE,
        Vector4::NORMALIZED_ONE,
        Vector4::NORMALIZED_ONE,
    );
}

impl<T: Zero + One> Matrix4x4<T> {
    /// A [`Matrix4x4`] with only the diagonal set to 1 and the rest set to 0
    pub const IDENTITY: Matrix4x4<T> = Matrix4x4::diagonal(T::ONE, T::ONE, T::ONE, T::ONE);
}
