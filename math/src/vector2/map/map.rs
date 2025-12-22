use crate::Vector2;
use std::marker::Destruct;

impl<T> Vector2<T> {
    /// Map the x-axis value of this vector using `f`
    pub const fn map_x<F: [const] FnOnce(T) -> T>(self, f: F) -> Self
    where
        T: [const] Destruct,
    {
        Vector2::new(f(self.x), self.y)
    }

    /// Map the y-axis value of this vector using `f`
    pub const fn map_y<F: [const] FnOnce(T) -> T>(self, f: F) -> Self
    where
        T: [const] Destruct,
    {
        Vector2::new(self.x, f(self.y))
    }

    /// Map all the elements of this vector component-wise using `f`
    pub const fn map<U, F: [const] FnMut(T) -> U + [const] Destruct>(self, mut f: F) -> Vector2<U>
    where
        T: [const] Destruct,
    {
        Vector2::new(f(self.x), f(self.y))
    }
}
