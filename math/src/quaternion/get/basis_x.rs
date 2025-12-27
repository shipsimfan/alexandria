use crate::{
    Quaternion, Vector3,
    number::{FromF32, One},
};
use std::{
    marker::Destruct,
    ops::{Add, Mul, Sub},
};

impl<T: One> Quaternion<T> {
    /// Get the basis vector in the x-direction
    pub const fn basis_x(self) -> Vector3<T>
    where
        T: [const] Add<Output = T>
            + [const] Sub<Output = T>
            + [const] Mul<Output = T>
            + [const] FromF32
            + [const] Clone
            + [const] Destruct,
    {
        let two = T::from_f32(2.0);

        let xy = self.x.clone() * self.y.clone();
        let xz = self.x * self.z.clone();
        let yy = self.y.clone() * self.y.clone();
        let yw = self.y * self.w.clone();
        let zz = self.z.clone() * self.z.clone();
        let zw = self.z * self.w;

        Vector3::new(
            T::ONE - two.clone() * (yy + zz),
            two.clone() * (xy + zw),
            two * (xz - yw),
        )
    }

    /// Get the right vector of this quaternion
    pub const fn right(self) -> Vector3<T>
    where
        T: [const] Add<Output = T>
            + [const] Sub<Output = T>
            + [const] Mul<Output = T>
            + [const] FromF32
            + [const] Clone
            + [const] Destruct,
    {
        self.basis_x()
    }
}
