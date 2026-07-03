use crate::math::{Rect, Vector2, number::FromSigned};
use std::{marker::Destruct, ops::Sub};

impl<P, S> Rect<P, S> {
    /// Create a new [`Rect`] from a `min` and `max` point
    pub const fn from_min_max(min: Vector2<P>, max: Vector2<P>) -> Rect<P, S>
    where
        P: [const] Sub<Output = P> + [const] Clone + [const] Destruct,
        S: [const] FromSigned<P>,
    {
        Rect::new(min.clone(), Vector2::from_signed(max - min))
    }
}
