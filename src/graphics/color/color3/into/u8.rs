use crate::graphics::color::{Color3, ColorSpace};

/// Clamp an f32 before converting to a u8
fn clamp_f32(value: f32) -> f32 {
    if value.is_nan() {
        return 0.0;
    }

    value.clamp(0.0, 1.0)
}

impl<Space: ColorSpace<u8> + ColorSpace<f32>> Color3<f32, Space> {
    /// Convert this [`Color3`] from [`f32`] to [`u8`] rounding to the nearest integer
    pub fn into_u8(self) -> Color3<u8, Space> {
        self.into_u8_round()
    }

    /// Convert this [`Color3`] from [`f32`] to [`u8`] rounding down to the nearest integer
    pub fn into_u8_floor(self) -> Color3<u8, Space> {
        self.map_channels(|channel| (clamp_f32(channel) * 255.0).floor() as u8)
    }

    /// Convert this [`Color3`] from [`f32`] to [`u8`] rounding to the nearest integer
    pub fn into_u8_round(self) -> Color3<u8, Space> {
        self.map_channels(|channel| (clamp_f32(channel) * 255.0).round() as u8)
    }

    /// Convert this [`Color3`] from [`f32`] to [`u8`] rounding up to the nearest integer
    pub fn into_u8_ceil(self) -> Color3<u8, Space> {
        self.map_channels(|channel| (clamp_f32(channel) * 255.0).ceil() as u8)
    }
}

impl<Space: ColorSpace<u8> + ColorSpace<f32>> From<Color3<f32, Space>> for Color3<u8, Space> {
    fn from(color: Color3<f32, Space>) -> Self {
        color.into_u8()
    }
}
