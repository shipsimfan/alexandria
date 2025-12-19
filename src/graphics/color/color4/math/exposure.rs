use crate::{
    graphics::color::{Color4, Linear},
    math::number::{FromF32, IntoF32},
};
use std::marker::Destruct;

impl<T: Destruct + FromF32 + IntoF32> Color4<T, Linear> {
    /// Adjust the exposure of this color by `ev`
    pub fn exposure(self, ev: f32) -> Self {
        let (color, a) = self.rgb_a();
        color.exposure(ev).with_alpha(a)
    }
}
