use crate::math::{Vector3, number::PowF};

impl<T: PowF> Vector3<T> {
    /// Raises the contained values to the contained values of another vector, component-wise
    pub fn powf_v(self, n: Vector3<T::Pow>) -> Self {
        self.zip(n, PowF::powf)
    }

    /// Raises the contained values to a value, component-wise
    pub fn powf(self, n: T::Pow) -> Self
    where
        T::Pow: Clone,
    {
        self.powf_v(Vector3::splat(n))
    }
}

impl<T: PowF> PowF for Vector3<T>
where
    T::Pow: Clone,
{
    type Pow = T::Pow;

    fn powf(self, n: Self::Pow) -> Self {
        self.powf(n)
    }
}
