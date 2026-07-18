use crate::math::{Color4, ColorSpace, number::Min};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Set all channels to be at most `min`
    pub const fn min(self, min: T) -> Color4<T, Space>
    where
        T: [const] Min + [const] Clone + [const] Destruct,
    {
        self.min_c(Color4::new(min.clone(), min.clone(), min.clone(), min))
    }

    /// Set all channels to be at most `min` channel-wise
    pub const fn min_c(self, min: Color4<T, Space>) -> Color4<T, Space>
    where
        T: [const] Min + [const] Destruct,
    {
        Color4::new(
            self.r.min(min.r),
            self.g.min(min.g),
            self.b.min(min.b),
            self.a.min(min.a),
        )
    }
}

const impl<T: [const] Min + [const] Destruct, Space: ColorSpace<T>> Min for Color4<T, Space> {
    fn min(self, other: Self) -> Self {
        self.min_c(other)
    }
}
