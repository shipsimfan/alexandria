use crate::math::{Rect, number::IntoSigned};
use std::{marker::Destruct, ops::Add};

impl<P, S> Rect<P, S> {
    /// Get the position of the left side of this [`Rect`]
    pub const fn left(self) -> P
    where
        P: [const] Destruct,
        S: [const] Destruct,
    {
        self.position.x
    }

    /// Get the position of the right side of this [`Rect`]
    pub const fn right(self) -> P
    where
        P: [const] Add<Output = P> + [const] Destruct,
        S: [const] IntoSigned<P> + [const] Destruct,
    {
        self.position.x + self.size.x.into_signed()
    }

    /// Get the position of the top side of this [`Rect`]
    pub const fn top(self) -> P
    where
        P: [const] Destruct,
        S: [const] Destruct,
    {
        self.position.y
    }

    /// Get the position of the bottom side of this [`Rect`]
    pub const fn bottom(self) -> P
    where
        P: [const] Add<Output = P> + [const] Destruct,
        S: [const] IntoSigned<P> + [const] Destruct,
    {
        self.position.y + self.size.y.into_signed()
    }
}
