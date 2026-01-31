use crate::math::{Color4, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Set all channels to be at most `min`
    pub const fn min(self, min: T) -> Color4<T, Space>
    where
        T: [const] Ord + [const] Clone + [const] Destruct,
    {
        self.min_c(Color4::new(min.clone(), min.clone(), min.clone(), min))
    }

    /// Set all channels to be at most `min` channel-wise
    pub const fn min_c(self, min: Color4<T, Space>) -> Color4<T, Space>
    where
        T: [const] Ord + [const] Destruct,
    {
        Color4::new(
            std::cmp::min(self.r, min.r),
            std::cmp::min(self.g, min.g),
            std::cmp::min(self.b, min.b),
            std::cmp::min(self.a, min.a),
        )
    }
}
