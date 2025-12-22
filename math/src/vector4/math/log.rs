use crate::{Vector4, number::Log};

impl<T: Log> Vector4<T> {
    /// Computes the logarithm of the contained values with respect to an arbitrary base,
    /// component-wise
    pub fn log(self, base: T::Base) -> Self
    where
        T::Base: Clone,
    {
        self.zip(Vector4::splat(base), Log::log)
    }
}

impl<T: Log> Log for Vector4<T>
where
    T::Base: Clone,
{
    type Base = T::Base;

    fn log(self, base: Self::Base) -> Self {
        self.log(base)
    }
}
