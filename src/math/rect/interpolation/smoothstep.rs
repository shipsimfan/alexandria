use crate::math::{
    Rect,
    number::{FromSigned, One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Add, Mul, Sub},
};

impl<P, S> Rect<P, S> {
    /// Interpolates using the smoothstep algorithm between two vectors
    pub const fn smoothstep(self, other: Rect<P, S>, t: P) -> Rect<P, S>
    where
        P: [const] Add<Output = P>
            + [const] Sub<Output = P>
            + [const] Mul<Output = P>
            + [const] Clone
            + [const] PartialOrd
            + [const] Destruct
            + Zero
            + One,
        S: [const] Add<Output = S>
            + [const] Sub<Output = S>
            + [const] Mul<Output = S>
            + [const] FromSigned<P>
            + [const] Clone
            + [const] PartialOrd
            + [const] Destruct
            + Zero
            + One,
    {
        Rect::new(
            self.position.smoothstep(other.position, t.clone()),
            self.size.smoothstep(other.size, S::from_signed(t)),
        )
    }
}
