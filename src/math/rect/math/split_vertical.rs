use crate::math::{Rect, number::Clamp};
use std::{
    marker::Destruct,
    ops::{Add, Sub},
};

impl<T> Rect<T> {
    /// Splits this [`Rect`] vertically along `x` into two [`Rect`]s
    pub const fn split_vertical(self, x: T) -> (Rect<T>, Rect<T>)
    where
        T: [const] Add<Output = T>
            + [const] Sub<Output = T>
            + [const] Clamp<Bound = T>
            + [const] Clone
            + [const] Destruct,
    {
        let right = self.position.x.clone() + self.size.x;
        let x = x.clamp(self.position.x.clone(), right.clone());
        (
            Rect::from_xywh(
                self.position.x.clone(),
                self.position.y.clone(),
                x.clone() - self.position.x,
                self.size.y.clone(),
            ),
            Rect::from_xywh(x.clone(), self.position.y, right - x, self.size.y),
        )
    }
}
