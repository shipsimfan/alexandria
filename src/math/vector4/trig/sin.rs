use crate::math::{Vector4, number::Sin};

impl<T: Sin> Vector4<T> {
    /// Computes sine of the contained values, in radians component-wise
    pub fn sin(self) -> Self {
        self.map(Sin::sin)
    }
}
