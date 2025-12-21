use crate::{Color4, ColorSpace};

impl<T: [const] PartialEq, Space: ColorSpace<T>> const PartialEq for Color4<T, Space> {
    fn eq(&self, other: &Self) -> bool {
        self.r.eq(&other.r) && self.g.eq(&other.g) && self.b.eq(&other.b) && self.a.eq(&other.a)
    }
}

impl<T: Eq, Space: ColorSpace<T>> Eq for Color4<T, Space> {}
