use crate::math::{Rect, number::FromF32};
use std::{
    marker::Destruct,
    ops::{Add, Mul},
};

impl<T> Rect<T> {
    /// Calculate the perimeter of this [`Rect`]
    pub const fn perimeter(self) -> T
    where
        T: [const] Add<Output = T> + [const] Mul<Output = T> + [const] FromF32 + [const] Destruct,
    {
        self.size.x * T::from_f32(2.0) + self.size.y * T::from_f32(2.0)
    }
}
