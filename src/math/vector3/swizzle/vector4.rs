use crate::math::{Vector3, Vector4};
use std::marker::Destruct;

impl<T> Vector3<T> {
    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, x, x, x)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, x, x, y)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, x, x, z)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, x, y, x)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, x, y, y)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, x, y, z)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, x, z, x)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, x, z, y)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, x, z, z)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, y, x, x)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, y, x, y)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, y, x, z)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, y, y, x)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, y, y, y)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, y, y, z)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, y, z, x)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, y, z, y)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, y, z, z)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, z, x, x)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, z, x, y)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, z, x, z)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, z, y, x)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, z, y, y)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, z, y, z)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, z, z, x)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, z, z, y)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, z, z, z)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, x, x, x)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, x, x, y)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, x, x, z)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, x, y, x)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, x, y, y)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, x, y, z)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, x, z, x)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, x, z, y)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, x, z, z)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, y, x, x)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, y, x, y)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, y, x, z)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, y, y, x)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, y, y, y)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, y, y, z)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, y, z, x)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, y, z, y)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, y, z, z)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, z, x, x)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, z, x, y)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, z, x, z)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, z, y, x)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, z, y, y)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, z, y, z)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, z, z, x)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, z, z, y)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, z, z, z)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, x, x, x)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, x, x, y)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, x, x, z)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, x, y, x)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, x, y, y)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, x, y, z)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, x, z, x)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, x, z, y)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, x, z, z)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, y, x, x)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, y, x, y)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, y, x, z)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, y, y, x)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, y, y, y)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, y, y, z)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, y, z, x)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, y, z, y)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, y, z, z)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, z, x, x)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, z, x, y)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, z, x, z)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, z, y, x)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, z, y, y)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, z, y, z)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, z, z, x)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, z, z, y)`
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

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, z, z, z)`
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
}
