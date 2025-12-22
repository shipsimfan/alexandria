use crate::{Vector2, number::Sin};

impl<T: Sin> Vector2<T> {
    /// Computes sine of the contained values, in radians component-wise
    pub fn sin(self) -> Self {
        self.map(Sin::sin)
    }
}
