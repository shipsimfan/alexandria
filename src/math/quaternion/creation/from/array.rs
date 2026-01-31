use crate::math::Quaternion;
use std::marker::Destruct;

impl<T> Quaternion<T> {
    /// Create a new [`Quaternion`] from an array
    pub const fn from_array([x, y, z, w]: [T; 4]) -> Quaternion<T>
    where
        T: [const] Destruct,
    {
        Quaternion::new(x, y, z, w)
    }
}

impl<T: [const] Destruct> const From<[T; 4]> for Quaternion<T> {
    fn from(array: [T; 4]) -> Self {
        Quaternion::from_array(array)
    }
}
