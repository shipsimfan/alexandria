use crate::math::{
    Rect,
    number::{One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Add, Mul, Sub},
};

impl<T> Rect<T> {
    /// Interpolates linearly between two [`Rect`]s, clamping `t` between 0 and 1
    pub const fn lerp(self, other: Rect<T>, t: T) -> Rect<T>
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
        self.lerp_unclamped(
            other,
            if t < T::ZERO {
                T::ZERO
            } else if t > T::ONE {
                T::ONE
            } else {
                t
            },
        )
    }

    /// Interpolates linearly between two [`Rect`]s, without clamping `t` between 0 and 1
    pub const fn lerp_unclamped(self, other: Rect<T>, t: T) -> Rect<T>
    where
        T: [const] Add<Output = T>
            + [const] Sub<Output = T>
            + [const] Mul<Output = T>
            + [const] Clone
            + [const] Destruct,
    {
        Rect::new(
            self.position.lerp_unclamped(other.position, t.clone()),
            self.size.lerp_unclamped(other.size, t),
        )
    }
}
