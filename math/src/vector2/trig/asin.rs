use crate::{Vector2, number::Asin};

impl<T: Asin> Vector2<T> {
    /// Computes inverse sine of the contained values, in radians component-wise
    pub fn asin(self) -> Self {
        self.map(Asin::asin)
    }
}
