use crate::math::{Vector2, number::IsNaN};

impl<T> Vector2<T> {
    /// Are any of the contained values `NaN`?
    pub const fn is_nan(&self) -> bool
    where
        T: [const] IsNaN,
    {
        self.x.is_nan() || self.y.is_nan()
    }
}
