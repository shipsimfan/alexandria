use crate::math::{Color4, ColorSpace};
use std::hash::Hash;

impl<T: Hash, Space: ColorSpace<T>> Hash for Color4<T, Space> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.r.hash(state);
        self.g.hash(state);
        self.b.hash(state);
        self.a.hash(state);
    }
}
