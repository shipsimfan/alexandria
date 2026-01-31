use crate::math::{Vector2, number::Cos};

impl<T: Cos> Vector2<T> {
    /// Computes cosine of the contained values, in radians component-wise
    pub fn cos(self) -> Self {
        self.map(Cos::cos)
    }
}
