use crate::math::{Vector3, number::Sqrt};

impl<T: Sqrt> Vector3<T> {
    /// Calculate the component-wise square root
    pub fn sqrt(self) -> Self {
        self.map(Sqrt::sqrt)
    }
}

impl<T: Sqrt> Sqrt for Vector3<T> {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
