use crate::math::{Vector2, number::Sqrt};

impl<T: Sqrt> Vector2<T> {
    /// Calculate the component-wise square root
    pub fn sqrt(self) -> Self {
        self.map(Sqrt::sqrt)
    }
}

impl<T: Sqrt> Sqrt for Vector2<T> {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
