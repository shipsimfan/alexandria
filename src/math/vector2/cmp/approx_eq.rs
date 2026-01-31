use crate::math::{Vector2, number::ApproxEq};
use std::marker::Destruct;

impl<T> Vector2<T> {
    /// Is this color approximately equal to `other`?
    pub const fn approx_eq(self, other: Vector2<T>, epsilon: T::Epsilon) -> bool
    where
        T: [const] ApproxEq + [const] Destruct,
        T::Epsilon: [const] Clone + [const] Destruct,
    {
        self.x.approx_eq(other.x, epsilon.clone()) && self.y.approx_eq(other.y, epsilon)
    }
}

impl<T: [const] ApproxEq + [const] Destruct> const ApproxEq for Vector2<T>
where
    T::Epsilon: [const] Clone + [const] Destruct,
{
    type Epsilon = T::Epsilon;

    fn approx_eq(self, other: Self, epsilon: T::Epsilon) -> bool {
        self.approx_eq(other, epsilon)
    }
}
