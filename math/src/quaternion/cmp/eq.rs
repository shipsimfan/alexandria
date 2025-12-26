use crate::Quaternion;

impl<T: [const] PartialEq> const PartialEq for Quaternion<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

impl<T: Eq> Eq for Quaternion<T> {}
