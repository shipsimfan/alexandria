use crate::graphics::color::{Color4, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Set all channels to be at most `min`
    pub const fn min(self, min: T) -> Self
    where
        T: [const] Ord + [const] Clone + [const] Destruct,
    {
        Color4::new(
            std::cmp::min(self.r, min.clone()),
            std::cmp::min(self.g, min.clone()),
            std::cmp::min(self.b, min.clone()),
            std::cmp::min(self.a, min),
        )
    }
}
