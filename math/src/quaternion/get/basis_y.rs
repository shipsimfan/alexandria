use crate::{
    Quaternion, Vector3,
    number::{FromF32, One},
};
use std::{
    marker::Destruct,
    ops::{Add, Mul, Sub},
};

impl<T: One> Quaternion<T> {
    /// Get the basis vector in the y-direction
    pub const fn basis_y(self) -> Vector3<T>
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
        let xy = self.x.clone() * self.y.clone();
        let xw = self.x * self.w.clone();
        let yz = self.y * self.z.clone();
        let zz = self.z.clone() * self.z.clone();
        let zw = self.z * self.w;

        Vector3::new(
            two.clone() * (xy.clone() - zw.clone()),
            T::ONE - two.clone() * (xx.clone() + zz),
            two.clone() * (yz + xw),
        )
    }

    /// Get the up vector of this quaternion
    pub const fn up(self) -> Vector3<T>
    where
        T: [const] Add<Output = T>
            + [const] Sub<Output = T>
            + [const] Mul<Output = T>
            + [const] FromF32
            + [const] Clone
            + [const] Destruct,
    {
        self.basis_y()
    }
}
