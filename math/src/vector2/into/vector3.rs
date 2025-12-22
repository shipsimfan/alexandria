use crate::{Vector2, Vector3};
use std::marker::Destruct;

impl<T> Vector2<T> {
    /// Create a new [`Vector3`] from this this combined with `z`
    pub const fn extend(self, z: T) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.x, self.y, z)
    }
}
