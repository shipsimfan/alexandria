use crate::{
    graphics::color::{Color3, Linear},
    math::number::{FromF32, IntoF32},
};
use std::marker::Destruct;

impl<T: Destruct + FromF32 + IntoF32> Color3<T, Linear> {
    /// Adjust the exposure of this color by `ev`
    pub fn exposure(self, ev: f32) -> Self {
        Color3::from_f32(self.into_f32() * 2.0f32.powf(ev))
    }
}
