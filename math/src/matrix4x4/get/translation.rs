use crate::{Matrix4x4, Vector3};
use std::marker::Destruct;

impl<T> Matrix4x4<T> {
    /// Extract the translation elements from this matrix
    pub const fn translation(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.r0.w, self.r1.w, self.r2.w)
    }
}
