use crate::math::{Vector2, Vector3};
use std::marker::Destruct;

impl<T> Vector2<T> {
    /// Gets this [`Vector2`] as a [`Vector3`] with values `(x, x, x)`
    pub const fn xxx(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.x.clone(), self.x.clone(), self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector3`] with values `(x, x, y)`
    pub const fn xxy(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.x.clone(), self.x, self.y)
    }

    /// Gets this [`Vector2`] as a [`Vector3`] with values `(x, y, x)`
    pub const fn xyx(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.x.clone(), self.y, self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector3`] with values `(x, y, y)`
    pub const fn xyy(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.x, self.y.clone(), self.y)
    }

    /// Gets this [`Vector2`] as a [`Vector3`] with values `(y, x, x)`
    pub const fn yxx(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.y, self.x.clone(), self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector3`] with values `(y, x, y)`
    pub const fn yxy(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.y.clone(), self.x, self.y)
    }

    /// Gets this [`Vector2`] as a [`Vector3`] with values `(y, y, x)`
    pub const fn yyx(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.y.clone(), self.y, self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector3`] with values `(y, y, y)`
    pub const fn yyy(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.y.clone(), self.y.clone(), self.y)
    }
}
