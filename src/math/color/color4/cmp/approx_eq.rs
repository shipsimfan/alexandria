use crate::math::{Color4, ColorSpace, number::ApproxEq};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Is this color approximately equal to `other`?
    pub const fn approx_eq(self, other: Color4<T, Space>, epsilon: T::Epsilon) -> bool
    where
        T: [const] ApproxEq + [const] Destruct,
        T::Epsilon: [const] Clone + [const] Destruct,
    {
        self.r.approx_eq(other.r, epsilon.clone())
            && self.g.approx_eq(other.g, epsilon.clone())
            && self.b.approx_eq(other.b, epsilon.clone())
            && self.a.approx_eq(other.a, epsilon)
    }
}

impl<T: [const] ApproxEq + [const] Destruct, Space: ColorSpace<T>> const ApproxEq
    for Color4<T, Space>
where
    T::Epsilon: [const] Clone + [const] Destruct,
{
    type Epsilon = T::Epsilon;

    fn approx_eq(self, other: Self, epsilon: T::Epsilon) -> bool {
        self.approx_eq(other, epsilon)
    }
}
