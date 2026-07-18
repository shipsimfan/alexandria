use crate::math::{Color3, ColorSpace};

const impl<T: [const] PartialEq, Space: ColorSpace<T>> PartialEq for Color3<T, Space> {
    fn eq(&self, other: &Self) -> bool {
        self.r.eq(&other.r) && self.g.eq(&other.g) && self.b.eq(&other.b)
    }
}

const impl<T: [const] Eq, Space: ColorSpace<T>> Eq for Color3<T, Space> {}
