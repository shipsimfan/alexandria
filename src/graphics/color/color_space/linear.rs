use crate::graphics::color::ColorSpace;

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
/// Textures and UI colors are often authored/stored as [`Srgb`](crate::graphics::color::Srgb)
/// bytes. Convert to [`Linear`] before doing math like lerp, filtering, or blending.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Linear;

impl ColorSpace for Linear {}
