use crate::math::{Rect, number::IntoSigned};
use std::{
    marker::Destruct,
    ops::{Add, Sub},
};

impl<P, S> Rect<P, S> {
    /// Create a new [`Rect`] from this one with the giving padding
    pub const fn pad(self, left: P, right: P, top: P, bottom: P) -> Rect<P, S>
    where
        P: [const] Add<Output = P>
            + [const] Sub<Output = P>
            + [const] IntoSigned<S>
            + [const] Clone
            + [const] Destruct,
        S: [const] Sub<Output = S> + [const] Destruct,
    {
        Rect::from_xywh(
            self.position.x + left.clone(),
            self.position.y + top.clone(),
            self.size.x - (left - right).into_signed(),
            self.size.y - (top - bottom).into_signed(),
        )
    }
}
