use crate::{
    Matrix3x3, Matrix4x4,
    number::{One, Zero},
};
use std::marker::Destruct;

impl<T: Zero + One> Matrix4x4<T> {
    /// Make a [`Matrix4x4`] representing uniform `scale`
    pub const fn from_uniform_scale(scale: T) -> Matrix4x4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Matrix3x3::from_uniform_scale(scale).into()
    }
}
