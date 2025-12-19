use crate::{
    graphics::color::{Color3, Linear},
    math::number::{FromF32, IntoF32},
};
use std::marker::Destruct;

impl<T> Color3<T, Linear> {
    /// Tone map this color using the Reinhard algorithm
    pub const fn tone_map_reinhard(self) -> Self
    where
        T: [const] Destruct + [const] Clone + [const] FromF32 + [const] IntoF32,
    {
        let luminance = self.clone().luminance_rec709();
        Color3::from_f32(self.into_f32() / (1.0 + luminance))
    }
}
