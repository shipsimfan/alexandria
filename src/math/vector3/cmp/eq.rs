use crate::math::Vector3;

impl<T: [const] PartialEq> const PartialEq for Vector3<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<T: [const] Eq> const Eq for Vector3<T> {}
