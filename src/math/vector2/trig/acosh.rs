use crate::math::{Vector2, number::Acosh};

impl<T: Acosh> Vector2<T> {
    /// Computes inverse hyperbolic cosine of the contained values, in radians component-wise
    pub fn acosh(self) -> Self {
        self.map(Acosh::acosh)
    }
}
