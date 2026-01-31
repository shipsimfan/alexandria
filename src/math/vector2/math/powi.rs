use crate::math::{Vector2, number::PowI};

impl<T: PowI> Vector2<T> {
    /// Raises the contained values to the contained values of another vector, component-wise
    pub fn powi_v(self, n: Vector2<u32>) -> Self {
        self.zip(n, PowI::powi)
    }

    /// Raises the contained values to a value, component-wise
    pub fn powi(self, n: u32) -> Self {
        self.powi_v(Vector2::splat(n))
    }
}

impl<T: PowI> PowI for Vector2<T> {
    fn powi(self, n: u32) -> Self {
        self.powi(n)
    }
}
