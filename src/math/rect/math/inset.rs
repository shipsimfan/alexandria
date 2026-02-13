use crate::math::Rect;
use std::{
    marker::Destruct,
    ops::{Add, Sub},
};

impl<T> Rect<T> {
    /// Create a new [`Rect`] from this one with the giving `inset`
    pub const fn inset(self, inset: T) -> Rect<T>
    where
        T: [const] Add<Output = T> + [const] Sub<Output = T> + [const] Clone + [const] Destruct,
    {
        self.pad(inset.clone(), inset.clone(), inset.clone(), inset)
    }
}
