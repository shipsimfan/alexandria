use crate::{Color3, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color3<T, Space> {
    /// Set all channels to be at most `min`
    pub const fn min(self, min: T) -> Self
    where
        T: [const] Ord + [const] Clone + [const] Destruct,
    {
        Color3::new(
            std::cmp::min(self.r, min.clone()),
            std::cmp::min(self.g, min.clone()),
            std::cmp::min(self.b, min),
        )
    }

    /// Set all channels to be at most `min` channel-wise
    pub const fn min_v(self, min: Self) -> Self
    where
        T: [const] Ord + [const] Destruct,
    {
        Color3::new(
            std::cmp::min(self.r, min.r),
            std::cmp::min(self.g, min.g),
            std::cmp::min(self.b, min.b),
        )
    }
}
