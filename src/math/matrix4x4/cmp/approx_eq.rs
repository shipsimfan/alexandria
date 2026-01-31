use crate::math::{Matrix4x4, number::ApproxEq};
use std::marker::Destruct;

impl<T> Matrix4x4<T> {
    /// Is this matrix approximately equal to `other`?
    pub const fn approx_eq(self, other: Matrix4x4<T>, epsilon: T::Epsilon) -> bool
    where
        T: [const] ApproxEq + [const] Destruct,
        T::Epsilon: [const] Clone + [const] Destruct,
    {
        self.r0.approx_eq(other.r0, epsilon.clone())
            && self.r1.approx_eq(other.r1, epsilon.clone())
            && self.r2.approx_eq(other.r2, epsilon.clone())
            && self.r3.approx_eq(other.r3, epsilon)
    }
}

impl<T: [const] ApproxEq + [const] Destruct> const ApproxEq for Matrix4x4<T>
where
    T::Epsilon: [const] Clone + [const] Destruct,
{
    type Epsilon = T::Epsilon;

    fn approx_eq(self, other: Self, epsilon: T::Epsilon) -> bool {
        self.approx_eq(other, epsilon)
    }
}
