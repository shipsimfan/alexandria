use crate::{Vector4, number::Acosh};

impl<T: Acosh> Vector4<T> {
    /// Computes inverse hyperbolic cosine of the contained values, in radians component-wise
    pub fn acosh(self) -> Self {
        self.map(Acosh::acosh)
    }
}
