use crate::math::{Vector2, number::Acos};

impl<T: Acos> Vector2<T> {
    /// Computes inverse cosine of the contained values, in radians component-wise
    pub fn acos(self) -> Self {
        self.map(Acos::acos)
    }
}
