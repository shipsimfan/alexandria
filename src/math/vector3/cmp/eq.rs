use crate::math::Vector3;

const impl<T: [const] PartialEq> PartialEq for Vector3<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

const impl<T: [const] Eq> Eq for Vector3<T> {}
