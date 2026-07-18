use crate::math::{Color4, ColorSpace, number::Max};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Set all channels to be at least `max`
    pub const fn max(self, max: T) -> Color4<T, Space>
    where
        T: [const] Max + [const] Clone + [const] Destruct,
    {
        self.max_c(Color4::new(max.clone(), max.clone(), max.clone(), max))
    }

    /// Set all channels to be at least `max` channel-wise
    pub const fn max_c(self, max: Color4<T, Space>) -> Color4<T, Space>
    where
        T: [const] Max + [const] Destruct,
    {
        Color4::new(
            self.r.max(max.r),
            self.g.max(max.g),
            self.b.max(max.b),
            self.a.max(max.a),
        )
    }
}

const impl<T: [const] Max + [const] Destruct, Space: ColorSpace<T>> Max for Color4<T, Space> {
    fn max(self, other: Self) -> Self {
        self.max_c(other)
    }
}
