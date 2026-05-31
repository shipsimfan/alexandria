use crate::math::{ColorHsva, ColorSpace, number::ApproxEq};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> ColorHsva<T, Space> {
    /// Is this color approximately equal to `other`?
    pub const fn approx_eq(self, other: ColorHsva<T, Space>, epsilon: T::Epsilon) -> bool
    where
        T: [const] ApproxEq + [const] Destruct,
        T::Epsilon: [const] Clone + [const] Destruct,
    {
        self.h.approx_eq(other.h, epsilon.clone())
            && self.s.approx_eq(other.s, epsilon.clone())
            && self.v.approx_eq(other.v, epsilon.clone())
            && self.a.approx_eq(other.a, epsilon)
    }
}

impl<T: [const] ApproxEq + [const] Destruct, Space: ColorSpace<T>> const ApproxEq
    for ColorHsva<T, Space>
where
    T::Epsilon: [const] Clone + [const] Destruct,
{
    type Epsilon = T::Epsilon;

    fn approx_eq(self, other: Self, epsilon: T::Epsilon) -> bool {
        self.approx_eq(other, epsilon)
    }
}
