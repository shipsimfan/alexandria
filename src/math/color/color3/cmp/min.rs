use crate::math::{Color3, ColorSpace, number::Min};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color3<T, Space> {
    /// Set all channels to be at most `min`
    pub const fn min(self, min: T) -> Color3<T, Space>
    where
        T: [const] Min + [const] Clone + [const] Destruct,
    {
        self.min_c(Color3::new(min.clone(), min.clone(), min))
    }

    /// Set all channels to be at most `min` channel-wise
    pub const fn min_c(self, min: Color3<T, Space>) -> Color3<T, Space>
    where
        T: [const] Min + [const] Destruct,
    {
        Color3::new(self.r.min(min.r), self.g.min(min.g), self.b.min(min.b))
    }
}

impl<T: [const] Min + [const] Destruct, Space: ColorSpace<T>> const Min for Color3<T, Space> {
    fn min(self, other: Self) -> Self {
        self.min_c(other)
    }
}
