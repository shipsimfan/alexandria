use crate::{Vector3, number::Sinh};

impl<T: Sinh> Vector3<T> {
    /// Computes hyperbolic sine of the contained values, in radians component-wise
    pub fn sinh(self) -> Self {
        self.map(Sinh::sinh)
    }
}
