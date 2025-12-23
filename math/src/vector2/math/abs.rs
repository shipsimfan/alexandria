use crate::{Vector2, number::Abs};

impl<T: Abs> Vector2<T> {
    /// Calculate the component-wise aboslute value
    pub fn abs(self) -> Vector2<T> {
        self.map(Abs::abs)
    }
}

impl<T: Abs> Abs for Vector2<T> {
    fn abs(self) -> Self {
        self.abs()
    }
}
