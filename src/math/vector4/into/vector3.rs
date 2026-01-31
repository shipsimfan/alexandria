use crate::math::{Vector3, Vector4};
use std::marker::Destruct;

impl<T> Vector4<T> {
    /// Splits this [`Vector4`] into separate xyz and w components
    pub const fn xyz_w(self) -> (Vector3<T>, T)
    where
        T: [const] Destruct,
    {
        (Vector3::new(self.x, self.y, self.z), self.w)
    }
}
