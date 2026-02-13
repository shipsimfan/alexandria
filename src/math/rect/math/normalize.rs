use crate::math::{Rect, number::Zero};
use std::{
    marker::Destruct,
    ops::{AddAssign, Neg},
};

impl<T: Zero> Rect<T> {
    /// Normalizes this [`Rect`] so that both width and height are >= 0
    pub const fn normalize(&mut self)
    where
        T: [const] Neg<Output = T>
            + [const] AddAssign
            + [const] PartialOrd
            + [const] Clone
            + [const] Destruct,
    {
        if self.size.x < T::ZERO {
            self.position.x += self.size.x.clone();
            self.size.x = -self.size.x.clone();
        }

        if self.size.y < T::ZERO {
            self.position.y += self.size.y.clone();
            self.size.y = -self.size.y.clone();
        }
    }
}
