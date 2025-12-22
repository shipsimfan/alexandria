use crate::{Vector4, number::IsNaN};

impl<T> Vector4<T> {
    /// Are any of the contained values `NaN`?
    pub const fn is_nan(&self) -> bool
    where
        T: [const] IsNaN,
    {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan() || self.w.is_nan()
    }
}
