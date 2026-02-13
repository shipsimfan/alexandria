use crate::math::{Rect, Vector2};
use std::marker::Destruct;

impl<T> Rect<T> {
    /// Create a new [`Rect`] from a tuple of vectors
    pub const fn from_vec_tuple((position, size): (Vector2<T>, Vector2<T>)) -> Rect<T>
    where
        T: [const] Destruct,
    {
        Rect::new(position, size)
    }
}

impl<T: [const] Destruct> const From<(Vector2<T>, Vector2<T>)> for Rect<T> {
    fn from(tuple: (Vector2<T>, Vector2<T>)) -> Self {
        Rect::from_vec_tuple(tuple)
    }
}
