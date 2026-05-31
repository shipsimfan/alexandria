use crate::math::{ColorHsva, ColorSpace};

impl<T: [const] PartialEq, Space: ColorSpace<T>> const PartialEq for ColorHsva<T, Space> {
    fn eq(&self, other: &Self) -> bool {
        self.h.eq(&other.h) && self.s.eq(&other.s) && self.v.eq(&other.v) && self.a.eq(&other.a)
    }
}

impl<T: [const] Eq, Space: ColorSpace<T>> const Eq for ColorHsva<T, Space> {}
