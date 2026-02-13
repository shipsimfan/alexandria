use crate::math::{Rect, number::ApproxEq};
use std::marker::Destruct;

impl<T> Rect<T> {
    /// Is this color approximately equal to `other`?
    pub const fn approx_eq(self, other: Rect<T>, epsilon: T::Epsilon) -> bool
    where
        T: [const] ApproxEq + [const] Destruct,
        T::Epsilon: [const] Clone + [const] Destruct,
    {
        self.position.approx_eq(other.position, epsilon.clone())
            && self.size.approx_eq(other.size, epsilon)
    }
}

impl<T: [const] ApproxEq + [const] Destruct> const ApproxEq for Rect<T>
where
    T::Epsilon: [const] Clone + [const] Destruct,
{
    type Epsilon = T::Epsilon;

    fn approx_eq(self, other: Self, epsilon: T::Epsilon) -> bool {
        self.approx_eq(other, epsilon)
    }
}
