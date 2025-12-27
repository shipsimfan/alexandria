use crate::{Matrix3x3, number::ApproxEq};
use std::marker::Destruct;

impl<T> Matrix3x3<T> {
    /// Is this matrix approximately equal to `other`?
    pub const fn approx_eq(self, other: Matrix3x3<T>, epsilon: T::Epsilon) -> bool
    where
        T: [const] ApproxEq + [const] Destruct,
        T::Epsilon: [const] Clone + [const] Destruct,
    {
        self.r0.approx_eq(other.r0, epsilon.clone())
            && self.r1.approx_eq(other.r1, epsilon.clone())
            && self.r2.approx_eq(other.r2, epsilon)
    }
}

impl<T: [const] ApproxEq + [const] Destruct> const ApproxEq for Matrix3x3<T>
where
    T::Epsilon: [const] Clone + [const] Destruct,
{
    type Epsilon = T::Epsilon;

    fn approx_eq(self, other: Self, epsilon: T::Epsilon) -> bool {
        self.approx_eq(other, epsilon)
    }
}
