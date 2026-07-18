use crate::math::{ColorHsv, ColorSpace, number::IsFinite};

impl<T, Space: ColorSpace<T>> ColorHsv<T, Space> {
    /// Does this color contain only finite components?
    pub const fn is_finite(&self) -> bool
    where
        T: [const] IsFinite,
    {
        self.h.is_finite() && self.s.is_finite() && self.v.is_finite()
    }
}

const impl<T: [const] IsFinite, Space: ColorSpace<T>> IsFinite for ColorHsv<T, Space> {
    fn is_finite(&self) -> bool {
        self.is_finite()
    }
}
