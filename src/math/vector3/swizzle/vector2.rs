use crate::math::{Vector2, Vector3};
use std::marker::Destruct;

impl<T> Vector3<T> {
    /// Gets this [`Vector3`] as a [`Vector2`] with values `(x, y)`
    pub const fn xy(self) -> Vector2<T>
    where
        T: [const] Destruct,
    {
        Vector2::new(self.x, self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector2`] with values `(x, z)`
    pub const fn xz(self) -> Vector2<T>
    where
        T: [const] Destruct,
    {
        Vector2::new(self.x, self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector2`] with values `(y, x)`
    pub const fn yx(self) -> Vector2<T>
    where
        T: [const] Destruct,
    {
        Vector2::new(self.y, self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector2`] with values `(y, z)`
    pub const fn yz(self) -> Vector2<T>
    where
        T: [const] Destruct,
    {
        Vector2::new(self.y, self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector2`] with values `(z, x)`
    pub const fn zx(self) -> Vector2<T>
    where
        T: [const] Destruct,
    {
        Vector2::new(self.z, self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector2`] with values `(z, y)`
    pub const fn zy(self) -> Vector2<T>
    where
        T: [const] Destruct,
    {
        Vector2::new(self.z, self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector2`] with values `(x, x)`
    pub const fn xx(self) -> Vector2<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector2::new(self.x.clone(), self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector2`] with values `(y, y)`
    pub const fn yy(self) -> Vector2<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector2::new(self.y.clone(), self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector2`] with values `(z, z)`
    pub const fn zz(self) -> Vector2<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector2::new(self.z.clone(), self.z)
    }
}
