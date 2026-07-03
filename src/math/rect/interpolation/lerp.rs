use crate::math::{
    Rect,
    number::{FromSigned, One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Add, Mul, Sub},
};

impl<P, S> Rect<P, S> {
    /// Interpolates linearly between two [`Rect`]s, clamping `t` between 0 and 1
    pub const fn lerp(self, other: Rect<P, S>, t: P) -> Rect<P, S>
    where
        P: [const] Add<Output = P>
            + [const] Sub<Output = P>
            + [const] Mul<Output = P>
            + [const] PartialOrd
            + [const] Clone
            + [const] Destruct
            + Zero
            + One,
        S: [const] Add<Output = S>
            + [const] Sub<Output = S>
            + [const] Mul<Output = S>
            + [const] FromSigned<P>
            + [const] Clone
            + [const] Destruct,
    {
        self.lerp_unclamped(
            other,
            if t < P::ZERO {
                P::ZERO
            } else if t > P::ONE {
                P::ONE
            } else {
                t
            },
        )
    }

    /// Interpolates linearly between two [`Rect`]s, without clamping `t` between 0 and 1
    pub const fn lerp_unclamped(self, other: Rect<P, S>, t: P) -> Rect<P, S>
    where
        P: [const] Add<Output = P>
            + [const] Sub<Output = P>
            + [const] Mul<Output = P>
            + [const] Clone
            + [const] Destruct,
        S: [const] Add<Output = S>
            + [const] Sub<Output = S>
            + [const] Mul<Output = S>
            + [const] FromSigned<P>
            + [const] Clone
            + [const] Destruct,
    {
        Rect::new(
            self.position.lerp_unclamped(other.position, t.clone()),
            self.size.lerp_unclamped(other.size, S::from_signed(t)),
        )
    }
}
