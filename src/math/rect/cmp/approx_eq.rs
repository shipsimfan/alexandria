use crate::math::{
    Rect,
    number::{ApproxEq, IntoSigned},
};
use std::marker::Destruct;

impl<P, S> Rect<P, S> {
    /// Is this color approximately equal to `other`?
    pub const fn approx_eq(self, other: Rect<P, S>, epsilon: S::Epsilon) -> bool
    where
        S: [const] ApproxEq + [const] Destruct,
        P: [const] ApproxEq + [const] Destruct,
        S::Epsilon: [const] Clone + [const] Destruct + [const] IntoSigned<P::Epsilon>,
        P::Epsilon: [const] Clone + [const] Destruct,
    {
        self.position
            .approx_eq(other.position, epsilon.clone().into_signed())
            && self.size.approx_eq(other.size, epsilon)
    }
}

impl<P, S> const ApproxEq for Rect<P, S>
where
    S: [const] ApproxEq + [const] Destruct,
    P: [const] ApproxEq + [const] Destruct,
    S::Epsilon: [const] Clone + [const] Destruct + [const] IntoSigned<P::Epsilon>,
    P::Epsilon: [const] Clone + [const] Destruct,
{
    type Epsilon = S::Epsilon;

    fn approx_eq(self, other: Self, epsilon: S::Epsilon) -> bool {
        self.approx_eq(other, epsilon)
    }
}
