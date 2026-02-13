use crate::math::{Rect, Vector2, number::FromF32};
use std::{
    marker::Destruct,
    ops::{Div, Sub},
};

impl<T> Rect<T> {
    /// Create a new [`Rect`] centered on `center` and with `size`
    pub const fn from_center_size(center: Vector2<T>, size: Vector2<T>) -> Rect<T>
    where
        T: [const] Sub<Output = T>
            + [const] Div<Output = T>
            + [const] Clone
            + [const] FromF32
            + [const] Destruct,
    {
        Rect::from_position_size(center - size.clone() / T::from_f32(2.0), size)
    }
}
