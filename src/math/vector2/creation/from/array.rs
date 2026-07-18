use crate::math::Vector2;
use std::marker::Destruct;

impl<T> Vector2<T> {
    /// Create a new [`Vector2`] from an array
    pub const fn from_array([x, y]: [T; 2]) -> Vector2<T>
    where
        T: [const] Destruct,
    {
        Vector2::new(x, y)
    }
}

const impl<T: [const] Destruct> From<[T; 2]> for Vector2<T> {
    fn from(array: [T; 2]) -> Vector2<T> {
        Vector2::from_array(array)
    }
}
