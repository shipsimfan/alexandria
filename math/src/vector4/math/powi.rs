use crate::{Vector4, number::PowI};

impl<T: PowI> Vector4<T> {
    /// Raises the contained values to the contained values of another vector, component-wise
    pub fn powi_v(self, n: Vector4<u32>) -> Self {
        self.zip(n, PowI::powi)
    }

    /// Raises the contained values to a value, component-wise
    pub fn powi(self, n: u32) -> Self {
        self.powi_v(Vector4::splat(n))
    }
}

impl<T: PowI> PowI for Vector4<T> {
    fn powi(self, n: u32) -> Self {
        self.powi(n)
    }
}
