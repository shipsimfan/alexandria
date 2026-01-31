use crate::math::Vector2;
use std::marker::Destruct;

impl<T> Vector2<T> {
    /// Gets this [`Vector2`] as a [`Vector2`] with values `(x, y)`
    pub const fn xy(self) -> Vector2<T>
    where
        T: [const] Destruct,
    {
        self
    }

    /// Gets this [`Vector2`] as a [`Vector2`] with values `(y, x)`
    pub const fn yx(self) -> Vector2<T>
    where
        T: [const] Destruct,
    {
        Vector2::new(self.y, self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector2`] with values `(x, x)`
    pub const fn xx(self) -> Vector2<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector2::new(self.x.clone(), self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector2`] with values `(y, y)`
    pub const fn yy(self) -> Vector2<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector2::new(self.y.clone(), self.y)
    }
}
