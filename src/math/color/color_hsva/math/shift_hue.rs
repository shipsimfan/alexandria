use crate::math::{
    ColorHsva, ColorSpace,
    number::{NormalizedWrappingAdd, NormalizedWrappingSub},
};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> ColorHsva<T, Space> {
    /// Shift the hue of this color by the given amount, wrapping around at normalized 1
    ///
    /// The saturation and value channels are unchanged.
    pub const fn shift_hue(self, amount: T) -> ColorHsva<T, Space>
    where
        T: [const] NormalizedWrappingAdd + [const] Destruct,
    {
        self.add_hue(amount)
    }

    /// Shift the hue of this color by the given amount, wrapping around at normalized 1
    ///
    /// The saturation and value channels are unchanged.
    pub const fn add_hue(mut self, amount: T) -> ColorHsva<T, Space>
    where
        T: [const] NormalizedWrappingAdd + [const] Destruct,
    {
        self.h = self.h.normalized_wrapping_add(amount);
        self
    }

    /// Shift the hue of this color by the given negative amount, wrapping around at normalized 1
    ///
    /// The saturation and value channels are unchanged.
    pub const fn sub_hue(mut self, amount: T) -> ColorHsva<T, Space>
    where
        T: [const] NormalizedWrappingSub + [const] Destruct,
    {
        self.h = self.h.normalized_wrapping_sub(amount);
        self
    }
}
