use crate::math::Vector3;
use std::marker::Destruct;

impl<T> Vector3<T> {
    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, y, z)`
    pub const fn xyz(self) -> Vector3<T> {
        self
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, z, y)`
    pub const fn xzy(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.x, self.z, self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, x, z)`
    pub const fn yxz(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.y, self.x, self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, z, x)`
    pub const fn yzx(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.y, self.z, self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, x, y)`
    pub const fn zxy(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.z, self.x, self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, y, x)`
    pub const fn zyx(self) -> Vector3<T>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.z, self.y, self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, x, x)`
    pub const fn xxx(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.x.clone(), self.x.clone(), self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, x, y)`
    pub const fn xxy(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.x.clone(), self.x, self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, x, z)`
    pub const fn xxz(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.x.clone(), self.x, self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, y, x)`
    pub const fn xyx(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.x.clone(), self.y, self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, y, y)`
    pub const fn xyy(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.x, self.y.clone(), self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, z, x)`
    pub const fn xzx(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.x.clone(), self.z, self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, z, z)`
    pub const fn xzz(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.x, self.z.clone(), self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, x, x)`
    pub const fn yxx(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.y, self.x.clone(), self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, x, y)`
    pub const fn yxy(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.y.clone(), self.x, self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, y, x)`
    pub const fn yyx(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.y.clone(), self.y, self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, y, y)`
    pub const fn yyy(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.y.clone(), self.y.clone(), self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, y, z)`
    pub const fn yyz(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.y.clone(), self.y, self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, z, y)`
    pub const fn yzy(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.y.clone(), self.z, self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, z, z)`
    pub const fn yzz(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.y, self.z.clone(), self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, x, x)`
    pub const fn zxx(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.z, self.x.clone(), self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, x, z)`
    pub const fn zxz(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.z.clone(), self.x, self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, y, y)`
    pub const fn zyy(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.z, self.y.clone(), self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, y, z)`
    pub const fn zyz(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.z.clone(), self.y, self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, z, x)`
    pub const fn zzx(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.z.clone(), self.z, self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, z, y)`
    pub const fn zzy(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.z.clone(), self.z, self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, z, z)`
    pub const fn zzz(self) -> Vector3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Vector3::new(self.z.clone(), self.z.clone(), self.z)
    }
}
