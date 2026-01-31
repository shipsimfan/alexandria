use crate::math::{Vector3, number::Acosh};

impl<T: Acosh> Vector3<T> {
    /// Computes inverse hyperbolic cosine of the contained values, in radians component-wise
    pub fn acosh(self) -> Self {
        self.map(Acosh::acosh)
    }
}
