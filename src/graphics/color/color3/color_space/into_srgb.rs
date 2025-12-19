use crate::{
    graphics::color::{Color3, ColorSpace, Linear, Srgb},
    math::number::{FromF32, IntoF32},
};

impl<T: FromF32 + IntoF32> Color3<T, Linear> {
    /// Convert this color from [`Linear`] color space into [`Srgb`] color space
    pub fn into_srgb(self) -> Color3<T, Srgb> {
        Srgb::from_linear(self)
    }
}
