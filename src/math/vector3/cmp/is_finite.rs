use crate::math::{Vector3, number::IsFinite};

impl<T> Vector3<T> {
    /// Are all the contained values finite?
    pub const fn is_finite(&self) -> bool
    where
        T: [const] IsFinite,
    {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

impl<T> const IsFinite for Vector3<T>
where
    T: [const] IsFinite,
{
    fn is_finite(&self) -> bool {
        self.is_finite()
    }
}
