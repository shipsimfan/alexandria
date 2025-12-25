use crate::{
    Matrix4x4, Vector3,
    number::{One, Zero},
};
use std::marker::Destruct;

impl<T: Zero + One> Matrix4x4<T> {
    /// Make a [`Matrix4x4`] representing uniform `scale`
    pub const fn from_uniform_scale(scale: T) -> Matrix4x4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Matrix4x4::from_scale(Vector3::splat(scale))
    }
}
