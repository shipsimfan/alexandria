use crate::math::{Vector4, number::PowF};

impl<T: PowF> Vector4<T> {
    /// Raises the contained values to the contained values of another vector, component-wise
    pub fn powf_v(self, n: Vector4<T::Pow>) -> Self {
        self.zip(n, PowF::powf)
    }

    /// Raises the contained values to a value, component-wise
    pub fn powf(self, n: T::Pow) -> Self
    where
        T::Pow: Clone,
    {
        self.powf_v(Vector4::splat(n))
    }
}

impl<T: PowF> PowF for Vector4<T>
where
    T::Pow: Clone,
{
    type Pow = T::Pow;

    fn powf(self, n: Self::Pow) -> Self {
        self.powf(n)
    }
}
