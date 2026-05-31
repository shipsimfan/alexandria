use crate::math::{ColorHsv, ColorHsva, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> ColorHsva<T, Space> {
    /// Create a new [`ColorHsv`] with this color's hue, saturation, and value
    pub const fn hsv(self) -> ColorHsv<T, Space>
    where
        T: [const] Destruct,
    {
        ColorHsv::new(self.h, self.s, self.v)
    }
}

impl<T: [const] Destruct, Space: ColorSpace<T>> const Into<ColorHsv<T, Space>>
    for ColorHsva<T, Space>
{
    fn into(self) -> ColorHsv<T, Space> {
        self.hsv()
    }
}
