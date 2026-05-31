use crate::math::{
    Color3, ColorHsva, ColorSpace,
    number::{Floor, FromF32, Normalize, One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Mul, Sub},
};

impl<T: Zero + One, Space: ColorSpace<T>> ColorHsva<T, Space> {
    /// Convert this [`ColorHsva`] into an RGB [`Color3`]
    pub const fn into_rgb(self) -> Color3<T, Space>
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
        self.hsv().into_rgb()
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
> const Into<Color3<T, Space>> for ColorHsva<T, Space>
{
    fn into(self) -> Color3<T, Space> {
        self.into_rgb()
    }
}
