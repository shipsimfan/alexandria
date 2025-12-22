use crate::Vector3;
use std::marker::Destruct;

impl<T> Vector3<T> {
    /// Sets the x-axis value of this [`Vector3`]
    pub const fn with_x(mut self, x: T) -> Self
    where
        T: [const] Destruct,
    {
        self.x = x;
        self
    }

    /// Sets the y-axis value of this [`Vector3`]
    pub const fn with_y(mut self, y: T) -> Self
    where
        T: [const] Destruct,
    {
        self.y = y;
        self
    }

    /// Sets the z-axis value of this [`Vector3`]
    pub const fn with_z(mut self, z: T) -> Self
    where
        T: [const] Destruct,
    {
        self.z = z;
        self
    }
}
