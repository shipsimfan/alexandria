use crate::math::{Rect, Vector2};
use std::{marker::Destruct, ops::Sub};

impl<T> Rect<T> {
    /// Create a new [`Rect`] from a `min` and `max` point
    pub const fn from_min_max(min: Vector2<T>, max: Vector2<T>) -> Rect<T>
    where
        T: [const] Sub<Output = T> + [const] Clone + [const] Destruct,
    {
        Rect::new(min.clone(), max - min)
    }
}
