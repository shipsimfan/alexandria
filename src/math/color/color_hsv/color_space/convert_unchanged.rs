use crate::math::{ColorHsv, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> ColorHsv<T, Space> {
    /// Convert this color into a different color space without changing the values
    pub const unsafe fn convert_unchanged<Space2: ColorSpace<T>>(self) -> ColorHsv<T, Space2>
    where
        T: [const] Destruct,
    {
        ColorHsv::new(self.h, self.s, self.v)
    }
}
