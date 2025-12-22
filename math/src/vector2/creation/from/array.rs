use crate::Vector2;
use std::marker::Destruct;

impl<T> Vector2<T> {
    /// Create a new [`Vector2`] from an array
    pub const fn from_array([x, y]: [T; 2]) -> Self
    where
        T: [const] Destruct,
    {
        Vector2::new(x, y)
    }
}

impl<T: [const] Destruct> const From<[T; 2]> for Vector2<T> {
    fn from(array: [T; 2]) -> Self {
        Vector2::from_array(array)
    }
}
