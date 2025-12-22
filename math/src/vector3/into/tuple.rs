use crate::Vector3;
use std::marker::Destruct;

impl<T> Vector3<T> {
    /// Convert the elements of this [`Vector3`] into a tuple
    pub const fn into_tuple(self) -> (T, T, T)
    where
        T: [const] Destruct,
    {
        (self.x, self.y, self.z)
    }
}

impl<T: [const] Destruct> const Into<(T, T, T)> for Vector3<T> {
    fn into(self) -> (T, T, T) {
        self.into_tuple()
    }
}
