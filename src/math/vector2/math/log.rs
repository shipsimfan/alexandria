use crate::math::{Vector2, number::Log};

impl<T: Log> Vector2<T> {
    /// Computes the logarithm of the contained values with respect to an arbitrary base,
    /// component-wise
    pub fn log(self, base: T::Base) -> Vector2<T>
    where
        T::Base: Clone,
    {
        self.zip(Vector2::splat(base), Log::log)
    }
}

impl<T: Log> Log for Vector2<T>
where
    T::Base: Clone,
{
    type Base = T::Base;

    fn log(self, base: Self::Base) -> Self {
        self.log(base)
    }
}
