use crate::math::Vector3;
use std::marker::Destruct;

impl<T> Vector3<T> {
    /// Map the x-axis value of this vector using `f`
    pub const fn map_x<F: [const] FnOnce(T) -> T>(self, f: F) -> Self
    where
        T: [const] Destruct,
    {
        Vector3::new(f(self.x), self.y, self.z)
    }

    /// Map the y-axis value of this vector using `f`
    pub const fn map_y<F: [const] FnOnce(T) -> T>(self, f: F) -> Self
    where
        T: [const] Destruct,
    {
        Vector3::new(self.x, f(self.y), self.z)
    }

    /// Map the z-axis value of this vector using `f`
    pub const fn map_z<F: [const] FnOnce(T) -> T>(self, f: F) -> Self
    where
        T: [const] Destruct,
    {
        Vector3::new(self.x, self.y, f(self.z))
    }

    /// Map all the elements of this vector component-wise using `f`
    pub const fn map<U, F: [const] FnMut(T) -> U + [const] Destruct>(self, mut f: F) -> Vector3<U>
    where
        T: [const] Destruct,
    {
        Vector3::new(f(self.x), f(self.y), f(self.z))
    }
}
