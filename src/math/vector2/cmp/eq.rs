use crate::math::Vector2;

impl<T: [const] PartialEq> const PartialEq for Vector2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: [const] Eq> const Eq for Vector2<T> {}
