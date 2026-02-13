use crate::math::{Rect, Vector2};
use std::marker::Destruct;

impl<T> Rect<T> {
    /// Create a new [`Rect`] from an array of vectors
    pub const fn from_vec_array([position, size]: [Vector2<T>; 2]) -> Rect<T>
    where
        T: [const] Destruct,
    {
        Rect::from_position_size(position, size)
    }
}

impl<T: [const] Destruct> const From<[Vector2<T>; 2]> for Rect<T> {
    fn from(array: [Vector2<T>; 2]) -> Self {
        Rect::from_vec_array(array)
    }
}
