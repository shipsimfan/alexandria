use crate::{Vector4, number::Abs};

impl<T: Abs> Vector4<T> {
    /// Calculate the component-wise aboslute value
    pub fn abs(self) -> Self {
        self.map(Abs::abs)
    }
}

impl<T: Abs> Abs for Vector4<T> {
    fn abs(self) -> Self {
        self.abs()
    }
}
