use crate::Vector4;
use std::marker::Destruct;

impl<T> Vector4<T> {
    /// Sets the x-axis value of this [`Vector4`]
    pub const fn with_x(mut self, x: T) -> Self
    where
        T: [const] Destruct,
    {
        self.x = x;
        self
    }

    /// Sets the y-axis value of this [`Vector4`]
    pub const fn with_y(mut self, y: T) -> Self
    where
        T: [const] Destruct,
    {
        self.y = y;
        self
    }

    /// Sets the z-axis value of this [`Vector4`]
    pub const fn with_z(mut self, z: T) -> Self
    where
        T: [const] Destruct,
    {
        self.z = z;
        self
    }

    /// Sets the w-axis value of this [`Vector4`]
    pub const fn with_w(mut self, w: T) -> Self
    where
        T: [const] Destruct,
    {
        self.w = w;
        self
    }
}
