use crate::math::{Color3, ColorSpace};

/// Linear-light RGB (no transfer function).
///
/// # Meaning
/// [`Linear`] indicates that RGB channels represent linear light intensity. This is the space you
/// generally want for:
/// - Lighting computations
/// - Blending/compositing
/// - Physically-based shading inputs/outputs (prior to display encoding)
///
/// # Interop
/// Textures and UI colors are often authored/stored as [`Srgb`](crate::Srgb)
/// bytes. Convert to [`Linear`] before doing math like lerp, filtering, or blending.
pub struct Linear;

impl<T: Sized> const ColorSpace<T> for Linear {
    const NAME: &'static str = "Linear";

    fn from_linear(color: Color3<T, Linear>) -> Color3<T, Self> {
        color
    }

    fn into_linear(color: Color3<T, Self>) -> Color3<T, Linear> {
        color
    }
}
