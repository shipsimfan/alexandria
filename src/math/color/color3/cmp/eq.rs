use crate::math::{Color3, ColorSpace};

impl<T: [const] PartialEq, Space: ColorSpace<T>> const PartialEq for Color3<T, Space> {
    fn eq(&self, other: &Self) -> bool {
        self.r.eq(&other.r) && self.g.eq(&other.g) && self.b.eq(&other.b)
    }
}

impl<T: [const] Eq, Space: ColorSpace<T>> const Eq for Color3<T, Space> {}
