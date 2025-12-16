mod linear;
mod srgb;

pub use linear::Linear;
pub use srgb::Srgb;

/// [`ColorSpace`] is a **zero-sized type marker** used to distinguish color representations at
/// compile time (e.g. linear-light vs sRGB-encoded).
pub trait ColorSpace {}
