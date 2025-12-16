use crate::graphics::color::{Color4, ColorSpace};

impl<Space: ColorSpace<u8> + ColorSpace<f32> + ?Sized> Color4<f32, Space> {
    /// Convert this [`Color4`] from [`f32`] to [`u8`] rounding to the nearest integer
    pub fn into_u8(self) -> Color4<u8, Space> {
        self.into_u8_round()
    }

    /// Convert this [`Color4`] from [`f32`] to [`u8`] rounding down to the nearest integer
    pub fn into_u8_floor(self) -> Color4<u8, Space> {
        self.map_channels(|channel| (channel * 255.0).floor() as u8)
    }

    /// Convert this [`Color4`] from [`f32`] to [`u8`] rounding to the nearest integer
    pub fn into_u8_round(self) -> Color4<u8, Space> {
        self.map_channels(|channel| (channel * 255.0).round() as u8)
    }

    /// Convert this [`Color4`] from [`f32`] to [`u8`] rounding up to the nearest integer
    pub fn into_u8_ceil(self) -> Color4<u8, Space> {
        self.map_channels(|channel| (channel * 255.0).ceil() as u8)
    }
}

impl<Space: ColorSpace<u8> + ColorSpace<f32> + ?Sized> Into<Color4<u8, Space>>
    for Color4<f32, Space>
{
    fn into(self) -> Color4<u8, Space> {
        self.into_u8()
    }
}
