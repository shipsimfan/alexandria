use crate::graphics::color::{Color3, ColorSpace};

impl<Space: ColorSpace<u8> + ColorSpace<f32>> Color3<u8, Space> {
    /// Convert this [`Color3`] from [`u8`] to [`f32`]
    pub fn into_f32(self) -> Color3<f32, Space> {
        self.map_channels(|channel| channel as f32 / 255.0)
    }
}

impl<Space: ColorSpace<u8> + ColorSpace<f32>> From<Color3<u8, Space>> for Color3<f32, Space> {
    fn from(color: Color3<u8, Space>) -> Self {
        color.into_f32()
    }
}
