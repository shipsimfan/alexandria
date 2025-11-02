use crate::math::{Vector2, Vector3};

impl<T> Vector3<T> {
    /// Gets this [`Vector3`] as a [`Vector2`] with values `(x, y)`
    pub fn xy(self) -> Vector2<T> {
        Vector2::new(self.x, self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector2`] with values `(x, z)`
    pub fn xz(self) -> Vector2<T> {
        Vector2::new(self.x, self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector2`] with values `(y, x)`
    pub fn yx(self) -> Vector2<T> {
        Vector2::new(self.y, self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector2`] with values `(y, z)`
    pub fn yz(self) -> Vector2<T> {
        Vector2::new(self.y, self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector2`] with values `(z, x)`
    pub fn zx(self) -> Vector2<T> {
        Vector2::new(self.z, self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector2`] with values `(z, y)`
    pub fn zy(self) -> Vector2<T> {
        Vector2::new(self.z, self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, y, z)`
    pub const fn xyz(self) -> Vector3<T> {
        self
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, z, y)`
    pub fn xzy(self) -> Vector3<T> {
        Vector3::new(self.x, self.z, self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, x, z)`
    pub fn yxz(self) -> Vector3<T> {
        Vector3::new(self.y, self.x, self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, z, x)`
    pub fn yzx(self) -> Vector3<T> {
        Vector3::new(self.y, self.z, self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, x, y)`
    pub fn zxy(self) -> Vector3<T> {
        Vector3::new(self.z, self.x, self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, y, x)`
    pub fn zyx(self) -> Vector3<T> {
        Vector3::new(self.z, self.y, self.x)
    }
}

impl<T: Clone> Vector3<T> {
    /// Gets this [`Vector3`] as a [`Vector2`] with values `(x, x)`
    pub fn xx(self) -> Vector2<T> {
        Vector2::new(self.x.clone(), self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector2`] with values `(y, y)`
    pub fn yy(self) -> Vector2<T> {
        Vector2::new(self.y.clone(), self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector2`] with values `(z, z)`
    pub fn zz(self) -> Vector2<T> {
        Vector2::new(self.z.clone(), self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, x, x)`
    pub fn xxx(self) -> Vector3<T> {
        Vector3::new(self.x.clone(), self.x.clone(), self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, x, y)`
    pub fn xxy(self) -> Vector3<T> {
        Vector3::new(self.x.clone(), self.x, self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, x, z)`
    pub fn xxz(self) -> Vector3<T> {
        Vector3::new(self.x.clone(), self.x, self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, y, x)`
    pub fn xyx(self) -> Vector3<T> {
        Vector3::new(self.x.clone(), self.y, self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, y, y)`
    pub fn xyy(self) -> Vector3<T> {
        Vector3::new(self.x, self.y.clone(), self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, y, z)`
    /// (alias of [`Self::xyz`])
    pub fn xyz_clone(self) -> Vector3<T> {
        // included only for symmetry; prefer `xyz()` which is `const` and clone-free
        Vector3::new(self.x, self.y, self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, z, x)`
    pub fn xzx(self) -> Vector3<T> {
        Vector3::new(self.x.clone(), self.z, self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, z, z)`
    pub fn xzz(self) -> Vector3<T> {
        Vector3::new(self.x, self.z.clone(), self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, x, x)`
    pub fn yxx(self) -> Vector3<T> {
        Vector3::new(self.y, self.x.clone(), self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, x, y)`
    pub fn yxy(self) -> Vector3<T> {
        Vector3::new(self.y.clone(), self.x, self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, x, z)`
    pub fn yxz_clone(self) -> Vector3<T> {
        // alias of `yxz()`; provided only for completeness
        Vector3::new(self.y, self.x, self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, y, x)`
    pub fn yyx(self) -> Vector3<T> {
        Vector3::new(self.y.clone(), self.y, self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, y, y)`
    pub fn yyy(self) -> Vector3<T> {
        Vector3::new(self.y.clone(), self.y.clone(), self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, y, z)`
    pub fn yyz(self) -> Vector3<T> {
        Vector3::new(self.y.clone(), self.y, self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, z, x)`
    pub fn yzx_clone(self) -> Vector3<T> {
        // alias of `yzx()`; provided only for completeness
        Vector3::new(self.y, self.z, self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, z, y)`
    pub fn yzy(self) -> Vector3<T> {
        Vector3::new(self.y.clone(), self.z, self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, z, z)`
    pub fn yzz(self) -> Vector3<T> {
        Vector3::new(self.y, self.z.clone(), self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, x, x)`
    pub fn zxx(self) -> Vector3<T> {
        Vector3::new(self.z, self.x.clone(), self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, x, y)`
    pub fn zxy_clone(self) -> Vector3<T> {
        // alias of `zxy()`; provided only for completeness
        Vector3::new(self.z, self.x, self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, x, z)`
    pub fn zxz(self) -> Vector3<T> {
        Vector3::new(self.z.clone(), self.x, self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, y, x)`
    pub fn zyx_clone(self) -> Vector3<T> {
        // alias of `zyx()`; provided only for completeness
        Vector3::new(self.z, self.y, self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, y, y)`
    pub fn zyy(self) -> Vector3<T> {
        Vector3::new(self.z, self.y.clone(), self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, y, z)`
    pub fn zyz(self) -> Vector3<T> {
        Vector3::new(self.z.clone(), self.y, self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, z, x)`
    pub fn zzx(self) -> Vector3<T> {
        Vector3::new(self.z.clone(), self.z, self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, z, y)`
    pub fn zzy(self) -> Vector3<T> {
        Vector3::new(self.z.clone(), self.z, self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, z, z)`
    pub fn zzz(self) -> Vector3<T> {
        Vector3::new(self.z.clone(), self.z.clone(), self.z)
    }
}
