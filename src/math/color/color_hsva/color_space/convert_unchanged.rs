use crate::math::{ColorHsva, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> ColorHsva<T, Space> {
    /// Convert this color into a different color space without changing the values
    pub const unsafe fn convert_unchanged<Space2: ColorSpace<T>>(self) -> ColorHsva<T, Space2>
    where
        T: [const] Destruct,
    {
        ColorHsva::new(self.h, self.s, self.v, self.a)
    }
}
