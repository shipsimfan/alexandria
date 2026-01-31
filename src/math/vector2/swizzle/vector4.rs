use crate::math::{Vector2, Vector4};
use std::marker::Destruct;

impl<T> Vector2<T> {
    /// Gets this [`Vector2`] as a [`Vector4`] with values `(x, x, x, x)`
    pub const fn xxxx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(self.x.clone(), self.x.clone(), self.x.clone(), self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(x, x, x, y)`
    pub const fn xxxy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(self.x.clone(), self.x.clone(), self.x.clone(), self.y)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(x, x, y, x)`
    pub const fn xxyx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(self.x.clone(), self.x.clone(), self.y.clone(), self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(x, x, y, y)`
    pub const fn xxyy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(self.x.clone(), self.x.clone(), self.y.clone(), self.y)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(x, y, x, x)`
    pub const fn xyxx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(self.x.clone(), self.y.clone(), self.x.clone(), self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(x, y, x, y)`
    pub const fn xyxy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(self.x.clone(), self.y.clone(), self.x.clone(), self.y)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(x, y, y, x)`
    pub const fn xyyx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(self.x.clone(), self.y.clone(), self.y.clone(), self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(x, y, y, y)`
    pub const fn xyyy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(self.x.clone(), self.y.clone(), self.y.clone(), self.y)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(y, x, x, x)`
    pub const fn yxxx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(self.y.clone(), self.x.clone(), self.x.clone(), self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(y, x, x, y)`
    pub const fn yxxy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(self.y.clone(), self.x.clone(), self.x.clone(), self.y)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(y, x, y, x)`
    pub const fn yxyx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(self.y.clone(), self.x.clone(), self.y.clone(), self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(y, x, y, y)`
    pub const fn yxyy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(self.y.clone(), self.x.clone(), self.y.clone(), self.y)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(y, y, x, x)`
    pub const fn yyxx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(self.y.clone(), self.y.clone(), self.x.clone(), self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(y, y, x, y)`
    pub const fn yyxy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(self.y.clone(), self.y.clone(), self.x.clone(), self.y)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(y, y, y, x)`
    pub const fn yyyx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(self.y.clone(), self.y.clone(), self.y.clone(), self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(y, y, y, y)`
    pub const fn yyyy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(self.y.clone(), self.y.clone(), self.y.clone(), self.y)
    }
}
