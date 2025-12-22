use crate::{Vector3, number::Tan};

impl<T: Tan> Vector3<T> {
    /// Computes tangent of the contained values, in radians component-wise
    pub fn tan(self) -> Self {
        self.map(Tan::tan)
    }
}
