use crate::{Color4, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Set all channels to be at least `max`
    pub const fn max(self, max: T) -> Self
    where
        T: [const] Ord + [const] Clone + [const] Destruct,
    {
        Color4::new(
            std::cmp::max(self.r, max.clone()),
            std::cmp::max(self.g, max.clone()),
            std::cmp::max(self.b, max.clone()),
            std::cmp::max(self.a, max),
        )
    }
}
