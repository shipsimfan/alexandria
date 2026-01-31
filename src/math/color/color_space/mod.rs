use crate::math::Color3;

mod linear;
mod srgb;

pub use linear::Linear;
pub use srgb::Srgb;

/// [`ColorSpace`] is a **zero-sized type marker** used to distinguish color representations at
/// compile time (e.g. linear-light vs sRGB-encoded).
pub const trait ColorSpace<T: Sized>: 'static + Sized {
    /// The name of the color space used in serializing and deserializing
    const NAME: &'static str;

    /// Convert a color in `Self` into the canonical linear-light representation
    fn into_linear(color: Color3<T, Self>) -> Color3<T, Linear>;

    /// Convert a canonical linear-light color into `Self`
    fn from_linear(color: Color3<T, Linear>) -> Color3<T, Self>;
}
