use crate::math::{Rect, Vector2, number::FromF32};
use std::{marker::Destruct, ops::Div};

impl<T> Rect<T> {
    /// Get the extents of this [`Rect`], which is half of its `size`
    pub const fn extents(self) -> Vector2<T>
    where
        T: [const] Div<Output = T> + [const] FromF32 + [const] Clone + [const] Destruct,
    {
        self.size / T::from_f32(2.0)
    }
}
