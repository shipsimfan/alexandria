use crate::math::{ColorHsva, ColorSpace};

const impl<T: [const] PartialEq, Space: ColorSpace<T>> PartialEq for ColorHsva<T, Space> {
    fn eq(&self, other: &Self) -> bool {
        self.h.eq(&other.h) && self.s.eq(&other.s) && self.v.eq(&other.v) && self.a.eq(&other.a)
    }
}

const impl<T: [const] Eq, Space: ColorSpace<T>> Eq for ColorHsva<T, Space> {}
