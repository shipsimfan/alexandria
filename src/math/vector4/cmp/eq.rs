use crate::math::Vector4;

const impl<T: [const] PartialEq> PartialEq for Vector4<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

const impl<T: [const] Eq> Eq for Vector4<T> {}
