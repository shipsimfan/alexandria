use crate::math::{
    Color4, ColorHsv, ColorSpace,
    number::{Floor, FromF32, Normalize, One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Mul, Sub},
};

impl<T: Zero + One, Space: ColorSpace<T>> ColorHsv<T, Space> {
    /// Convert this [`ColorHsv`] into an RGB [`Color4`]
    pub const fn into_rgba(self, a: T) -> Color4<T, Space>
    where
        T: [const] Sub<T, Output = T>
            + [const] Mul<T, Output = T>
            + [const] Normalize
            + [const] Floor
            + [const] PartialEq
            + [const] Clone
            + [const] FromF32
            + [const] Destruct,
    {
        self.into_rgb().with_alpha(a)
    }
}

impl<
    T: Zero
        + One
        + [const] Sub<T, Output = T>
        + [const] Mul<T, Output = T>
        + [const] Normalize
        + [const] Floor
        + [const] PartialEq
        + [const] Clone
        + [const] FromF32
        + [const] Destruct,
    Space: ColorSpace<T>,
> const Into<Color4<T, Space>> for ColorHsv<T, Space>
{
    fn into(self) -> Color4<T, Space> {
        self.into_rgba(T::NORMALIZED_ONE)
    }
}
