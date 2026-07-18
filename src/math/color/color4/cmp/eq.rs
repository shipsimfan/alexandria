use crate::math::{Color4, ColorSpace};

const impl<T: [const] PartialEq, Space: ColorSpace<T>> PartialEq for Color4<T, Space> {
    fn eq(&self, other: &Self) -> bool {
        self.r.eq(&other.r) && self.g.eq(&other.g) && self.b.eq(&other.b) && self.a.eq(&other.a)
    }
}

const impl<T: [const] Eq, Space: ColorSpace<T>> Eq for Color4<T, Space> {}
