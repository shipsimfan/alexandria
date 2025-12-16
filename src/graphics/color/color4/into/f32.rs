use crate::graphics::color::{Color4, ColorSpace};

impl<Space: ColorSpace<u8> + ColorSpace<f32>> Color4<u8, Space> {
    /// Convert this [`Color4`] from [`u8`] to [`f32`]
    pub fn into_f32(self) -> Color4<f32, Space> {
        self.map_channels(|channel| channel as f32 / 255.0)
    }
}

impl<Space: ColorSpace<u8> + ColorSpace<f32>> Into<Color4<f32, Space>>
    for Color4<u8, Space>
{
    fn into(self) -> Color4<f32, Space> {
        self.into_f32()
    }
}
