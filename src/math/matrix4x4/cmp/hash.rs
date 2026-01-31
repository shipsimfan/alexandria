use crate::math::Matrix4x4;
use std::hash::Hash;

impl<T: Hash> Hash for Matrix4x4<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.r0.hash(state);
        self.r1.hash(state);
        self.r2.hash(state);
        self.r3.hash(state);
    }
}
