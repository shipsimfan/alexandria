use crate::math::{
    Color3, Color4, ColorHsva, ColorSpace,
    number::{FromF32, Max, Min, One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Add, AddAssign, Div, Sub},
};

impl<T, Space: ColorSpace<T>> ColorHsva<T, Space> {
    /// Create a new [`ColorHsva`] from an RGB [`Color3`]
    pub const fn from_rgba(rgba: Color4<T, Space>) -> ColorHsva<T, Space>
    where
        T: Zero
            + One
            + [const] Add<T, Output = T>
            + [const] AddAssign<T>
            + [const] Sub<T, Output = T>
            + [const] Div<T, Output = T>
            + [const] Max
            + [const] Min
            + [const] PartialEq
            + [const] PartialOrd
            + [const] Clone
            + [const] Destruct
            + [const] FromF32,
    {
        ColorHsva::from_rgb(Color3::new(rgba.r, rgba.g, rgba.b), rgba.a)
    }
}

const impl<
    T: Zero
        + One
        + [const] Add<T, Output = T>
        + [const] AddAssign<T>
        + [const] Sub<T, Output = T>
        + [const] Div<T, Output = T>
        + [const] Max
        + [const] Min
        + [const] PartialEq
        + [const] PartialOrd
        + [const] Clone
        + [const] Destruct
        + [const] FromF32,
    Space: ColorSpace<T>,
> From<Color4<T, Space>> for ColorHsva<T, Space>
{
    fn from(value: Color4<T, Space>) -> Self {
        ColorHsva::from_rgba(value)
    }
}
