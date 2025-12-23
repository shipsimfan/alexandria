use crate::{
    Matrix4x4, Vector4,
    number::{One, Zero},
};

impl<T: Zero> Matrix4x4<T> {
    /// A [`Matrix4x4`] with all values set to 0
    pub const ZERO: Matrix4x4<T> =
        Matrix4x4::new_rows(Vector4::ZERO, Vector4::ZERO, Vector4::ZERO, Vector4::ZERO);
}

impl<T: Zero + One> Matrix4x4<T> {
    /// A [`Matrix4x4`] with only the diagonal set to 1 and the rest set to 0
    pub const IDENTITY: Matrix4x4<T> =
        Matrix4x4::new_rows(Vector4::X, Vector4::Y, Vector4::Z, Vector4::W);
}
