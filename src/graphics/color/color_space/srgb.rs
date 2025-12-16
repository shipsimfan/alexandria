use crate::graphics::color::ColorSpace;

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

impl ColorSpace for Srgb {}
