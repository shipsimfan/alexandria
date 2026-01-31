use crate::math::{
    Color4, Linear, Srgb,
    number::{FromF32, IntoF32},
};

impl<T: FromF32 + IntoF32> Color4<T, Linear> {
    /// Convert this color from [`Linear`] color space into [`Srgb`] color space
    pub fn into_srgb(self) -> Color4<T, Srgb> {
        let (color, a) = self.rgb_a();
        color.into_srgb().with_alpha(a)
    }
}
