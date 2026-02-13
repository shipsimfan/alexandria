use crate::math::{Rect, Vector2};
use std::{
    marker::Destruct,
    ops::{Add, Mul},
};

impl<T> Rect<T> {
    /// Get the position of the anchor located `pivot` percent along this [`Rect`]
    pub const fn anchor(self, pivot: Vector2<T>) -> Vector2<T>
    where
        T: [const] Add<Output = T> + [const] Mul<Output = T> + [const] Destruct,
    {
        self.position + self.size * pivot
    }
}
