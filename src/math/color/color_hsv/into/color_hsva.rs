use crate::math::{ColorHsv, ColorHsva, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> ColorHsv<T, Space> {
    /// Create a new [`ColorHsva`] with this color and an alpha of `a`
    pub const fn with_alpha(self, a: T) -> ColorHsva<T, Space>
    where
        T: [const] Destruct,
    {
        ColorHsva::new(self.h, self.s, self.v, a)
    }
}
