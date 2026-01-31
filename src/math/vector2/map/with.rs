use crate::math::Vector2;
use std::marker::Destruct;

impl<T> Vector2<T> {
    /// Sets the x-axis value of this [`Vector2`]
    pub const fn with_x(mut self, x: T) -> Vector2<T>
    where
        T: [const] Destruct,
    {
        self.x = x;
        self
    }

    /// Sets the y-axis value of this [`Vector2`]
    pub const fn with_y(mut self, y: T) -> Vector2<T>
    where
        T: [const] Destruct,
    {
        self.y = y;
        self
    }
}
