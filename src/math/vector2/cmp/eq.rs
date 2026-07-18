use crate::math::Vector2;

const impl<T: [const] PartialEq> PartialEq for Vector2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

const impl<T: [const] Eq> Eq for Vector2<T> {}
