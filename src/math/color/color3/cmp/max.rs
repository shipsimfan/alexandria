use crate::math::{Color3, ColorSpace, number::Max};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color3<T, Space> {
    /// Set all channels to be at least `max`
    pub const fn max(self, max: T) -> Color3<T, Space>
    where
        T: [const] Max + [const] Clone + [const] Destruct,
    {
        self.max_c(Color3::new(max.clone(), max.clone(), max))
    }

    /// Set all channels to be at least `max` channel-wise
    pub const fn max_c(self, max: Color3<T, Space>) -> Color3<T, Space>
    where
        T: [const] Max + [const] Destruct,
    {
        Color3::new(self.r.max(max.r), self.g.max(max.g), self.b.max(max.b))
    }
}

const impl<T: [const] Max + [const] Destruct, Space: ColorSpace<T>> Max for Color3<T, Space> {
    fn max(self, other: Self) -> Self {
        self.max_c(other)
    }
}
