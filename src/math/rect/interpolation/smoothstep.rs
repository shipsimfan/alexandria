use crate::math::{
    Rect,
    number::{One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Add, Mul, Sub},
};

impl<T> Rect<T> {
    /// Interpolates using the smoothstep algorithm between two vectors
    pub const fn smoothstep(self, other: Rect<T>, t: T) -> Rect<T>
    where
        T: [const] Add<Output = T>
            + [const] Sub<Output = T>
            + [const] Mul<Output = T>
            + [const] Clone
            + [const] PartialOrd
            + [const] Destruct
            + Zero
            + One,
    {
        Rect::new(
            self.position.smoothstep(other.position, t.clone()),
            self.size.smoothstep(other.size, t),
        )
    }
}
