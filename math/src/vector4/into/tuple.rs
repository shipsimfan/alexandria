use crate::Vector4;
use std::marker::Destruct;

impl<T> Vector4<T> {
    /// Convert the elements of this [`Vector4`] into a tuple
    pub const fn into_tuple(self) -> (T, T, T, T)
    where
        T: [const] Destruct,
    {
        (self.x, self.y, self.z, self.w)
    }
}

impl<T: [const] Destruct> const Into<(T, T, T, T)> for Vector4<T> {
    fn into(self) -> (T, T, T, T) {
        self.into_tuple()
    }
}
