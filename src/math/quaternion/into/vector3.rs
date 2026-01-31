use crate::math::{Quaternion, Vector3};
use std::marker::Destruct;

impl<T> Quaternion<T> {
    /// Splits this [`Quaternion`] into separate xyz and w components
    pub const fn xyz_w(self) -> (Vector3<T>, T)
    where
        T: [const] Destruct,
    {
        (Vector3::new(self.x, self.y, self.z), self.w)
    }
}
