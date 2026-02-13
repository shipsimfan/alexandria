use crate::math::{Rect, Vector2, number::FromF32};
use std::{
    marker::Destruct,
    ops::{Div, Mul, Sub},
};

impl<T> Rect<T> {
    /// Scale this [`Rect`] by `scale` uniformally along both axes from its center
    pub const fn uniform_scale_center(self, scale: T) -> Rect<T>
    where
        T: [const] Sub<Output = T>
            + [const] Mul<Output = T>
            + [const] Div<Output = T>
            + [const] FromF32
            + [const] Clone
            + [const] Destruct,
    {
        self.scale_center(Vector2::splat(scale))
    }
}
