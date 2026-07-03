use crate::math::{Rect, Vector2, number::IntoSigned};
use std::{marker::Destruct, ops::Add};

impl<P, S> Rect<P, S> {
    /// Get the position of the top-left corner of this [`Rect`]
    pub const fn top_left(self) -> Vector2<P>
    where
        P: [const] Destruct,
        S: [const] Destruct,
    {
        self.position
    }

    /// Get the position of the top-right corner of this [`Rect`]
    pub const fn top_right(self) -> Vector2<P>
    where
        P: [const] Add<Output = P> + [const] Destruct,
        S: [const] IntoSigned<P> + [const] Destruct,
    {
        Vector2::new(self.position.x + self.size.x.into_signed(), self.position.y)
    }

    /// Get the position of the bottom-left corner of this [`Rect`]
    pub const fn bottom_left(self) -> Vector2<P>
    where
        P: [const] Add<Output = P> + [const] Destruct,
        S: [const] IntoSigned<P> + [const] Destruct,
    {
        Vector2::new(self.position.x, self.position.y + self.size.y.into_signed())
    }

    /// Get the position of the bottom-right corner of this [`Rect`]
    pub const fn bottom_right(self) -> Vector2<P>
    where
        P: [const] Add<Output = P> + [const] Destruct,
        S: [const] IntoSigned<P> + [const] Destruct,
    {
        self.position + self.size.into_signed()
    }
}
