use crate::math::Quaternion;
use std::{marker::Destruct, ops::Neg};

impl<T> Quaternion<T> {
    /// Calculate the conjugate of this [`Quaternion`]
    pub const fn conjugate(self) -> Quaternion<T>
    where
        T: [const] Neg<Output = T> + [const] Destruct,
    {
        Quaternion::new(-self.x, -self.y, -self.z, self.w)
    }
}
