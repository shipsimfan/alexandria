use crate::math::{ColorHsv, ColorSpace};
use std::hash::Hash;

impl<T: Hash, Space: ColorSpace<T>> Hash for ColorHsv<T, Space> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.h.hash(state);
        self.s.hash(state);
        self.v.hash(state);
    }
}
