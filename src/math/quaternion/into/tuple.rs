use crate::math::Quaternion;
use std::marker::Destruct;

impl<T> Quaternion<T> {
    /// Convert the elements of this [`Quaternion`] into a tuple
    pub const fn into_tuple(self) -> (T, T, T, T)
    where
        T: [const] Destruct,
    {
        (self.x, self.y, self.z, self.w)
    }
}

impl<T: [const] Destruct> const Into<(T, T, T, T)> for Quaternion<T> {
    fn into(self) -> (T, T, T, T) {
        self.into_tuple()
    }
}
