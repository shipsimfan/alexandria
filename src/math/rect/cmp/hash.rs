use crate::math::Rect;
use std::hash::Hash;

impl<T: Hash> Hash for Rect<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.size.hash(state);
    }
}
