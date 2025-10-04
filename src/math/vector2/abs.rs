use crate::math::{number::Absolute, Vector2};

impl<T: Absolute> Vector2<T> {
    /// Gets the aboslute value of a [`Vector2`] component wise
    pub fn abs(self) -> Self {
        Vector2::new(self.x.abs(), self.y.abs())
    }
}
