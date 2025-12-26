use crate::Quaternion;
use std::{
    marker::Destruct,
    ops::{Add, Mul, MulAssign, Sub},
};

impl<
    T: [const] Add<Output = T>
        + [const] Sub<Output = T>
        + [const] Mul<Output = T>
        + [const] Clone
        + [const] Destruct,
> const Mul for Quaternion<T>
{
    type Output = Quaternion<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Quaternion::new(
            self.w.clone() * rhs.x.clone()
                + self.x.clone() * rhs.w.clone()
                + self.y.clone() * rhs.z.clone()
                - self.z.clone() * rhs.y.clone(),
            self.w.clone() * rhs.y.clone() - self.x.clone() * rhs.z.clone()
                + self.y.clone() * rhs.w.clone().clone()
                + self.z.clone() * rhs.x.clone(),
            self.w.clone() * rhs.z.clone() + self.x.clone() * rhs.y.clone()
                - self.y.clone() * rhs.x.clone()
                + self.z.clone() * rhs.w.clone(),
            self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        )
    }
}

impl<
    T: [const] Add<Output = T>
        + [const] Sub<Output = T>
        + [const] Mul<Output = T>
        + [const] Clone
        + [const] Destruct,
> const MulAssign for Quaternion<T>
{
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}
