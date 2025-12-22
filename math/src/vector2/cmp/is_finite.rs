use crate::{Vector2, number::IsFinite};

impl<T> Vector2<T> {
    /// Are all the contained values finite?
    pub const fn is_finite(&self) -> bool
    where
        T: [const] IsFinite,
    {
        self.x.is_finite() && self.y.is_finite()
    }
}
