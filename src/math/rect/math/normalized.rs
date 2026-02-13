use crate::math::{Rect, number::Zero};
use std::{
    marker::Destruct,
    ops::{AddAssign, Neg},
};

impl<T: Zero> Rect<T> {
    /// Normalizes this [`Rect`] so that both width and height are >= 0
    pub const fn normalized(mut self) -> Rect<T>
    where
        T: [const] Neg<Output = T>
            + [const] AddAssign
            + [const] PartialOrd
            + [const] Clone
            + [const] Destruct,
    {
        self.normalize();
        self
    }
}
