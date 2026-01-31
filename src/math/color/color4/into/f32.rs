use crate::math::{Color4, ColorSpace, number::IntoF32};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T> + ColorSpace<f32>> Color4<T, Space> {
    /// Convert this [`Color4`] from [`u8`] to [`f32`]
    pub const fn into_f32(self) -> Color4<f32, Space>
    where
        T: [const] Destruct + [const] IntoF32,
    {
        self.map_channels(T::into_normalized_f32)
    }
}

impl<Space: ColorSpace<u8> + ColorSpace<f32>> const From<Color4<u8, Space>> for Color4<f32, Space> {
    fn from(color: Color4<u8, Space>) -> Self {
        color.into_f32()
    }
}
