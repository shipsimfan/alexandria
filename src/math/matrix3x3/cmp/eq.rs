use crate::math::Matrix3x3;

const impl<T: [const] PartialEq> PartialEq for Matrix3x3<T> {
    fn eq(&self, other: &Self) -> bool {
        self.r0 == other.r0 && self.r1 == other.r1 && self.r2 == other.r2
    }
}

const impl<T: [const] Eq> Eq for Matrix3x3<T> {}
