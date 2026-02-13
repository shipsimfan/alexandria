use crate::math::Rect;
use std::{marker::Destruct, ops::Div};

impl<T> Rect<T> {
    /// Calculate the aspect ratio of this [`Rect`]
    pub const fn aspect(self) -> T
    where
        T: [const] Div<Output = T> + [const] Destruct,
    {
        self.size.x / self.size.y
    }
}
