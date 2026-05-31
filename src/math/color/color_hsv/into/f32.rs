use crate::math::{ColorHsv, ColorSpace, number::IntoF32};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T> + ColorSpace<f32>> ColorHsv<T, Space> {
    /// Convert this [`ColorHsv`] to [`f32`]
    pub const fn into_f32(self) -> ColorHsv<f32, Space>
    where
        T: [const] Destruct + [const] IntoF32,
    {
        self.map_channels(T::into_normalized_f32)
    }
}

impl<Space: ColorSpace<u8> + ColorSpace<f32>> const From<ColorHsv<u8, Space>>
    for ColorHsv<f32, Space>
{
    fn from(color: ColorHsv<u8, Space>) -> Self {
        color.into_f32()
    }
}
