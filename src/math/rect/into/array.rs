use crate::math::Rect;
use std::marker::Destruct;

impl<T> Rect<T> {
    /// Convert this [`Rect`] into an array of `[x, y, width, height]`
    pub const fn into_array(self) -> [T; 4]
    where
        T: [const] Destruct,
    {
        [self.position.x, self.position.y, self.size.x, self.size.y]
    }
}

impl<T: [const] Destruct> const Into<[T; 4]> for Rect<T> {
    fn into(self) -> [T; 4] {
        self.into_array()
    }
}
