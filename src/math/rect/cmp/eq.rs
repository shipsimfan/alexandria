use crate::math::Rect;

impl<T: [const] PartialEq> const PartialEq for Rect<T> {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.size == other.size
    }
}

impl<T: [const] Eq> const Eq for Rect<T> {}
