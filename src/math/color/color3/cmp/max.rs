use crate::math::{Color3, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color3<T, Space> {
    /// Set all channels to be at least `max`
    pub const fn max(self, max: T) -> Color3<T, Space>
    where
        T: [const] Ord + [const] Clone + [const] Destruct,
    {
        self.max_c(Color3::new(max.clone(), max.clone(), max))
    }

    /// Set all channels to be at least `max` channel-wise
    pub const fn max_c(self, max: Color3<T, Space>) -> Color3<T, Space>
    where
        T: [const] Ord + [const] Destruct,
    {
        Color3::new(
            std::cmp::max(self.r, max.r),
            std::cmp::max(self.g, max.g),
            std::cmp::max(self.b, max.b),
        )
    }
}
