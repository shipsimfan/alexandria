use crate::math::Rect;
use std::{marker::Destruct, ops::Mul};

impl<T> Rect<T> {
    /// Calculate the area of this [`Rect`]
    pub const fn area(self) -> T
    where
        T: [const] Mul<Output = T> + [const] Destruct,
    {
        self.size.x * self.size.y
    }
}
