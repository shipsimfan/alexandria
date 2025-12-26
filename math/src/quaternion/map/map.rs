use crate::Quaternion;
use std::marker::Destruct;

impl<T> Quaternion<T> {
    /// Map the x-component value of this quaternion using `f`
    pub const fn map_x<F: [const] FnOnce(T) -> T>(self, f: F) -> Self
    where
        T: [const] Destruct,
    {
        Quaternion::new(f(self.x), self.y, self.z, self.w)
    }

    /// Map the y-component value of this quaternion using `f`
    pub const fn map_y<F: [const] FnOnce(T) -> T>(self, f: F) -> Self
    where
        T: [const] Destruct,
    {
        Quaternion::new(self.x, f(self.y), self.z, self.w)
    }

    /// Map the z-component value of this quaternion using `f`
    pub const fn map_z<F: [const] FnOnce(T) -> T>(self, f: F) -> Self
    where
        T: [const] Destruct,
    {
        Quaternion::new(self.x, self.y, f(self.z), self.w)
    }

    /// Map the w-component value of this quaternion using `f`
    pub const fn map_w<F: [const] FnOnce(T) -> T>(self, f: F) -> Self
    where
        T: [const] Destruct,
    {
        Quaternion::new(self.x, self.y, self.z, f(self.w))
    }

    /// Map all the elements of this quaternion component-wise using `f`
    pub const fn map<U, F: [const] FnMut(T) -> U + [const] Destruct>(
        self,
        mut f: F,
    ) -> Quaternion<U>
    where
        T: [const] Destruct,
    {
        Quaternion::new(f(self.x), f(self.y), f(self.z), f(self.w))
    }

    /// Map the x, y, and z components elements of this quaternion component-wise using `f`
    pub const fn map_xyz<F: [const] FnMut(T) -> T + [const] Destruct>(self, mut f: F) -> Self
    where
        T: [const] Destruct,
    {
        Quaternion::new(f(self.x), f(self.y), f(self.z), self.w)
    }

    /// Map all the x, y, and z components elements of this quaternion component-wise using `f3`,
    /// and map the w component value with `fw`
    pub const fn map_xyz_w<
        U,
        F3: [const] FnMut(T) -> U + [const] Destruct,
        FW: [const] FnOnce(T) -> U,
    >(
        self,
        mut f3: F3,
        fw: FW,
    ) -> Quaternion<U>
    where
        T: [const] Destruct,
    {
        Quaternion::new(f3(self.x), f3(self.y), f3(self.z), fw(self.w))
    }
}
