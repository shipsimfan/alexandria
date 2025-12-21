use crate::Vector4;
use std::marker::Destruct;

impl<T> Vector4<T> {
    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, z, w)`
    pub const fn xyzw(self) -> Vector4<T> {
        self
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, w, z)`
    pub const fn xywz(self) -> Vector4<T>
    where
        T: [const] Destruct,
    {
        Vector4::new(self.x, self.y, self.w, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, y, w)`
    pub const fn xzyw(self) -> Vector4<T>
    where
        T: [const] Destruct,
    {
        Vector4::new(self.x, self.z, self.y, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, w, y)`
    pub const fn xzwy(self) -> Vector4<T>
    where
        T: [const] Destruct,
    {
        Vector4::new(self.x, self.z, self.w, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, y, z)`
    pub const fn xwyz(self) -> Vector4<T>
    where
        T: [const] Destruct,
    {
        Vector4::new(self.x, self.w, self.y, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, z, y)`
    pub const fn xwzy(self) -> Vector4<T>
    where
        T: [const] Destruct,
    {
        Vector4::new(self.x, self.w, self.z, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, z, w)`
    pub const fn yxzw(self) -> Vector4<T>
    where
        T: [const] Destruct,
    {
        Vector4::new(self.y, self.x, self.z, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, w, z)`
    pub const fn yxwz(self) -> Vector4<T>
    where
        T: [const] Destruct,
    {
        Vector4::new(self.y, self.x, self.w, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, x, w)`
    pub const fn yzxw(self) -> Vector4<T>
    where
        T: [const] Destruct,
    {
        Vector4::new(self.y, self.z, self.x, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, w, x)`
    pub const fn yzwx(self) -> Vector4<T>
    where
        T: [const] Destruct,
    {
        Vector4::new(self.y, self.z, self.w, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, x, z)`
    pub const fn ywxz(self) -> Vector4<T>
    where
        T: [const] Destruct,
    {
        Vector4::new(self.y, self.w, self.x, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, z, x)`
    pub const fn ywzx(self) -> Vector4<T>
    where
        T: [const] Destruct,
    {
        Vector4::new(self.y, self.w, self.z, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, y, w)`
    pub const fn zxyw(self) -> Vector4<T>
    where
        T: [const] Destruct,
    {
        Vector4::new(self.z, self.x, self.y, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, w, y)`
    pub const fn zxwy(self) -> Vector4<T>
    where
        T: [const] Destruct,
    {
        Vector4::new(self.z, self.x, self.w, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, x, w)`
    pub const fn zyxw(self) -> Vector4<T>
    where
        T: [const] Destruct,
    {
        Vector4::new(self.z, self.y, self.x, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, w, x)`
    pub const fn zywx(self) -> Vector4<T>
    where
        T: [const] Destruct,
    {
        Vector4::new(self.z, self.y, self.w, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, x, y)`
    pub const fn zwxy(self) -> Vector4<T>
    where
        T: [const] Destruct,
    {
        Vector4::new(self.z, self.w, self.x, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, y, x)`
    pub const fn zwyx(self) -> Vector4<T>
    where
        T: [const] Destruct,
    {
        Vector4::new(self.z, self.w, self.y, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, y, z)`
    pub const fn wxyz(self) -> Vector4<T>
    where
        T: [const] Destruct,
    {
        Vector4::new(self.w, self.x, self.y, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, z, y)`
    pub const fn wxzy(self) -> Vector4<T>
    where
        T: [const] Destruct,
    {
        Vector4::new(self.w, self.x, self.z, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, x, z)`
    pub const fn wyxz(self) -> Vector4<T>
    where
        T: [const] Destruct,
    {
        Vector4::new(self.w, self.y, self.x, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, z, x)`
    pub const fn wyzx(self) -> Vector4<T>
    where
        T: [const] Destruct,
    {
        Vector4::new(self.w, self.y, self.z, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, x, y)`
    pub const fn wzxy(self) -> Vector4<T>
    where
        T: [const] Destruct,
    {
        Vector4::new(self.w, self.z, self.x, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, y, x)`
    pub const fn wzyx(self) -> Vector4<T>
    where
        T: [const] Destruct,
    {
        Vector4::new(self.w, self.z, self.y, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, x, x)`
    pub const fn xxxx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, x, y)`
    pub const fn xxxy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, x, z)`
    pub const fn xxxz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, x, w)`
    pub const fn xxxw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.x.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, y, x)`
    pub const fn xxyx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, y, y)`
    pub const fn xxyy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, y, z)`
    pub const fn xxyz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, y, w)`
    pub const fn xxyw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.y.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, z, x)`
    pub const fn xxzx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, z, y)`
    pub const fn xxzy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, z, z)`
    pub const fn xxzz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, z, w)`
    pub const fn xxzw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, w, x)`
    pub const fn xxwx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.w.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, w, y)`
    pub const fn xxwy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.w.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, w, z)`
    pub const fn xxwz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.w.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, w, w)`
    pub const fn xxww(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, x, x)`
    pub const fn xyxx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, x, y)`
    pub const fn xyxy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, x, z)`
    pub const fn xyxz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, x, w)`
    pub const fn xyxw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.x.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, y, x)`
    pub const fn xyyx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, y, y)`
    pub const fn xyyy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, y, z)`
    pub const fn xyyz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, y, w)`
    pub const fn xyyw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.y.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, z, x)`
    pub const fn xyzx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, z, y)`
    pub const fn xyzy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, z, z)`
    pub const fn xyzz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, w, x)`
    pub const fn xywx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.w.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, w, y)`
    pub const fn xywy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.w.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, w, w)`
    pub const fn xyww(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, x, x)`
    pub const fn xzxx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, x, y)`
    pub const fn xzxy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, x, z)`
    pub const fn xzxz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, x, w)`
    pub const fn xzxw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.x.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, y, x)`
    pub const fn xzyx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, y, y)`
    pub const fn xzyy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, y, z)`
    pub const fn xzyz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, z, x)`
    pub const fn xzzx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, z, y)`
    pub const fn xzzy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, z, z)`
    pub const fn xzzz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, z, w)`
    pub const fn xzzw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, w, x)`
    pub const fn xzwx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.w.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, w, z)`
    pub const fn xzwz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.w.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, w, w)`
    pub const fn xzww(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, x, x)`
    pub const fn xwxx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.w.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, x, y)`
    pub const fn xwxy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.w.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, x, z)`
    pub const fn xwxz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.w.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, x, w)`
    pub const fn xwxw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.w.clone(),
            self.x.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, y, x)`
    pub const fn xwyx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.w.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, y, y)`
    pub const fn xwyy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.w.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, y, w)`
    pub const fn xwyw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.w.clone(),
            self.y.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, z, x)`
    pub const fn xwzx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.w.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, z, z)`
    pub const fn xwzz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.w.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, z, w)`
    pub const fn xwzw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.w.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, w, x)`
    pub const fn xwwx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.w.clone(),
            self.w.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, w, y)`
    pub const fn xwwy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.w.clone(),
            self.w.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, w, z)`
    pub const fn xwwz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.w.clone(),
            self.w.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, w, w)`
    pub const fn xwww(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.x.clone(),
            self.w.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, x, x)`
    pub const fn yxxx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, x, y)`
    pub const fn yxxy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, x, z)`
    pub const fn yxxz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, x, w)`
    pub const fn yxxw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.x.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, y, x)`
    pub const fn yxyx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, y, y)`
    pub const fn yxyy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, y, z)`
    pub const fn yxyz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, y, w)`
    pub const fn yxyw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.y.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, z, x)`
    pub const fn yxzx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, z, y)`
    pub const fn yxzy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, z, z)`
    pub const fn yxzz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, w, x)`
    pub const fn yxwx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.w.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, w, y)`
    pub const fn yxwy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.w.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, w, w)`
    pub const fn yxww(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, x, x)`
    pub const fn yyxx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, x, y)`
    pub const fn yyxy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, x, z)`
    pub const fn yyxz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, x, w)`
    pub const fn yyxw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.x.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, y, x)`
    pub const fn yyyx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, y, y)`
    pub const fn yyyy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, y, z)`
    pub const fn yyyz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, y, w)`
    pub const fn yyyw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.y.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, z, x)`
    pub const fn yyzx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, z, y)`
    pub const fn yyzy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, z, z)`
    pub const fn yyzz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, z, w)`
    pub const fn yyzw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, w, x)`
    pub const fn yywx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.w.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, w, y)`
    pub const fn yywy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.w.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, w, z)`
    pub const fn yywz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.w.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, w, w)`
    pub const fn yyww(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, x, x)`
    pub const fn yzxx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, x, y)`
    pub const fn yzxy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, x, z)`
    pub const fn yzxz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, y, x)`
    pub const fn yzyx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, y, y)`
    pub const fn yzyy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, y, z)`
    pub const fn yzyz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, y, w)`
    pub const fn yzyw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.y.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, z, x)`
    pub const fn yzzx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, z, y)`
    pub const fn yzzy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, z, z)`
    pub const fn yzzz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, z, w)`
    pub const fn yzzw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, w, y)`
    pub const fn yzwy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.w.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, w, z)`
    pub const fn yzwz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.w.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, w, w)`
    pub const fn yzww(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, x, x)`
    pub const fn ywxx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.w.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, x, y)`
    pub const fn ywxy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.w.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, x, w)`
    pub const fn ywxw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.w.clone(),
            self.x.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, y, x)`
    pub const fn ywyx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.w.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, y, y)`
    pub const fn ywyy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.w.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, y, z)`
    pub const fn ywyz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.w.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, y, w)`
    pub const fn ywyw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.w.clone(),
            self.y.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, z, y)`
    pub const fn ywzy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.w.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, z, z)`
    pub const fn ywzz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.w.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, z, w)`
    pub const fn ywzw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.w.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, w, x)`
    pub const fn ywwx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.w.clone(),
            self.w.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, w, y)`
    pub const fn ywwy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.w.clone(),
            self.w.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, w, z)`
    pub const fn ywwz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.w.clone(),
            self.w.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, w, w)`
    pub const fn ywww(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.y.clone(),
            self.w.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, x, x)`
    pub const fn zxxx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, x, y)`
    pub const fn zxxy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, x, z)`
    pub const fn zxxz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, x, w)`
    pub const fn zxxw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.x.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, y, x)`
    pub const fn zxyx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, y, y)`
    pub const fn zxyy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, y, z)`
    pub const fn zxyz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, z, x)`
    pub const fn zxzx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, z, y)`
    pub const fn zxzy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, z, z)`
    pub const fn zxzz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, z, w)`
    pub const fn zxzw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, w, x)`
    pub const fn zxwx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.w.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, w, z)`
    pub const fn zxwz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.w.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, w, w)`
    pub const fn zxww(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, x, x)`
    pub const fn zyxx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, x, y)`
    pub const fn zyxy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, x, z)`
    pub const fn zyxz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, y, x)`
    pub const fn zyyx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, y, y)`
    pub const fn zyyy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, y, z)`
    pub const fn zyyz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, y, w)`
    pub const fn zyyw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.y.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, z, x)`
    pub const fn zyzx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, z, y)`
    pub const fn zyzy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, z, z)`
    pub const fn zyzz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, z, w)`
    pub const fn zyzw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, w, y)`
    pub const fn zywy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.w.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, w, z)`
    pub const fn zywz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.w.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, w, w)`
    pub const fn zyww(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, z, x, x)`
    pub const fn zzxx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, z, x, y)`
    pub const fn zzxy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, z, x, z)`
    pub const fn zzxz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, z, x, w)`
    pub const fn zzxw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.x.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, z, y, x)`
    pub const fn zzyx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, z, y, y)`
    pub const fn zzyy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, z, y, z)`
    pub const fn zzyz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, z, y, w)`
    pub const fn zzyw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.y.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, z, z, x)`
    pub const fn zzzx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, z, z, y)`
    pub const fn zzzy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, z, z, z)`
    pub const fn zzzz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, z, z, w)`
    pub const fn zzzw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, x, x)`
    pub const fn zwxx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.w.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, x, z)`
    pub const fn zwxz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.w.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, x, w)`
    pub const fn zwxw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.w.clone(),
            self.x.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, y, y)`
    pub const fn zwyy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.w.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, y, z)`
    pub const fn zwyz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.w.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, y, w)`
    pub const fn zwyw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.w.clone(),
            self.y.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, z, x)`
    pub const fn zwzx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.w.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, z, y)`
    pub const fn zwzy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.w.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, z, z)`
    pub const fn zwzz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.w.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, z, w)`
    pub const fn zwzw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.w.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, w, x)`
    pub const fn zwwx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.w.clone(),
            self.w.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, w, y)`
    pub const fn zwwy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.w.clone(),
            self.w.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, w, z)`
    pub const fn zwwz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.w.clone(),
            self.w.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, w, w)`
    pub const fn zwww(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.z.clone(),
            self.w.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, x, x)`
    pub const fn wxxx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.x.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, x, y)`
    pub const fn wxxy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.x.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, x, z)`
    pub const fn wxxz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.x.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, x, w)`
    pub const fn wxxw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.x.clone(),
            self.x.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, y, x)`
    pub const fn wxyx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.x.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, y, y)`
    pub const fn wxyy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.x.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, y, w)`
    pub const fn wxyw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.x.clone(),
            self.y.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, z, x)`
    pub const fn wxzx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.x.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, z, z)`
    pub const fn wxzz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.x.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, z, w)`
    pub const fn wxzw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.x.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, w, x)`
    pub const fn wxwx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.x.clone(),
            self.w.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, w, y)`
    pub const fn wxwy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.x.clone(),
            self.w.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, w, z)`
    pub const fn wxwz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.x.clone(),
            self.w.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, w, w)`
    pub const fn wxww(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.x.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, x, x)`
    pub const fn wyxx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.y.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, x, y)`
    pub const fn wyxy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.y.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, x, w)`
    pub const fn wyxw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.y.clone(),
            self.x.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, y, x)`
    pub const fn wyyx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.y.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, y, y)`
    pub const fn wyyy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.y.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, y, z)`
    pub const fn wyyz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.y.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, y, w)`
    pub const fn wyyw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.y.clone(),
            self.y.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, z, y)`
    pub const fn wyzy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.y.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, z, z)`
    pub const fn wyzz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.y.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, z, w)`
    pub const fn wyzw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.y.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, w, x)`
    pub const fn wywx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.y.clone(),
            self.w.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, w, y)`
    pub const fn wywy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.y.clone(),
            self.w.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, w, z)`
    pub const fn wywz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.y.clone(),
            self.w.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, w, w)`
    pub const fn wyww(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.y.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, x, x)`
    pub const fn wzxx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.z.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, x, z)`
    pub const fn wzxz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.z.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, x, w)`
    pub const fn wzxw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.z.clone(),
            self.x.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, y, y)`
    pub const fn wzyy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.z.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, y, z)`
    pub const fn wzyz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.z.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, y, w)`
    pub const fn wzyw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.z.clone(),
            self.y.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, z, x)`
    pub const fn wzzx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.z.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, z, y)`
    pub const fn wzzy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.z.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, z, z)`
    pub const fn wzzz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.z.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, z, w)`
    pub const fn wzzw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.z.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, w, x)`
    pub const fn wzwx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.z.clone(),
            self.w.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, w, y)`
    pub const fn wzwy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.z.clone(),
            self.w.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, w, z)`
    pub const fn wzwz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.z.clone(),
            self.w.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, w, w)`
    pub const fn wzww(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.z.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, x, x)`
    pub const fn wwxx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, x, y)`
    pub const fn wwxy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, x, z)`
    pub const fn wwxz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, x, w)`
    pub const fn wwxw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.x.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, y, x)`
    pub const fn wwyx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, y, y)`
    pub const fn wwyy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, y, z)`
    pub const fn wwyz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, y, w)`
    pub const fn wwyw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.y.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, z, x)`
    pub const fn wwzx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, z, y)`
    pub const fn wwzy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, z, z)`
    pub const fn wwzz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, z, w)`
    pub const fn wwzw(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, w, x)`
    pub const fn wwwx(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.w.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, w, y)`
    pub const fn wwwy(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.w.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, w, z)`
    pub const fn wwwz(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.w.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, w, w)`
    pub const fn wwww(self) -> Vector4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }
}
