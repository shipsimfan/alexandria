use crate::math::{Vector3, number::Cos};

impl<T: Cos> Vector3<T> {
    /// Computes cosine of the contained values, in radians component-wise
    pub fn cos(self) -> Self {
        self.map(Cos::cos)
    }
}

impl<T: Cos> Cos for Vector3<T> {
    fn cos(self) -> Self {
        self.cos()
    }
}
