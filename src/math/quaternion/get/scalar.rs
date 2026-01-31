use crate::math::Quaternion;
use std::marker::Destruct;

impl<T> Quaternion<T> {
    /// Get the scalar part of the quaternion
    pub const fn scalar(self) -> T
    where
        T: [const] Destruct,
    {
        self.w
    }
}
