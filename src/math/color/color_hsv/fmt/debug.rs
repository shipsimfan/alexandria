use crate::math::{ColorHsv, ColorSpace};
use std::fmt::Debug;

impl<T: Debug, Space: ColorSpace<T>> std::fmt::Debug for ColorHsv<T, Space> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(ColorHsv))
            .field(stringify!(h), &self.h)
            .field(stringify!(s), &self.s)
            .field(stringify!(v), &self.v)
            .finish()
    }
}
