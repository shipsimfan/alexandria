use crate::math::Rect;
use std::{marker::Destruct, ops::Div};

impl<P, S> Rect<P, S> {
    /// Calculate the aspect ratio of this [`Rect`]
    pub const fn aspect(self) -> S
    where
        P: [const] Destruct,
        S: [const] Div<Output = S> + [const] Destruct,
    {
        self.size.x / self.size.y
    }
}
