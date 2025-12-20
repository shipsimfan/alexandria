use crate::{Color3, ColorSpace};
use alexandria_math::number::IntoF32;
use std::marker::Destruct;

impl<T, Space: ColorSpace<T> + ColorSpace<f32>> Color3<T, Space> {
    /// Convert this [`Color3`] to [`f32`]
    pub const fn into_f32(self) -> Color3<f32, Space>
    where
        T: [const] Destruct + [const] IntoF32,
    {
        self.map_channels(T::into_normalized_f32)
    }
}

impl<Space: ColorSpace<u8> + ColorSpace<f32>> const From<Color3<u8, Space>> for Color3<f32, Space> {
    fn from(color: Color3<u8, Space>) -> Self {
        color.into_f32()
    }
}
