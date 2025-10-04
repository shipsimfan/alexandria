use crate::math::Vector2;

impl<T> Vector2<T> {
    /// Gets this [`Vector2`] as a [`Vector2`] with values `(x, y)`
    pub const fn xy(self) -> Vector2<T> {
        self
    }

    /// Gets this [`Vector2`] as a [`Vector2`] with values `(y, x)`
    pub fn yx(self) -> Vector2<T> {
        Vector2::new(self.y, self.x)
    }
}
