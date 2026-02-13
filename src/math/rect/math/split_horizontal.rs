use crate::math::{Rect, number::Clamp};
use std::{
    marker::Destruct,
    ops::{Add, Sub},
};

impl<T> Rect<T> {
    /// Splits this [`Rect`] horizontally along `y` into two [`Rect`]s
    pub const fn split_horizontal(self, y: T) -> (Rect<T>, Rect<T>)
    where
        T: [const] Add<Output = T>
            + [const] Sub<Output = T>
            + [const] Clamp<Bound = T>
            + [const] Clone
            + [const] Destruct,
    {
        let bottom = self.position.y.clone() + self.size.y;
        let y = y.clamp(self.position.y.clone(), bottom.clone());
        (
            Rect::from_xywh(
                self.position.x.clone(),
                self.position.y.clone(),
                self.size.x.clone(),
                y.clone() - self.position.y,
            ),
            Rect::from_xywh(self.position.x, y.clone(), self.size.x, bottom - y),
        )
    }
}
