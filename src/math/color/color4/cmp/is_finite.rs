use crate::math::{Color4, ColorSpace, number::IsFinite};

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Does this color contain only finite components?
    pub const fn is_finite(&self) -> bool
    where
        T: [const] IsFinite,
    {
        self.r.is_finite() && self.g.is_finite() && self.b.is_finite() && self.a.is_finite()
    }
}

const impl<T: [const] IsFinite, Space: ColorSpace<T>> IsFinite for Color4<T, Space> {
    fn is_finite(&self) -> bool {
        self.is_finite()
    }
}
