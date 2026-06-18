use crate::math::Rect;
use std::hash::Hash;

impl<P, S> Hash for Rect<P, S>
where
    P: Hash,
    S: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.size.hash(state);
    }
}
