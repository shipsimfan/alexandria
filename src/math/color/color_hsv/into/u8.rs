use crate::math::{ColorHsv, ColorSpace};

/// Clamp an f32 before converting to a u8
const fn clamp_f32(value: f32) -> f32 {
    if value.is_nan() {
        return 0.0;
    }

    value.clamp(0.0, 1.0) * 255.0
}

impl<Space: ColorSpace<u8> + ColorSpace<f32>> ColorHsv<f32, Space> {
    /// Convert this [`ColorHsv`] from [`f32`] to [`u8`] rounding to the nearest integer
    pub const fn into_u8(self) -> ColorHsv<u8, Space> {
        self.into_u8_round()
    }

    /// Convert this [`ColorHsv`] from [`f32`] to [`u8`] rounding down to the nearest integer
    pub const fn into_u8_floor(self) -> ColorHsv<u8, Space> {
        const fn floor_channel(channel: f32) -> u8 {
            clamp_f32(channel).floor() as u8
        }

        self.map_channels(floor_channel)
    }

    /// Convert this [`ColorHsv`] from [`f32`] to [`u8`] rounding to the nearest integer
    pub const fn into_u8_round(self) -> ColorHsv<u8, Space> {
        const fn round_channel(channel: f32) -> u8 {
            clamp_f32(channel).round() as u8
        }

        self.map_channels(round_channel)
    }

    /// Convert this [`ColorHsv`] from [`f32`] to [`u8`] rounding up to the nearest integer
    pub const fn into_u8_ceil(self) -> ColorHsv<u8, Space> {
        const fn ceil_channel(channel: f32) -> u8 {
            clamp_f32(channel).ceil() as u8
        }

        self.map_channels(ceil_channel)
    }
}

const impl<Space: ColorSpace<u8> + ColorSpace<f32>> From<ColorHsv<f32, Space>>
    for ColorHsv<u8, Space>
{
    fn from(color: ColorHsv<f32, Space>) -> Self {
        color.into_u8()
    }
}
