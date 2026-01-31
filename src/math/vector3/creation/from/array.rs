use crate::math::Vector3;
use std::marker::Destruct;

impl<T> Vector3<T> {
    /// Create a new [`Vector3`] from an array
    pub const fn from_array([x, y, z]: [T; 3]) -> Self
    where
        T: [const] Destruct,
    {
        Vector3::new(x, y, z)
    }
}

impl<T: [const] Destruct> const From<[T; 3]> for Vector3<T> {
    fn from(array: [T; 3]) -> Self {
        Vector3::from_array(array)
    }
}
