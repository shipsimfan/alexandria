use crate::{Vector4, number::ApproxEq};
use std::marker::Destruct;

impl<T> Vector4<T> {
    /// Is this vector approximately equal to `other`?
    pub const fn approx_eq(self, other: Vector4<T>, epsilon: T::Epsilon) -> bool
    where
        T: [const] ApproxEq + [const] Destruct,
        T::Epsilon: [const] Clone + [const] Destruct,
    {
        self.x.approx_eq(other.x, epsilon.clone())
            && self.y.approx_eq(other.y, epsilon.clone())
            && self.z.approx_eq(other.z, epsilon.clone())
            && self.w.approx_eq(other.w, epsilon)
    }
}

impl<T: [const] ApproxEq + [const] Destruct> const ApproxEq for Vector4<T>
where
    T::Epsilon: [const] Clone + [const] Destruct,
{
    type Epsilon = T::Epsilon;

    fn approx_eq(self, other: Self, epsilon: T::Epsilon) -> bool {
        self.approx_eq(other, epsilon)
    }
}
