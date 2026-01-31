use crate::math::Vector4;
use std::marker::Destruct;

impl<T> Vector4<T> {
    /// Map the x-axis value of this vector using `f`
    pub const fn map_x<F: [const] FnOnce(T) -> T>(self, f: F) -> Self
    where
        T: [const] Destruct,
    {
        Vector4::new(f(self.x), self.y, self.z, self.w)
    }

    /// Map the y-axis value of this vector using `f`
    pub const fn map_y<F: [const] FnOnce(T) -> T>(self, f: F) -> Self
    where
        T: [const] Destruct,
    {
        Vector4::new(self.x, f(self.y), self.z, self.w)
    }

    /// Map the z-axis value of this vector using `f`
    pub const fn map_z<F: [const] FnOnce(T) -> T>(self, f: F) -> Self
    where
        T: [const] Destruct,
    {
        Vector4::new(self.x, self.y, f(self.z), self.w)
    }

    /// Map the w-axis value of this vector using `f`
    pub const fn map_w<F: [const] FnOnce(T) -> T>(self, f: F) -> Self
    where
        T: [const] Destruct,
    {
        Vector4::new(self.x, self.y, self.z, f(self.w))
    }

    /// Map all the elements of this vector component-wise using `f`
    pub const fn map<U, F: [const] FnMut(T) -> U + [const] Destruct>(self, mut f: F) -> Vector4<U>
    where
        T: [const] Destruct,
    {
        Vector4::new(f(self.x), f(self.y), f(self.z), f(self.w))
    }

    /// Map the x, y, and z axis elements of this vector component-wise using `f`
    pub const fn map_xyz<F: [const] FnMut(T) -> T + [const] Destruct>(self, mut f: F) -> Self
    where
        T: [const] Destruct,
    {
        Vector4::new(f(self.x), f(self.y), f(self.z), self.w)
    }

    /// Map all the x, y, and z axis elements of this vector component-wise using `f3`, and map the
    /// w-axis value with `fw`
    pub const fn map_xyz_w<
        U,
        F3: [const] FnMut(T) -> U + [const] Destruct,
        FW: [const] FnOnce(T) -> U,
    >(
        self,
        mut f3: F3,
        fw: FW,
    ) -> Vector4<U>
    where
        T: [const] Destruct,
    {
        Vector4::new(f3(self.x), f3(self.y), f3(self.z), fw(self.w))
    }
}
