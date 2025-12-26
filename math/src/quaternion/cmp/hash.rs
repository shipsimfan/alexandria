use crate::Quaternion;
use std::hash::Hash;

impl<T: Hash> Hash for Quaternion<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
        self.w.hash(state);
    }
}
