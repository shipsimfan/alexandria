use crate::math::Quaternion;
use std::marker::Destruct;

impl<T> Quaternion<T> {
    /// Sets the x component value of this [`Quaternion`]
    pub const fn with_x(mut self, x: T) -> Self
    where
        T: [const] Destruct,
    {
        self.x = x;
        self
    }

    /// Sets the y component value of this [`Quaternion`]
    pub const fn with_y(mut self, y: T) -> Self
    where
        T: [const] Destruct,
    {
        self.y = y;
        self
    }

    /// Sets the z component value of this [`Quaternion`]
    pub const fn with_z(mut self, z: T) -> Self
    where
        T: [const] Destruct,
    {
        self.z = z;
        self
    }

    /// Sets the w component value of this [`Quaternion`]
    pub const fn with_w(mut self, w: T) -> Self
    where
        T: [const] Destruct,
    {
        self.w = w;
        self
    }
}
