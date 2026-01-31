use crate::math::Vector2;
use std::marker::Destruct;

impl<T> Vector2<T> {
    /// Create a new [`Vector2`] from a tuple
    pub const fn from_tuple((x, y): (T, T)) -> Vector2<T>
    where
        T: [const] Destruct,
    {
        Vector2::new(x, y)
    }
}

impl<T: [const] Destruct> const From<(T, T)> for Vector2<T> {
    fn from(tuple: (T, T)) -> Self {
        Vector2::from_tuple(tuple)
    }
}
