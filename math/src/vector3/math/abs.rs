use crate::{Vector3, number::Abs};

impl<T: Abs> Vector3<T> {
    /// Calculate the component-wise aboslute value
    pub fn abs(self) -> Self {
        self.map(Abs::abs)
    }
}

impl<T: Abs> Abs for Vector3<T> {
    fn abs(self) -> Self {
        self.abs()
    }
}
