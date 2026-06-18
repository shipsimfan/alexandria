use crate::math::Rect;
use std::{marker::Destruct, ops::Mul};

impl<P, S> Rect<P, S> {
    /// Calculate the area of this [`Rect`]
    pub const fn area(self) -> S
    where
        P: [const] Destruct,
        S: [const] Mul<Output = S> + [const] Destruct,
    {
        self.size.x * self.size.y
    }
}
