use crate::{Color4, ColorSpace};
use alexandria_math::number::ApproxEq;
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Is this color approximately equal to `other`?
    pub const fn approx_eq(self, other: Self, epsilon: T) -> bool
    where
        T: [const] ApproxEq + [const] Clone + [const] Destruct,
    {
        self.r.approx_eq(other.r, epsilon.clone())
            && self.g.approx_eq(other.g, epsilon.clone())
            && self.b.approx_eq(other.b, epsilon.clone())
            && self.a.approx_eq(other.a, epsilon)
    }
}
