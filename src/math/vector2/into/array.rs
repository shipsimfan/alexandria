use crate::math::Vector2;
use std::marker::Destruct;

impl<T> Vector2<T> {
    /// Convert the elements of this [`Vector2`] into an array
    pub const fn into_array(self) -> [T; 2]
    where
        T: [const] Destruct,
    {
        [self.x, self.y]
    }
}

impl<T: [const] Destruct> const Into<[T; 2]> for Vector2<T> {
    fn into(self) -> [T; 2] {
        self.into_array()
    }
}
