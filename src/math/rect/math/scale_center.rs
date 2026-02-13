use crate::math::{Rect, Vector2, number::FromF32};
use std::{
    marker::Destruct,
    ops::{Div, Mul, Sub},
};

impl<T> Rect<T> {
    /// Scale this [`Rect`] by `scale` from its center
    pub const fn scale_center(self, scale: Vector2<T>) -> Rect<T>
    where
        T: [const] Sub<Output = T>
            + [const] Mul<Output = T>
            + [const] Div<Output = T>
            + [const] FromF32
            + [const] Clone
            + [const] Destruct,
    {
        let scaled_size = self.size.clone() * scale;
        let scale_diff = scaled_size.clone() - self.size;

        Rect::new(self.position - scale_diff / T::from_f32(2.0), scaled_size)
    }
}
