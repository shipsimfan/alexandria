use crate::math::{Rect, Vector2, number::FromF32};
use std::{marker::Destruct, ops::Div};

impl<P, S> Rect<P, S> {
    /// Get the extents of this [`Rect`], which is half of its `size`
    pub const fn extents(self) -> Vector2<S>
    where
        P: [const] Destruct,
        S: [const] Div<Output = S> + [const] FromF32 + [const] Clone + [const] Destruct,
    {
        self.size / S::from_f32(2.0)
    }
}
