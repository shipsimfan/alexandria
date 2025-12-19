use crate::graphics::color::{Color3, ColorSpace};
use std::fmt::Debug;

impl<T: Debug, Space: ColorSpace<T>> std::fmt::Debug for Color3<T, Space> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(Color3))
            .field(stringify!(r), &self.r)
            .field(stringify!(g), &self.g)
            .field(stringify!(b), &self.b)
            .finish()
    }
}
