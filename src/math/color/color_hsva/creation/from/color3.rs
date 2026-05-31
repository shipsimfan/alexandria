use crate::math::{
    Color3, ColorHsv, ColorHsva, ColorSpace,
    number::{FromF32, Max, Min, One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Add, AddAssign, Div, Sub},
};

impl<T, Space: ColorSpace<T>> ColorHsva<T, Space> {
    /// Create a new [`ColorHsva`] from an RGB [`Color3`]
    pub const fn from_rgb(rgb: Color3<T, Space>, a: T) -> ColorHsva<T, Space>
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
        ColorHsv::from_rgb(rgb).with_alpha(a)
    }
}

impl<
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
> const From<Color3<T, Space>> for ColorHsva<T, Space>
{
    fn from(value: Color3<T, Space>) -> Self {
        ColorHsva::from_rgb(value, T::NORMALIZED_ONE)
    }
}
