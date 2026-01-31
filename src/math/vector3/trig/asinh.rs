use crate::math::{Vector3, number::Asinh};

impl<T: Asinh> Vector3<T> {
    /// Computes inverse hyperbolic sine of the contained values, in radians component-wise
    pub fn asinh(self) -> Self {
        self.map(Asinh::asinh)
    }
}
