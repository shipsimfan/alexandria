use crate::math::{
    Color4, ColorHsv, ColorHsva, ColorSpace,
    number::{Floor, FromF32, Normalize, One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Mul, Sub},
};

impl<T: Zero + One, Space: ColorSpace<T>> ColorHsva<T, Space> {
    /// Convert this [`ColorHsva`] into an RGB [`Color4`]
    pub const fn into_rgba(self) -> Color4<T, Space>
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
        ColorHsv::new(self.h, self.s, self.v).into_rgba(self.a)
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
> const Into<Color4<T, Space>> for ColorHsva<T, Space>
{
    fn into(self) -> Color4<T, Space> {
        self.into_rgba()
    }
}
