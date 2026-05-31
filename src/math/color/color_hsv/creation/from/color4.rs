use crate::math::{
    Color3, Color4, ColorHsv, ColorSpace,
    number::{FromF32, Max, Min, One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Add, AddAssign, Div, Sub},
};

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
> const From<Color4<T, Space>> for ColorHsv<T, Space>
{
    fn from(value: Color4<T, Space>) -> Self {
        ColorHsv::from_rgb(Color3::new(value.r, value.g, value.b)).into()
    }
}
