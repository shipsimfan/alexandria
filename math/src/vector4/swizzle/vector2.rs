use crate::{Vector2, Vector4};
use std::marker::Destruct;

impl<T> Vector4<T> {
    /// Gets this [`Vector4`] as a [`Vector2`] with values `(x, y)`
    pub const fn xy(self) -> Vector2<T>
    where
        T: [const] Destruct,
    {
        Vector2::new(self.x, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(x, z)`
    pub const fn xz(self) -> Vector2<T>
    where
        T: [const] Destruct,
    {
        Vector2::new(self.x, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(x, w)`
    pub const fn xw(self) -> Vector2<T>
    where
        T: [const] Destruct,
    {
        Vector2::new(self.x, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(y, x)`
    pub const fn yx(self) -> Vector2<T>
    where
        T: [const] Destruct,
    {
        Vector2::new(self.y, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(y, z)`
    pub const fn yz(self) -> Vector2<T>
    where
        T: [const] Destruct,
    {
        Vector2::new(self.y, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(y, w)`
    pub const fn yw(self) -> Vector2<T>
    where
        T: [const] Destruct,
    {
        Vector2::new(self.y, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(z, x)`
    pub const fn zx(self) -> Vector2<T>
    where
        T: [const] Destruct,
    {
        Vector2::new(self.z, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(z, y)`
    pub const fn zy(self) -> Vector2<T>
    where
        T: [const] Destruct,
    {
        Vector2::new(self.z, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(z, w)`
    pub const fn zw(self) -> Vector2<T>
    where
        T: [const] Destruct,
    {
        Vector2::new(self.z, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(w, x)`
    pub const fn wx(self) -> Vector2<T>
    where
        T: [const] Destruct,
    {
        Vector2::new(self.w, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(w, y)`
    pub const fn wy(self) -> Vector2<T>
    where
        T: [const] Destruct,
    {
        Vector2::new(self.w, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(w, z)`
    pub const fn wz(self) -> Vector2<T>
    where
        T: [const] Destruct,
    {
        Vector2::new(self.w, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(x, x)`
    pub const fn xx(self) -> Vector2<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector2::new(self.x.clone(), self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(y, y)`
    pub const fn yy(self) -> Vector2<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector2::new(self.y.clone(), self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(z, z)`
    pub const fn zz(self) -> Vector2<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector2::new(self.z.clone(), self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(w, w)`
    pub const fn ww(self) -> Vector2<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector2::new(self.w.clone(), self.w)
    }
}
