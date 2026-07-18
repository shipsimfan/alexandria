use crate::math::Quaternion;

const impl<T: [const] PartialEq> PartialEq for Quaternion<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

const impl<T: [const] Eq> Eq for Quaternion<T> {}
