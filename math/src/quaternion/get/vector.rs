use crate::{Quaternion, Vector3};
use std::marker::Destruct;

impl<T> Quaternion<T> {
    /// Get the vector/imaginary part of this quaternion
    pub const fn vector(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.x, self.y, self.z)
    }
}
