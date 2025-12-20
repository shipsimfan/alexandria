use crate::{Color4, ColorSpace};
use alexandria_math::number::FromF32;

impl<T, Space: ColorSpace<T> + ColorSpace<f32>> Color4<T, Space> {
    /// Convert a color in [`f32`] into a specific typed color
    pub const fn from_f32(color: Color4<f32, Space>) -> Self
    where
        T: [const] FromF32,
    {
        Color4::new(
            T::from_normalized_f32(color.r),
            T::from_normalized_f32(color.g),
            T::from_normalized_f32(color.b),
            T::from_normalized_f32(color.a),
        )
    }
}
