use crate::math::{Rect, number::FromF32};
use std::{
    marker::Destruct,
    ops::{Add, Mul},
};

impl<P, S> Rect<P, S> {
    /// Calculate the perimeter of this [`Rect`]
    pub const fn perimeter(self) -> S
    where
        P: [const] Destruct,
        S: [const] Add<Output = S> + [const] Mul<Output = S> + [const] FromF32 + [const] Destruct,
    {
        self.size.x * S::from_f32(2.0) + self.size.y * S::from_f32(2.0)
    }
}
