use crate::math::{Vector3, number::Atanh};

impl<T: Atanh> Vector3<T> {
    /// Computes inverse hyperbolic tangent of the contained values, in radians component-wise
    pub fn atanh(self) -> Self {
        self.map(Atanh::atanh)
    }
}

impl<T: Atanh> Atanh for Vector3<T> {
    fn atanh(self) -> Self {
        self.atanh()
    }
}
