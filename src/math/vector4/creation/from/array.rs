use crate::math::Vector4;
use std::marker::Destruct;

impl<T> Vector4<T> {
    /// Create a new [`Vector4`] from an array
    pub const fn from_array([x, y, z, w]: [T; 4]) -> Self
    where
        T: [const] Destruct,
    {
        Vector4::new(x, y, z, w)
    }
}

impl<T: [const] Destruct> const From<[T; 4]> for Vector4<T> {
    fn from(array: [T; 4]) -> Self {
        Vector4::from_array(array)
    }
}
