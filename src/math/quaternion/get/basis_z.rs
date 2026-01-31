use crate::math::{
    Quaternion, Vector3,
    number::{FromF32, One},
};
use std::{
    marker::Destruct,
    ops::{Add, Mul, Sub},
};

impl<T: One> Quaternion<T> {
    /// Get the basis vector in the z-direction
    pub const fn basis_z(self) -> Vector3<T>
    where
        T: [const] Add<Output = T>
            + [const] Sub<Output = T>
            + [const] Mul<Output = T>
            + [const] FromF32
            + [const] Clone
            + [const] Destruct,
    {
        let two = T::from_f32(2.0);

        let xx = self.x.clone() * self.x.clone();
        let xz = self.x.clone() * self.z.clone();
        let xw = self.x * self.w.clone();
        let yy = self.y.clone() * self.y.clone();
        let yz = self.y.clone() * self.z;
        let yw = self.y * self.w;

        Vector3::new(
            two.clone() * (xz.clone() + yw.clone()),
            two.clone() * (yz.clone() - xw.clone()),
            T::ONE - two * (xx + yy),
        )
    }

    /// Get the forward vector of this quaternion
    pub const fn forward(self) -> Vector3<T>
    where
        T: [const] Add<Output = T>
            + [const] Sub<Output = T>
            + [const] Mul<Output = T>
            + [const] FromF32
            + [const] Clone
            + [const] Destruct,
    {
        self.basis_z()
    }
}
