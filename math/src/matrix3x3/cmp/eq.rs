use crate::Matrix3x3;

impl<T: [const] PartialEq> const PartialEq for Matrix3x3<T> {
    fn eq(&self, other: &Self) -> bool {
        self.r0 == other.r0 && self.r1 == other.r1 && self.r2 == other.r2
    }
}

impl<T: [const] Eq> const Eq for Matrix3x3<T> {}
