use crate::math::Rect;
use std::marker::Destruct;

impl<T> Rect<T> {
    /// Create a new [`Rect`] from an array of values
    pub const fn from_array([x, y, width, height]: [T; 4]) -> Rect<T>
    where
        T: [const] Destruct,
    {
        Rect::from_xywh(x, y, width, height)
    }
}

impl<T: [const] Destruct> const From<[T; 4]> for Rect<T> {
    fn from(array: [T; 4]) -> Self {
        Rect::from_array(array)
    }
}
