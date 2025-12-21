use crate::Vector4;
use std::marker::Destruct;

impl<T> Vector4<T> {
    /// Create a new [`Vector4`] from a tuple
    pub const fn from_tuple((x, y, z, w): (T, T, T, T)) -> Self
    where
        T: [const] Destruct,
    {
        Vector4::new(x, y, z, w)
    }
}

impl<T: [const] Destruct> const From<(T, T, T, T)> for Vector4<T> {
    fn from(tuple: (T, T, T, T)) -> Self {
        Vector4::from_tuple(tuple)
    }
}
