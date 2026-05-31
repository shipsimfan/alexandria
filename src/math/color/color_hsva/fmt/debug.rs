use crate::math::{ColorHsva, ColorSpace};
use std::fmt::Debug;

impl<T: Debug, Space: ColorSpace<T>> std::fmt::Debug for ColorHsva<T, Space> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(ColorHsva))
            .field(stringify!(h), &self.h)
            .field(stringify!(s), &self.s)
            .field(stringify!(v), &self.v)
            .field(stringify!(a), &self.a)
            .finish()
    }
}
