use crate::math::{Vector3, number::Cosh};

impl<T: Cosh> Vector3<T> {
    /// Computes hyperbolic cosine of the contained values, in radians component-wise
    pub fn cosh(self) -> Self {
        self.map(Cosh::cosh)
    }
}
