use crate::math::{Matrix4x4, Vector3};
use std::marker::Destruct;

impl<T> Matrix4x4<T> {
    /// Extract the scale elements from this matrix, assuming there is no shear
    pub const fn scale(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.r0.x, self.r1.y, self.r2.z)
    }
}
