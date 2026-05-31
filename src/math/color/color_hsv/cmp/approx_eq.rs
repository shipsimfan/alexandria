use crate::math::{ColorHsv, ColorSpace, number::ApproxEq};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> ColorHsv<T, Space> {
    /// Is this color approximately equal to `other`?
    pub const fn approx_eq(self, other: ColorHsv<T, Space>, epsilon: T::Epsilon) -> bool
    where
        T: [const] ApproxEq + [const] Destruct,
        T::Epsilon: [const] Clone + [const] Destruct,
    {
        self.h.approx_eq(other.h, epsilon.clone())
            && self.s.approx_eq(other.s, epsilon.clone())
            && self.v.approx_eq(other.v, epsilon)
    }
}

impl<T: [const] ApproxEq + [const] Destruct, Space: ColorSpace<T>> const ApproxEq
    for ColorHsv<T, Space>
where
    T::Epsilon: [const] Clone + [const] Destruct,
{
    type Epsilon = T::Epsilon;

    fn approx_eq(self, other: Self, epsilon: T::Epsilon) -> bool {
        self.approx_eq(other, epsilon)
    }
}
