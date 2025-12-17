use crate::{
    graphics::color::{Color4, Linear},
    math::number::{FromF32, IntoF32},
};

impl<T: FromF32 + IntoF32> Color4<T, Linear> {
    /// Linear interpolate between this color and another color, using a parameter
    pub fn lerp(self, b: Self, t: f32) -> Self {
        Color4::new(
            T::from_f32((1.0 - t) * self.r.into_f32() + t * b.r.into_f32()),
            T::from_f32((1.0 - t) * self.g.into_f32() + t * b.g.into_f32()),
            T::from_f32((1.0 - t) * self.b.into_f32() + t * b.b.into_f32()),
            T::from_f32((1.0 - t) * self.a.into_f32() + t * b.a.into_f32()),
        )
    }
}
