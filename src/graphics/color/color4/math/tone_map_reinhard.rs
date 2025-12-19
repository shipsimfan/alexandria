use crate::{
    graphics::color::{Color4, Linear},
    math::number::{FromF32, IntoF32},
};
use std::marker::Destruct;

impl<T> Color4<T, Linear> {
    /// Tone map this color using the Reinhard algorithm
    pub const fn tone_map_reinhard(self) -> Self
    where
        T: [const] Destruct + [const] Clone + [const] FromF32 + [const] IntoF32,
    {
        let (color, a) = self.rgb_a();
        color.tone_map_reinhard().with_alpha(a)
    }
}
