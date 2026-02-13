use crate::math::{Rect, Vector2};
use std::marker::Destruct;

impl<T> Rect<T> {
    /// Convert this [`Rect`] into an array of `[position, size]`
    pub const fn into_vec_array(self) -> [Vector2<T>; 2]
    where
        T: [const] Destruct,
    {
        [self.position, self.size]
    }
}

impl<T: [const] Destruct> const Into<[Vector2<T>; 2]> for Rect<T> {
    fn into(self) -> [Vector2<T>; 2] {
        self.into_vec_array()
    }
}
