use crate::math::{ColorHsva, ColorSpace, number::IntoF32};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T> + ColorSpace<f32>> ColorHsva<T, Space> {
    /// Convert this [`ColorHsva`] to [`f32`]
    pub const fn into_f32(self) -> ColorHsva<f32, Space>
    where
        T: [const] Destruct + [const] IntoF32,
    {
        self.map_channels(T::into_normalized_f32)
    }
}

const impl<Space: ColorSpace<u8> + ColorSpace<f32>> From<ColorHsva<u8, Space>>
    for ColorHsva<f32, Space>
{
    fn from(color: ColorHsva<u8, Space>) -> Self {
        color.into_f32()
    }
}
