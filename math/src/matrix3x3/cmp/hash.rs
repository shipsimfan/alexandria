use crate::Matrix3x3;
use std::hash::Hash;

impl<T: Hash> Hash for Matrix3x3<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.r0.hash(state);
        self.r1.hash(state);
        self.r2.hash(state);
    }
}
