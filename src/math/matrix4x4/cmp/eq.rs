use crate::math::Matrix4x4;

const impl<T: [const] PartialEq> PartialEq for Matrix4x4<T> {
    fn eq(&self, other: &Self) -> bool {
        self.r0 == other.r0 && self.r1 == other.r1 && self.r2 == other.r2 && self.r3 == other.r3
    }
}

const impl<T: [const] Eq> Eq for Matrix4x4<T> {}
