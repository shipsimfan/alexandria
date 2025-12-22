use crate::{Vector3, number::IsNaN};

impl<T> Vector3<T> {
    /// Are any of the contained values `NaN`?
    pub const fn is_nan(&self) -> bool
    where
        T: [const] IsNaN,
    {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }
}
