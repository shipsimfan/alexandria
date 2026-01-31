use crate::math::{Vector3, number::Log};

impl<T: Log> Vector3<T> {
    /// Computes the logarithm of the contained values with respect to an arbitrary base,
    /// component-wise
    pub fn log(self, base: T::Base) -> Self
    where
        T::Base: Clone,
    {
        self.zip(Vector3::splat(base), Log::log)
    }
}

impl<T: Log> Log for Vector3<T>
where
    T::Base: Clone,
{
    type Base = T::Base;

    fn log(self, base: Self::Base) -> Self {
        self.log(base)
    }
}
