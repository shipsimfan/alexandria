use crate::{Vector4, number::Sqrt};

impl<T: Sqrt> Vector4<T> {
    /// Calculate the component-wise square root
    pub fn sqrt(self) -> Self {
        self.map(Sqrt::sqrt)
    }
}

impl<T: Sqrt> Sqrt for Vector4<T> {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
