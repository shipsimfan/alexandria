use crate::math::{Vector4, number::Cos};

impl<T: Cos> Vector4<T> {
    /// Computes cosine of the contained values, in radians component-wise
    pub fn cos(self) -> Self {
        self.map(Cos::cos)
    }
}

impl<T: Cos> Cos for Vector4<T> {
    fn cos(self) -> Self {
        self.cos()
    }
}
