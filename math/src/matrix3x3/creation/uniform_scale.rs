use crate::{
    Matrix3x3, Vector3,
    number::{One, Zero},
};
use std::marker::Destruct;

impl<T: Zero + One> Matrix3x3<T> {
    /// Make a [`Matrix3x3`] representing uniform `scale`
    pub const fn from_uniform_scale(scale: T) -> Matrix3x3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Matrix3x3::from_scale(Vector3::splat(scale))
    }
}
