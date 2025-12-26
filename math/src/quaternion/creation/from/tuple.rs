use crate::Quaternion;
use std::marker::Destruct;

impl<T> Quaternion<T> {
    /// Create a new [`Quaternion`] from a tuple
    pub const fn from_tuple((x, y, z, w): (T, T, T, T)) -> Self
    where
        T: [const] Destruct,
    {
        Quaternion::new(x, y, z, w)
    }
}

impl<T: [const] Destruct> const From<(T, T, T, T)> for Quaternion<T> {
    fn from(tuple: (T, T, T, T)) -> Self {
        Quaternion::from_tuple(tuple)
    }
}
