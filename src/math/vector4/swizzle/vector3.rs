use crate::math::{Vector3, Vector4};
use std::marker::Destruct;

impl<T> Vector4<T> {
    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, y, z)`
    pub const fn xyz(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.x, self.y, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, y, w)`
    pub const fn xyw(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.x, self.y, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, z, y)`
    pub const fn xzy(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.x, self.z, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, z, w)`
    pub const fn xzw(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.x, self.z, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, w, y)`
    pub const fn xwy(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.x, self.w, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, w, z)`
    pub const fn xwz(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.x, self.w, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, x, z)`
    pub const fn yxz(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.y, self.x, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, x, w)`
    pub const fn yxw(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.y, self.x, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, z, x)`
    pub const fn yzx(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.y, self.z, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, z, w)`
    pub const fn yzw(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.y, self.z, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, w, x)`
    pub const fn ywx(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.y, self.w, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, w, z)`
    pub const fn ywz(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.y, self.w, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, x, y)`
    pub const fn zxy(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.z, self.x, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, x, w)`
    pub const fn zxw(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.z, self.x, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, y, x)`
    pub const fn zyx(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.z, self.y, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, y, w)`
    pub const fn zyw(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.z, self.y, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, w, x)`
    pub const fn zwx(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.z, self.w, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, w, y)`
    pub const fn zwy(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.z, self.w, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, x, y)`
    pub const fn wxy(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.w, self.x, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, x, z)`
    pub const fn wxz(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.w, self.x, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, y, x)`
    pub const fn wyx(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.w, self.y, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, y, z)`
    pub const fn wyz(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.w, self.y, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, z, x)`
    pub const fn wzx(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.w, self.z, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, z, y)`
    pub const fn wzy(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.w, self.z, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, x, x)`
    pub const fn xxx(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.x.clone(), self.x.clone(), self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, x, y)`
    pub const fn xxy(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.x.clone(), self.x, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, x, z)`
    pub const fn xxz(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.x.clone(), self.x, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, x, w)`
    pub const fn xxw(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.x.clone(), self.x, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, y, x)`
    pub const fn xyx(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.x.clone(), self.y, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, y, y)`
    pub const fn xyy(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.x, self.y.clone(), self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, z, x)`
    pub const fn xzx(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.x.clone(), self.z, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, z, z)`
    pub const fn xzz(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.x, self.z.clone(), self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, w, x)`
    pub const fn xwx(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.x.clone(), self.w, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, w, w)`
    pub const fn xww(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.x, self.w.clone(), self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, x, x)`
    pub const fn yxx(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.y, self.x.clone(), self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, x, y)`
    pub const fn yxy(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.y.clone(), self.x, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, y, y)`
    pub const fn yyy(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.y.clone(), self.y.clone(), self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, y, x)`
    pub const fn yyx(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.y.clone(), self.y, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, y, z)`
    pub const fn yyz(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.y.clone(), self.y, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, y, w)`
    pub const fn yyw(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.y.clone(), self.y, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, z, y)`
    pub const fn yzy(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.y.clone(), self.z, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, z, z)`
    pub const fn yzz(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.y, self.z.clone(), self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, w, y)`
    pub const fn ywy(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.y.clone(), self.w, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, w, w)`
    pub const fn yww(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.y, self.w.clone(), self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, x, x)`
    pub const fn zxx(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.z, self.x.clone(), self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, x, z)`
    pub const fn zxz(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.z.clone(), self.x, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, y, z)`
    pub const fn zyz(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.z.clone(), self.y, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, y, y)`
    pub const fn zyy(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.z, self.y.clone(), self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, z, z)`
    pub const fn zzz(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.z.clone(), self.z.clone(), self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, z, x)`
    pub const fn zzx(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.z.clone(), self.z, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, z, y)`
    pub const fn zzy(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.z.clone(), self.z, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, z, w)`
    pub const fn zzw(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.z.clone(), self.z, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, w, z)`
    pub const fn zwz(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.z.clone(), self.w, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, w, w)`
    pub const fn zww(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.z, self.w.clone(), self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, x, x)`
    pub const fn wxx(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.w, self.x.clone(), self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, x, w)`
    pub const fn wxw(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.w.clone(), self.x, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, y, y)`
    pub const fn wyy(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.w, self.y.clone(), self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, y, w)`
    pub const fn wyw(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.w.clone(), self.y, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, z, z)`
    pub const fn wzz(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.w, self.z.clone(), self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, z, w)`
    pub const fn wzw(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.w.clone(), self.z, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, w, w)`
    pub const fn www(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.w.clone(), self.w.clone(), self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, w, x)`
    pub const fn wwx(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.w.clone(), self.w, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, w, y)`
    pub const fn wwy(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.w.clone(), self.w, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, w, z)`
    pub const fn wwz(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.w.clone(), self.w, self.z)
    }
}
