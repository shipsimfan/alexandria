use crate::math::{Matrix4x4, number::ApproxEq};
use std::marker::Destruct;

impl<T> Matrix4x4<T> {
    /// Is this matrix symmetric?
    pub const fn is_symmetric(self, epsilon: T::Epsilon) -> bool
    where
        T: [const] ApproxEq + [const] Destruct,
        T::Epsilon: [const] Clone + [const] Destruct,
    {
        self.r1.x.approx_eq(self.r0.y, epsilon.clone())
            && self.r2.x.approx_eq(self.r0.z, epsilon.clone())
            && self.r2.y.approx_eq(self.r1.z, epsilon.clone())
            && self.r3.x.approx_eq(self.r0.w, epsilon.clone())
            && self.r3.y.approx_eq(self.r1.w, epsilon.clone())
            && self.r3.z.approx_eq(self.r2.w, epsilon.clone())
    }
}
