use crate::math::{Rect, Vector2, number::FromF32};
use std::{
    marker::Destruct,
    ops::{Add, Div},
};

impl<T> Rect<T> {
    /// Get the center of this [`Rect`]
    pub const fn center(self) -> Vector2<T>
    where
        T: [const] Add<Output = T>
            + [const] Div<Output = T>
            + [const] FromF32
            + [const] Clone
            + [const] Destruct,
    {
        self.position.clone() + self.extents()
    }
}
