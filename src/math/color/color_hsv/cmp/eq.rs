use crate::math::{ColorHsv, ColorSpace};

impl<T: [const] PartialEq, Space: ColorSpace<T>> const PartialEq for ColorHsv<T, Space> {
    fn eq(&self, other: &Self) -> bool {
        self.h.eq(&other.h) && self.s.eq(&other.s) && self.v.eq(&other.v)
    }
}

impl<T: [const] Eq, Space: ColorSpace<T>> const Eq for ColorHsv<T, Space> {}
