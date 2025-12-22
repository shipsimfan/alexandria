use crate::{Vector3, number::Atanh};

impl<T: Atanh> Vector3<T> {
    /// Computes inverse hyperbolic tangent of the contained values, in radians component-wise
    pub fn atanh(self) -> Self {
        self.map(Atanh::atanh)
    }
}
