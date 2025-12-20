use crate::{Color4, ColorSpace};

/// Clamp an f32 before converting to a u8
const fn clamp_f32(value: f32) -> f32 {
    if value.is_nan() {
        return 0.0;
    }

    value.clamp(0.0, 1.0) * 255.0
}

impl<Space: ColorSpace<u8> + ColorSpace<f32>> Color4<f32, Space> {
    /// Convert this [`Color4`] from [`f32`] to [`u8`] rounding to the nearest integer
    pub const fn into_u8(self) -> Color4<u8, Space> {
        self.into_u8_round()
    }

    /// Convert this [`Color4`] from [`f32`] to [`u8`] rounding down to the nearest integer
    pub const fn into_u8_floor(self) -> Color4<u8, Space> {
        const fn floor_channel(channel: f32) -> u8 {
            clamp_f32(channel).floor() as u8
        }

        self.map_channels(floor_channel)
    }

    /// Convert this [`Color4`] from [`f32`] to [`u8`] rounding to the nearest integer
    pub const fn into_u8_round(self) -> Color4<u8, Space> {
        const fn round_channel(channel: f32) -> u8 {
            clamp_f32(channel).round() as u8
        }

        self.map_channels(round_channel)
    }

    /// Convert this [`Color4`] from [`f32`] to [`u8`] rounding up to the nearest integer
    pub const fn into_u8_ceil(self) -> Color4<u8, Space> {
        const fn ceil_channel(channel: f32) -> u8 {
            clamp_f32(channel).ceil() as u8
        }

        self.map_channels(ceil_channel)
    }
}

impl<Space: ColorSpace<u8> + ColorSpace<f32>> const From<Color4<f32, Space>> for Color4<u8, Space> {
    fn from(color: Color4<f32, Space>) -> Self {
        color.into_u8()
    }
}
