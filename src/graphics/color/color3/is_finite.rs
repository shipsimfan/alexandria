use crate::{
    graphics::color::{Color3, ColorSpace},
    math::number::IsFinite,
};

impl<T, Space: ColorSpace<T>> Color3<T, Space> {
    /// Does this color contain only finite components?
    pub const fn is_finite(&self) -> bool
    where
        T: [const] IsFinite,
    {
        self.r.is_finite() && self.g.is_finite() && self.b.is_finite()
    }
}
