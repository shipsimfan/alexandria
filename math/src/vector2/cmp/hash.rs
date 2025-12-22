use crate::Vector2;
use std::hash::Hash;

impl<T: Hash> Hash for Vector2<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
