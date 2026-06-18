use crate::math::{Vector3, number::Atan};

impl<T: Atan> Vector3<T> {
    /// Computes inverse tangent of the contained values, in radians component-wise
    pub fn atan(self) -> Self {
        self.map(Atan::atan)
    }
}

impl<T: Atan> Atan for Vector3<T> {
    fn atan(self) -> Self {
        self.atan()
    }
}
