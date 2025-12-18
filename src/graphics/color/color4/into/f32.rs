use crate::graphics::color::{Color4, ColorSpace};

impl<Space: ColorSpace<u8> + ColorSpace<f32>> Color4<u8, Space> {
    /// Convert this [`Color4`] from [`u8`] to [`f32`]
    pub const fn into_f32(self) -> Color4<f32, Space> {
        const fn convert_channel(channel: u8) -> f32 {
            channel as f32 / 255.0
        }

        self.map_channels(convert_channel)
    }
}

impl<Space: ColorSpace<u8> + ColorSpace<f32>> const From<Color4<u8, Space>> for Color4<f32, Space> {
    fn from(color: Color4<u8, Space>) -> Self {
        color.into_f32()
    }
}
