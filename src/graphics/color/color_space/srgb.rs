use crate::{
    graphics::color::{Color3, ColorSpace, Linear},
    math::number::{FromF32, One, Pow},
};
use std::ops::{Add, Div, Mul, Sub};

/// sRGB-encoded RGB (standard display transfer function)
///
/// # Meaning
/// [`Srgb`] indicates that RGB channels are **non-linear, display-referred** values following the
/// sRGB transfer function.
///
/// You typically use [`Srgb`] for:
/// - UI/theme colors
/// - Authoring-friendly “what you see” color pickers
/// - Interop with sRGB textures or 8-bit RGBA assets
///
/// # Important
/// Many operations (blending, lerp, lighting) should **not** be performed in sRGB space. Convert
/// to [`Linear`] first, operate, then convert back if needed.
///
/// # Alpha
/// Alpha is commonly stored alongside sRGB RGB in asset formats, but alpha itself is not
/// “sRGB-encoded”; it’s still a linear coverage value.
pub struct Srgb;

/// Convert an individual color `channel` from sRGB to linear
fn channel_into_linear<T: Add<Output = T> + Div<Output = T> + Pow + FromF32 + PartialOrd>(
    channel: T,
) -> T {
    if channel < T::from_f32(0.04045) {
        channel / T::from_f32(12.92)
    } else {
        ((channel + T::from_f32(0.055)) / T::from_f32(1.055)).pow(T::from_f32(2.4))
    }
}

/// Convert an individual color `channel` from linear to sRGB
fn channel_from_linear<
    T: Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Pow + FromF32 + PartialOrd + One,
>(
    channel: T,
) -> T {
    if channel < T::from_f32(0.0031308) {
        channel * T::from_f32(12.92)
    } else {
        T::from_f32(1.055) * (channel.pow(T::ONE / T::from_f32(2.4))) - T::from_f32(0.055)
    }
}

impl<
        T: Sized
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Pow
            + FromF32
            + PartialOrd
            + One,
    > ColorSpace<T> for Srgb
{
    fn from_linear(color: Color3<T, Linear>) -> Color3<T, Self> {
        Color3::new(
            channel_from_linear(color.r),
            channel_from_linear(color.g),
            channel_from_linear(color.b),
        )
    }

    fn into_linear(color: Color3<T, Self>) -> Color3<T, Linear> {
        Color3::new(
            channel_into_linear(color.r),
            channel_into_linear(color.g),
            channel_into_linear(color.b),
        )
    }
}
