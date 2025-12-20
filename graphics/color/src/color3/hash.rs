use crate::{Color3, ColorSpace};
use std::hash::Hash;

impl<T: Hash, Space: ColorSpace<T>> Hash for Color3<T, Space> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.r.hash(state);
        self.g.hash(state);
        self.b.hash(state);
    }
}
