use crate::math::{ColorHsv, ColorSpace};

const impl<T: [const] PartialEq, Space: ColorSpace<T>> PartialEq for ColorHsv<T, Space> {
    fn eq(&self, other: &Self) -> bool {
        self.h.eq(&other.h) && self.s.eq(&other.s) && self.v.eq(&other.v)
    }
}

const impl<T: [const] Eq, Space: ColorSpace<T>> Eq for ColorHsv<T, Space> {}
