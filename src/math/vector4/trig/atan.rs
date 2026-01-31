use crate::math::{Vector4, number::Atan};

impl<T: Atan> Vector4<T> {
    /// Computes inverse tangent of the contained values, in radians component-wise
    pub fn atan(self) -> Self {
        self.map(Atan::atan)
    }
}
