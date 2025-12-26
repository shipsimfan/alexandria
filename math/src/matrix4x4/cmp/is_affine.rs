use crate::{
    Matrix4x4, Vector4,
    number::{ApproxEq, One, Zero},
};
use std::marker::Destruct;

impl<T: Zero + One> Matrix4x4<T> {
    /// Is this matrix affine?
    pub const fn is_affine(self, epsilon: T::Epsilon) -> bool
    where
        T: [const] ApproxEq + [const] Destruct,
        T::Epsilon: [const] Clone + [const] Destruct,
    {
        self.r3.approx_eq(Vector4::W, epsilon)
    }
}
