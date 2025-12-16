use crate::graphics::color::{Color3, ColorSpace, Linear};

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

/// Convert an individual color `value` from sRGB to linear
fn component_into_linear<T>(value: T) -> T {
    todo!()
}

/// Convert an individual color `value` from linear to sRGB
fn component_from_linear<T>(value: T) -> T {
    todo!()
}

impl<T> ColorSpace<T> for Srgb {
    fn from_linear(color: Color3<T, Linear>) -> Color3<T, Self> {
        Color3::new(
            component_from_linear(color.r),
            component_from_linear(color.g),
            component_from_linear(color.b),
        )
    }

    fn into_linear(color: Color3<T, Self>) -> Color3<T, Linear> {
        Color3::new(
            component_into_linear(color.r),
            component_into_linear(color.g),
            component_into_linear(color.b),
        )
    }
}
