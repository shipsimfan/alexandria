use crate::math::Vector3;
use std::marker::Destruct;

impl<T> Vector3<T> {
    /// Create a new [`Vector3`] from a tuple
    pub const fn from_tuple((x, y, z): (T, T, T)) -> Self
    where
        T: [const] Destruct,
    {
        Vector3::new(x, y, z)
    }
}

impl<T: [const] Destruct> const From<(T, T, T)> for Vector3<T> {
    fn from(tuple: (T, T, T)) -> Self {
        Vector3::from_tuple(tuple)
    }
}
