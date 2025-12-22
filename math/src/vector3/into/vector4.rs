use crate::{Vector3, Vector4};
use std::marker::Destruct;

impl<T> Vector3<T> {
    /// Create a new [`Vector4`] from this this combined with `w`
    pub const fn extend(self, w: T) -> Vector4<T>
    where
        T: [const] Destruct,
    {
        Vector4::new(self.x, self.y, self.z, w)
    }
}
