use crate::math::{Rect, number::IntoSigned};
use std::{
    marker::Destruct,
    ops::{Add, Sub},
};

impl<P, S> Rect<P, S> {
    /// Create a new [`Rect`] from this one with the giving `inset`
    pub const fn inset(self, inset: P) -> Rect<P, S>
    where
        P: [const] Add<Output = P>
            + [const] Sub<Output = P>
            + [const] IntoSigned<S>
            + [const] Clone
            + [const] Destruct,
        S: [const] Sub<Output = S> + [const] Destruct,
    {
        self.pad(inset.clone(), inset.clone(), inset.clone(), inset)
    }
}
